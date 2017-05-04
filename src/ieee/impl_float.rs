use fp;
use fp::{Float, Sign};

use std::f64;
use std::num::ParseFloatError;
use std::str::FromStr;

impl fp::From<f64> for f64 {
    #[inline]
    fn from_lo(val: f64, _: usize) -> Self { val }

    #[inline]
    fn from_hi(val: f64, _: usize) -> Self { val }
}

impl fp::FromStr for f64 {
    type Err = ParseFloatError;

    #[inline]
    fn from_str_lo(s: &str, _precision: usize) -> Result<Self, Self::Err> {
        // WORKAROUND: Rust rounds negative numbers to the wrong direction.
        if s.len() >= 1 && s.as_bytes()[0] == b'-' {
            Ok(-Self::from_str_hi(&s[1..], _precision)?)
        } else {
            lo!({ f64::from_str(s) })
        }
    }

    #[inline]
    fn from_str_hi(s: &str, _precision: usize) -> Result<Self, Self::Err> {
        // WORKAROUND: Rust rounds negative numbers to the wrong direction.
        if s.len() >= 1 && s.as_bytes()[0] == b'-' {
            Ok(-Self::from_str_lo(&s[1..], _precision)?)
        } else {
            hi!({ f64::from_str(s) })
        }
    }
}

impl fp::Into<f64> for f64 {
    #[inline]
    fn into_lo(self) -> f64 { self }

    #[inline]
    fn into_hi(self) -> f64 { self }
}

impl fp::MinMax for f64 {
    type Output = Self;

    #[inline]
    fn min(self, rhs: Self) -> Self { self.min(rhs) }

    #[inline]
    fn max(self, rhs: Self) -> Self { self.max(rhs) }
}

impl fp::Abs for f64 {
    type Output = Self;

    #[inline]
    fn abs(self) -> Self::Output { self.abs() }
}

impl fp::Add for f64 {
    type Output = Self;

    #[inline]
    fn add_lo(self, rhs: Self) -> Self::Output { lo!({ self + rhs }) }

    #[inline]
    fn add_hi(self, rhs: Self) -> Self::Output { hi!({ self + rhs }) }
}

impl fp::Sub for f64 {
    type Output = Self;

    #[inline]
    fn sub_lo(self, rhs: Self) -> Self::Output { lo!({ self - rhs }) }

    #[inline]
    fn sub_hi(self, rhs: Self) -> Self::Output { hi!({ self - rhs }) }
}

impl fp::Mul for f64 {
    type Output = Self;

    #[inline]
    fn mul_lo(self, rhs: Self) -> Self::Output { lo!({ self * rhs }) }

    #[inline]
    fn mul_hi(self, rhs: Self) -> Self::Output { hi!({ self * rhs }) }
}

impl fp::Div for f64 {
    type Output = Self;

    #[inline]
    fn div_lo(self, rhs: Self) -> Self::Output { lo!({ self / rhs }) }

    #[inline]
    fn div_hi(self, rhs: Self) -> Self::Output { hi!({ self / rhs }) }
}

impl fp::Transc for f64 {
    type Output = Self;

    #[inline]
    fn log_lo(self) -> Self::Output { lo!({ self.ln() }) }

    #[inline]
    fn log_hi(self) -> Self::Output { hi!({ self.ln() }) }

    #[inline]
    fn exp_lo(self) -> Self::Output { lo!({ self.exp() }) }

    #[inline]
    fn exp_hi(self) -> Self::Output { hi!({ self.exp() }) }

    #[inline]
    fn pow_lo(self, rhs: Self) -> Self::Output { lo!({ self.powf(rhs) }) }

    #[inline]
    fn pow_hi(self, rhs: Self) -> Self::Output { hi!({ self.powf(rhs) }) }
}

impl Float for f64 {
    #[inline]
    fn zero(_precision: usize) -> Self {
        0.0
    }

    #[inline]
    fn neg_zero(_precision: usize) -> Self {
        -0.0
    }

    #[inline]
    fn one(_precision: usize) -> Self {
        1.0
    }

    #[inline]
    fn infinity(_precision: usize) -> Self {
        f64::INFINITY
    }

    #[inline]
    fn neg_infinity(_precision: usize) -> Self {
        f64::NEG_INFINITY
    }

    #[inline]
    fn nan(_precision: usize) -> Self {
        f64::NAN
    }

    #[inline]
    fn sign(&self) -> Sign {
        if *self < 0.0 {
            Sign::Negative
        } else if *self > 0.0 {
            Sign::Positive
        } else {
            Sign::Zero
        }
    }

    #[inline]
    fn precision(&self) -> usize {
        53
    }

    #[inline]
    fn is_finite(&self) -> bool {
        f64::is_finite(*self)
    }

    #[inline]
    fn is_infinite(&self) -> bool {
        f64::is_infinite(*self)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    #[inline]
    fn is_infinity(&self) -> bool {
        f64::is_infinite(*self) && self.is_sign_positive()
    }

    #[inline]
    fn is_neg_infinity(&self) -> bool {
        f64::is_infinite(*self) && self.is_sign_negative()
    }

    #[inline]
    fn is_nan(&self) -> bool {
        f64::is_nan(*self)
    }
}
