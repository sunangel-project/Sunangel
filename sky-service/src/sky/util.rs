use std::f64::consts::{FRAC_PI_2, PI, TAU};

use chrono::{NaiveDateTime, Timelike};

use crate::{angle::AngleExtensions, julian, Location};

const SECONDS_PER_HOUR: f64 = 60. * 60.;
const NANOSECONDS_PER_HOUR: f64 = SECONDS_PER_HOUR * 1e9;

const STAR_TIME_C0: f64 = 6.697376;
const STAR_TIME_C1: f64 = 2400.05134;
const STAR_TIME_C2: f64 = 1.002738;

const REFRACTION_C0: f64 = 1.02;
const REFRACTION_C1: f64 = 10.3;
const REFRACTION_C2: f64 = 5.11;
const REFRACTION_C3: f64 = 60.;

fn sidereal_time(time: &NaiveDateTime, location: &Location, alpha: f64) -> f64 {
    let t0 = julian::centuries_of_midnight_since_2000(time);
    let t = time.num_seconds_from_midnight() as f64 / SECONDS_PER_HOUR
        + time.nanosecond() as f64 / NANOSECONDS_PER_HOUR;
    let theta_g_h = STAR_TIME_C0 + STAR_TIME_C1 * t0 + STAR_TIME_C2 * t;
    let theta_g = theta_g_h * 15f64.to_radians();

    let theta = theta_g + location.lon.to_radians();
    (theta - alpha).normalize_radians()
}

pub fn convert_ecliptic_to_horizontal(
    time: &NaiveDateTime,
    location: &Location,
    alpha: f64,
    delta: f64,
) -> (f64, f64) {
    let tau = sidereal_time(time, location, alpha);

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

    // normalize
    let mut altitude = altitude.normalize_radians();
    if altitude > FRAC_PI_2 {
        altitude -= TAU; // => altitude \in [-PI/2, PI/2]
    }

    let azimuth = azimuth.normalize_radians();

    (altitude, azimuth)
}

pub fn refraction(altitude: f64) -> f64 {
    let altitude = altitude.to_degrees();
    let r = REFRACTION_C0
        / (altitude + REFRACTION_C1 / (altitude + REFRACTION_C2))
            .to_radians()
            .tan();

    (altitude + r / REFRACTION_C3).to_radians()
}
