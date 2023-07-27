use async_nats::{jetstream::Context, Message};
use futures_util::stream::select;

use juniper::{graphql_value, FieldError};
use log::info;
use messages_common::MessageStream;
use serde::{Deserialize, Serialize};
use std::{error::Error, str};
use uuid::Uuid;

use crate::structs::{SearchError, SearchQueryMessage, SearchResponse, SpotsSuccess};

const SEARCH_STREAM: &str = "SEARCH";
const SEARCH_Q: &str = "SEARCH.request";

const IN_STREAM: &str = "SUNSETS";
const IN_ERR_STREAM: &str = "ERRORS";

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
) -> Result<MessageStream, Box<dyn Error + Send + Sync>> {
    let messages_in = messages_common::try_pub_sub_subscribe(jetstream, IN_STREAM).await?;
    let messages_err = messages_common::try_pub_sub_subscribe(jetstream, IN_ERR_STREAM).await?;

    let subscriber = select(messages_in, messages_err);

    Ok(Box::pin(subscriber))
}
