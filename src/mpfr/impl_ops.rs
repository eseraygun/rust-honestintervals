use super::capi::*;
use super::def::Mpfr;

use fp::Float;

use std::ops::{Add, Div, Mul, Sub};

impl Mpfr {
    #[inline]
    fn add_custom(mut self, rhs: Mpfr, rounding_mode: MpfrRnd) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_add(&mut self.mpfr, &self.mpfr, &rhs.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn sub_custom(mut self, rhs: Mpfr, rounding_mode: MpfrRnd) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_sub(&mut self.mpfr, &self.mpfr, &rhs.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn mul_custom(mut self, rhs: Mpfr, rounding_mode: MpfrRnd) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_mul(&mut self.mpfr, &self.mpfr, &rhs.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn div_custom(mut self, rhs: Mpfr, rounding_mode: MpfrRnd) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe { mpfr_div(&mut self.mpfr, &self.mpfr, &rhs.mpfr, rounding_mode); }
        self
    }
}

impl Add<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn add(self, other: Mpfr) -> Self::Output {
        self.add_custom(other, MpfrRnd::HalfToEven)
    }
}

impl Sub<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn sub(self, other: Mpfr) -> Self::Output {
        self.sub_custom(other, MpfrRnd::HalfToEven)
    }
}

impl Mul<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn mul(self, other: Mpfr) -> Self::Output {
        self.mul_custom(other, MpfrRnd::HalfToEven)
    }
}

impl Div<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn div(self, other: Mpfr) -> Self::Output {
        self.div_custom(other, MpfrRnd::HalfToEven)
    }
}

#[cfg(test)]
mod test {
    use super::super::def::Mpfr;

    use fp::Float;

    #[test]
    fn test_add() {
        assert_str_eq!("0", mpfr!(0) + mpfr!(0));
        assert_str_eq!("1", mpfr!(0) + mpfr!(1));
        assert_str_eq!("1", mpfr!(1) + mpfr!(0));
        assert_str_eq!("-1", mpfr!(-1) + mpfr!(0));
        assert_str_eq!("0", mpfr!(1) + mpfr!(-1));
        assert_str_eq!("NaN", mpfr!(0) + mpfr_nan!());
        assert_str_eq!("inf", mpfr!(0) + mpfr_inf!());
        assert_str_eq!("-inf", mpfr!(0) + mpfr_neg_inf!());
    }

    #[test]
    fn test_sub() {
        assert_str_eq!("0", mpfr!(0) - mpfr!(0));
        assert_str_eq!("-1", mpfr!(0) - mpfr!(1));
        assert_str_eq!("1", mpfr!(1) - mpfr!(0));
        assert_str_eq!("-1", mpfr!(-1) - mpfr!(0));
        assert_str_eq!("2", mpfr!(1) - mpfr!(-1));
        assert_str_eq!("NaN", mpfr!(0) - mpfr_nan!());
        assert_str_eq!("-inf", mpfr!(0) - mpfr_inf!());
        assert_str_eq!("inf", mpfr!(0) - mpfr_neg_inf!());
    }

    #[test]
    fn test_mul() {
        assert_str_eq!("0", mpfr!(0) * mpfr!(0));
        assert_str_eq!("0", mpfr!(0) * mpfr!(1));
        assert_str_eq!("0", mpfr!(1) * mpfr!(0));
        assert_str_eq!("0", mpfr!(-1) * mpfr!(0));
        assert_str_eq!("-1", mpfr!(1) * mpfr!(-1));
        assert_str_eq!("NaN", mpfr!(0) * mpfr_nan!());
        assert_str_eq!("NaN", mpfr!(0) * mpfr_inf!());
        assert_str_eq!("NaN", mpfr!(0) * mpfr_neg_inf!());
    }

    #[test]
    fn test_div() {
        assert_str_eq!("NaN", mpfr!(0) / mpfr!(0));
        assert_str_eq!("0", mpfr!(0) / mpfr!(1));
        assert_str_eq!("inf", mpfr!(1) / mpfr!(0));
        assert_str_eq!("-inf", mpfr!(-1) / mpfr!(0));
        assert_str_eq!("-1", mpfr!(1) / mpfr!(-1));
        assert_str_eq!("NaN", mpfr!(0) / mpfr_nan!());
        assert_str_eq!("0", mpfr!(0) / mpfr_inf!());
        assert_str_eq!("0", mpfr!(0) / mpfr_neg_inf!());
    }
}
