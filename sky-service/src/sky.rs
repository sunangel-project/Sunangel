use chrono::{DateTime, Duration, Utc};

use crate::location::Location;

pub mod sun;

pub struct SkyPosition {
    pub altitude: f64,
    pub azimuth: f64,
}

pub trait SkyObject {
    fn new() -> Self;
    fn period(&self) -> Duration;
    fn position(&self, time: &DateTime<Utc>, location: &Location) -> SkyPosition;
}
