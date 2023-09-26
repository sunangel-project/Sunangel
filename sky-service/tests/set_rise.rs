use std::f64::consts::PI;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

use sky_service::Horizon;
use sky_service::HorizonEvent;
use sky_service::HorizonEvents;
use sky_service::Location;
use sky_service::SkyObject;
use sky_service::SkyPosition;
use sky_service::HORIZON_SAMPLES;

const SECONDS_IN_DAY: u32 = 24 * 60 * 60;

struct TestSkyObject;

impl SkyObject for TestSkyObject {
    fn new() -> Self {
        TestSkyObject {}
    }

    fn period(&self) -> Duration {
        Duration::days(1)
    }

    fn position(&self, time: &NaiveDateTime, _location: &Location) -> SkyPosition {
        let seconds = time.num_seconds_from_midnight() as f64;

        let azimuth = 2. * PI * (seconds / SECONDS_IN_DAY as f64);
        let altitude = -(PI / 2.) * azimuth.cos();

        SkyPosition { altitude, azimuth }
    }
}

#[test]
fn set_flat() {
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

    let HorizonEvents {
        rise: HorizonEvent { time: rise, .. },
        set: HorizonEvent { time: set, .. },
    } = sky_service::calculate_rise_and_set(test_object, &time, &location, &horizon).unwrap();

    assert_eq!(rise.hour(), 6);
    assert_eq!(rise.minute(), 0);
    assert_eq!(rise.second(), 0);

    assert_eq!(set.hour(), 18);
    assert_eq!(set.minute(), 0);
    assert_eq!(set.second(), 0);
}
