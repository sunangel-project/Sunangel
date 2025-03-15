use log::info;
use messages_common::handle_messages;
use sky_service::messaging::{self, handle_message, GROUP};

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Starting up (version {})", version_common::BACKEND_VERSION);

    let (jetstream, store) = messaging::setup_nats().await;
    let messages = messaging::messages(&jetstream).await;

    handle_messages(
        &(jetstream.clone()),
        GROUP,
        messages,
        Box::new(move |message| {
            let jetstream = jetstream.clone();
            let store = store.clone();
            Box::pin(async move { handle_message(&jetstream, &message, &store).await })
        }),
    )
    .await
}
