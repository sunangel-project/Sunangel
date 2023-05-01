use chrono::{DateTime, Utc};

use crate::location::Location;

pub mod sun;

struct SkyPosition {
    altitude: f64,
    azimuth: f64,
}

trait SkyObject {
    fn position(time: DateTime<Utc>, location: Location) -> SkyPosition;
}
