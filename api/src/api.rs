use async_stream::stream;
use futures::{future, StreamExt};
use log::error;
use std::{pin::Pin, str};

use api::{
    messaging::{get_messages_stream, send_search_query, MessageStream},
    structs::{
        APISearchQuery, SearchError, SearchQuery, SearchQueryMessage, SearchResponse, SpotsSuccess,
    },
};
use async_nats::{Client, Message};
use futures_util::Stream;
use juniper::{graphql_object, graphql_subscription, graphql_value, EmptyMutation, FieldError};
use uuid::Uuid;

///////////
// State //
///////////

#[derive(Clone)]
pub struct Context {
    client: Client,
}
impl juniper::Context for Context {}

impl Context {
    pub async fn new() -> Result<Self, async_nats::Error> {
        let client = async_nats::connect("localhost").await?;

        Ok(Self { client })
    }
}

/////////////
// Queries //
/////////////

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn apiVersion() -> &'static str {
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

        let sent = send_search_query(&context.client, search_message).await.map_err(|err| {
            error!("Couldn't send search query to NATS");

            Box::pin(stream! {
                yield Err(
                    FieldError::new("Couldn't send search query to NATS", graphql_value!(err.to_string()))
                )
            })
        });

        match sent {
            Err(err_stream) => err_stream,
            Ok(_) => connect_to_response_messages(&context.client, request_id).await,
        }
    }
}

async fn connect_to_response_messages(client: &Client, request_id: String) -> SpotStreamPin {
    let res_messages = get_messages_stream(client).await.map_err(|err| {
        error!("Couldn't subscribe to NATS");

        Box::pin(stream! {
            yield Err(
                FieldError::new("Couldn't subscribe to NATS", graphql_value!(err.to_string()))
            )
        })
    });

    match res_messages {
        Err(error_stream) => error_stream,
        Ok(messages) => filter_messages(messages, request_id),
    }
}

fn filter_messages(messages: MessageStream, request_id: String) -> SpotStreamPin {
    messages
        .filter(move |message| {
            future::ready(messages_common::get_request_id(&message.payload) == request_id)
        })
        .map(transform_spot_message)
        .boxed()
}

fn transform_spot_message(message: Message) -> Result<SpotsSuccess, FieldError> {
    let payload_str = str::from_utf8(&message.payload)?;
    let res_response: Result<SearchResponse, serde_json::Error> = serde_json::from_str(payload_str);

    match res_response {
        Ok(response) => Ok(response.into()),
        Err(_) => {
            let error: SearchError = serde_json::from_str(payload_str)?;

            Err(FieldError::new(
                "Internal server error",
                graphql_value!(serde_json::to_string(&error)?),
            ))
        }
    }
}

////////////
// Schema //
////////////

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::<Context>::new(), Subscription)
}
