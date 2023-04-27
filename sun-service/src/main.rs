use async_nats::{
    jetstream::{kv::Store, Context, Message},
    Error,
};
use futures_util::{Future, StreamExt};
use log::info;
use messages_common::MessageStream;

const IN_STREAM: &str = "HORIZONS";
const HORIZON_STORE: &str = "horizons";
const GROUP: &str = "sun-service";

const OUT_STREAM: &str = "SUNSETS";
const ERR_STREAM: &str = "ERRORS";

#[tokio::main]
async fn main() {
    env_logger::init();

    let (jetstream, store) = setup_nats().await;
    let messages = messages(&jetstream).await;

    // Somehow generate in function
    let handle_message_res = |message| async {
        info!("Received message {:?}", message);

        match message {
            Ok(message) => handle_message(message, &jetstream, &store)
                .await
                .unwrap_or_else(|_| todo!("send error message")),
            Err(_) => todo!("send error message"),
        };
    };

    messages.for_each_concurrent(16, handle_message_res).await;
}

async fn setup_nats() -> (Context, Store) {
    info!("Setting up NATS");

    let client = messages_common::connect_nats().await;
    let jetstream = messages_common::connect_jetstream(client);

    messages_common::create_stream(&jetstream, OUT_STREAM).await;
    messages_common::create_stream(&jetstream, ERR_STREAM).await;

    let store = messages_common::connect_kv_store(&jetstream, HORIZON_STORE).await;

    (jetstream, store)
}

async fn messages(jetstream: &Context) -> MessageStream {
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

async fn handle_message(message: Message, jetstream: &Context, store: &Store) -> Result<(), Error> {
    let message = (); // TODO: decode message

    // Get horizon from store

    // Calculations

    // Send result

    Ok(())
}
