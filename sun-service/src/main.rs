use futures_util::StreamExt;
use log::info;

mod messaging;

#[tokio::main]
async fn main() {
    env_logger::init();

    let (jetstream, store) = messaging::setup_nats().await;
    let messages = messaging::messages(&jetstream).await;

    // Somehow generate in function
    let handle_message_res = |message| async {
        info!("Received message {:?}", message);

        match message {
            Ok(message) => messaging::handle_message(message, &jetstream, &store)
                .await
                .unwrap_or_else(|_| todo!("send error message")),
            Err(_) => todo!("send error message"),
        };
    };

    messages.for_each_concurrent(16, handle_message_res).await;
}
