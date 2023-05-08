use anyhow::anyhow;
use async_nats::{
    jetstream::{kv::Store, Context, Message},
    Error,
};
use chrono::Utc;
use futures_util::Future;
use log::{info, warn};
use messages_common::MessageStream;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;
use std::{pin::Pin, str};

use crate::{sky::sun::Sun, Horizon, HorizonEvents, Location, SkyObject};

const IN_STREAM: &str = "HORIZONS";
const HORIZON_STORE: &str = "horizons";
const GROUP: &str = "sun-service";

const OUT_STREAM: &str = "SUNSETS";
const ERR_STREAM: &str = "ERRORS";

pub async fn setup_nats() -> (Context, Store) {
    info!("Setting up NATS");

    let client = messages_common::connect_nats().await;
    let jetstream = messages_common::connect_jetstream(client);

    messages_common::create_stream(&jetstream, OUT_STREAM).await;
    messages_common::create_stream(&jetstream, ERR_STREAM).await;

    let store = messages_common::connect_kv_store(&jetstream, HORIZON_STORE).await;

    (jetstream, store)
}

pub async fn messages(jetstream: &Context) -> MessageStream {
    messages_common::queue_subscribe(jetstream, IN_STREAM, GROUP).await
}

type HandleMessageFun<'a> =
    Box<dyn FnMut(Result<Message, Error>) -> Pin<Box<dyn Future<Output = ()> + 'a>> + 'a>;

pub fn generate_handle_message_res<'a>(
    jetstream: &'a Context,
    store: &'a Store,
) -> HandleMessageFun<'a> {
    Box::new(move |message| {
        Box::pin(async move {
            info!("Received message {:?}", message);

            match message {
                Ok(message) => handle_message(message, jetstream, store)
                    .await
                    .unwrap_or_else(|err| {
                        warn!("{err}");
                        todo!("send error message")
                    }),
                Err(_) => todo!("send error message"),
            };
        })
    })
}

#[derive(Serialize, Deserialize)]
struct Spot {
    loc: Location,
}

#[derive(Serialize, Deserialize)]
struct InMessage {
    horizon: String,
    spot: Spot,
}

pub async fn handle_message(
    message: Message,
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

    let sun = Sun::new();
    let time = Utc::now(); // TODO: get from message (need to add to message)
    let result = crate::calculate_rise_and_set(sun, &time, &decoded_message.spot.loc, &horizon)?;

    let in_value = Value::from_str(payload)?;
    jetstream
        .publish(
            OUT_STREAM.to_string(),
            build_output(in_value, result)?.to_string().into(),
        )
        .await?;

    message.ack().await?;

    Ok(())
}

fn build_output(in_value: Value, result: HorizonEvents) -> Result<Value, Error> {
    let mut output = in_value.clone();
    let output_obj = output.as_object_mut().ok_or(anyhow!(
        "in message was not an object, could not build output message"
    ))?;

    output_obj.insert("sunset".to_string(), json!(result));

    Ok(output)
}
