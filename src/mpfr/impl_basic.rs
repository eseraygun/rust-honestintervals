use super::capi::*;
use super::def::{Mpfr, ParseMpfrError};

use fp::Float;

use std::ffi::CString;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::mem::MaybeUninit;
use std::str::FromStr;

impl Mpfr {
    /// Constructs an uninitialized MPFR.
    #[inline]
    pub unsafe fn uninitialized(precision: usize) -> Self {
        let mut mpfr = MaybeUninit::uninit();
        mpfr_init2(mpfr.assume_init_mut(), precision as MpfrPrec);
        Self {
            mpfr: mpfr.assume_init(),
        }
    }

    /// Sets the value of `self` to zero.
    #[inline]
    pub fn set_zero(mut self) -> Self {
        unsafe { mpfr_set_zero(&mut self.mpfr, 1) };
        self
    }

    /// Sets the value of `self` to negative zero.
    #[inline]
    pub fn set_neg_zero(mut self) -> Self {
        unsafe { mpfr_set_zero(&mut self.mpfr, -1) };
        self
    }

    /// Sets the value of `self` to positive infinity.
    #[inline]
    pub fn set_infinity(mut self) -> Self {
        unsafe { mpfr_set_inf(&mut self.mpfr, 1) };
        self
    }

    /// Sets the value of `self` to negative infinity.
    #[inline]
    pub fn set_neg_infinity(mut self) -> Self {
        unsafe { mpfr_set_inf(&mut self.mpfr, -1) };
        self
    }

    /// Sets the value of `self` to NaN.
    #[inline]
    pub fn set_nan(mut self) -> Self {
        unsafe { mpfr_set_nan(&mut self.mpfr) };
        self
    }

    /// Sets the value of `self` to `other`.
    #[inline]
    pub fn set(mut self, other: &Self, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_set(&mut self.mpfr, &other.mpfr, rounding_mode) };
        self
    }

    /// Sets the value of `self` to `val`.
    #[inline]
    pub fn set_f64(mut self, val: f64, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_set_d(&mut self.mpfr, val, rounding_mode) };
        self
    }

    /// Parses `c` and sets the value of `self` to the result.
    #[inline]
    pub fn set_str(mut self, c: CString, rounding_mode: MpfrRnd) -> Option<Self> {
        if unsafe { mpfr_set_str(&mut self.mpfr, c.as_ptr(), 10, rounding_mode) } == 0 {
            Some(self)
        } else {
            None
        }
    }

    /// Returns an `f64` representing `self`.
    #[inline]
    pub fn as_f64(&self, rounding_mode: MpfrRnd) -> f64 {
        unsafe { mpfr_get_d(&self.mpfr, rounding_mode) }
    }
}

impl Drop for Mpfr {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            mpfr_clear(&mut self.mpfr);
        }
    }
}

impl Mpfr {
    /// Constructs an MPFR from an `f64` with custom precision and rounding mode.
    #[inline]
    pub fn from_custom(val: f64, precision: usize, rounding_mode: MpfrRnd) -> Self {
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
    /// Constructs an MPFR from an `&str` with custom precision and rounding mode.
    pub fn from_str_custom(
        s: &str,
        precision: usize,
        rounding_mode: MpfrRnd,
    ) -> Result<Self, ParseMpfrError> {
        if let Ok(c) = CString::new(s) {
            if let Some(res) = unsafe { Mpfr::uninitialized(precision) }.set_str(c, rounding_mode) {
                Ok(res)
            } else {
                Err(ParseMpfrError::MpfrParseError)
            }
        } else {
            Err(ParseMpfrError::CStringError)
        }
    }

    /// Constructs an MPFR from an `&str` with custom precision. Rounds to the nearest.
    pub fn from_str_with_prec(s: &str, precision: usize) -> Result<Self, ParseMpfrError> {
        Self::from_str_custom(s, precision, MpfrRnd::HalfToEven)
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
        self.as_f64(MpfrRnd::HalfToEven).fmt(f)
    }
}

impl Into<f64> for Mpfr {
    #[inline]
    fn into(self) -> f64 {
        self.as_f64(MpfrRnd::HalfToEven)
    }
}
