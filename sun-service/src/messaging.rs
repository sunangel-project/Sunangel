use async_nats::{
    jetstream::{kv::Store, Context, Message},
    Error,
};
use log::info;
use messages_common::MessageStream;
use serde::{Deserialize, Serialize};
use std::str;

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

/*
fn generate_handle_message_res(
    jetstream: &Context,
    store: &Store,
) -> Box<dyn Fn(Result<Message, Error>) -> dyn Future<Output = ()>> {
    Box::new()
}
*/

pub async fn handle_message(
    message: Message,
    jetstream: &Context,
    store: &Store,
) -> Result<(), Error> {
    let payload = str::from_utf8(&message.payload)?;
    let message: InMessage = serde_json::from_str(payload)?;

    let horizon = store.get(&message.horizon).await?;
    info!("Retreived horizon '{}'", message.horizon);

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
