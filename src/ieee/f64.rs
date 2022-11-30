use fp;
use fp::{Float, Sign};
use transc::Transc;

impl Transc for f64 {
    type Output = Self;

    #[inline]
    fn log(self) -> Self::Output {
        self.ln()
    }

    #[inline]
    fn exp(self) -> Self::Output {
        self.exp()
    }

    #[inline]
    fn pow(self, rhs: Self) -> Self::Output {
        self.powf(rhs)
    }
}

impl fp::From<f64> for f64 {
    #[inline]
    fn from_lo(val: f64, _: usize) -> Self {
        val
    }

    #[inline]
    fn from_hi(val: f64, _: usize) -> Self {
        val
    }
}

impl fp::FromStr for f64 {}

impl fp::Into<f64> for f64 {
    #[inline]
    fn into_lo(self) -> f64 {
        self
    }

    #[inline]
    fn into_hi(self) -> f64 {
        self
    }
}

impl fp::MinMax for f64 {
    #[inline]
    fn min(self, rhs: Self) -> Self {
        self.min(rhs)
    }

    #[inline]
    fn max(self, rhs: Self) -> Self {
        self.max(rhs)
    }
}

impl fp::Abs for f64 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}

impl fp::Add for f64 {}
impl fp::Sub for f64 {}
impl fp::Mul for f64 {}
impl fp::Div for f64 {}
impl fp::Transc for f64 {}

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

#[cfg(test)]
mod tests {
    const PREC: usize = 53;

    #[test]
    fn test_from_lo() {
        use fp::From;
        assert_eq!(1.1000000000000001, f64::from_lo(1.1, PREC));
        assert_eq!(0.9000000000000000, f64::from_lo(0.9, PREC));
        assert_eq!(f64::INFINITY, f64::from_lo(f64::INFINITY, PREC));
        assert_eq!(f64::NEG_INFINITY, f64::from_lo(f64::NEG_INFINITY, PREC));
        assert!(f64::from_lo(f64::NAN, PREC).is_nan());
    }

    #[test]
    fn test_from_hi() {
        use fp::From;
        assert_eq!(1.1000000000000001, f64::from_hi(1.1, PREC));
        assert_eq!(0.9000000000000000, f64::from_hi(0.9, PREC));
        assert_eq!(f64::INFINITY, f64::from_hi(f64::INFINITY, PREC));
        assert_eq!(f64::NEG_INFINITY, f64::from_hi(f64::NEG_INFINITY, PREC));
        assert!(f64::is_nan(f64::from_hi(f64::NAN, PREC)));
    }

    #[test]
    fn test_from_str_lo() {
        use fp::FromStr;
        assert_eq!(0.9, f64::from_str_lo("0.9", PREC).unwrap());
        assert_eq!(1.1, f64::from_str_lo("1.1", PREC).unwrap());
        assert_eq!(0.0000000000000000, f64::from_str_lo("0", PREC).unwrap());
        assert_eq!(-0.9000000000000000, f64::from_str_lo("-0.9", PREC).unwrap());
        assert_eq!(-1.1000000000000001, f64::from_str_lo("-1.1", PREC).unwrap());
        assert_eq!(f64::INFINITY, f64::from_str_lo("inf", PREC).unwrap());
        assert_eq!(f64::NEG_INFINITY, f64::from_str_lo("-inf", PREC).unwrap());
        assert!(f64::is_nan(f64::from_str_lo("NaN", PREC).unwrap()));
    }

    #[test]
    fn test_from_str_hi() {
        use fp::FromStr;
        assert_eq!(0.0000000000000000, f64::from_str_hi("0", PREC).unwrap());
        assert_eq!(0.9000000000000000, f64::from_str_hi("0.9", PREC).unwrap());
        assert_eq!(1.1000000000000001, f64::from_str_hi("1.1", PREC).unwrap());
        assert_eq!(-0.9, f64::from_str_hi("-0.9", PREC).unwrap());
        assert_eq!(-1.1, f64::from_str_hi("-1.1", PREC).unwrap());
        assert_eq!(f64::INFINITY, f64::from_str_hi("inf", PREC).unwrap());
        assert_eq!(f64::NEG_INFINITY, f64::from_str_hi("-inf", PREC).unwrap());
        assert!(f64::is_nan(f64::from_str_hi("NaN", PREC).unwrap()));
    }

