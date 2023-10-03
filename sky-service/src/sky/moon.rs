use std::f64::consts::PI;

use chrono::{Duration, NaiveDateTime};

use crate::{angle::AngleExtensions, julian, Location, SkyObject, SkyPosition};

use super::util;

const LONG_ASC_NODE_0: f64 = 125.1228f64 * PI / 180.;
const LONG_ASC_NODE_1: f64 = 0.0529538083 * PI / 180.;

const INCLINATION: f64 = 5.1454 * PI / 180.;
const MEAN_DISTANCE: f64 = 60.2666;
const ECCENTRICITY: f64 = 0.0549;
const ECCENTRICITY_EPS: f64 = 0.005 * PI / 180.;

const ARG_OF_PERIGEE_0: f64 = 318.0634 * PI / 180.;
const ARG_OF_PERIGEE_1: f64 = 0.1643573223 * PI / 180.;

const MEAN_ANOMALY_0: f64 = 115.3654 * PI / 180.;
const MEAN_ANOMALY_1: f64 = 13.0649929509 * PI / 180.;

pub struct Moon;
impl SkyObject for Moon {
    fn period(&self) -> Duration {
        Duration::days(1)
    }

    // source: http://www.stjarnhimlen.se/comp/tutorial.html#7
    #[allow(non_snake_case)]
    fn position(&self, time: &NaiveDateTime, location: &Location) -> SkyPosition {
        let d = julian::day_of(time) - 2451543.5;

        let N = (LONG_ASC_NODE_0 - LONG_ASC_NODE_1 * d).normalize_radians();
        let w = (ARG_OF_PERIGEE_0 + ARG_OF_PERIGEE_1 * d).normalize_radians();
        let M = (MEAN_ANOMALY_0 + MEAN_ANOMALY_1 * d).normalize_radians();

        // Eccentric anomaly
        let mut E0 =
            (M + ECCENTRICITY * M.sin() * (1. + ECCENTRICITY * M.cos())).normalize_radians();
        loop {
            let E1 = (E0 - (E0 - ECCENTRICITY * E0.sin() - M) / (1. - ECCENTRICITY * E0.cos()))
                .normalize_radians();

            let delta = (E0 - E1).abs();
            E0 = E1;
            if delta < ECCENTRICITY_EPS {
                break;
            }
        }
        let E = E0;

        // Rectangular coordinates
        let x = MEAN_DISTANCE * (E.cos() - ECCENTRICITY);
        let y = MEAN_DISTANCE * (1. - ECCENTRICITY.powi(2)).sqrt() * E.sin();

        // Distance and true anomaly
        let r = (x.powi(2) + y.powi(2)).sqrt();
        let v = y.atan2(x);

        // Intermediate ecliptic coordinates
        let vwsin = (v + w).sin();
        let xeclip = r * (N.cos() * (v + w).cos() - N.sin() * vwsin * INCLINATION.cos());
        let yeclip = r * (N.sin() * (v + w).cos() + N.cos() * vwsin * INCLINATION.cos());
        let zeclip = r * vwsin * INCLINATION.sin();

        // Ecliptic coordinates
        let alpha = yeclip.atan2(xeclip);
        let delta = zeclip.atan2((xeclip.powi(2) + yeclip.powi(2)).sqrt());

        let (altitude, azimuth) =
            util::convert_ecliptic_to_horizontal(time, location, alpha, delta);

        SkyPosition { altitude, azimuth }
    }
}

#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveTime};

    use crate::util::assert_approx_eq;

    use super::*;

    #[test]
    fn moon_position_website() {
        let time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(1990, 4, 19).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(), // 19 april 1990
        );

        let location = Location {
            lat: 48.1,
            lon: 11.6,
        };

        let pos = Moon.position(&time, &location);

        assert_approx_eq(pos.altitude, 0.004801301260915783);
        assert_approx_eq(pos.azimuth, 1.5763198897358466);
    }
}
