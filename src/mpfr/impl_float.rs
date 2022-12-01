use super::capi::*;
use super::def::Mpfr;

use fp;
use fp::{Float, Sign};

use std::ops::Neg;

impl fp::From<f64> for Mpfr {
    #[inline]
    fn from_lo(val: f64, precision: usize) -> Self {
        Self::from_custom(val, precision, MpfrRnd::Down)
    }

    #[inline]
    fn from_hi(val: f64, precision: usize) -> Self {
        Self::from_custom(val, precision, MpfrRnd::Up)
    }
}

impl fp::FromStr for Mpfr {
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
        self.as_f64(MpfrRnd::Down)
    }

    #[inline]
    fn into_hi(self) -> f64 {
        self.as_f64(MpfrRnd::Up)
    }
}

impl fp::MinMax for Mpfr {
    #[inline]
    fn min(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_min(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::HalfToEven);
        }
        self
    }

    #[inline]
    fn max(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_max(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::HalfToEven);
        }
        self
    }
}

impl Neg for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn neg(mut self) -> Self {
        unsafe {
            mpfr_neg(&mut self.mpfr, &self.mpfr, MpfrRnd::HalfToEven);
        }
        self
    }
}

impl fp::Abs for Mpfr {
    #[inline]
    fn abs(mut self) -> Self {
        unsafe {
            mpfr_abs(&mut self.mpfr, &self.mpfr, MpfrRnd::HalfToEven);
        }
        self
    }
}

impl fp::Add for Mpfr {
    #[inline]
    fn add_lo(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_add(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Down);
        }
        self
    }

    #[inline]
    fn add_hi(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_add(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Up);
        }
        self
    }
}

impl fp::Sub for Mpfr {
    #[inline]
    fn sub_lo(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_sub(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Down);
        }
        self
    }

    #[inline]
    fn sub_hi(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_sub(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Up);
        }
        self
    }
}

impl fp::Mul for Mpfr {
    #[inline]
    fn mul_lo(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_mul(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Down);
        }
        self
    }

    #[inline]
    fn mul_hi(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_mul(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Up);
        }
        self
    }
}

impl fp::Div for Mpfr {
    #[inline]
    fn div_lo(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_div(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Down);
        }
        self
    }

    #[inline]
    fn div_hi(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_div(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Up);
        }
        self
    }
}

impl fp::Transc for Mpfr {
    #[inline]
    fn log_lo(mut self) -> Self {
        unsafe {
            mpfr_log(&mut self.mpfr, &self.mpfr, MpfrRnd::Down);
        }
        self
    }

    #[inline]
    fn log_hi(mut self) -> Self {
        unsafe {
            mpfr_log(&mut self.mpfr, &self.mpfr, MpfrRnd::Up);
        }
        self
    }

    #[inline]
    fn exp_lo(mut self) -> Self {
        unsafe {
            mpfr_exp(&mut self.mpfr, &self.mpfr, MpfrRnd::Down);
        }
        self
    }

    #[inline]
    fn exp_hi(mut self) -> Self {
        unsafe {
            mpfr_exp(&mut self.mpfr, &self.mpfr, MpfrRnd::Up);
        }
        self
    }

    fn pow_lo(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_pow(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Down);
        }
        self
    }

    fn pow_hi(mut self, rhs: Self) -> Self {
        assert_eq!(self.precision(), rhs.precision());
        unsafe {
            mpfr_pow(&mut self.mpfr, &self.mpfr, &rhs.mpfr, MpfrRnd::Up);
        }
        self
    }
}

impl Float for Mpfr {
    #[inline]
    fn zero(precision: usize) -> Self {
        unsafe { Self::uninitialized(precision) }.set_zero()
    }

    #[inline]
    fn neg_zero(precision: usize) -> Self {
        unsafe { Self::uninitialized(precision) }.set_neg_zero()
    }

    #[inline]
    fn one(precision: usize) -> Self {
        unsafe { Self::uninitialized(precision) }.set_f64(1.0, MpfrRnd::HalfToEven)
    }

    #[inline]
    fn infinity(precision: usize) -> Self {
        unsafe { Self::uninitialized(precision) }.set_infinity()
    }

    #[inline]
    fn neg_infinity(precision: usize) -> Self {
        unsafe { Self::uninitialized(precision) }.set_neg_infinity()
    }

    #[inline]
    fn nan(precision: usize) -> Self {
        unsafe { Self::uninitialized(precision) } // MPFR actually initializes new values as NaN
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
    fn is_zero(&self) -> bool {
        unsafe { mpfr_zero_p(&self.mpfr) != 0 }
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
    fn has_odd_denominator(&self) -> bool {
        (unsafe { mpfr_integer_p(&self.mpfr) }) != 0
    }
}
