use super::capi::*;
use super::def::{Mpfr, ParseMpfrError};

use fp;
use fp::{Float, RoundingMode, Sign};

use std::ffi::CString;
use std::mem::uninitialized;
use std::ops;

impl fp::From<f64> for Mpfr {
    #[inline]
    fn from(val: f64, precision: usize, rounding_mode: RoundingMode) -> Self {
        Self::new(precision).set_f64(val, MpfrRnd::from(rounding_mode))
    }
}

impl fp::FromStr for Mpfr {
    type Err = ParseMpfrError;

    #[inline]
    fn from_str(s: &str, precision: usize, rounding_mode: RoundingMode) -> Result<Self, Self::Err> {
        if let Ok(c_val) = CString::new(s) {
            let mut mpfr = unsafe { uninitialized() };
            unsafe {
                mpfr_init2(&mut mpfr, precision as MpfrPrec);
            }
            let ret = unsafe {
                mpfr_set_str(&mut mpfr, c_val.as_ptr(), 10, MpfrRnd::from(rounding_mode))
            };
            if ret == 0 {
                Ok(Mpfr { mpfr: mpfr })
            } else {
                Err(ParseMpfrError::MpfrError)
            }
        } else {
            Err(ParseMpfrError::CStringError)
        }
    }
}

impl fp::Into<f64> for Mpfr {
    fn into(self, rounding_mode: RoundingMode) -> f64 {
        unsafe { mpfr_get_d(&self.mpfr, MpfrRnd::from(rounding_mode)) }
    }
}

impl fp::MinMax<Mpfr> for Mpfr {
    type Output = Mpfr;

    fn min(mut self, rhs: Mpfr) -> Mpfr {
        unsafe { mpfr_min(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::HalfToEven); }
        self
    }

    fn max(mut self, rhs: Mpfr) -> Mpfr {
        unsafe { mpfr_max(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::HalfToEven); }
        self
    }
}

impl ops::Neg for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn neg(mut self) -> Self::Output {
        unsafe { mpfr_neg(&mut self.mpfr, &self.mpfr, MpfrRnd::HalfToEven); }
        self
    }
}

impl fp::Abs for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn abs(mut self) -> Self::Output {
        unsafe { mpfr_abs(&mut self.mpfr, &self.mpfr, MpfrRnd::HalfToEven); }
        self
    }
}

impl fp::Add<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn add(mut self, rhs: Mpfr, rounding_mode: RoundingMode) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_add(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::from(rounding_mode)); }
        self
    }
}

impl fp::Sub<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn sub(mut self, rhs: Mpfr, rounding_mode: RoundingMode) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_sub(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::from(rounding_mode)); }
        self
    }
}

impl fp::Mul<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn mul(mut self, rhs: Mpfr, rounding_mode: RoundingMode) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_mul(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::from(rounding_mode)); }
        self
    }
}

impl fp::Div<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn div(mut self, rhs: Mpfr, rounding_mode: RoundingMode) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_div(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::from(rounding_mode)); }
        self
    }
}

impl fp::Transc for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn log(mut self, rounding_mode: RoundingMode) -> Mpfr {
        unsafe { mpfr_log(&mut self.mpfr, &self.mpfr, MpfrRnd::from(rounding_mode)); }
        self
    }

    #[inline]
    fn exp(mut self, rounding_mode: RoundingMode) -> Mpfr {
        unsafe { mpfr_exp(&mut self.mpfr, &self.mpfr, MpfrRnd::from(rounding_mode)); }
        self
    }
}

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
}

#[cfg(test)]
mod test {
    use super::super::def::Mpfr;

    use fp;
    use fp::{Abs, Float, MinMax, RoundingMode, Sign};

    use std::f64;

