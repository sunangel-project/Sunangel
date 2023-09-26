use chrono::{Duration, NaiveDateTime};

use crate::location::Location;

pub mod sun;

#[cfg(test)]
mod util;

#[derive(Debug)]
pub struct SkyPosition {
    pub altitude: f64,
    pub azimuth: f64,
}

pub trait SkyObject {
    fn new() -> Self;
    fn period(&self) -> Duration;
    fn position(&self, time: &NaiveDateTime, location: &Location) -> SkyPosition;
}
