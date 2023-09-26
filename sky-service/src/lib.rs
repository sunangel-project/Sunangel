use std::error::Error;

use chrono::{Duration, NaiveDateTime};
use log::warn;
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize)]
pub struct HorizonEvent {
    pub time: NaiveDateTime,
    pub altitude: f64,
    pub azimuth: f64,
}

#[derive(Serialize, Deserialize)]
pub struct HorizonEvents {
    pub rise: HorizonEvent,
    pub set: HorizonEvent,
}

#[derive(Debug, Error)]
pub enum HorizonEventError {
    #[error("could not determine rise and set candidate ranges")]
    CandidateRange,
}

type CandidateRange = (NaiveDateTime, NaiveDateTime);

pub fn calculate_rise_and_set<O>(
    object: O,
    time: &NaiveDateTime,
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
    time: &NaiveDateTime,
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

const TOLERANCE: f64 = 1e-3; // About 0.06°

fn calculate_horizon_point<O>(
    object: &O,
    range: CandidateRange,
    location: &Location,
    horizon: &Horizon,
) -> HorizonEvent
where
    O: SkyObject,
{
    let (mut left, mut right) = range;

    // If the left altitude is less than the horizon, we are searching for a rise
    // Swapping left and right will allow us to reuse the algorithm for finding a set below
    let SkyPosition { altitude, azimuth } = object.position(&left, location);
    let left_horizon_altitude = horizon.altitude_at(azimuth);
    if altitude < left_horizon_altitude {
        (right, left) = (left, right);
    }

    loop {
        let difference = right - left;
        let middle = left
            .checked_add_signed(difference / 2)
            .expect("should never overflow");

        let SkyPosition { altitude, azimuth } = object.position(&middle, location);
        let target_altitude = horizon.altitude_at(azimuth);

        if (left - right).num_milliseconds().abs() < Duration::seconds(1).num_milliseconds() {
            warn!("Below 1s interval: {middle},");
            warn!("{altitude}, target: {target_altitude}");
            // Most of the time ok, sometimes horribly wrong
            // TODO: catch horribly wrong results
            return HorizonEvent {
                time: middle,
                altitude,
                azimuth,
            };
        }

        if (altitude - target_altitude).abs() < TOLERANCE {
            return HorizonEvent {
                time: middle,
                altitude,
                azimuth,
            };
        } else if altitude > target_altitude {
            left = middle;
        } else {
            right = middle;
        }
    }
}

fn is_up<O>(object: &O, time: &NaiveDateTime, location: &Location, horizon: &Horizon) -> bool
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

    use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

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
            time: &NaiveDateTime,
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
        let time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2006, 8, 6).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
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
