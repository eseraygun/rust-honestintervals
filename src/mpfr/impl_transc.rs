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

//impl<'a> Transc for &'a Mpfr {
//    type Output = Mpfr;
//
//    #[inline]
//    fn log(self) -> Self::Output {
//        Float::log(self, RoundingMode::HalfToEven)
//    }
//
//    #[inline]
//    fn exp(self) -> Self::Output {
//        Float::exp(self, RoundingMode::HalfToEven)
//    }
//}

#[cfg(test)]
mod test {
    use super::super::def::Mpfr;

    use fp::Float;
    use transc::Transc;

    #[test]
    fn test_log() {
        assert_str_eq!("-inf", mpfr!(0).log());
        assert_str_eq!("0", mpfr!(1).log());
        assert_str_eq!("NaN", mpfr!(-1).log());
        assert_str_eq!("NaN", mpfr_nan!().log());
        assert_str_eq!("inf", mpfr_inf!().log());
        assert_str_eq!("NaN", mpfr_neg_inf!().log());
    }

    #[test]
    fn test_exp() {
        assert_str_eq!("1", mpfr!(0).exp());
        assert_str_eq!("2.718281828459045", mpfr!(1).exp());
        assert_str_eq!("0.36787944117144233", mpfr!(-1).exp());
        assert_str_eq!("NaN", mpfr_nan!().exp());
        assert_str_eq!("inf", mpfr_inf!().exp());
        assert_str_eq!("0", mpfr_neg_inf!().exp());
    }
}
