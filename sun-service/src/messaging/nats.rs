pub async fn setup() -> (Context, Store) {
    info!("Setting up NATS");

    let client = messages_common::connect_nats().await;
    let jetstream = messages_common::connect_jetstream(client);

    messages_common::create_stream(&jetstream, OUT_STREAM).await;
    messages_common::create_stream(&jetstream, ERR_STREAM).await;

    let store = messages_common::connect_kv_store(&jetstream, HORIZON_STORE).await;

    let messages = messages_common::queue_subscribe(&jetstream, IN_STREAM, GROUP).await;
}

pub async fn messages(jetstream: Context, store: Store) -> MessageStream {
    info!("Listening to NATS for messages in queue '{IN_STREAM}'");

    (jetstream, store)
}
