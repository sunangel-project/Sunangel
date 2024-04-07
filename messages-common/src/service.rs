use std::pin::Pin;

use async_nats::{
    jetstream::{Context, Message},
    Error,
};
use bytes::Bytes;
use futures_util::{Future, StreamExt};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{try_get_request_id, MessageStream};

// TODO: replace with async closures as soon as they are stable
// also why does it have to be FnMut...
pub type HandleMessageFun<'a> = Box<
    dyn FnMut(&'a Context, Message) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>> + 'a,
>;

const ERR_STREAM: &str = "ERRORS";

pub async fn handle_messages<'a>(
    jetstream: &'a Context,
    group: &str,
    mut messages: MessageStream,
    mut handle_message: HandleMessageFun<'a>,
) {
    // TODO: return to concurrently processing
    while let Some(message) = messages.next().await {
        info!("Received message {:?}", message);

        match message {
            Ok(message) => {
                let res = handle_message(jetstream, message.clone()).await;
                if let Err(err) = res {
                    error!("Could not handle received message: {err}");
                    send_error_with_message(jetstream, group, &message, err)
                        .await
                        .unwrap_or_else(|err| error!("Could not send error message: {err}"));
                }

                message
                    .ack()
                    .await
                    .unwrap_or_else(|err| error!("Could not ack received message: {err}"))
            }
            Err(err) => {
                error!("Problem with received message: {err}");
                send_error_without_message(jetstream, err.into())
                    .await
                    .unwrap_or_else(|err| error!("Could not send out error message: {err}"));
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorMessage {
    request_id: String,
    sender: String,
    reason: String,
    input: String,
}

async fn send_error_with_message(
    jetstream: &Context,
    group: &str,
    message: &Message,
    error: async_nats::Error,
) -> Result<(), async_nats::Error> {
    let request_id = try_get_request_id(&message.payload).unwrap_or("UNKNOWN".to_string());

    let payload = json!(ErrorMessage {
        request_id: request_id.clone(),
        sender: group.to_string(),
        reason: error.to_string(),
        input: format!("{message:?}"),
    })
    .to_string()
    .into();

    send_error(jetstream, format!("{ERR_STREAM}.{request_id}"), payload).await
}

async fn send_error_without_message(
    jetstream: &Context,
    error: async_nats::Error,
) -> Result<(), async_nats::Error> {
    send_error(
        jetstream,
        format!("{ERR_STREAM}.api"),
        error.to_string().into(),
    )
    .await
}

async fn send_error(
    jetstream: &Context,
    queue: String,
    payload: Bytes,
) -> Result<(), async_nats::Error> {
    let published = jetstream.publish(queue, payload).await?;
    published.await?;

    Ok(())
}
