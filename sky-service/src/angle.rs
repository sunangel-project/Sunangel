use std::f64::consts::TAU;

/// Extensions for normalizing angles
pub trait AngleExtensions {
    /// Normalize angles given in radians.
    fn normalize_radians(self) -> Self;
    /// Normalize angles given in degrees.
    fn normalize_degrees(self) -> Self;
}

impl AngleExtensions for f64 {
    /// Normalize angles given in radians.
    ///
    /// ```
    /// use std::f64::consts::TAU;
    /// use sky_service::angle::AngleExtensions;
    ///
    /// let angle =  1f64 + 20. * TAU;
    /// let normalized = angle.normalize_radians();
    ///
    /// assert_eq!(1f64, normalized);
    /// ```
    fn normalize_radians(self) -> Self {
        self.rem_euclid(TAU)
    }

    /// Normalize angles given in degrees.
    ///
    /// ```
    /// use sky_service::angle::AngleExtensions;
    ///
    /// let angle =  1f64 + 20. * 360.;
    /// let normalized = angle.normalize_radians();
    ///
    /// assert_eq!(1f64, normalized);
    /// ```
    fn normalize_degrees(self) -> Self {
        self.rem_euclid(360.)
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::TAU;

    use crate::{angle::AngleExtensions, util::assert_precisely_eq};

    #[test]
    fn normalize_radians() {
        let want = 2. / 3.;
        let high = want + 1000. * TAU;
        assert_precisely_eq(high.normalize_radians(), want);
    }
}
