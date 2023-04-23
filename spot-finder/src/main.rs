#[tokio::main]
pub async fn main() -> Result<(), async_nats::Error> {
    run().await
}

use std::str::{self, FromStr};

use anyhow::anyhow;
use async_nats::jetstream::{Context, Message};
use futures_util::{FutureExt, StreamExt};
use log::info;
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

const IN_Q: &str = "SEARCH";
const GROUP: &str = "spot-finder";

//const OUT_Q: &str = "spots";
//const ERR_Q: &str = "error";

async fn run() -> Result<(), async_nats::Error> {
    env_logger::init();

    let client = messages_common::connect_nats().await;
    let jetstream = messages_common::connect_jetstream(client);
    // let subscriber = client.queue_subscribe(IN_Q.into(), GROUP.into()).await?;

    let messages = messages_common::queue_subscribe(&jetstream, IN_Q, GROUP).await;

    info!("Listening to NATS for messages in queue '{IN_Q}'");

    messages
        .for_each_concurrent(16, |message| async {
            //if let Err(err) = message.map() {
            //    let reason = err.to_string();
            //    send_error_message(&jetstream, message, reason).await;
            //}
            info!("Received message {:?}", message);

            match message {
                Ok(message) => handle_message(&jetstream, message).await,
                Err(_err) => panic!("help"), // todo log error, send error message etc
            }
            .unwrap();

            ()
        })
        .await;

    Ok(())
}

// Event Loop
async fn handle_message(_jetstream: &Context, message: Message) -> Result<(), async_nats::Error> {
    let payload = str::from_utf8(&message.payload)?;

    let spots = handle_payload(payload).await?;
    let total_num = spots.len();

    if total_num == 0 {
        return Err(anyhow!("Could not find any spots in this area").into());
    }

    let _in_value = Value::from_str(payload)?;
    for (_i, _spot) in spots.into_iter().enumerate() {
        //    jetstream
        //        .publish(
        //            OUT_Q.to_string(),
        //            build_output_payload(spot, i, total_num, &in_value)?
        //                .to_string()
        //                .into(),
        //        )
        //        .await?;
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
    _jetstream: &Context,
    message: Result<Message, async_nats::Error>,
    _reason: String,
) -> Result<(), async_nats::Error> {
    let message = message.unwrap();

    //  jetstream
    //      .publish(
    //          ERR_Q.to_string(),
    //          build_error_payload(&message, &reason).to_string().into(),
    //      )
    //      .await
    //      .unwrap();

    Ok(())
}

fn build_error_payload(msg: &Message, reason: &String) -> String {
    json!(ErrorMessage {
        request_id: try_get_request_id(&msg.payload).unwrap_or("UNKNOWN".to_string()),
        sender: GROUP.to_string(),
        reason: reason.to_string(),
        input: format!("{msg:?}"),
    })
    .to_string()
}
