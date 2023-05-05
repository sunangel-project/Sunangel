use futures_util::StreamExt;

use sun_service::messaging;

#[tokio::main]
async fn main() {
    env_logger::init();

    let (jetstream, store) = messaging::setup_nats().await;
    let messages = messaging::messages(&jetstream).await;

    // Somehow generate in function
    let handle_message_res = messaging::generate_handle_message_res(&jetstream, &store);

    messages.for_each_concurrent(16, handle_message_res).await;
}
