use std::{env, error::Error, pin::Pin};

use async_nats::jetstream::{self, Context};
use futures_util::Stream;
use log::debug;

pub async fn connect_nats() -> async_nats::Client {
    let host = env::var("NATS_HOST").unwrap_or("localhost".to_string());
    async_nats::connect(host)
        .await
        .expect("Could not connect to NATS")
}

pub fn connect_jetstream(client: async_nats::Client) -> Context {
    async_nats::jetstream::new(client)
}

pub async fn try_create_stream(
    jetstream: &Context,
    stream: &str,
) -> Result<jetstream::stream::Stream, Box<dyn Error + Send + Sync>> {
    jetstream
        .get_or_create_stream(async_nats::jetstream::stream::Config {
            name: stream.to_string(),
            max_messages: 10_000,
            subjects: vec![stream.to_string(), format!("{stream}.*")],
            ..Default::default()
        })
        .await
}

pub async fn create_stream(jetstream: &Context, stream: &str) -> jetstream::stream::Stream {
    try_create_stream(jetstream, stream)
        .await
        .expect("Could not create stream")
}

pub type MessageStream =
    Pin<Box<dyn Stream<Item = Result<jetstream::Message, Box<dyn Error + Send + Sync>>> + Send>>;

pub async fn try_connect_to_stream(
    jetstream: &Context,
    stream: &str,
) -> Result<MessageStream, Box<dyn Error + Send + Sync>> {
    try_create_stream(jetstream, stream).await?;

    debug!("Trying to connect to {stream}");
    let stream = jetstream.get_stream(stream).await?;

    debug!("Trying to create consumer for {stream:?}");
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
