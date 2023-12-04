use std::{env, error::Error, pin::Pin};

use async_nats::jetstream::{self, consumer::pull::MessagesError, kv::Store, Context};
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
) -> Result<jetstream::stream::Stream, async_nats::Error> {
    Ok(jetstream
        .get_or_create_stream(async_nats::jetstream::stream::Config {
            name: stream.to_string(),
            max_messages: 10_000,
            subjects: vec![stream.to_string(), format!("{stream}.*")],
            ..Default::default()
        })
        .await?)
}

pub async fn create_stream(jetstream: &Context, stream: &str) -> jetstream::stream::Stream {
    try_create_stream(jetstream, stream)
        .await
        .expect("Could not create stream")
}

pub type MessageStream =
    Pin<Box<dyn Stream<Item = Result<jetstream::Message, MessagesError>> + Send>>;

async fn try_subscribe(
    jetstream: &Context,
    stream_name: &str,
    subject: Option<&str>,
    group: Option<&str>,
) -> Result<MessageStream, async_nats::Error> {
    try_create_stream(jetstream, stream_name).await?;

    debug!("Trying to connect to {}", stream_name);
    let stream = jetstream.get_stream(stream_name).await?;

    let mut subjects = vec![];
    if let Some(subject) = subject {
        subjects.push(format!("{}.{}", stream_name, subject));
    }

    debug!("Trying to create consumer for {:?}", stream_name);
    let consumer = stream
        .get_or_create_consumer(
            group.unwrap_or("sole_consumer"),
            async_nats::jetstream::consumer::pull::Config {
                durable_name: group.map(str::to_string),
                filter_subjects: subjects,
                ..Default::default()
            },
        )
        .await?;

    let messages = consumer.messages().await?;

    Ok(Box::pin(messages))
}

pub async fn try_pub_sub_subscribe(
    jetstream: &Context,
    stream: &str,
) -> Result<MessageStream, Box<dyn Error + Send + Sync>> {
    try_subscribe(jetstream, stream, None, None).await
}

pub async fn try_queue_subscribe(
    jetstream: &Context,
    stream: &str,
    group: &str,
) -> Result<MessageStream, Box<dyn Error + Send + Sync>> {
    try_subscribe(jetstream, stream, None, Some(group)).await
}

pub async fn try_queue_subscribe_subject(
    jetstream: &Context,
    stream: &str,
    subject: &str,
    group: &str,
) -> Result<MessageStream, Box<dyn Error + Send + Sync>> {
    try_subscribe(
        jetstream,
        stream,
        Some(subject),
        Some(&format!("{}-{}", group, subject)),
    )
    .await
}

pub async fn queue_subscribe(jetstream: &Context, stream: &str, group: &str) -> MessageStream {
    try_queue_subscribe(jetstream, stream, group)
        .await
        .expect("Could not connect to stream")
}

pub async fn connect_kv_store(jetstream: &Context, name: &str) -> Store {
    let created = jetstream.get_key_value(name).await;

    match created {
        Ok(store) => store,
        Err(_) => {
            // TODO: distinguish errors
            let conf = async_nats::jetstream::kv::Config {
                bucket: name.to_string(),
                ..Default::default()
            };

            jetstream
                .create_key_value(conf)
                .await
                .expect("could not create key value store")
        }
    }
}