    #[test]
    fn test_into_lo_f64() {
        use fp::Into;
        assert_eq!(0.9999999999999999, 0.9999999999999999.into_lo());
        assert_eq!(1.000000000000001, 1.000000000000001.into_lo());
        assert_eq!(-0.9999999999999999, -0.9999999999999999.into_lo());
        assert_eq!(-1.000000000000001, -1.000000000000001.into_lo());
        assert_eq!(f64::INFINITY, f64::INFINITY.into_lo());
        assert_eq!(f64::NEG_INFINITY, f64::NEG_INFINITY.into_lo());
        assert!(f64::is_nan(Into::<f64>::into_lo(f64::NAN)));
    }

    #[test]
    fn test_into_hi_f64() {
        use fp::Into;
        assert_eq!(0.9999999999999999, 0.9999999999999999.into_hi());
        assert_eq!(1.000000000000001, 1.000000000000001.into_hi());
        assert_eq!(-0.9999999999999999, -0.9999999999999999.into_hi());
        assert_eq!(-1.000000000000001, -1.000000000000001.into_hi());
        assert_eq!(f64::INFINITY, f64::INFINITY.into_hi());
        assert_eq!(f64::NEG_INFINITY, f64::NEG_INFINITY.into_hi());
        assert!(f64::is_nan(Into::<f64>::into_hi(f64::NAN)));
    }

    #[test]
    fn test_min() {
        use fp::MinMax;
        assert_eq!(0.0, 0.0.min(1.0));
        assert_eq!(0.0, 1.0.min(0.0));
    }

    #[test]
    fn test_max() {
        use fp::MinMax;
        assert_eq!(1.0, 0.0.max(1.0));
        assert_eq!(1.0, 1.0.max(0.0));
    }

    #[test]
    fn test_abs() {
        use fp::Abs;
        assert_eq!(0.0, 0.0.abs());
        assert_eq!(1.0, 1.0.abs());
        assert_eq!(1.0, (-1.0).abs());
    }

    #[test]
    fn test_add_lo() {
        use fp::Add;
        assert_eq!(1.1, 0.1.add_lo(1.0));
        assert_eq!(-1.1, (-0.1).add_lo(-1.0));
    }

    #[test]
    fn test_add_hi() {
        use fp::Add;
        assert_eq!(1.1, 0.1.add_hi(1.0));
        assert_eq!(-1.1, (-0.1).add_hi(-1.0));
    }

    #[test]
    fn test_sub_lo() {
        use fp::Sub;
        assert_eq!(-0.9, 0.1.sub_lo(1.0));
        assert_eq!(0.9, (-0.1).sub_lo(-1.0));
    }

    #[test]
    fn test_sub_hi() {
        use fp::Sub;
        assert_eq!(-0.9, 0.1.sub_hi(1.0));
        assert_eq!(0.9, (-0.1).sub_hi(-1.0));
    }

    #[test]
    fn test_mul_lo() {
        use fp::Mul;
        assert_eq!(0.9900000000000001, 1.1.mul_lo(0.9));
        assert_eq!(-0.9900000000000001, (-1.1).mul_lo(0.9));
    }

    #[test]
    fn test_mul_hi() {
        use fp::Mul;
        assert_eq!(0.9900000000000001, 1.1.mul_hi(0.9));
        assert_eq!(-0.9900000000000001, (-1.1).mul_hi(0.9));
    }

    #[test]
    fn test_div_lo() {
        use fp::Div;
        assert_eq!(1.2222222222222223, 1.1.div_lo(0.9));
        assert_eq!(-1.2222222222222223, (-1.1).div_lo(0.9));
    }

    #[test]
    fn test_div_hi() {
        use fp::Div;
        assert_eq!(1.2222222222222223, 1.1.div_hi(0.9));
        assert_eq!(-1.2222222222222223, (-1.1).div_hi(0.9));
    }

    #[test]
    fn test_log_lo() {
        use fp::Transc;
        assert_eq!(0.0, 1.0.log_lo());
        assert_eq!(-0.6931471805599453, 0.5.log_lo());
        assert_eq!(0.6931471805599453, 2.0.log_lo());
    }

