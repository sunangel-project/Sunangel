#[tokio::main]
pub async fn main() {
    run().await
}

use std::str::{self, FromStr};

use anyhow::anyhow;
use async_nats::jetstream::{Context, Message};
use futures_util::StreamExt;
use log::{error, info};
use messages_common::try_get_request_id;
use serde::{Deserialize, Serialize};

use serde_json::{json, Value};
use spot_finder::location::Location;
use spot_finder::{find_spots, Spot};

#[derive(Debug, Serialize, Deserialize)]
struct InMessage {
    request_id: String,
    search_query: SearchQuery,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchQuery {
    loc: Location,
    rad: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PartMessage {
    id: usize,
    of: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorMessage {
    request_id: String,
    sender: String,
    reason: String,
    input: String,
}

const IN_STREAM: &str = "SEARCH";
const GROUP: &str = "spot-finder";

const OUT_STREAM: &str = "SPOTS";
const OUT_SUBJECT: &str = "get-horizon";
const ERR_STREAM: &str = "ERRORS";

async fn run() {
    env_logger::init();

    let jetstream = messages_common::connect_jetstream().await;

    messages_common::create_stream(&jetstream, OUT_STREAM).await;
    messages_common::create_stream(&jetstream, ERR_STREAM).await;

    let messages = messages_common::queue_subscribe(&jetstream, IN_STREAM, GROUP).await;

    info!("Listening to NATS for messages in queue '{IN_STREAM}'");

    messages
        .for_each_concurrent(16, |message| async {
            info!("Received message {:?}", message);

            match message {
                Ok(message) => {
                    let res = handle_message(&jetstream, &message).await;
                    if let Err(err) = res {
                        error!("Could not handle received message: {err}");
                        send_error_message(&jetstream, Some(message), err)
                            .await
                            .unwrap_or_else(|err| error!("Could not send error message: {err}"));
                    }
                }
                Err(err) => {
                    error!("Problem with received message: {err}");
                    send_error_message(&jetstream, None, err.into())
                        .await
                        .unwrap_or_else(|err| error!("Could not send out error message: {err}"));
                }
            }
        })
        .await;
}

// Event Loop
async fn handle_message(jetstream: &Context, message: &Message) -> Result<(), async_nats::Error> {
    let payload = str::from_utf8(&message.payload)?;

    let spots = handle_payload(payload).await?;
    let total_num = spots.len();

    if total_num == 0 {
        return Err(anyhow!("Could not find any spots in this area").into());
    }

    info!("Found {total_num} spots");

    let in_value = Value::from_str(payload)?;
    for (i, spot) in spots.into_iter().enumerate() {
        jetstream
            .publish(
                format!("{OUT_STREAM}.{OUT_SUBJECT}"),
                build_output_payload(spot, i, total_num, &in_value)?
                    .to_string()
                    .into(),
            )
            .await?;
    }

    message.ack().await.unwrap();

    Ok(())
}

async fn handle_payload(payload: &str) -> Result<Vec<Spot>, async_nats::Error> {
    let in_message: InMessage = serde_json::from_str(payload)?;
    let query = in_message.search_query;

    info!("Extraxted query {:?}, running spot finder", query);
    find_spots(&query.loc, query.rad).await
}

fn build_output_payload(
    spot: Spot,
    part_num: usize,
    total_num: usize,
    query_value: &Value,
) -> Result<Value, async_nats::Error> {
    let mut output = query_value.clone();
    let output_obj = output
        .as_object_mut()
        .ok_or(anyhow!("query was not an object: {query_value:?}"))?;

    output_obj.insert("spot".into(), json!(spot));
    output_obj.insert(
        "part".into(),
        json!(PartMessage {
            id: part_num,
            of: total_num,
        }),
    );

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

fn build_error_payload(msg: &Message, error: async_nats::Error) -> String {
    json!(ErrorMessage {
        request_id: try_get_request_id(&msg.payload).unwrap_or("UNKNOWN".to_string()),
        sender: GROUP.to_string(),
        reason: error.to_string(),
        input: format!("{msg:?}"),
    })
    .to_string()
}
