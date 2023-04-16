use chrono::{DateTime, Utc};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};

////////////
/// NATS ///
////////////

/// In

#[derive(Serialize, Deserialize)]
struct Part {
    id: u32,
    of: u32,
}

#[derive(Serialize, Deserialize)]
struct Spot {
    dir: Option<f64>,
    kind: String,
    loc: Location,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    part: Part,
    request_id: String,
    search_query: SearchQuery,
    spot: Spot,
    horizon: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(GraphQLObject, Serialize, Deserialize)]
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

#[derive(GraphQLObject)]
pub struct HorizonEvent {
    pub time: DateTime<Utc>,
    pub alt: f64,
    pub azi: f64,
}

#[derive(GraphQLObject)]
pub struct APISpot {
    pub location: Location,
    pub kind: String,
    pub sunset: HorizonEvent,
}

impl From<Spot> for APISpot {
    fn from(value: Spot) -> Self {
        APISpot {
            location: value.loc,
            kind: value.kind,
            sunset: HorizonEvent {
                time: Utc::now(),
                alt: 270.,
                azi: 1.,
            },
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

        let spot = APISpot::from(value.spot);

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

#[derive(Serialize, Deserialize)]
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
