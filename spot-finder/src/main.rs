use std::{
    error::Error,
    str::{self, FromStr},
};

use anyhow::anyhow;
use async_nats::{Client, Message};
use futures_util::stream::StreamExt;
use messages_common::try_get_request_id;
use serde::{Deserialize, Serialize};

pub mod direction;
pub mod location;
pub mod spot_finder;

use location::Location;
use serde_json::{json, Value};
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

const IN_Q: &str = "search";
const GROUP: &str = "spot-finder";

const OUT_Q: &str = "spots";
const ERR_Q: &str = "error";

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let host = std::env::var("NATS_HOST").unwrap_or("localhost".into());
    let client = &async_nats::connect(&host).await?;
    let subscriber = client.queue_subscribe(IN_Q.into(), GROUP.into()).await?;

    println!("Listening to {} for messages in queue '{IN_Q}'", &host);

    subscriber
        .for_each_concurrent(16, |msg| async move {
            if let Err(err) = handle_message(client, &msg).await {
                let reason = err.to_string();

                send_error_message(client, &msg, reason).await
            }
        })
        .await;

    Ok(())
}

// Event Loop
async fn handle_message(client: &Client, msg: &Message) -> Result<(), Box<dyn Error>> {
    let payload = str::from_utf8(&msg.payload)?;

    let spots = handle_payload(payload).await?;
    let total_num = spots.len();

    if total_num == 0 {
        return Err(anyhow!("Could not find any spots in this area").into());
    }

    let in_value = Value::from_str(payload)?;
    for (i, spot) in spots.into_iter().enumerate() {
        client
            .publish(
                OUT_Q.to_string(),
                build_output_payload(spot, i, total_num, &in_value)?
                    .to_string()
                    .into(),
            )
            .await?;
    }

    Ok(())
}

async fn handle_payload(payload: &str) -> Result<Vec<Spot>, Box<dyn Error>> {
    let in_message: InMessage = serde_json::from_str(payload)?;
    let query = in_message.search_query;
    find_spots(&query.loc, query.rad).await
}

fn build_output_payload(
    spot: Spot,
    part_num: usize,
    total_num: usize,
    query_value: &Value,
) -> Result<Value, Box<dyn Error>> {
    let mut output = query_value.clone();
    let output_obj = output
        .as_object_mut()
        .ok_or(anyhow!("query was not an object: {query_value:?}"))?;

    output_obj.insert("spot".into(), serde_json::to_value(spot)?);
    output_obj.insert(
        "part".into(),
        serde_json::to_value(PartMessage {
            id: part_num,
            of: total_num,
        })?,
    );

    Ok(output)
}

async fn send_error_message(client: &Client, msg: &Message, reason: String) {
    client
        .publish(
            ERR_Q.to_string(),
            build_error_payload(msg, &reason).to_string().into(),
        )
        .await
        .unwrap_or_else(|err| println!("error {err} when trying to send {reason}"))
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
