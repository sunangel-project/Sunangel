use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};

const SECONDS_PER_DAY: f64 = (60 * 60 * 24) as f64;

/// Return the julian day number of the given DateTime
///
/// Source: https://github.com/soniakeys/meeus/blob/master/v3/julian/julian.go
///
/// Examples
///
/// ```
/// use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
/// use sun_service::julian;
///
/// let time: DateTime<Utc> = DateTime::from_utc(
///     NaiveDateTime::new(
///         NaiveDate::from_ymd_opt(2006, 8, 6).unwrap(),
///         NaiveTime::from_hms_opt(6, 0, 9).unwrap(),
///     ),
///     Utc,
/// );
///
/// let jd = julian::day_of(time);
///
/// let epsilon = 0.0005;
/// assert!((jd - 2453953.75).abs() < epsilon);
/// ```
pub fn day_of(time: DateTime<Utc>) -> f64 {
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
