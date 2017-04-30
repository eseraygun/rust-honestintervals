use super::capi::*;
use super::def::{Mpfr, ParseMpfrError};

use fp;
use fp::{Float, Sign};

use std::ops::Neg;

impl fp::From<f64> for Mpfr {
    #[inline]
    fn from_lo(val: f64, precision: usize) -> Self {
        Self::new(precision).set_f64(val, MpfrRnd::Down)
    }

    #[inline]
    fn from_hi(val: f64, precision: usize) -> Self {
        Self::new(precision).set_f64(val, MpfrRnd::Up)
    }
}

impl fp::FromStr for Mpfr {
    type Err = ParseMpfrError;

    #[inline]
    fn from_str_lo(s: &str, precision: usize) -> Result<Self, Self::Err> {
        Self::from_str_custom(s, precision, MpfrRnd::Down)
    }

    #[inline]
    fn from_str_hi(s: &str, precision: usize) -> Result<Self, Self::Err> {
        Self::from_str_custom(s, precision, MpfrRnd::Up)
    }
}

impl fp::Into<f64> for Mpfr {
    #[inline]
    fn into_lo(self) -> f64 {
        unsafe { mpfr_get_d(&self.mpfr, MpfrRnd::Down) }
    }

    #[inline]
    fn into_hi(self) -> f64 {
        unsafe { mpfr_get_d(&self.mpfr, MpfrRnd::Up) }
    }
}

impl fp::MinMax<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn min(mut self, rhs: Mpfr) -> Mpfr {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_min(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::HalfToEven); }
        self
    }

    #[inline]
    fn max(mut self, rhs: Mpfr) -> Mpfr {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_max(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::HalfToEven); }
        self
    }
}

impl Neg for Mpfr {
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
    fn add_lo(mut self, rhs: Mpfr) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_add(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Down); }
        self
    }

    #[inline]
    fn add_hi(mut self, rhs: Mpfr) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_add(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Up); }
        self
    }
}

impl fp::Sub<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn sub_lo(mut self, rhs: Mpfr) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_sub(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Down); }
        self
    }

    #[inline]
    fn sub_hi(mut self, rhs: Mpfr) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_sub(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Up); }
        self
    }
}

impl fp::Mul<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn mul_lo(mut self, rhs: Mpfr) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_mul(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Down); }
        self
    }

    #[inline]
    fn mul_hi(mut self, rhs: Mpfr) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_mul(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Up); }
        self
    }
}

impl fp::Div<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn div_lo(mut self, rhs: Mpfr) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_div(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Down); }
        self
    }

    #[inline]
    fn div_hi(mut self, rhs: Mpfr) -> Self::Output {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_div(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Up); }
        self
    }
}

impl fp::Transc for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn log_lo(mut self) -> Mpfr {
        unsafe { mpfr_log(&mut self.mpfr, &self.mpfr, MpfrRnd::Down); }
        self
    }


    #[inline]
    fn log_hi(mut self) -> Mpfr {
        unsafe { mpfr_log(&mut self.mpfr, &self.mpfr, MpfrRnd::Up); }
        self
    }

    #[inline]
    fn exp_lo(mut self) -> Mpfr {
        unsafe { mpfr_exp(&mut self.mpfr, &self.mpfr, MpfrRnd::Down); }
        self
    }


    #[inline]
    fn exp_hi(mut self) -> Mpfr {
        unsafe { mpfr_exp(&mut self.mpfr, &self.mpfr, MpfrRnd::Up); }
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

    use fp::Float;

    #[test]
    fn test_from_f64_lo() {
        use fp::From;
        assert_str_eq!("1", Mpfr::from_lo(1.1, 2));
        assert_str_eq!("2", Mpfr::from_lo(2.5, 2));
        assert_str_eq!("3", Mpfr::from_lo(3.5, 2));
        assert_str_eq!("-3", Mpfr::from_lo(-2.5, 2));
        assert_str_eq!("-4", Mpfr::from_lo(-3.5, 2));
    }

    #[test]
    fn test_from_f64_hi() {
        use fp::From;
        assert_str_eq!("1.5", Mpfr::from_hi(1.1, 2));
        assert_str_eq!("3", Mpfr::from_hi(2.5, 2));
        assert_str_eq!("4", Mpfr::from_hi(3.5, 2));
        assert_str_eq!("-2", Mpfr::from_hi(-2.5, 2));
        assert_str_eq!("-3", Mpfr::from_hi(-3.5, 2));
    }

    #[test]
    fn test_from_str_lo() {
        use fp::FromStr;
        assert_str_eq!("0", Mpfr::from_str_lo("0", 2).unwrap());
        assert_str_eq!("0.09375", Mpfr::from_str_lo("0.123", 2).unwrap());
        assert_str_eq!("-1.5", Mpfr::from_str_lo("-1.23", 2).unwrap());
        assert_str_eq!("inf", Mpfr::from_str_lo("inf", 2).unwrap());
        assert_str_eq!("-inf", Mpfr::from_str_lo("-inf", 2).unwrap());
        assert_str_eq!("NaN", Mpfr::from_str_lo("NaN", 2).unwrap());
        assert!(Mpfr::from_str_lo("123a456", 2).is_err());
        assert!(Mpfr::from_str_lo("123\0456", 2).is_err());
    }

    #[test]
    fn test_from_str_hi() {
        use fp::FromStr;
        assert_str_eq!("0", Mpfr::from_str_hi("0", 2).unwrap());
        assert_str_eq!("0.125", Mpfr::from_str_hi("0.123", 2).unwrap());
        assert_str_eq!("-1", Mpfr::from_str_hi("-1.23", 2).unwrap());
        assert_str_eq!("inf", Mpfr::from_str_hi("inf", 2).unwrap());
        assert_str_eq!("-inf", Mpfr::from_str_hi("-inf", 2).unwrap());
        assert_str_eq!("NaN", Mpfr::from_str_hi("NaN", 2).unwrap());
        assert!(Mpfr::from_str_hi("123a456", 2).is_err());
        assert!(Mpfr::from_str_hi("123\0456", 2).is_err());
    }

    #[test]
    fn test_into_f64_lo() {
        use fp::Into;
        use std::f64;
        assert_eq!(0.0, Mpfr::into_lo(mpfr!(0)));
        assert_eq!(0.123, Mpfr::into_lo(mpfr!(0.123)));
        assert_eq!(-1.23, Mpfr::into_lo(mpfr!(-1.23)));
        assert_eq!(f64::INFINITY, Mpfr::into_lo(mpfr_inf!()));
        assert_eq!(f64::NEG_INFINITY, Mpfr::into_lo(mpfr_neg_inf!()));
        assert!(Mpfr::into_lo(mpfr_nan!()).is_nan());
    }

    #[test]
    fn test_into_f64_hi() {
        use fp::Into;
        use std::f64;
        assert_eq!(0.0, Mpfr::into_hi(mpfr!(0)));
        assert_eq!(0.123, Mpfr::into_hi(mpfr!(0.123)));
        assert_eq!(-1.23, Mpfr::into_hi(mpfr!(-1.23)));
        assert_eq!(f64::INFINITY, Mpfr::into_hi(mpfr_inf!()));
        assert_eq!(f64::NEG_INFINITY, Mpfr::into_hi(mpfr_neg_inf!()));
        assert!(Mpfr::into_hi(mpfr_nan!()).is_nan());
    }

    #[test]
    fn test_min() {
        use fp::MinMax;
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
        use fp::MinMax;
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
        use fp::Abs;
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
        use fp::Sign;
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