    #[test]
    fn test_log_hi() {
        use fp::Transc;
        assert_eq!(0.0, 1.0.log_hi());
        assert_eq!(-0.6931471805599453, 0.5.log_hi());
        assert_eq!(0.6931471805599453, 2.0.log_hi());
    }

    #[test]
    fn test_exp_lo() {
        use fp::Transc;
        assert_eq!(2.718281828459045, 1.0.exp_lo());
        assert_eq!(0.36787944117144233, (-1.0).exp_lo());
        assert_eq!(1.6487212707001282, 0.5.exp_lo());
        assert_eq!(0.6065306597126334, (-0.5).exp_lo());
    }

    #[test]
    fn test_exp_hi() {
        use fp::Transc;
        assert_eq!(2.7182818284590453, 1.0.exp_hi());
        assert_eq!(0.36787944117144233, (-1.0).exp_hi());
        assert_eq!(1.6487212707001282, 0.5.exp_hi());
        assert_eq!(0.6065306597126334, (-0.5).exp_hi());
    }

    #[test]
    fn test_pow_lo() {
        use fp::Transc;
        assert_eq!(1.0, 1.0.pow_lo(2.0));
        assert_eq!(1.4142135623730951, 2.0.pow_lo(0.5));
    }

    #[test]
    fn test_pow_hi() {
        use fp::Transc;
        assert_eq!(1.0, 1.0.pow_hi(2.0));
        assert_eq!(1.4142135623730951, 2.0.pow_hi(0.5));
    }

    #[test]
    fn test_constants() {
        use fp::Float;
        assert_eq!(0.0, f64::zero(PREC));
        assert_eq!(f64::INFINITY, f64::one(PREC) / f64::zero(PREC));
        assert_eq!(0.0, f64::neg_zero(PREC));
        assert_eq!(f64::NEG_INFINITY, f64::one(PREC) / f64::neg_zero(PREC));
        assert_eq!(1.0, f64::one(PREC));
        assert_eq!(f64::INFINITY, f64::infinity(PREC));
        assert_eq!(f64::NEG_INFINITY, f64::neg_infinity(PREC));
        assert!(f64::is_nan(f64::nan(PREC)));
    }

    #[test]
    fn test_is_finite() {
        use fp::Float;
        assert!(0.0.is_finite());
        assert!(1.0.is_finite());
        assert!(!f64::infinity(PREC).is_finite());
        assert!(!f64::neg_infinity(PREC).is_finite());
        assert!(!f64::nan(PREC).is_finite());
    }

    #[test]
    fn test_is_infinite() {
        use fp::Float;
        assert!(!0.0.is_infinite());
        assert!(!1.0.is_infinite());
        assert!(f64::infinity(PREC).is_infinite());
        assert!(f64::neg_infinity(PREC).is_infinite());
        assert!(!f64::nan(PREC).is_infinite());
    }

    #[test]
    fn test_is_zero() {
        use fp::Float;
        assert!(0.0.is_zero());
        assert!(!1.0.is_zero());
        assert!(!f64::infinity(PREC).is_zero());
        assert!(!f64::neg_infinity(PREC).is_zero());
        assert!(!f64::nan(PREC).is_zero());
    }

    #[test]
    fn test_is_infinity() {
        use fp::Float;
        assert!(!0.0.is_infinity());
        assert!(!1.0.is_infinity());
        assert!(f64::infinity(PREC).is_infinity());
        assert!(!f64::neg_infinity(PREC).is_infinity());
        assert!(!f64::nan(PREC).is_infinity());
    }

    #[test]
    fn test_is_neg_infinity() {
        use fp::Float;
        assert!(!0.0.is_neg_infinity());
        assert!(!1.0.is_neg_infinity());
        assert!(!f64::infinity(PREC).is_neg_infinity());
        assert!(f64::neg_infinity(PREC).is_neg_infinity());
        assert!(!f64::nan(PREC).is_neg_infinity());
    }

    #[test]
    fn test_is_nan() {
        use fp::Float;
        assert!(!0.0.is_nan());
        assert!(!1.0.is_nan());
        assert!(!f64::infinity(PREC).is_nan());
        assert!(!f64::neg_infinity(PREC).is_nan());
        assert!(f64::nan(PREC).is_nan());
    }
}
