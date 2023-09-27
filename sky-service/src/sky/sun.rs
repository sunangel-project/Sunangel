use std::f64::consts::{FRAC_PI_2, PI, TAU};

use chrono::{Duration, NaiveDateTime, Timelike};

use super::{SkyObject, SkyPosition};
use crate::{angle::AngleExtensions, julian, location::Location};

const JD_SINCE_2000: f64 = 2451545.;
const SECONDS_PER_HOUR: f64 = 60. * 60.;
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
const REFRACTION_C3: f64 = 60.;

pub struct Sun;
impl SkyObject for Sun {
    fn period(&self) -> Duration {
        Duration::days(1)
    }

    // source: https://de.wikipedia.org/wiki/Sonnenstand
    fn position(&self, time: &NaiveDateTime, location: &Location) -> SkyPosition {
        // ecliptic coordinates
        let n = julian::day_of(time) - JD_SINCE_2000;

        let l = MEAN_ECLIPTIC_LENGTH_C0 + MEAN_ECLIPTIC_LENGTH_C1 * n;
        let g = MEAN_ECLIPTIC_ANOMALY_C0 + MEAN_ECLIPTIC_ANOMALY_C1 * n;
        let lambda = l + ECLIPTIC_LENGTH_C0 * g.sin() + ECLIPTIC_LENGTH_C1 * (2. * g).sin();

        let epsilon = SKEW_OF_ECLIPTIC_C0 + n * SKEW_OF_ECLIPTIC_C1;

        // equatorial coordinates
        let mut alpha = (epsilon.cos() * lambda.tan()).atan();
        if lambda.cos() < 0. {
            alpha += PI;
        }
        let delta = (epsilon.sin() * lambda.sin()).asin();

        // horizontal coordinates
        let t0 = (julian::day_of_midnight(time) - JD_SINCE_2000) / DAYS_PER_CENTURY;
        let t = time.num_seconds_from_midnight() as f64 / SECONDS_PER_HOUR
            + time.nanosecond() as f64 / NANOSECONDS_PER_HOUR;
        let theta_g_h = STAR_TIME_C0 + STAR_TIME_C1 * t0 + STAR_TIME_C2 * t;
        let theta_g = theta_g_h * 15f64.to_radians();

        let theta = theta_g + location.lon.to_radians();
        let tau = (theta - alpha).normalize_radians();

        let phi = location.lat.to_radians();
        let azimuth_enumerator = tau.cos() * phi.sin() - delta.tan() * phi.cos();
        let mut azimuth = (tau.sin()).atan2(azimuth_enumerator);
        azimuth += PI;

        let altitude = (delta.cos() * tau.cos() * phi.cos() + delta.sin() * phi.sin()).asin();
        /* This is necessary in the source, but in practice produces wrong results...
        if azimuth_enumerator < 0. {
            altitude += PI;
        }
        */
        let altitude = altitude.to_degrees(); // needed in degrees for correction

        // correction of altitude
        let r = REFRACTION_C0
            / (altitude + REFRACTION_C1 / (altitude + REFRACTION_C2))
                .to_radians()
                .tan();

        let altitude = (altitude + r / REFRACTION_C3).to_radians(); // result in radians again

        // normalize
        let mut altitude = altitude.normalize_radians();
        if altitude > FRAC_PI_2 {
            altitude -= TAU; // => altitude \in [-PI/2, PI/2]
        }

        let azimuth = azimuth.normalize_radians();

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
