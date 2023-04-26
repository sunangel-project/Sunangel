use log::info;

const IN_STREAM: &str = "HORIZONS";
const GROUP: &str = "sun-service";

const OUT_STREAM: &str = "SUNSETS";
const ERR_STREAM: &str = "ERRORS";

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = messages_common::connect_nats().await;
    let jetstream = messages_common::connect_jetstream(client);

    messages_common::create_stream(&jetstream, OUT_STREAM).await;
    messages_common::create_stream(&jetstream, ERR_STREAM).await;

    let _messages = messages_common::queue_subscribe(&jetstream, IN_STREAM, GROUP).await;

    info!("Listening to NATS for messages in queue '{IN_STREAM}'");
}
