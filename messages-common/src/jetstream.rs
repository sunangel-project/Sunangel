use std::{env, error::Error, pin::Pin};

use async_nats::jetstream::{self, Context};
use futures_util::Stream;

pub async fn connect_nats() -> async_nats::Client {
    let host = env::var("NATS_HOST").unwrap_or("localhost".to_string());
    async_nats::connect(host)
        .await
        .expect("Could not connect to NATS")
}

pub fn connect_jetstream(client: async_nats::Client) -> Context {
    async_nats::jetstream::new(client)
}

pub type MessageStream =
    Pin<Box<dyn Stream<Item = Result<jetstream::Message, Box<dyn Error + Send + Sync>>> + Send>>;

pub async fn try_connect_to_stream(
    jetstream: &Context,
    stream: &str,
    _subject: &str,
) -> Result<MessageStream, Box<dyn Error + Send + Sync>> {
    let stream = jetstream
        .get_or_create_stream(async_nats::jetstream::stream::Config {
            name: stream.to_string(),
            max_messages: 10_000,
            ..Default::default()
        })
        .await?;

    let consumer = stream
        .get_or_create_consumer(
            "consumer",
            async_nats::jetstream::consumer::pull::Config {
                durable_name: Some("consumer".to_string()),
                ..Default::default()
            },
        )
        .await?;

    let messages = consumer.messages().await?;

    Ok(Box::pin(messages))
}

/*
pub async fn connect_to_stream(jetstream: &Context, subject: &str) -> MessageStream {
    try_connect_to_stream(jetstream, subject)
        .await
        .expect("Could not connect to stream")
}
*/
