use std::f64::consts::PI;

use chrono::{Duration, NaiveDateTime, Timelike};
use julianday::JulianDay;

use super::{SkyObject, SkyPosition};
use crate::{angle, location::Location};

const JD_SINCE_2000: f64 = 2451545.;
const HOURS_PER_DAY: f64 = 24.;
const SECONDS_PER_HOUR: f64 = 60. * 60.;
const SECONDS_PER_DAY: f64 = HOURS_PER_DAY * SECONDS_PER_HOUR;
const NANOSECONDS_PER_HOUR: f64 = SECONDS_PER_HOUR * 1e9;
const DAYS_PER_CENTURY: f64 = 36525.;

const MEAN_ECLIPTIC_LENGTH_C0: f64 = 280.460f64 * PI / 180.;
const MEAN_ECLIPTIC_LENGTH_C1: f64 = 0.9856474f64 * PI / 180.;
const MEAN_ECLIPTIC_ANOMALY_C0: f64 = 357.528f64 * PI / 180.;
const MEAN_ECLIPTIC_ANOMALY_C1: f64 = 0.9856003f64 * PI / 180.;
const ECLIPTIC_LENGTH_C0: f64 = 1.915 * PI / 180.;
const ECLIPTIC_LENGTH_C1: f64 = 0.01997 * PI / 180.;
const SKEW_OF_ECLIPTIC_C0: f64 = 23.439 * PI / 180.;
const SKEW_OF_ECLIPTIC_C1: f64 = 0.4e-6 * PI / 180.;

const STAR_TIME_C0: f64 = 6.697376;
const STAR_TIME_C1: f64 = 2400.05134;
const STAR_TIME_C2: f64 = 1.002738;
const REFRACTION_C0: f64 = 1.02;
const REFRACTION_C1: f64 = 10.3;
const REFRACTION_C2: f64 = 5.11;

pub struct Sun;
impl SkyObject for Sun {
    fn new() -> Self {
        Sun {}
    }

    fn period(&self) -> Duration {
        Duration::days(1)
    }

    // source: https://de.wikipedia.org/wiki/Sonnenstand
    fn position(&self, time: &NaiveDateTime, location: &Location) -> SkyPosition {
        // ecliptic coordinates
        let jd0 = JulianDay::from(time.date()).inner() as f64 - 0.5; // TODO: custom implementation
        let jd = jd0 + time.num_seconds_from_midnight() as f64 / SECONDS_PER_DAY;
        let n = jd - JD_SINCE_2000;

        let l = angle::normalize_radians(MEAN_ECLIPTIC_LENGTH_C0 + MEAN_ECLIPTIC_LENGTH_C1 * n);
        let g = angle::normalize_radians(MEAN_ECLIPTIC_ANOMALY_C0 + MEAN_ECLIPTIC_ANOMALY_C1 * n);
        let lambda = l + ECLIPTIC_LENGTH_C0 * g.sin() + ECLIPTIC_LENGTH_C1 * (2. * g).sin();

        let epsilon = SKEW_OF_ECLIPTIC_C0 - SKEW_OF_ECLIPTIC_C1 * n;

        // equatorial coordinates
        let mut alpha = (epsilon.cos() * lambda.tan()).atan();
        if lambda.cos() < 0. {
            alpha += PI;
        }
        let alpha = alpha; // make immutable
        println!("{}", alpha.to_degrees());
        let delta = (epsilon.sin() * lambda.sin()).asin();

        // horizontal coordinates
        let t0 = n / DAYS_PER_CENTURY;
        let t = time.num_seconds_from_midnight() as f64 / SECONDS_PER_HOUR;
        //+ time.nanosecond() as f64 / NANOSECONDS_PER_HOUR;
        let theta_g_h = STAR_TIME_C0 + STAR_TIME_C1 * t0 + STAR_TIME_C2 * t;
        let theta_g = theta_g_h * 15f64.to_radians();

        let theta = theta_g + location.lon.to_radians();
        let tau = angle::normalize_radians(theta - alpha);

        let phi = location.lat.to_radians();
        let azimuth_enumerator = tau.cos() * phi.sin() - delta.tan() * phi.cos();
        let mut azimuth = (tau.sin()).atan2(azimuth_enumerator);
        if azimuth_enumerator >= 0. {
            azimuth += PI;
        }
        let azimuth = azimuth; // make immutable
        println!("azimuth: {}", azimuth.to_degrees());

        // in degrees!
        let altitude = (delta.cos() * tau.cos() * phi.cos() + delta.sin() * phi.sin())
            .asin()
            .to_degrees();

        // correction of altitude
        let r = REFRACTION_C0
            / ((altitude + REFRACTION_C1 / (altitude + REFRACTION_C2))
                .to_radians()
                .tan()
                .to_degrees()); // R needs to be in degrees also!

        // finally in radians again:
        let altitude = (altitude + r / 60.).to_radians();
        println!("altitude: {}", altitude.to_degrees());
        println!("altitude (rad): {}", altitude);

        // normalize
        let altitude = angle::normalize_radians(altitude);
        let azimuth = angle::normalize_radians(azimuth);

        SkyPosition { altitude, azimuth }
    }
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    use super::super::util::assert_approx_eq;
    use super::*;
    use crate::location::Location;

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

        let pos = Sun::new().position(&time, &location);

        assert_approx_eq(pos.altitude, 19.11f64.to_radians());
        assert_approx_eq(pos.azimuth, 1.19716);
    }

    #[test]
    fn sun_position_custom() {
        let time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2022, 2, 11).unwrap(),
            NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
        );

        let location = Location {
            lat: 48.8187132,
            lon: 9.5878127,
        };

        let pos = Sun::new().position(&time, &location);

        assert_approx_eq(pos.altitude, 0.00902);
        assert_approx_eq(pos.azimuth, 1.19716);
    }
}
