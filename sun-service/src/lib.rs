use std::error::Error;

use chrono::{DateTime, Utc};
use thiserror::Error;

pub mod angle;
pub mod horizon;
pub mod julian;
pub mod location;
pub mod sky;

pub mod messaging;

pub use horizon::{Horizon, HORIZON_SAMPLES};
pub use location::Location;
pub use sky::{SkyObject, SkyPosition};

pub struct HorizonEvents {
    pub rise: DateTime<Utc>,
    pub set: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum HorizonEventError {
    #[error("could not determine rise and set candidate ranges")]
    CandidateRange,
}

type CandidateRange = (DateTime<Utc>, DateTime<Utc>);

pub fn calculate_rise_and_set<O>(
    object: O,
    time: &DateTime<Utc>,
    location: &Location,
    horizon: &Horizon,
) -> Result<HorizonEvents, Box<dyn Error + Send + Sync>>
where
    O: SkyObject,
{
    let (rise_range, set_range) = calculate_candidate_ranges(&object, time, location, horizon)?;

    let rise = calculate_horizon_point(&object, rise_range, location, horizon);
    let set = calculate_horizon_point(&object, set_range, location, horizon);

    Ok(HorizonEvents { rise, set })
}

const MAX_RESOLUTION_EXP: usize = 5;

#[derive(PartialEq)]
enum CandidateType {
    Rise,
    Set,
}

fn calculate_candidate_ranges<O>(
    object: &O,
    time: &DateTime<Utc>,
    location: &Location,
    horizon: &Horizon,
) -> Result<(CandidateRange, CandidateRange), HorizonEventError>
where
    O: SkyObject,
{
    for r in 1..MAX_RESOLUTION_EXP {
        let duration = object.period() / r as i32;
        let candidates: Vec<(CandidateRange, CandidateType)> = (0..(2i32.pow(r as u32)))
            .filter_map(|i| {
                let left = time.checked_add_signed(duration * i)?;
                let right = left.checked_add_signed(duration)?;

                let left_up = is_up(object, &left, location, horizon);
                let right_up = is_up(object, &right, location, horizon);

                if left_up != right_up {
                    let candidate_type = if left_up {
                        CandidateType::Set
                    } else {
                        CandidateType::Rise
                    };
                    Some(((left, right), candidate_type))
                } else {
                    None
                }
            })
            .take(2)
            .collect();

        if candidates.len() == 2 && candidates[0].1 != candidates[1].1 {
            if candidates[0].1 == CandidateType::Rise {
                return Ok((candidates[0].0, candidates[1].0));
            } else {
                return Ok((candidates[1].0, candidates[0].0));
            }
        }
    }

    Err(HorizonEventError::CandidateRange)
}

const TOLERANCE: f64 = 1e-5;

fn calculate_horizon_point<O>(
    object: &O,
    range: CandidateRange,
    location: &Location,
    horizon: &Horizon,
) -> DateTime<Utc>
where
    O: SkyObject,
{
    let (mut left, mut right) = range;

    // If the left altitude is less than the horizon, we are searching for a rise
    // Swapping left and right will allow us to reuse the algorithm for finding a set below
    let SkyPosition { altitude, azimuth } = object.position(&left, location);
    let left_horizontarget_altitude = horizon.altitude_at(azimuth);
    if altitude < left_horizontarget_altitude {
        (right, left) = (left, right);
    }

    loop {
        let difference = right - left;
        let middle = left
            .checked_add_signed(difference / 2)
            .expect("should never overflow");

        let SkyPosition { altitude, azimuth } = object.position(&middle, location);
        let target_altitude = horizon.altitude_at(azimuth);

        if (altitude - target_altitude).abs() < TOLERANCE {
            return middle;
        } else if altitude > target_altitude {
            left = middle;
        } else {
            right = middle;
        }
    }
}

fn is_up<O>(object: &O, time: &DateTime<Utc>, location: &Location, horizon: &Horizon) -> bool
where
    O: SkyObject,
{
    let SkyPosition {
        altitude: obj_altitude,
        azimuth,
    } = object.position(time, location);
    let hor_altitude = horizon.altitude_at(azimuth.to_radians());

    obj_altitude > hor_altitude
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};

    use crate::{
        calculate_candidate_ranges,
        horizon::{Horizon, HORIZON_SAMPLES},
        location::Location,
        sky::{SkyObject, SkyPosition},
    };

    const SECONDS_IN_DAY: u32 = 24 * 60 * 60;

    struct TestSkyObject;

    impl SkyObject for TestSkyObject {
        fn new() -> Self {
            TestSkyObject {}
        }

        fn period(&self) -> Duration {
            Duration::days(1)
        }

        fn position(
            &self,
            time: &chrono::DateTime<chrono::Utc>,
            _location: &crate::location::Location,
        ) -> crate::sky::SkyPosition {
            let seconds = time.num_seconds_from_midnight() as f64;

            let azimuth = 2. * PI * (seconds / SECONDS_IN_DAY as f64);
            let altitude = -(PI / 2.) * azimuth.cos();

            SkyPosition { altitude, azimuth }
        }
    }

    #[test]
    fn candidate_ranges_flat() {
        let altitudes = [0.; HORIZON_SAMPLES];
        let horizon = Horizon::new(altitudes);

        let test_object = TestSkyObject {};
        let time: DateTime<Utc> = DateTime::from_utc(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2006, 8, 6).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ),
            Utc,
        );

        let location = Location {
            lat: 48.1,
            lon: 11.6,
        };

        let (rise_range, set_range) =
            calculate_candidate_ranges(&test_object, &time, &location, &horizon).unwrap();

        assert_eq!(0, rise_range.0.hour());
        assert_eq!(0, rise_range.0.minute());

        assert_eq!(12, rise_range.1.hour());
        assert_eq!(0, rise_range.1.minute());

        assert_eq!(12, set_range.0.hour());
        assert_eq!(0, set_range.0.minute());

        assert_eq!(0, set_range.1.hour());
        assert_eq!(0, set_range.1.minute());
    }
}
