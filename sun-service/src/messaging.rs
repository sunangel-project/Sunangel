use anyhow::anyhow;
use async_nats::{
    jetstream::{kv::Store, Context, Message},
    Error,
};
use futures_util::Future;
use log::info;
use messages_common::MessageStream;
use serde::{Deserialize, Serialize};
use std::{pin::Pin, str};

use crate::horizon::Horizon;

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
                Ok(message) => handle_message(message, &jetstream, &store)
                    .await
                    .unwrap_or_else(|_| todo!("send error message")),
                Err(_) => todo!("send error message"),
            };
        })
    })
}

pub async fn handle_message(
    message: Message,
    jetstream: &Context,
    store: &Store,
) -> Result<(), Error> {
    let payload = str::from_utf8(&message.payload)?;
    let message: InMessage = serde_json::from_str(payload)?;

    let horizon = store.get(&message.horizon).await?.ok_or(anyhow!(
        "Could not get a byte array for horizon '{}'",
        message.horizon
    ))?;
    let horizon: Horizon = horizon.try_into()?;
    info!("Retreived and decoded horizon '{}'", message.horizon);

    // Calculations

    // Send result

    Ok(())
}

// In Message
// b"{\"horizon\":\"horizon-v1.0.0-30752d0b-cfe7-5d6b-9bd0-61cea706c6ea\",
// "part\":{\"id\":48,\"of\":49},\"request_id\":\"232d243e-285f-433c-b143-c216492f115f\",
// "search_query\":{\"loc\":{\"lat\":48.81909,\"lon\":9.59523},\"rad\":2000},\"spot\":{\"dir\":null,\"kind\":\"bench\",
// "loc\":{\"lat\":48.8292947,\"lon\":9.588803}}}"

#[derive(Debug, Serialize, Deserialize)]
struct InMessage {
    horizon: String,
}
