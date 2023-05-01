use super::{SkyObject, SkyPosition};

struct Sun;

impl SkyObject for Sun {
    fn position(
        time: chrono::DateTime<chrono::Utc>,
        location: crate::location::Location,
    ) -> super::SkyPosition {
        let solar_pos = spa::calc_solar_position(time, location.lat, location.lon)
            .expect("Coordinates should always be valid");

        SkyPosition {
            altitude: 90. - solar_pos.zenith_angle,
            azimuth: solar_pos.azimuth - 180.,
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

        let pos = Sun::position(time, location);

        let epsilon = 0.05;
        assert!(
            (pos.altitude - 19.110).abs() < epsilon,
            "altitude was {}",
            pos.altitude
        );
        assert!(
            (pos.azimuth + 94.062).abs() < epsilon,
            "azimuth was {}",
            pos.azimuth
        );
    }
}
