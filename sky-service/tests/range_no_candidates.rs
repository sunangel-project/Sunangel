use std::fs;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use sky_service::{sky::moon::Moon, Horizon, HorizonEvents, Location, HORIZON_SAMPLES};

fn find_range(
    uuid: &str,
    time: NaiveDateTime,
    location: Location,
) -> Result<HorizonEvents, anyhow::Error> {
    let mut horizon = Vec::new();
    let altitudes_data =
        fs::read_to_string(format!("tests/Data/horizon-v1.0.0-{}.dat", uuid)).unwrap();
    for line in altitudes_data.split("\n").filter(|l| l.len() > 0) {
        let val = line.parse::<f64>().unwrap();
        horizon.push(val);
    }
    let altitudes: [f64; HORIZON_SAMPLES] = horizon.try_into().unwrap();
    let horizon = Horizon::new(altitudes);

    sky_service::calculate_rise_and_set(&Moon, &time, &location, &horizon)
}

#[test]
fn test_no_range_found1() {
    let horizon_uuid = "dd8a326c-5065-5fdb-80ef-d033e6e34270";
    let location = Location {
        lat: 48.8300769,
        lon: 9.5739522,
    };
    let time = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2023, 10, 16).unwrap(),
        NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
    );
    find_range(horizon_uuid, time, location).unwrap();
}

// This one should panic
#[test]
fn test_no_range_found2() {
    let horizon_uuid = "dd8a326c-5065-5fdb-80ef-d033e6e34270";
    let location = Location {
        lat: 48.81855,
        lon: 9.5868,
    };
    let time = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2023, 10, 19).unwrap(),
        NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
    );
    let res = find_range(horizon_uuid, time, location);
    assert!(res.is_err_and(|err| {
        err.to_string() == "could not determine rise and set candidate ranges"
    }))
}
