use chrono::{Duration, NaiveDateTime};

use crate::{julian, Location, SkyObject, SkyPosition};

use super::util;

pub struct Moon;
impl SkyObject for Moon {
    fn period(&self) -> Duration {
        Duration::days(1)
    }

    // source: http://www.stjarnhimlen.se/comp/tutorial.html#7
    fn position(&self, time: &NaiveDateTime, location: &Location) -> SkyPosition {
        // ecliptic coordinates
        let d = julian::day_of(time) - 2451543.5;

        let alpha = (308.3616f64).to_radians();
        let delta = (-0.3937f64).to_radians();

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
