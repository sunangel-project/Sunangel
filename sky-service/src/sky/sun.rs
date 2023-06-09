use chrono::{DateTime, Duration, Utc};
use spa::FloatOps;

use super::{SkyObject, SkyPosition};
use crate::{angle, location::Location};

pub struct StdFloatOps;
impl FloatOps for StdFloatOps {
    fn sin(x: f64) -> f64 {
        x.sin()
    }
    fn cos(x: f64) -> f64 {
        x.cos()
    }
    fn tan(x: f64) -> f64 {
        x.tan()
    }
    fn asin(x: f64) -> f64 {
        x.asin()
    }
    fn acos(x: f64) -> f64 {
        x.acos()
    }
    fn atan(x: f64) -> f64 {
        x.atan()
    }
    fn atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }
    fn trunc(x: f64) -> f64 {
        x.trunc()
    }
}

pub struct Sun;
impl SkyObject for Sun {
    fn new() -> Self {
        Sun {}
    }

    fn period(&self) -> Duration {
        Duration::days(1)
    }

    // TODO: replace with own implementation, clone is not sustainable all the time
    fn position(&self, time: &DateTime<Utc>, location: &Location) -> SkyPosition {
        let solar_pos = spa::solar_position::<StdFloatOps>(*time, location.lat, location.lon)
            .expect("Coordinates should always be valid");

        SkyPosition {
            altitude: angle::normalize_degrees(90. - solar_pos.zenith_angle).to_radians(),
            azimuth: angle::normalize_degrees(solar_pos.azimuth - 180.).to_radians(),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

    use super::*;
    use crate::location::Location;

    #[test]
    fn sun_position() {
        let time: DateTime<Utc> = DateTime::from_utc(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2006, 8, 6).unwrap(),
                NaiveTime::from_hms_opt(6, 0, 9).unwrap(),
            ),
            Utc,
        );

        let location = Location {
            lat: 48.1,
            lon: 11.6,
        };

        let pos = Sun::new().position(&time, &location);

        let epsilon = 0.05;
        assert!(
            (pos.altitude - 19.110_f64.to_radians()).abs() < epsilon,
            "altitude was {}",
            pos.altitude
        );
        assert!(
            (pos.azimuth + 94.062_f64.to_radians()).abs() < epsilon,
            "azimuth was {}",
            pos.azimuth
        );
    }
}
