use anyhow::anyhow;
use async_nats::{
    jetstream::{consumer::pull::MessagesError, kv::Store, Context, Message},
    Error,
};
use chrono::{DateTime, NaiveDateTime, Timelike, Utc};
use chrono_tz::Tz;
use futures_util::Future;
use log::{error, info};
use messages_common::MessageStream;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;
use std::{pin::Pin, str};

use crate::{
    sky::{moon::Moon, sun::Sun},
    Horizon, HorizonEvents, Location,
};

const IN_STREAM: &str = "HORIZONS";
const HORIZON_STORE: &str = "horizons";
const GROUP: &str = "sun-service";

const OUT_STREAM: &str = "SUNSETS";
const ERR_STREAM: &str = "ERRORS";

pub async fn setup_nats() -> (Context, Store) {
    info!("Setting up NATS");

    let jetstream = messages_common::connect_jetstream().await;

    messages_common::create_stream(&jetstream, OUT_STREAM).await;
    messages_common::create_stream(&jetstream, ERR_STREAM).await;

    let store = messages_common::connect_kv_store(&jetstream, HORIZON_STORE).await;

    (jetstream, store)
}

pub async fn messages(jetstream: &Context) -> MessageStream {
    messages_common::queue_subscribe(jetstream, IN_STREAM, GROUP).await
}

type HandleMessageFun<'a> =
    Box<dyn FnMut(Result<Message, MessagesError>) -> Pin<Box<dyn Future<Output = ()> + 'a>> + 'a>;

pub fn generate_handle_message_res<'a>(
    jetstream: &'a Context,
    store: &'a Store,
) -> HandleMessageFun<'a> {
    Box::new(move |message| {
        Box::pin(async move {
            info!("Received message {:?}", message);

            match message {
                Ok(message) => {
                    let res = handle_message(&message, jetstream, store).await;
                    if let Err(err) = res {
                        error!("Could not handle received message: {err}");
                        send_error_message(jetstream, Some(message), err)
                            .await
                            .unwrap_or_else(|err| error!("Could not send error message: {err}"));
                    }
                }
                Err(err) => {
                    error!("Problem with received message: {err}");
                    send_error_message(jetstream, None, err.into())
                        .await
                        .unwrap_or_else(|err| error!("Could not send out error message: {err}"));
                }
            };
        })
    })
}

#[derive(Serialize, Deserialize)]
struct SearchQuery {
    time: DateTime<Utc>,
    timezone: Tz,
}

#[derive(Serialize, Deserialize)]
struct Spot {
    loc: Location,
}

#[derive(Serialize, Deserialize)]
struct InMessage {
    request_id: String,
    search_query: SearchQuery,
    horizon: String,
    spot: Spot,
}

#[derive(Serialize, Deserialize)]
struct OutEvents {
    sun: Option<HorizonEvents>,
    moon: Option<HorizonEvents>,
}

pub async fn handle_message(
    message: &Message,
    jetstream: &Context,
    store: &Store,
) -> Result<(), Error> {
    let payload = str::from_utf8(&message.payload)?;
    let decoded_message: InMessage = serde_json::from_str(payload)?;

    let horizon = store.get(&decoded_message.horizon).await?.ok_or(anyhow!(
        "Could not get a byte array for horizon '{}'",
        decoded_message.horizon
    ))?;
    let horizon: Horizon = horizon.try_into()?;
    info!(
        "Retreived and decoded horizon '{}'",
        decoded_message.horizon
    );

    let time = get_time(&decoded_message);
    let sun_events =
        crate::calculate_rise_and_set(&Sun, &time, &decoded_message.spot.loc, &horizon).ok();
    let moon_events =
        crate::calculate_rise_and_set(&Moon, &time, &decoded_message.spot.loc, &horizon).ok();

    let result = OutEvents {
        sun: sun_events,
        moon: moon_events,
    };

    let in_value = Value::from_str(payload)?;
    jetstream
        .publish(
            format!("{}.{}", OUT_STREAM, decoded_message.request_id),
            build_output(in_value, result)?.to_string().into(),
        )
        .await?;
    info!("sent out results");

    message.ack().await?;

    Ok(())
}

fn get_time(message: &InMessage) -> NaiveDateTime {
    let time = message.search_query.time;
    let time = time.with_timezone(&message.search_query.timezone);

    let hour = if time.hour() < 12 { 0 } else { 12 };
    let time = time.with_hour(hour).unwrap();
    let time = time.with_minute(0).unwrap();
    let time = time.with_second(0).unwrap();
    let time = time.with_nanosecond(0).unwrap();

    time.naive_utc()
}

fn build_output(in_value: Value, result: OutEvents) -> Result<Value, Error> {
    let mut output = in_value;
    let output_obj = output.as_object_mut().ok_or(anyhow!(
        "in message was not an object, could not build output message"
    ))?;

    output_obj.insert("events".to_string(), json!(result));

    Ok(output)
}

async fn send_error_message(
    jetstream: &Context,
    message: Option<Message>,
    error: async_nats::Error,
) -> Result<(), async_nats::Error> {
    let message = message.unwrap();

    jetstream
        .publish(
            format!("{ERR_STREAM}.{GROUP}"),
            build_error_payload(&message, error).to_string().into(),
        )
        .await
        .unwrap();

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorMessage {
    request_id: String,
    sender: String,
    reason: String,
    input: String,
}

fn build_error_payload(msg: &Message, error: async_nats::Error) -> String {
    json!(ErrorMessage {
        request_id: messages_common::try_get_request_id(&msg.payload)
            .unwrap_or("UNKNOWN".to_string()),
        sender: GROUP.to_string(),
        reason: error.to_string(),
        input: format!("{msg:?}"),
    })
    .to_string()
}
