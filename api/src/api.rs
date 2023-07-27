use async_stream::stream;

use futures::StreamExt;
use futures_util::Stream;
use log::{error, info};
use messages_common::MessageStream;
use std::collections::HashSet;
use std::{pin::Pin, str};

use async_nats::jetstream::{self, Message};
use juniper::{graphql_object, graphql_subscription, graphql_value, EmptyMutation, FieldError};
use uuid::Uuid;

use crate::messaging;
use crate::structs::{
    APISearchQuery, APISpot, HorizonEventsCollection, Location, SearchError, SearchQuery,
    SearchQueryMessage, SearchResponse, SpotAnswerStatus, SpotsSuccess,
};

///////////
// State //
///////////

#[derive(Clone)]
pub struct Context {
    jetstream: jetstream::Context,
    fake: bool,
}
impl juniper::Context for Context {}

impl Context {
    pub async fn new() -> Self {
        let client = messages_common::connect_nats().await;
        let jetstream = messages_common::connect_jetstream(client);

        messaging::create_streams(&jetstream).await;

        let fake = std::env::var("FAKE").map(|val| val == "1").unwrap_or(false);

        Self { jetstream, fake }
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

type SpotStream = dyn Stream<Item = Result<SpotsSuccess, FieldError>> + Send;
type SpotStreamPin = Pin<Box<SpotStream>>;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn spots(#[graphql(context)] context: &Context, query: APISearchQuery) -> SpotStreamPin {
        if context.fake {
            fake_result_stream(query)
        } else {
            real_result_stream(context, query).await
        }
    }
}

fn fake_result_stream(query: APISearchQuery) -> SpotStreamPin {
    let lat = query.location.lat;
    let lon = query.location.lon;
    let events = HorizonEventsCollection::fake();
    let dist = 0.001;
    Box::pin(stream! {
        for i in 0..4 {
            let lat = lat + if i > 1 { dist } else { -dist };
            let lon = lon + if i % 2 == 0 { dist } else { -dist };
            yield Ok(SpotsSuccess {
                status: if i == 3 {
                    SpotAnswerStatus::Finished
                } else {
                    SpotAnswerStatus::Running
                },
                spot: APISpot {
                    location: Location { lat, lon },
                    kind: String::from("fake"),
                    events: events.clone(),
                },
            })
        }
    })
}

async fn real_result_stream(context: &Context, search_query: APISearchQuery) -> SpotStreamPin {
    let request_id = Uuid::new_v4().to_string();

    let search_query = SearchQuery::from(search_query);
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
        Ok(messages) => translate_response_messages(messages, request_id).await,
    }
}

async fn translate_response_messages(
    mut messages: MessageStream,
    request_id: String,
) -> SpotStreamPin {
    Box::pin(stream! {
        let mut received_ids = HashSet::<u32>::new();
        while let Some(message) = messages.next().await {
            match message {
                Err(error) => yield Err(FieldError::new(
                    "Error while receiving responses",
                    graphql_value!(error.to_string()),
                )),
                Ok(message) => {
                    if messages_common::get_request_id(&message.payload) != request_id {
                        continue;
                    }

                    let (spot, last) = transform_spot_message(message, &mut received_ids).await?;
                    yield Ok(spot);
                    if last {
                        break;
                    }
                }
            }
        }
    })
}

async fn transform_spot_message(
    message: Message,
    received_ids: &mut HashSet<u32>,
) -> Result<(SpotsSuccess, bool), FieldError> {
    let payload_str = str::from_utf8(&message.payload)?;
    let res_response: Result<SearchResponse, serde_json::Error> = serde_json::from_str(payload_str);

    match res_response {
        Ok(response) => {
            info!("Received response from microservices: {response:?}");

            // This implementation using a HashSet is wasteful
            // TODO: Maybe alternative implementation using a vector?
            if received_ids.is_empty() {
                for id in 0..response.part.of {
                    received_ids.insert(id);
                }
            }
            received_ids.remove(&response.part.id);
            let last = received_ids.is_empty();

            let status = if last {
                SpotAnswerStatus::Finished
            } else {
                SpotAnswerStatus::Running
            };
            let spot = APISpot::from(response);

            message.ack().await?;
            Ok((SpotsSuccess { status, spot }, last))
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

////////////
// Schema //
////////////

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>, Subscription>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::<Context>::new(), Subscription)
}
