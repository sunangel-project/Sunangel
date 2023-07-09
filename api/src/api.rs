use async_stream::stream;
use futures::{future, StreamExt};
use futures_util::Stream;
use log::{error, info};
use messages_common::MessageStream;
use std::{pin::Pin, str};

use async_nats::jetstream::{self, Message};
use juniper::{graphql_object, graphql_subscription, graphql_value, EmptyMutation, FieldError};
use uuid::Uuid;

use crate::messaging;
use crate::structs::{
    APISearchQuery, SearchError, SearchQuery, SearchQueryMessage, SearchResponse, SpotsSuccess,
};

///////////
// State //
///////////

#[derive(Clone)]
pub struct Context {
    jetstream: jetstream::Context,
}
impl juniper::Context for Context {}

impl Context {
    pub async fn new() -> Self {
        let client = messages_common::connect_nats().await;
        let jetstream = messages_common::connect_jetstream(client);

        messaging::create_streams(&jetstream).await;

        Self { jetstream }
    }
}

/////////////
// Queries //
/////////////

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn api_version() -> &'static str {
        "1.0"
    }
}

///////////////////
// Subscriptions //
///////////////////

pub struct Subscription;

type SpotStreamPin = Pin<Box<dyn Stream<Item = Result<SpotsSuccess, FieldError>> + Send>>;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn spots(#[graphql(context)] context: &Context, query: APISearchQuery) -> SpotStreamPin {
        let request_id = Uuid::new_v4().to_string();

        let search_query = SearchQuery::from(query);
        let search_message = SearchQueryMessage {
            request_id: request_id.clone(),
            search_query,
        };

        let sent = messaging::send_search_query(&context.jetstream, search_message).await.map_err(|err| {
            error!("Couldn't send search query to NATS");

            Box::pin(stream! {
                yield Err(
                    FieldError::new("Couldn't send search query to NATS", graphql_value!(err.to_string()))
                )
            })
        });

        match sent {
            Err(err_stream) => err_stream,
            Ok(_) => connect_to_response_messages(context, request_id).await,
        }
    }
}

async fn connect_to_response_messages(context: &Context, request_id: String) -> SpotStreamPin {
    return Box::pin(stream! {
        yield Err(FieldError::new("todo: implement", graphql_value!(None)));
    });
}

/*
async fn connect_to_response_messages(context: &Context, request_id: String) -> SpotStreamPin {
    let messages = messaging::get_messages_stream(&context.jetstream)
        .await
        .map_err(|err| {
            error!("Couldn't subscribe to NATS: {err}");

            Box::pin(stream! {
                yield Err(
                    FieldError::new("Couldn't subscribe to NATS", graphql_value!(err.to_string()))
                )
            })
        });

    match messages {
        Err(error_stream) => error_stream,
        Ok(messages) => filter_messages(messages, request_id).await,
    }
}

async fn filter_messages(messages: MessageStream, request_id: String) -> SpotStreamPin {
    messages
        .filter_map(|message| async { message.ok() })
        .filter(move |message| {
            future::ready(messages_common::get_request_id(&message.payload) == request_id)
        })
        // Need filter_map, bc map does not allow async closures
        .filter_map(|message| async { Some(transform_spot_message(message).await) })
        .boxed()
}

async fn transform_spot_message(message: Message) -> Result<SpotsSuccess, FieldError> {
    let payload_str = str::from_utf8(&message.payload)?;
    let res_response: Result<SearchResponse, serde_json::Error> = serde_json::from_str(payload_str);

    match res_response {
        Ok(response) => {
            info!("Received response from microservices: {response:?}");

            message.ack().await?;

            Ok(response.into())
        }
        Err(_) => {
            let error: SearchError = serde_json::from_str(payload_str)?;

            error!("Received error from microservices: {error:?}");

            Err(FieldError::new(
                "Internal server error",
                graphql_value!(serde_json::to_string(&error)?),
            ))
        }
    }
}
*/

////////////
// Schema //
////////////

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::<Context>::new(), Subscription)
}
