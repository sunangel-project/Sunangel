use anyhow::anyhow;
use async_nats::{connection::State, Client, Message};
use futures_util::{stream::select, Stream};

use juniper::{graphql_value, FieldError};
use serde::{Deserialize, Serialize};
use std::{pin::Pin, str};

use crate::structs::{SearchError, SearchQueryMessage, SearchResponse, SpotsSuccess};

const SEARCH_Q: &str = "search";

const IN_Q: &str = "spots";
const IN_ERR_Q: &str = "error";

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
    client: &Client,
    message: SearchQueryMessage,
) -> Result<(), async_nats::Error> {
    is_connected(client)?;

    let payload = serde_json::to_string(&message)?;
    println!("\nSending out payload to {SEARCH_Q}\n{payload}\n");
    client.publish(SEARCH_Q.to_string(), payload.into()).await?;

    Ok(())
}

pub type MessageStream = Pin<Box<dyn Stream<Item = Message> + Send>>;

pub async fn get_messages_stream(client: &Client) -> Result<MessageStream, async_nats::Error> {
    is_connected(client)?;

    let subscriber_in = client.subscribe(IN_Q.to_string()).await?;
    let subsciber_err = client.subscribe(IN_ERR_Q.into()).await?;

    let subscriber = select(subscriber_in, subsciber_err);

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

fn is_connected(client: &Client) -> Result<(), anyhow::Error> {
    match client.connection_state() {
        State::Connected => Ok(()),
        _ => Err(anyhow!("No connection to NATS")),
    }
}
