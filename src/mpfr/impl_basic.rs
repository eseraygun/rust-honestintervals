use super::capi::*;
use super::def::{Mpfr, ParseMpfrError};

use fp::Float;

use std::ffi::CString;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::mem::uninitialized;
use std::str::FromStr;

impl Mpfr {
    #[inline]
    pub unsafe fn uninitialized(precision: usize) -> Self {
        let mut mpfr = uninitialized();
        mpfr_init2(&mut mpfr, precision as MpfrPrec);
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
    pub fn set_infinity(mut self) -> Self {
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

    #[inline]
    pub fn set_str(mut self, c: CString, rounding_mode: MpfrRnd) -> Option<Self> {
        if unsafe { mpfr_set_str(&mut self.mpfr, c.as_ptr(), 10, rounding_mode) } == 0 {
            Some(self)
        } else {
            None
        }
    }
}

impl Drop for Mpfr {
    #[inline]
    fn drop(&mut self) {
        unsafe { mpfr_clear(&mut self.mpfr); }
    }
}

impl Mpfr {
    #[inline]
    fn from_custom(val: f64, precision: usize, rounding_mode: MpfrRnd) -> Self {
        unsafe { Self::uninitialized(precision) }.set_f64(val, rounding_mode)
    }
}

impl From<f64> for Mpfr {
    #[inline]
    fn from(val: f64) -> Self {
        Self::from_custom(val, 53, MpfrRnd::HalfToEven)
    }
}

impl Mpfr {
    pub fn from_str_custom(s: &str,
                           precision: usize,
                           rounding_mode: MpfrRnd)
                           -> Result<Self, ParseMpfrError> {
        if let Ok(c) = CString::new(s) {
            if let Some(res) = unsafe { Mpfr::uninitialized(precision) }.set_str(c, rounding_mode) {
                Ok(res)
            } else {
                Err(ParseMpfrError::MpfrError)
            }
        } else {
            Err(ParseMpfrError::CStringError)
        }
    }
}

impl FromStr for Mpfr {
    type Err = ParseMpfrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_custom(s, 53, MpfrRnd::HalfToEven)
    }
}

impl Clone for Mpfr {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { Self::uninitialized(self.precision()) }.set(self, MpfrRnd::HalfToEven)
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        unsafe { mpfr_set(&mut self.mpfr, &source.mpfr, MpfrRnd::HalfToEven) };
    }
}

impl Display for Mpfr {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Into::<f64>::into(self.clone()).fmt(f)
    }
}

impl Mpfr {
    #[inline]
    fn into_custom(self, rounding_mode: MpfrRnd) -> f64 {
        unsafe { mpfr_get_d(&self.mpfr, rounding_mode) }
    }
}

impl Into<f64> for Mpfr {
    #[inline]
    fn into(self) -> f64 {
        self.into_custom(MpfrRnd::HalfToEven)
    }
}

#[cfg(test)]
mod test {
    use super::super::def::Mpfr;

    use fp::Float;

    #[test]
    fn test_from_f64() {
        use std::f64;
        assert_str_eq!("0", Mpfr::from(0f64));
        assert_str_eq!("0.123", Mpfr::from(0.123));
        assert_str_eq!("-1.23", Mpfr::from(-1.23));
        assert_str_eq!("inf", Mpfr::from(f64::INFINITY));
        assert_str_eq!("-inf", Mpfr::from(f64::NEG_INFINITY));
        assert_str_eq!("NaN", Mpfr::from(f64::NAN));
    }

    #[test]
    fn test_from_str() {
        use std::str::FromStr;
        assert_str_eq!("0", Mpfr::from_str("0").unwrap());
        assert_str_eq!("0.123", Mpfr::from_str("0.123").unwrap());
        assert_str_eq!("-1.23", Mpfr::from_str("-1.23").unwrap());
        assert_str_eq!("inf", Mpfr::from_str("inf").unwrap());
        assert_str_eq!("-inf", Mpfr::from_str("-inf").unwrap());
        assert_str_eq!("NaN", Mpfr::from_str("NaN").unwrap());
        assert!(Mpfr::from_str("123a456").is_err());
        assert!(Mpfr::from_str("123\0456").is_err());
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
        use std::f64;
        assert_eq!(0.0, mpfr!(0).into());
        assert_eq!(0.123, Mpfr::into(mpfr!(0.123)));
        assert_eq!(-1.23, Mpfr::into(mpfr!(-1.23)));
        assert_eq!(f64::INFINITY, Mpfr::into(mpfr_inf!()));
        assert_eq!(f64::NEG_INFINITY, Mpfr::into(mpfr_neg_inf!()));
        assert!(<Mpfr as Into<f64>>::into(mpfr_nan!()).is_nan());
    }
}
