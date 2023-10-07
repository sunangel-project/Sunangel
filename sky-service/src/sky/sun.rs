use std::f64::consts::PI;

use chrono::{Duration, NaiveDateTime};

use super::{util, SkyObject, SkyPosition};
use crate::{julian, location::Location};

const MEAN_ECLIPTIC_LENGTH_C0: f64 = 280.460f64 * PI / 180.;
const MEAN_ECLIPTIC_LENGTH_C1: f64 = 0.9856474f64 * PI / 180.;
const MEAN_ECLIPTIC_ANOMALY_C0: f64 = 357.528f64 * PI / 180.;
const MEAN_ECLIPTIC_ANOMALY_C1: f64 = 0.9856003f64 * PI / 180.;
const ECLIPTIC_LENGTH_C0: f64 = 1.915 * PI / 180.;
const ECLIPTIC_LENGTH_C1: f64 = 0.01997 * PI / 180.;
const SKEW_OF_ECLIPTIC_C0: f64 = 23.439 * PI / 180.;
const SKEW_OF_ECLIPTIC_C1: f64 = 0.4e-6 * PI / 180.;

pub struct Sun;
impl SkyObject for Sun {
    fn period(&self) -> Duration {
        Duration::days(1)
    }

    // source: https://de.wikipedia.org/wiki/Sonnenstand
    fn position(&self, time: &NaiveDateTime, location: &Location) -> SkyPosition {
        // Ecliptic coordinates
        let n = julian::day_of_since_2000(time);

        let l = MEAN_ECLIPTIC_LENGTH_C0 + MEAN_ECLIPTIC_LENGTH_C1 * n;
        let g = MEAN_ECLIPTIC_ANOMALY_C0 + MEAN_ECLIPTIC_ANOMALY_C1 * n;
        let lambda = l + ECLIPTIC_LENGTH_C0 * g.sin() + ECLIPTIC_LENGTH_C1 * (2. * g).sin();

        let epsilon = SKEW_OF_ECLIPTIC_C0 + n * SKEW_OF_ECLIPTIC_C1;

        // Equatorial coordinates
        let mut alpha = (epsilon.cos() * lambda.tan()).atan();
        if lambda.cos() < 0. {
            alpha += PI;
        }
        let delta = (epsilon.sin() * lambda.sin()).asin();

        let (altitude, azimuth) =
            util::convert_ecliptic_to_horizontal(time, location, alpha, delta);
        let altitude = util::refraction(altitude);

        SkyPosition { altitude, azimuth }
    }
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use super::*;
    use crate::location::Location;
    use crate::util::assert_approx_eq;

    #[test]
    fn sun_position_wiki() {
        let time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2006, 8, 6).unwrap(),
            NaiveTime::from_hms_opt(6, 0, 0).unwrap(),
        );

        let location = Location {
            lat: 48.1,
            lon: 11.6,
        };

        let pos = Sun.position(&time, &location);

        assert_approx_eq(pos.altitude, 19.11f64.to_radians());
        assert_approx_eq(pos.azimuth, 265.938f64.to_radians() - PI);
    }

    #[test]
    fn sun_position_custom() {
        let time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2022, 2, 11).unwrap(),
            NaiveTime::from_hms_opt(16, 30, 0).unwrap(),
        );

        let location = Location {
            lat: 48.8187132,
            lon: 9.5878127,
        };

        let pos = Sun.position(&time, &location);

        assert_approx_eq(pos.altitude, 0.00902);
        assert_approx_eq(pos.azimuth, 1.19716 + PI);
    }
}