    #[test]
    fn test_from_f64() {
        assert_str_eq!("1", <Mpfr as fp::From<f64>>::from(1.1, 2, RoundingMode::HalfToEven));
        assert_str_eq!("2", <Mpfr as fp::From<f64>>::from(2.5, 2, RoundingMode::HalfToEven));
        assert_str_eq!("4", <Mpfr as fp::From<f64>>::from(3.5, 2, RoundingMode::HalfToEven));
        assert_str_eq!("-2", <Mpfr as fp::From<f64>>::from(-2.5, 2, RoundingMode::HalfToEven));
        assert_str_eq!("-4", <Mpfr as fp::From<f64>>::from(-3.5, 2, RoundingMode::HalfToEven));

        assert_str_eq!("1.5", <Mpfr as fp::From<f64>>::from(1.1, 2, RoundingMode::HalfAwayFromZero));
        assert_str_eq!("3", <Mpfr as fp::From<f64>>::from(2.5, 2, RoundingMode::HalfAwayFromZero));
        assert_str_eq!("4", <Mpfr as fp::From<f64>>::from(3.5, 2, RoundingMode::HalfAwayFromZero));
        assert_str_eq!("-3", <Mpfr as fp::From<f64>>::from(-2.5, 2, RoundingMode::HalfAwayFromZero));
        assert_str_eq!("-4", <Mpfr as fp::From<f64>>::from(-3.5, 2, RoundingMode::HalfAwayFromZero));

        assert_str_eq!("1", <Mpfr as fp::From<f64>>::from(1.1, 2, RoundingMode::Down));
        assert_str_eq!("2", <Mpfr as fp::From<f64>>::from(2.5, 2, RoundingMode::Down));
        assert_str_eq!("3", <Mpfr as fp::From<f64>>::from(3.5, 2, RoundingMode::Down));
        assert_str_eq!("-3", <Mpfr as fp::From<f64>>::from(-2.5, 2, RoundingMode::Down));
        assert_str_eq!("-4", <Mpfr as fp::From<f64>>::from(-3.5, 2, RoundingMode::Down));

        assert_str_eq!("1.5", <Mpfr as fp::From<f64>>::from(1.1, 2, RoundingMode::Up));
        assert_str_eq!("3", <Mpfr as fp::From<f64>>::from(2.5, 2, RoundingMode::Up));
        assert_str_eq!("4", <Mpfr as fp::From<f64>>::from(3.5, 2, RoundingMode::Up));
        assert_str_eq!("-2", <Mpfr as fp::From<f64>>::from(-2.5, 2, RoundingMode::Up));
        assert_str_eq!("-3", <Mpfr as fp::From<f64>>::from(-3.5, 2, RoundingMode::Up));

        assert_str_eq!("1", <Mpfr as fp::From<f64>>::from(1.1, 2, RoundingMode::TowardsZero));
        assert_str_eq!("2", <Mpfr as fp::From<f64>>::from(2.5, 2, RoundingMode::TowardsZero));
        assert_str_eq!("3", <Mpfr as fp::From<f64>>::from(3.5, 2, RoundingMode::TowardsZero));
        assert_str_eq!("-2", <Mpfr as fp::From<f64>>::from(-2.5, 2, RoundingMode::TowardsZero));
        assert_str_eq!("-3", <Mpfr as fp::From<f64>>::from(-3.5, 2, RoundingMode::TowardsZero));

        assert_str_eq!("1.5", <Mpfr as fp::From<f64>>::from(1.1, 2, RoundingMode::AwayFromZero));
        assert_str_eq!("3", <Mpfr as fp::From<f64>>::from(2.5, 2, RoundingMode::AwayFromZero));
        assert_str_eq!("4", <Mpfr as fp::From<f64>>::from(3.5, 2, RoundingMode::AwayFromZero));
        assert_str_eq!("-3", <Mpfr as fp::From<f64>>::from(-2.5, 2, RoundingMode::AwayFromZero));
        assert_str_eq!("-4", <Mpfr as fp::From<f64>>::from(-3.5, 2, RoundingMode::AwayFromZero));
    }

    #[test]
    fn test_from_str() {
        assert_str_eq!("1", <Mpfr as fp::FromStr>::from_str("1.1", 2, RoundingMode::HalfToEven).unwrap());
        assert_str_eq!("inf", <Mpfr as fp::FromStr>::from_str("inf", 2, RoundingMode::HalfToEven).unwrap());
        assert_str_eq!("-inf", <Mpfr as fp::FromStr>::from_str("-inf", 2, RoundingMode::HalfToEven).unwrap());
        assert_str_eq!("NaN", <Mpfr as fp::FromStr>::from_str("NaN", 2, RoundingMode::HalfToEven).unwrap());
        assert!(<Mpfr as fp::FromStr>::from_str("1a1", 2, RoundingMode::HalfToEven).is_err());
        assert!(<Mpfr as fp::FromStr>::from_str("1\0.1", 2, RoundingMode::HalfToEven).is_err());
    }

