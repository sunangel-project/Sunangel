use std::f64::consts::PI;

/// Normalize an angle in degrees
///
/// d -> d mod 360
///
/// # Arguments
///
/// * `d` - The angle in degrees
///
/// # Examples
///
/// ```
/// use sun_service::angle;
///
/// let normalized = angle::normalize_degrees(360. + 12.5);
///
/// assert_eq!(normalized, 12.5);
/// ```
pub fn normalize_degrees(d: f64) -> f64 {
    modulo(d, 360.)
}

/// Normalize an angle in radians
///
/// r -> r mod 2 pi
///
/// # Arguments
///
/// * `r` - The angle in radians
///
/// # Examples
///
/// ```
/// use std::f64::consts::PI;
/// use sun_service::angle;
///
/// let normalized = angle::normalize_radians(2. * PI + 1.);
///
/// assert_eq!(normalized, 1.);
/// ```
pub fn normalize_radians(r: f64) -> f64 {
    modulo(r, 2. * PI)
}

fn modulo(a: f64, b: f64) -> f64 {
    a - (a / b).trunc() * b
}
