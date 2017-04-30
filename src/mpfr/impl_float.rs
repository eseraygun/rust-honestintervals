use super::capi::*;
use super::def::Mpfr;

use float::{Float, RoundingMode, Sign};

use libc::c_int;
use std::ffi::CString;
use std::mem::uninitialized;

impl Float for Mpfr {
    #[inline]
    fn zero(precision: usize) -> Self {
        Self::new(precision).set_zero()
    }

    #[inline]
    fn neg_zero(precision: usize) -> Self {
        Self::new(precision).set_neg_zero()
    }

    #[inline]
    fn one(precision: usize) -> Self {
        Self::new(precision).set_f64(1.0, MpfrRnd::HalfToEven)
    }

    #[inline]
    fn infinity(precision: usize) -> Self {
        Self::new(precision).set_pos_infinity()
    }

    #[inline]
    fn neg_infinity(precision: usize) -> Self {
        Self::new(precision).set_neg_infinity()
    }

    #[inline]
    fn nan(precision: usize) -> Self {
        Self::new(precision).set_nan()
    }

    #[inline]
    fn custom_from_f64(val: f64, precision: usize, rounding_mode: RoundingMode) -> Self {
        Self::new(precision).set_f64(val, MpfrRnd::from(rounding_mode))
    }

    fn custom_from_str(s: &str,
                       base: usize,
                       precision: usize,
                       rounding_mode: RoundingMode)
                       -> Option<Self>
    {
        if let Ok(c_val) = CString::new(s) {
            let mut mpfr = unsafe { uninitialized() };
            unsafe {
                mpfr_init2(&mut mpfr, precision as MpfrPrec);
            }
            let ret = unsafe {
                mpfr_set_str(&mut mpfr, c_val.as_ptr(),
                             base as c_int, MpfrRnd::from(rounding_mode))
            };
            if ret == 0 {
                Some(Mpfr { mpfr: mpfr })
            } else {
                None
            }
        } else {
            None
        }
    }

    #[inline]
    fn to_f64(&self, rounding_mode: RoundingMode) -> f64 {
        unsafe { mpfr_get_d(&self.mpfr, MpfrRnd::from(rounding_mode)) }
    }

    #[inline]
    fn precision(&self) -> usize {
        self.mpfr._mpfr_prec as usize
    }

    #[inline]
    fn is_finite(&self) -> bool {
        unsafe { mpfr_number_p(&self.mpfr) != 0 }
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        unsafe { mpfr_inf_p(&self.mpfr) != 0 }
    }

    #[inline]
    fn is_infinity(&self) -> bool {
        unsafe { mpfr_inf_p(&self.mpfr) != 0 && mpfr_sgn(&self.mpfr) > 0 }
    }

    #[inline]
    fn is_neg_infinity(&self) -> bool {
        unsafe { mpfr_inf_p(&self.mpfr) != 0 && mpfr_sgn(&self.mpfr) < 0 }
    }

    #[inline]
    fn is_nan(&self) -> bool {
        unsafe { mpfr_nan_p(&self.mpfr) != 0 }
    }

    #[inline]
    fn sign(&self) -> Sign {
        let sgn = unsafe { mpfr_sgn(&self.mpfr) };
        if sgn < 0 {
            Sign::Negative
        } else if sgn > 0 {
            Sign::Positive
        } else {
            Sign::Zero
        }
    }

    #[inline]
    fn neg(&self) -> Self {
        let mut z = Self::new(self.precision());
        unsafe { mpfr_neg(&mut z.mpfr, &self.mpfr, MpfrRnd::HalfToEven); }
        z
    }

    #[inline]
    fn abs(&self) -> Self {
        let mut z = Self::new(self.precision());
        unsafe { mpfr_abs(&mut z.mpfr, &self.mpfr, MpfrRnd::HalfToEven); }
        z
    }

    #[inline]
    fn add(&self, other: &Self, rounding_mode: RoundingMode) -> Self {
        let mut z = Self::new(self.precision());
        unsafe { mpfr_add(&mut z.mpfr, &self.mpfr, &other.mpfr, MpfrRnd::from(rounding_mode)); }
        z
    }

