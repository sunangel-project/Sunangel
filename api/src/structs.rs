use chrono::{DateTime, Utc};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

////////////
/// NATS ///
////////////

/// In

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    id: u32,
    of: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Spot {
    dir: Option<f64>,
    kind: String,
    loc: Location,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    part: Part,
    request_id: String,
    search_query: SearchQuery,
    spot: Spot,
    horizon: String,
    events: HorizonEventsCollection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchError {
    pub input: String,
    pub reason: String,
    pub request_id: String,
    pub sender: String,
}

///////////////
/// GraphQL ///
///////////////

// Out

#[derive(Debug, GraphQLObject, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

impl From<LocationIn> for Location {
    fn from(value: LocationIn) -> Self {
        Location {
            lat: value.lat,
            lon: value.lon,
        }
    }
}

#[derive(Debug, Clone, GraphQLObject, Serialize, Deserialize)]
pub struct HorizonEvent {
    pub time: DateTime<Utc>,
    pub altitude: f64,
    pub azimuth: f64,
}

#[derive(Debug, Clone, GraphQLObject, Serialize, Deserialize)]
pub struct HorizonEvents {
    pub rise: HorizonEvent,
    pub set: HorizonEvent,
}

#[derive(Debug, Clone, GraphQLObject, Serialize, Deserialize)]
pub struct HorizonEventsCollection {
    sun: HorizonEvents,
}

impl HorizonEventsCollection {
    pub fn fake() -> Self {
        Self {
            sun: HorizonEvents {
                rise: HorizonEvent {
                    time: Utc::now(),
                    altitude: 0.,
                    azimuth: 0.,
                },
                set: HorizonEvent {
                    time: Utc::now(),
                    altitude: 0.,
                    azimuth: 0.,
                },
            },
        }
    }
}

#[derive(GraphQLObject)]
pub struct APISpot {
    pub location: Location,
    pub kind: String,
    pub events: HorizonEventsCollection,
}

impl From<SearchResponse> for APISpot {
    fn from(value: SearchResponse) -> Self {
        APISpot {
            location: value.spot.loc,
            kind: value.spot.kind,
            events: value.events,
        }
    }
}

#[derive(GraphQLEnum)]
pub enum SpotAnswerStatus {
    Running,
    Finished,
}

#[derive(GraphQLObject)]
pub struct SpotsSuccess {
    pub status: SpotAnswerStatus,
    pub spot: APISpot,
}

impl From<SearchResponse> for SpotsSuccess {
    fn from(value: SearchResponse) -> Self {
        let status = if value.part.id < value.part.of - 1 {
            SpotAnswerStatus::Running
        } else {
            SpotAnswerStatus::Finished
        };

        let spot = APISpot::from(value);

        SpotsSuccess { status, spot }
    }
}

//////////////////////////////////////////////////////////////////////////////
/// Search

// In

#[derive(GraphQLInputObject)]
pub struct LocationIn {
    pub lat: f64,
    pub lon: f64,
}

#[derive(GraphQLInputObject)]
pub struct APISearchQuery {
    pub location: LocationIn,
    pub radius: i32,
}
////////////
/// NATS ///
////////////

/// Out

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    loc: Location,
    rad: i32,
}

impl From<APISearchQuery> for SearchQuery {
    fn from(value: APISearchQuery) -> Self {
        SearchQuery {
            loc: value.location.into(),
            rad: value.radius,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SearchQueryMessage {
    pub request_id: String,
    pub search_query: SearchQuery,
}