    #[test]
    fn test_into_f64() {
        assert_eq!(0.0, fp::Into::<f64>::into(mpfr!(0), RoundingMode::HalfToEven));
        assert_eq!(1.0, fp::Into::<f64>::into(mpfr!(1), RoundingMode::HalfToEven));
        assert_eq!(f64::INFINITY, fp::Into::<f64>::into(mpfr_inf!(), RoundingMode::HalfToEven));
        assert_eq!(f64::NEG_INFINITY, fp::Into::<f64>::into(mpfr_neg_inf!(), RoundingMode::HalfToEven));
        assert!(fp::Into::<f64>::into(mpfr_nan!(), RoundingMode::HalfToEven).is_nan());
    }

    #[test]
    fn test_min() {
        assert_str_eq!("0", mpfr!(0).min(mpfr!(0)));
        assert_str_eq!("0", mpfr!(0).min(mpfr!(1)));
        assert_str_eq!("0", mpfr!(1).min(mpfr!(0)));
        assert_str_eq!("-1", mpfr!(-1).min(mpfr!(0)));
        assert_str_eq!("-1", mpfr!(1).min(mpfr!(-1)));
        assert_str_eq!("0", mpfr!(0).min(mpfr_nan!()));
        assert_str_eq!("0", mpfr!(0).min(mpfr_inf!()));
        assert_str_eq!("-inf", mpfr!(0).min(mpfr_neg_inf!()));
    }

    #[test]
    fn test_max() {
        assert_str_eq!("0", mpfr!(0).max(mpfr!(0)));
        assert_str_eq!("1", mpfr!(0).max(mpfr!(1)));
        assert_str_eq!("1", mpfr!(1).max(mpfr!(0)));
        assert_str_eq!("0", mpfr!(-1).max(mpfr!(0)));
        assert_str_eq!("1", mpfr!(1).max(mpfr!(-1)));
        assert_str_eq!("0", mpfr!(0).max(mpfr_nan!()));
        assert_str_eq!("inf", mpfr!(0).max(mpfr_inf!()));
        assert_str_eq!("0", mpfr!(0).max(mpfr_neg_inf!()));
    }

    #[test]
    fn test_neg() {
        assert_str_eq!("0", -mpfr!(0));
        assert_str_eq!("-1", -mpfr!(1));
        assert_str_eq!("1", -mpfr!(-1));
        assert_str_eq!("NaN", -mpfr_nan!());
        assert_str_eq!("-inf", -mpfr_inf!());
        assert_str_eq!("inf", -mpfr_neg_inf!());
    }

    #[test]
    fn test_abs() {
        assert_str_eq!("0", mpfr!(0).abs());
        assert_str_eq!("1", mpfr!(1).abs());
        assert_str_eq!("1", mpfr!(-1).abs());
        assert_str_eq!("NaN", mpfr_nan!().abs());
        assert_str_eq!("inf", mpfr_inf!().abs());
        assert_str_eq!("inf", mpfr_neg_inf!().abs());
    }

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
    fn test_queries() {
        assert!(mpfr!(0).is_finite());
        assert!(!mpfr!(0).is_infinite());
        assert!(!mpfr!(0).is_infinity());
        assert!(!mpfr!(0).is_neg_infinity());
        assert!(!mpfr!(0).is_nan());

        assert!(!mpfr_inf!().is_finite());
        assert!(mpfr_inf!().is_infinite());
        assert!(mpfr_inf!().is_infinity());
        assert!(!mpfr_inf!().is_neg_infinity());
        assert!(!mpfr_inf!().is_nan());

        assert!(!mpfr_neg_inf!().is_finite());
        assert!(mpfr_neg_inf!().is_infinite());
        assert!(!mpfr_neg_inf!().is_infinity());
        assert!(mpfr_neg_inf!().is_neg_infinity());
        assert!(!mpfr_neg_inf!().is_nan());

        assert!(!mpfr_nan!().is_finite());
        assert!(!mpfr_nan!().is_infinite());
        assert!(!mpfr_nan!().is_infinity());
        assert!(!mpfr_nan!().is_neg_infinity());
        assert!(mpfr_nan!().is_nan());
    }

    #[test]
    fn test_sign() {
        assert_eq!(Sign::Zero, mpfr!(0).sign());
        assert_eq!(Sign::Positive, mpfr!(1).sign());
        assert_eq!(Sign::Negative, mpfr!(-1).sign());
        assert_eq!(Sign::Zero, mpfr_nan!().sign());
        assert_eq!(Sign::Positive, mpfr_inf!().sign());
        assert_eq!(Sign::Negative, mpfr_neg_inf!().sign());
    }

    #[test]
    fn test_precision() {
        assert_eq!(2usize, Mpfr::new(2).precision());
        assert_eq!(53usize, mpfr!(0).precision());
    }
}