    #[inline]
    fn sub(&self, other: &Self, rounding_mode: RoundingMode) -> Self {
        let mut z = Self::new(self.precision());
        unsafe { mpfr_sub(&mut z.mpfr, &self.mpfr, &other.mpfr, MpfrRnd::from(rounding_mode)); }
        z
    }

    #[inline]
    fn mul(&self, other: &Self, rounding_mode: RoundingMode) -> Self {
        let mut z = Self::new(self.precision());
        unsafe { mpfr_mul(&mut z.mpfr, &self.mpfr, &other.mpfr, MpfrRnd::from(rounding_mode)); }
        z
    }

    #[inline]
    fn div(&self, other: &Self, rounding_mode: RoundingMode) -> Self {
        let mut z = Self::new(self.precision());
        unsafe { mpfr_div(&mut z.mpfr, &self.mpfr, &other.mpfr, MpfrRnd::from(rounding_mode)); }
        z
    }

    #[inline]
    fn log(&self, rounding_mode: RoundingMode) -> Self {
        let mut z = Self::new(self.precision());
        unsafe { mpfr_log(&mut z.mpfr, &self.mpfr, MpfrRnd::from(rounding_mode)); }
        z
    }

    #[inline]
    fn exp(&self, rounding_mode: RoundingMode) -> Self {
        let mut z = Self::new(self.precision());
        unsafe { mpfr_exp(&mut z.mpfr, &self.mpfr, MpfrRnd::from(rounding_mode)); }
        z
    }

    fn min(&self, other: &Self, rounding_mode: RoundingMode) -> Self {
        let mut z = Self::new(self.precision());
        unsafe { mpfr_min(&mut z.mpfr, &self.mpfr, &other.mpfr, MpfrRnd::from(rounding_mode)); }
        z
    }

    fn max(&self, other: &Self, rounding_mode: RoundingMode) -> Self {
        let mut z = Self::new(self.precision());
        unsafe { mpfr_max(&mut z.mpfr, &self.mpfr, &other.mpfr, MpfrRnd::from(rounding_mode)); }
        z
    }
}

#[cfg(test)]
mod test {
    use super::super::def::Mpfr;

    use float::{Float, RoundingMode};

    use std::f64;

    #[test]
    fn test_constants() {
        assert_str_eq!("0", Mpfr::zero(53));
        assert!((Mpfr::one(53) / Mpfr::zero(53)).is_infinity());
        assert_str_eq!("0", Mpfr::neg_zero(53));
        assert!((Mpfr::one(53) / Mpfr::neg_zero(53)).is_neg_infinity());
        assert_eq!(53, Mpfr::zero(53).precision());
        assert_str_eq!("1", Mpfr::one(53));
        assert_str_eq!("inf", Mpfr::infinity(53));
        assert_str_eq!("-inf", Mpfr::neg_infinity(53));
        assert_str_eq!("NaN", Mpfr::nan(53));
    }

