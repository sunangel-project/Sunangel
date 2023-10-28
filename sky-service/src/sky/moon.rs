use std::f64::consts::PI;

use chrono::{Duration, NaiveDateTime};

use crate::{angle::AngleExtensions, julian, Location, SkyObject, SkyPosition};

use super::util;

const LONG_ASC_NODE_0: f64 = 125.1228f64 * PI / 180.;
const LONG_ASC_NODE_1: f64 = 0.0529538083 * PI / 180.;

const SUN_LONG_ASC_NODE_0: f64 = 282.9404 * PI / 180.;
const SUN_LONG_ASC_NODE_1: f64 = 4.70935e-5 * PI / 180.;

const INCLINATION: f64 = 5.1454 * PI / 180.;
const MEAN_DISTANCE: f64 = 60.2666;
const ECCENTRICITY: f64 = 0.0549;
const ECCENTRICITY_EPS: f64 = 0.005 * PI / 180.;

const ARG_OF_PERIGEE_0: f64 = 318.0634 * PI / 180.;
const ARG_OF_PERIGEE_1: f64 = 0.1643573223 * PI / 180.;

const MEAN_ANOMALY_0: f64 = 115.3654 * PI / 180.;
const MEAN_ANOMALY_1: f64 = 13.0649929509 * PI / 180.;

const SUN_MEAN_ANOMALY_0: f64 = 356.0470 * PI / 180.;
const SUN_MEAN_ANOMALY_1: f64 = 0.9856002585 * PI / 180.;

const OBLIQUITY_ECLIPTIC_0: f64 = 23.4393 * PI / 180.;
const OBLIQUITY_ECLIPTIC_1: f64 = 3.563e-7 * PI / 180.;

const PERT_LON_0: f64 = -1.274 * PI / 180.;
const PERT_LON_1: f64 = 0.658 * PI / 180.;
const PERT_LON_2: f64 = 0.186 * PI / 180.;
const PERT_LON_3: f64 = 0.059 * PI / 180.;
const PERT_LON_4: f64 = 0.057 * PI / 180.;
const PERT_LON_5: f64 = 0.053 * PI / 180.;
const PERT_LON_6: f64 = 0.046 * PI / 180.;
const PERT_LON_7: f64 = 0.041 * PI / 180.;
const PERT_LON_8: f64 = 0.035 * PI / 180.;
const PERT_LON_9: f64 = 0.031 * PI / 180.;
const PERT_LON_10: f64 = 0.015 * PI / 180.;
const PERT_LON_11: f64 = 0.011 * PI / 180.;

const PERT_LAT_0: f64 = 0.173 * PI / 180.;
const PERT_LAT_1: f64 = 0.055 * PI / 180.;
const PERT_LAT_2: f64 = 0.046 * PI / 180.;
const PERT_LAT_3: f64 = 0.033 * PI / 180.;
const PERT_LAT_4: f64 = 0.017 * PI / 180.;

const PERT_R_0: f64 = 0.58 * PI / 180.;
const PERT_R_1: f64 = 0.46 * PI / 180.;

pub struct Moon;
impl SkyObject for Moon {
    fn period(&self) -> Duration {
        Duration::hours(26)
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

        // Ecliptic coordinates
        let vwsin = (v + w).sin();
        let xeclip = r * (N.cos() * (v + w).cos() - N.sin() * vwsin * INCLINATION.cos());
        let yeclip = r * (N.sin() * (v + w).cos() + N.cos() * vwsin * INCLINATION.cos());
        let zeclip = r * vwsin * INCLINATION.sin();

        let mut mlon = yeclip.atan2(xeclip);
        let xeclip_yeclip_squared = xeclip.powi(2) + yeclip.powi(2);
        let mut mlat = zeclip.atan2((xeclip_yeclip_squared).sqrt());
        let mut r = (xeclip_yeclip_squared + zeclip.powi(2)).sqrt();

        // Pertubations

        let (Ls, Ms) = sun_mean_length_and_anomaly(d);
        let Lm = N + w + M;
        let Mm = M;
        let D = Lm - Ls;
        let F = Lm - N;

        let d2 = 2. * d;
        mlon += PERT_LON_0 * (M - d2).sin()
            + PERT_LON_1 * d2.sin()
            + PERT_LON_2 * Ms.sin()
            + PERT_LON_3 * (2. * Mm - d2).sin()
            + PERT_LON_4 * (Mm - d2 + Ms).sin()
            + PERT_LON_5 * (Mm + d2).sin()
            + PERT_LON_6 * (d2 - Ms).sin()
            + PERT_LON_7 * (Mm - Ms).sin()
            + PERT_LON_8 * D.sin()
            + PERT_LON_9 * (Mm + Ms).sin()
            + PERT_LON_10 * (2. * F - d2).sin()
            + PERT_LON_11 * (Mm - 4. * D).sin();

        mlat += PERT_LAT_0 * (F - d2).sin()
            + PERT_LAT_1 * (Mm - F - d2).sin()
            + PERT_LAT_2 * (Mm + F - d2).sin()
            + PERT_LAT_3 * (F + d2).sin()
            + PERT_LAT_4 * (2. * Mm + F).sin();

        r += PERT_R_0 * (Mm - d2).cos() + PERT_R_1 * d2.cos();

        // Equatorial coordinates
        let oblecl = OBLIQUITY_ECLIPTIC_0 - OBLIQUITY_ECLIPTIC_1 * d;

        let (mlon_sin, mlon_cos) = mlon.sin_cos();
        let (mlat_sin, mlat_cos) = mlat.sin_cos();
        let xeclip = mlon_cos * mlat_cos;
        let yeclip = mlon_sin * mlat_cos;
        let zeclip = mlat_sin;

        let xequat = xeclip;
        let (oblecl_sin, oblecl_cos) = oblecl.sin_cos();
        let yequat = yeclip * oblecl_cos - zeclip * oblecl_sin;
        let zequat = yeclip * oblecl_sin + zeclip * oblecl_cos;

        let alpha = yequat.atan2(xequat);
        let delta = zequat.atan2((xequat.powi(2) + yequat.powi(2)).sqrt());

        // Ecliptic coordinates for the observer
        let mpar = (1. / r).asin();

        let (altitude, azimuth) =
            util::convert_ecliptic_to_horizontal(time, location, alpha, delta);

        // Correct for parallax
        let altitude = altitude - mpar * altitude.cos();

        let altitude = util::refraction(altitude);

        SkyPosition { altitude, azimuth }
    }
}

#[allow(non_snake_case)]
fn sun_mean_length_and_anomaly(d: f64) -> (f64, f64) {
    let w = SUN_LONG_ASC_NODE_0 + SUN_LONG_ASC_NODE_1 * d;
    let M = SUN_MEAN_ANOMALY_0 + SUN_MEAN_ANOMALY_1 * d;
    let L = w + M;
    (L.normalize_radians(), M.normalize_radians())
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

        assert_approx_eq(pos.altitude, -0.28403595507008245);
        assert_approx_eq(pos.azimuth, 1.7608876071448318);
    }
}
