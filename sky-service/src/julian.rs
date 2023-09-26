use chrono::{Datelike, NaiveDateTime, NaiveTime, Timelike};

const SECONDS_PER_DAY: f64 = (60 * 60 * 24) as f64;

/// The julian day number of the given date
///
/// Source: https://github.com/soniakeys/meeus/blob/master/v3/julian/julian.go
///
/// # Example
///
/// ```
/// use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
/// use sky_service::julian;
///
/// let time = NaiveDateTime::new(
///     NaiveDate::from_ymd_opt(2006, 8, 6).unwrap(),
///     NaiveTime::from_hms_opt(6, 0, 9).unwrap(),
/// );
///
/// let jd = julian::day_of(&time);
///
/// let epsilon = 0.0005;
/// assert!((jd - 2453953.75).abs() < epsilon);
/// ```
pub fn day_of(time: &NaiveDateTime) -> f64 {
    let mut year = time.year();
    let mut month = time.month();

    if month < 3 {
        year -= 1;
        month += 12;
    }

    let year = year as i64;
    let month = month as i64;

    let days = time.day() as f64 + time.num_seconds_from_midnight() as f64 / SECONDS_PER_DAY;

    let a = year / 100;
    let b = 2 - a + a / 4;

    ((36525 * (year + 4716)) / 100) as f64 + ((306 * (month + 1) / 10) + b) as f64 + days - 1524.5
}

/// The julian day number at midnight of a given date
///
/// # Example
///
/// ```
/// use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
/// use sky_service::julian;
///
/// let time = NaiveDateTime::new(
///     NaiveDate::from_ymd_opt(2006, 8, 6).unwrap(),
///     NaiveTime::from_hms_opt(6, 0, 9).unwrap(),
/// );
///
/// let jd = julian::day_of_midnight(&time);
///
/// let epsilon = 0.0005;
/// assert!((jd - 2453953.5).abs() < epsilon);
/// ```
pub fn day_of_midnight(time: &NaiveDateTime) -> f64 {
    let time = NaiveDateTime::new(
        time.date(),
        NaiveTime::from_num_seconds_from_midnight_opt(0, 0)
            .expect("Agruments are staic 0, this cannot be None"),
    );
    day_of(&time)
}
