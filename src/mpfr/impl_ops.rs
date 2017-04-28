use super::def::Mpfr;

use float::{Float, RoundingMode};

use std::ops::{Add, Div, Mul, Neg, Sub};

impl Neg for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn neg(self) -> Self::Output {
        Float::neg(&self)
    }
}

impl<'a> Neg for &'a Mpfr {
    type Output = Mpfr;

    #[inline]
    fn neg(self) -> Self::Output {
        Float::neg(self)
    }
}

impl Add<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn add(self, other: Mpfr) -> Self::Output {
        Float::add(&self, &other, RoundingMode::HalfToEven)
    }
}

impl<'a, 'b> Add<&'b Mpfr> for &'a Mpfr {
    type Output = Mpfr;

    #[inline]
    fn add(self, other: &'b Mpfr) -> Self::Output {
        Float::add(self, other, RoundingMode::HalfToEven)
    }
}

impl Sub<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn sub(self, other: Mpfr) -> Self::Output {
        Float::sub(&self, &other, RoundingMode::HalfToEven)
    }
}

impl<'a, 'b> Sub<&'b Mpfr> for &'a Mpfr {
    type Output = Mpfr;

    #[inline]
    fn sub(self, other: &'b Mpfr) -> Self::Output {
        Float::sub(self, other, RoundingMode::HalfToEven)
    }
}

impl Mul<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn mul(self, other: Mpfr) -> Self::Output {
        Float::mul(&self, &other, RoundingMode::HalfToEven)
    }
}

impl<'a, 'b> Mul<&'b Mpfr> for &'a Mpfr {
    type Output = Mpfr;

    #[inline]
    fn mul(self, other: &'b Mpfr) -> Self::Output {
        Float::mul(self, other, RoundingMode::HalfToEven)
    }
}

impl Div<Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn div(self, other: Mpfr) -> Self::Output {
        Float::div(&self, &other, RoundingMode::HalfToEven)
    }
}

impl<'a, 'b> Div<&'b Mpfr> for &'a Mpfr {
    type Output = Mpfr;

    #[inline]
    fn div(self, other: &'b Mpfr) -> Self::Output {
        Float::div(self, other, RoundingMode::HalfToEven)
    }
}

#[cfg(test)]
mod test {
    use super::super::def::Mpfr;

    use float::{Float, RoundingMode};

    #[test]
    fn test_neg() {
        assert_str_eq!("0", -mpfr!(0));
        assert_str_eq!("-1", -mpfr!(1));
        assert_str_eq!("NaN", -mpfr_nan!());
        assert_str_eq!("-inf", -mpfr_inf!());
        assert_str_eq!("inf", -mpfr_neg_inf!());
    }

    #[test]
    fn test_add() {
        assert_str_eq!("0", mpfr!(0) + mpfr!(0));
        assert_str_eq!("1", mpfr!(0) + mpfr!(1));
        assert_str_eq!("0", mpfr!(1) + mpfr!(-1));
        assert_str_eq!("NaN", mpfr!(0) + mpfr_nan!());
        assert_str_eq!("inf", mpfr!(0) + mpfr_inf!());
        assert_str_eq!("-inf", mpfr!(0) + mpfr_neg_inf!());
    }

    #[test]
    fn test_sub() {
        assert_str_eq!("0", mpfr!(0) - mpfr!(0));
        assert_str_eq!("-1", mpfr!(0) - mpfr!(1));
        assert_str_eq!("2", mpfr!(1) - mpfr!(-1));
        assert_str_eq!("NaN", mpfr!(0) - mpfr_nan!());
        assert_str_eq!("-inf", mpfr!(0) - mpfr_inf!());
        assert_str_eq!("inf", mpfr!(0) - mpfr_neg_inf!());
    }

    #[test]
    fn test_mul() {
        assert_str_eq!("0", mpfr!(0) * mpfr!(0));
        assert_str_eq!("0", mpfr!(0) * mpfr!(1));
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