    #[test]
    fn test_from_f64() {
        assert_str_eq!("1", Mpfr::custom_from_f64(1.1, 2, RoundingMode::HalfToEven));
        assert_str_eq!("2", Mpfr::custom_from_f64(2.5, 2, RoundingMode::HalfToEven));
        assert_str_eq!("4", Mpfr::custom_from_f64(3.5, 2, RoundingMode::HalfToEven));
        assert_str_eq!("-2", Mpfr::custom_from_f64(-2.5, 2, RoundingMode::HalfToEven));
        assert_str_eq!("-4", Mpfr::custom_from_f64(-3.5, 2, RoundingMode::HalfToEven));

        assert_str_eq!("1.5", Mpfr::custom_from_f64(1.1, 2, RoundingMode::HalfAwayFromZero));
        assert_str_eq!("3", Mpfr::custom_from_f64(2.5, 2, RoundingMode::HalfAwayFromZero));
        assert_str_eq!("4", Mpfr::custom_from_f64(3.5, 2, RoundingMode::HalfAwayFromZero));
        assert_str_eq!("-3", Mpfr::custom_from_f64(-2.5, 2, RoundingMode::HalfAwayFromZero));
        assert_str_eq!("-4", Mpfr::custom_from_f64(-3.5, 2, RoundingMode::HalfAwayFromZero));

        assert_str_eq!("1", Mpfr::custom_from_f64(1.1, 2, RoundingMode::Down));
        assert_str_eq!("2", Mpfr::custom_from_f64(2.5, 2, RoundingMode::Down));
        assert_str_eq!("3", Mpfr::custom_from_f64(3.5, 2, RoundingMode::Down));
        assert_str_eq!("-3", Mpfr::custom_from_f64(-2.5, 2, RoundingMode::Down));
        assert_str_eq!("-4", Mpfr::custom_from_f64(-3.5, 2, RoundingMode::Down));

        assert_str_eq!("1.5", Mpfr::custom_from_f64(1.1, 2, RoundingMode::Up));
        assert_str_eq!("3", Mpfr::custom_from_f64(2.5, 2, RoundingMode::Up));
        assert_str_eq!("4", Mpfr::custom_from_f64(3.5, 2, RoundingMode::Up));
        assert_str_eq!("-2", Mpfr::custom_from_f64(-2.5, 2, RoundingMode::Up));
        assert_str_eq!("-3", Mpfr::custom_from_f64(-3.5, 2, RoundingMode::Up));

        assert_str_eq!("1", Mpfr::custom_from_f64(1.1, 2, RoundingMode::TowardsZero));
        assert_str_eq!("2", Mpfr::custom_from_f64(2.5, 2, RoundingMode::TowardsZero));
        assert_str_eq!("3", Mpfr::custom_from_f64(3.5, 2, RoundingMode::TowardsZero));
        assert_str_eq!("-2", Mpfr::custom_from_f64(-2.5, 2, RoundingMode::TowardsZero));
        assert_str_eq!("-3", Mpfr::custom_from_f64(-3.5, 2, RoundingMode::TowardsZero));

        assert_str_eq!("1.5", Mpfr::custom_from_f64(1.1, 2, RoundingMode::AwayFromZero));
        assert_str_eq!("3", Mpfr::custom_from_f64(2.5, 2, RoundingMode::AwayFromZero));
        assert_str_eq!("4", Mpfr::custom_from_f64(3.5, 2, RoundingMode::AwayFromZero));
        assert_str_eq!("-3", Mpfr::custom_from_f64(-2.5, 2, RoundingMode::AwayFromZero));
        assert_str_eq!("-4", Mpfr::custom_from_f64(-3.5, 2, RoundingMode::AwayFromZero));
    }

    #[test]
    fn test_from_str() {
        assert_str_eq!("1", Mpfr::custom_from_str("1.1", 10, 2, RoundingMode::HalfToEven).unwrap());
        assert_str_eq!("inf", Mpfr::custom_from_str("inf", 10, 2, RoundingMode::HalfToEven).unwrap());
        assert_str_eq!("-inf", Mpfr::custom_from_str("-inf", 10, 2, RoundingMode::HalfToEven).unwrap());
        assert_str_eq!("NaN", Mpfr::custom_from_str("NaN", 10, 2, RoundingMode::HalfToEven).unwrap());
        assert_str_eq!("1.5", Mpfr::custom_from_str("1.1", 2, 2, RoundingMode::HalfToEven).unwrap());
        assert!(Mpfr::custom_from_str("1\0.1", 10, 2, RoundingMode::HalfToEven).is_none());
    }

    #[test]
    fn test_to_f64() {
        assert_eq!(0.0, mpfr!(0).to_f64(RoundingMode::HalfToEven));
        assert_eq!(1.0, mpfr!(1).to_f64(RoundingMode::HalfToEven));
        assert_eq!(f64::INFINITY, mpfr_inf!().to_f64(RoundingMode::HalfToEven));
        assert_eq!(f64::NEG_INFINITY, mpfr_neg_inf!().to_f64(RoundingMode::HalfToEven));
        assert!(mpfr_nan!().to_f64(RoundingMode::HalfToEven).is_nan());
    }
}
