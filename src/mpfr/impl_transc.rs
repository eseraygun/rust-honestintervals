use super::capi::*;
use super::def::Mpfr;

use crate::transc::Transc;

impl Mpfr {
    #[inline]
    fn log_custom(mut self, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_log(&mut self.mpfr, &self.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn exp_custom(mut self, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_exp(&mut self.mpfr, &self.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn pow_custom(mut self, rhs: Self, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_pow(&mut self.mpfr, &self.mpfr, &rhs.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn sin_custom(mut self, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_sin(&mut self.mpfr, &self.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn cos_custom(mut self, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_cos(&mut self.mpfr, &self.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn tan_custom(mut self, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_tan(&mut self.mpfr, &self.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn sqrt_custom(mut self, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_sqrt(&mut self.mpfr, &self.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn abs_custom(mut self, rounding_mode: MpfrRnd) -> Self {
        unsafe { mpfr_abs(&mut self.mpfr, &self.mpfr, rounding_mode); }
        self
    }

    #[inline]
    fn sgn_custom(self) -> Self {
        unsafe { mpfr_sgn(&self.mpfr); }
        self
    }
}

impl Transc for Mpfr {
    type Output = Self;

    #[inline]
    fn log(self) -> Self::Output {
        self.log_custom(MpfrRnd::HalfToEven)
    }

    #[inline]
    fn exp(self) -> Self::Output {
        self.exp_custom(MpfrRnd::HalfToEven)
    }

    #[inline]
    fn pow(self, rhs: Self) -> Self::Output {
        self.pow_custom(rhs, MpfrRnd::HalfToEven)
    }

    #[inline]
    fn sin(self) -> Self::Output {
        self.sin_custom(MpfrRnd::HalfToEven)
    }

    #[inline]
    fn cos(self) -> Self::Output {
        self.cos_custom(MpfrRnd::HalfToEven)
    }

    #[inline]
    fn tan(self) -> Self::Output {
        self.tan_custom(MpfrRnd::HalfToEven)
    }

    #[inline]
    fn sqrt(self) -> Self::Output {
        self.sqrt_custom(MpfrRnd::HalfToEven)
    }

    #[inline]
    fn abs(self) -> Self::Output {
        self.abs_custom(MpfrRnd::HalfToEven)
    }

    #[inline]
    fn signum(self) -> Self::Output {
        self.sgn_custom()
    }
}
