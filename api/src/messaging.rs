use async_nats::jetstream::Context;
use futures_util::stream::select;

use log::info;
use messages_common::MessageStream;
use serde::{Deserialize, Serialize};
use std::{error::Error, str};

use crate::structs::SearchQueryMessage;

const SEARCH_STREAM: &str = "SEARCH";
const SEARCH_Q: &str = "SEARCH.request";

const IN_STREAM: &str = "SUNSETS";
const IN_ERR_STREAM: &str = "ERRORS";

const GROUP: &str = "api";

#[derive(Serialize, Deserialize)]
struct Location {
    lat: f64,
    lon: f64,
}

#[derive(Serialize, Deserialize)]
struct SearchQuery {
    loc: Location,
    rad: i32,
}

pub async fn create_streams(jetstream: &Context) {
    messages_common::create_stream(jetstream, SEARCH_STREAM).await;
}

pub async fn send_search_query(
    jetstream: &Context,
    message: SearchQueryMessage,
) -> Result<(), async_nats::Error> {
    let payload = serde_json::to_string(&message)?;

    info!("Sending out search request {payload}");
    jetstream
        .publish(SEARCH_Q.to_string(), payload.into())
        .await?;

    Ok(())
}

pub async fn get_messages_stream(
    jetstream: &Context,
    request_id: &str,
) -> Result<MessageStream, Box<dyn Error + Send + Sync>> {
    let messages_in =
        messages_common::try_queue_subscribe_subject(jetstream, IN_STREAM, request_id, GROUP)
            .await?;
    let messages_err = messages_common::try_pub_sub_subscribe(jetstream, IN_ERR_STREAM).await?;

    let subscriber = select(messages_in, messages_err);

    Ok(Box::pin(subscriber))
}

pub async fn delete_consumer(request_id: &str) -> Result<bool, anyhow::Error> {
    let jetstream = messages_common::connect_jetstream().await;
    messages_common::try_delete_queue_consumer(&jetstream, IN_STREAM, request_id, GROUP).await
}
