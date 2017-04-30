use super::capi::*;
use super::def::{Mpfr, ParseMpfrError};

use fp;
use fp::{Float, RoundingMode};

use std::fmt;
use std::fmt::{Display, Formatter};
use std::mem::{uninitialized};
use std::str::FromStr;

impl From<RoundingMode> for MpfrRnd {
    #[inline]
    fn from(other: RoundingMode) -> Self {
        match other {
            RoundingMode::HalfToEven => MpfrRnd::HalfToEven,
            RoundingMode::HalfAwayFromZero => MpfrRnd::HalfAwayFromZero,
            RoundingMode::Up => MpfrRnd::Up,
            RoundingMode::Down => MpfrRnd::Down,
            RoundingMode::TowardsZero => MpfrRnd::TowardsZero,
            RoundingMode::AwayFromZero => MpfrRnd::AwayFromZero,
        }
    }
}

impl Mpfr {
    #[inline]
    pub fn new(precision: usize) -> Self {
        let mut mpfr = unsafe { uninitialized() };
        unsafe { mpfr_init2(&mut mpfr, precision as MpfrPrec) };
        Self { mpfr: mpfr }
    }

    #[inline]
    pub fn set_zero(mut self) -> Self {
        unsafe { mpfr_set_zero(&mut self.mpfr, 1) };
        self
    }

    #[inline]
    pub fn set_neg_zero(mut self) -> Self {
        unsafe { mpfr_set_zero(&mut self.mpfr, -1) };
        self
    }

    #[inline]
    pub fn set_pos_infinity(mut self) -> Self {
        unsafe { mpfr_set_inf(&mut self.mpfr, 1) };
        self
    }

    #[inline]
    pub fn set_neg_infinity(mut self) -> Self {
        unsafe { mpfr_set_inf(&mut self.mpfr, -1) };
        self
    }

    #[inline]
    pub fn set_nan(mut self) -> Self {
        unsafe { mpfr_set_nan(&mut self.mpfr) };
        self
    }

    #[inline]
    pub fn set(mut self, val: &Self, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_set(&mut self.mpfr, &val.mpfr, rounding_mode) };
        self
    }

    #[inline]
    pub fn set_f64(mut self, val: f64, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_set_d(&mut self.mpfr, val, rounding_mode) };
        self
    }
}

impl Drop for Mpfr {
    #[inline]
    fn drop(&mut self) {
        unsafe { mpfr_clear(&mut self.mpfr); }
    }
}

impl From<f64> for Mpfr {
    #[inline]
    fn from(val: f64) -> Self {
        <Self as fp::From<f64>>::from(val, 53, RoundingMode::HalfToEven)
    }
}

impl FromStr for Mpfr {
    type Err = ParseMpfrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as fp::FromStr>::from_str(s, 53, RoundingMode::HalfToEven)
    }
}

impl Clone for Mpfr {
    #[inline]
    fn clone(&self) -> Self {
        Self::new(self.precision()).set(self, MpfrRnd::HalfToEven)
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        unsafe { mpfr_set(&mut self.mpfr, &source.mpfr, MpfrRnd::HalfToEven) };
    }
}

impl Display for Mpfr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <&Self as Into<f64>>::into(self).fmt(f)
    }
}

impl Into<f64> for Mpfr {
    #[inline]
    fn into(self) -> f64 {
        fp::Into::<f64>::into(self, RoundingMode::HalfToEven)
    }
}

impl<'a> Into<f64> for &'a Mpfr {
    #[inline]
    fn into(self) -> f64 {
        unsafe { mpfr_get_d(&self.mpfr, MpfrRnd::HalfToEven) }
    }
}

#[cfg(test)]
mod test {
    use super::super::def::Mpfr;

    use fp;
    use fp::RoundingMode;

    use std::str::FromStr;
    use std::f64;

    #[test]
    fn test_from_f64() {
        assert_str_eq!("0", Mpfr::from(0f64));
        assert_str_eq!("0.123", Mpfr::from(0.123));
        assert_str_eq!("-1.23", Mpfr::from(-1.23));
        assert_str_eq!("inf", Mpfr::from(f64::INFINITY));
        assert_str_eq!("-inf", Mpfr::from(f64::NEG_INFINITY));
        assert_str_eq!("NaN", Mpfr::from(f64::NAN));
    }

    #[test]
    fn test_from_str() {
        assert_str_eq!("0", <Mpfr as FromStr>::from_str("0").unwrap());
        assert_str_eq!("0.123", <Mpfr as FromStr>::from_str("0.123").unwrap());
        assert_str_eq!("-1.23", <Mpfr as FromStr>::from_str("-1.23").unwrap());
        assert_str_eq!("inf", <Mpfr as FromStr>::from_str("inf").unwrap());
        assert_str_eq!("-inf", <Mpfr as FromStr>::from_str("-inf").unwrap());
        assert_str_eq!("NaN", <Mpfr as FromStr>::from_str("NaN").unwrap());
        assert!(<Mpfr as FromStr>::from_str("-123.45x").is_err());
    }

    #[test]
    fn test_clone() {
        let x = mpfr!(-123.456);
        assert_eq!(x, x.clone());
        let mut y = mpfr!(-456.123);
        y.clone_from(&x);
        assert_eq!(x, y);
    }

    #[test]
    fn test_display() {
        assert_str_eq!("-123.456", mpfr!(-123.456));
    }

    #[test]
    fn test_into_f64() {
        assert_eq!(-123.456, mpfr!(-123.456).into());
    }
}
