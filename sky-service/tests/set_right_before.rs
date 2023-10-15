use std::fs;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use sky_service::{sky::sun::Sun, Horizon, HorizonEvent, HorizonEvents, Location, HORIZON_SAMPLES};

#[test]
fn sanity_test_sun_just_before_set() {
    let mut horizon = Vec::new();
    let altitudes_data = fs::read_to_string("tests/Data/aussicht_horizon.dat").unwrap();
    for line in altitudes_data.split("\n").filter(|l| l.len() > 0) {
        let val = line.parse::<f64>().unwrap();
        horizon.push(val);
    }
    let altitudes: [f64; HORIZON_SAMPLES] = horizon.try_into().unwrap();
    let horizon = Horizon::new(altitudes);

    let time = NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2023, 10, 14).unwrap(),
        NaiveTime::from_hms_opt(16, 24, 0).unwrap(),
    );

    let location = Location {
        lat: 48.818,
        lon: 9.587,
    };

    let HorizonEvents {
        rise: _,
        set: HorizonEvent { time: set, .. },
    } = sky_service::calculate_rise_and_set(&Sun, &time, &location, &horizon).unwrap();

    assert_eq!(set.hour(), 16);
    assert_eq!(set.minute(), 31);
}
