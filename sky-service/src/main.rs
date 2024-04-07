use messages_common::handle_messages;
use sky_service::messaging::{self, handle_message};

#[tokio::main]
async fn main() {
    env_logger::init();

    let (jetstream, store) = messaging::setup_nats().await;
    let messages = messaging::messages(&jetstream).await;

    handle_messages(
        &jetstream,
        "bla",
        messages,
        Box::new(move |jetstream, message| {
            let store = store.clone();
            Box::pin(async move { handle_message(jetstream, &message, &store).await })
        }),
    )
    .await
}
