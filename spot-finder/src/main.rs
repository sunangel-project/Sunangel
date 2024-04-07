#[tokio::main]
pub async fn main() {
    run().await
}

use std::str::{self, FromStr};

use anyhow::anyhow;
use async_nats::jetstream::{Context, Message};
use log::info;
use messages_common::handle_messages;
use serde::{Deserialize, Serialize};

use serde_json::{json, Value};
use spot_finder::location::Location;
use spot_finder::{find_spots, search_area_short_enough, Spot};

#[derive(Debug, Serialize, Deserialize)]
struct InMessage {
    request_id: String,
    search_query: SearchQuery,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchQuery {
    lower_left: Location,
    upper_right: Location,
}

#[derive(Debug, Serialize, Deserialize)]
struct PartMessage {
    id: usize,
    of: usize,
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

    handle_messages(
        &(jetstream.clone()),
        GROUP,
        messages,
        Box::new(move |message| {
            let jetstream = jetstream.clone();
            Box::pin(async move { handle_message(&jetstream, message).await })
        }),
    )
    .await;
}

const UPPER_SEARCH_AREA_DIAGONAL_LIMIT: u32 = 10_000;

// Event Loop
async fn handle_message(jetstream: &Context, message: Message) -> Result<(), async_nats::Error> {
    let payload = str::from_utf8(&message.payload)?;

    let in_message: InMessage = serde_json::from_str(payload)?;
    let query = in_message.search_query;
    info!("Extraxted query {:?}", query);

    let spots = if search_area_short_enough(
        query.lower_left,
        query.upper_right,
        UPPER_SEARCH_AREA_DIAGONAL_LIMIT,
    ) {
        find_spots(query.lower_left, query.upper_right).await
    } else {
        // _ = message.ack().await; // Ignore result, next error is more important
        // moved ack to main fun
        // TODO: work out concept for recoverable vs non-retryable errors (ack vs nack)

        Err(anyhow!(
            "search area too big, diagonal was larger than {} meters",
            UPPER_SEARCH_AREA_DIAGONAL_LIMIT
        ))
    }?;

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

    Ok(())
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
