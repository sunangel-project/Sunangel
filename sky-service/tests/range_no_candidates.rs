use std::fs;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use sky_service::{
    sky::moon::Moon, Horizon, HorizonEvent, HorizonEvents, Location, HORIZON_SAMPLES,
};

#[test]
fn test_no_range_found() {
    let mut horizon = Vec::new();
    let altitudes_data =
        fs::read_to_string("tests/Data/horizon-v1.0.0-dd8a326c-5065-5fdb-80ef-d033e6e34270.dat")
            .unwrap();
    for line in altitudes_data.split("\n").filter(|l| l.len() > 0) {
        let val = line.parse::<f64>().unwrap();
        horizon.push(val);
    }
    let altitudes: [f64; HORIZON_SAMPLES] = horizon.try_into().unwrap();
    let horizon = Horizon::new(altitudes);

    let time = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2023, 10, 16).unwrap(),
        NaiveTime::from_hms_opt(16, 24, 0).unwrap(),
    );

    let location = Location {
        lat: 48.8300769,
        lon: 9.5739522,
    };

    let HorizonEvents {
        rise: _,
        set: HorizonEvent { time: set, .. },
    } = sky_service::calculate_rise_and_set(&Moon, &time, &location, &horizon).unwrap();

    assert_eq!(set.hour(), 16);
    assert_eq!(set.minute(), 31);
}
