use std::error::Error;

use chrono::{DateTime, Utc};
use thiserror::Error;

use horizon::Horizon;
use location::Location;
use sky::{SkyObject, SkyPosition};

mod angle;
mod horizon;
mod julian;
mod location;
pub mod sky;

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
) -> Result<HorizonEvents, Box<dyn Error>>
where
    O: SkyObject,
{
    let (rise_range, set_range) = calculate_candidate_ranges(&object, time, location, horizon)?;

    // Binary search candidate ranges

    Ok(HorizonEvents {
        rise: Utc::now(),
        set: Utc::now(),
    })
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
    for r in 0..MAX_RESOLUTION_EXP {
        let duration = object.period() / r as i32;
        let candidates: Vec<(CandidateRange, CandidateType)> = (0..(2i32.pow(r as u32)))
            .filter_map(|i| {
                let left = time.checked_add_signed(duration * i)?;
                let right = left.checked_add_signed(duration)?;

                let left_up = is_up(object, &left, location, horizon);
                let right_up = is_up(object, &right, location, horizon);

                if left_up != right_up {
                    let candidate_type = if left_up {
                        CandidateType::Rise
                    } else {
                        CandidateType::Set
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
