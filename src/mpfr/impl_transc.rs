use super::capi::*;
use super::def::Mpfr;

use transc::Transc;

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
}
