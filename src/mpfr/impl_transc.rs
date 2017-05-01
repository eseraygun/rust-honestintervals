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
}

impl Transc for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn log(self) -> Self::Output {
        self.log_custom(MpfrRnd::HalfToEven)
    }

    #[inline]
    fn exp(self) -> Self::Output {
        self.exp_custom(MpfrRnd::HalfToEven)
    }
}
