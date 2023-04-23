use async_nats::{jetstream::Context, Message};
use futures_util::stream::select;

use juniper::{graphql_value, FieldError};
use messages_common::MessageStream;
use serde::{Deserialize, Serialize};
use std::{error::Error, str};

use crate::structs::{SearchError, SearchQueryMessage, SearchResponse, SpotsSuccess};

const SEARCH_STREAM: &str = "SEARCH";
const SEARCH_Q: &str = "SEARCH.request";

const IN_STREAM: &str = "HORIZONS";
const IN_Q: &str = "HORIZONS.sunset";

const IN_ERR_STREAM: &str = "ERRORS";
const IN_ERR_Q: &str = "ERRORS.*";

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

pub async fn send_search_query(
    jetstream: &Context,
    message: SearchQueryMessage,
) -> Result<(), async_nats::Error> {
    // is_connected(context.client)?;

    let payload = serde_json::to_string(&message)?;
    println!("\nSending out payload to {SEARCH_Q}\n{payload}\n");
    jetstream
        .publish(SEARCH_Q.to_string(), payload.into())
        .await?;

    Ok(())
}

pub async fn get_messages_stream(
    jetstream: &Context,
) -> Result<MessageStream, Box<dyn Error + Send + Sync>> {
    let messages_in = messages_common::try_connect_to_stream(jetstream, IN_STREAM, IN_Q).await?;
    let messages_err =
        messages_common::try_connect_to_stream(jetstream, IN_ERR_STREAM, IN_ERR_Q).await?;

    let subscriber = select(messages_in, messages_err);

    Ok(Box::pin(subscriber))
}

pub fn api_answer_from_message(message: Message) -> Result<SpotsSuccess, FieldError> {
    let payload = str::from_utf8(&message.payload)?;

    println!("{payload}");

    let maybe_spot = serde_json::from_str::<SearchResponse>(payload);
    match maybe_spot {
        Ok(spot) => Ok(SpotsSuccess::from(spot)),
        Err(_) => Err(try_decode_error(payload)),
    }
}

fn try_decode_error(payload: &str) -> FieldError {
    let maybe_error = serde_json::from_str::<SearchError>(payload);
    match maybe_error {
        Ok(error) => FieldError::new(
            "Internal Server Error while executing search request",
            graphql_value!(format!(
                // TODO: serde value to graphql error?
                "reason '{}' with input '{}' from '{}'",
                error.reason, error.input, error.sender
            )),
        ),
        Err(_) => FieldError::new(
            "Could not decode search request response",
            graphql_value!(payload),
        ),
    }
}

/*
fn is_connected(client: &Context) -> Result<(), anyhow::Error> {
    match client.connection_state() {
        State::Connected => Ok(()),
        _ => Err(anyhow!("No connection to NATS")),
    }
}
*/
