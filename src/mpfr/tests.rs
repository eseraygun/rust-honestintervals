use super::{Mpfr, MpfrRnd, ParseMpfrError};

const PREC: usize = 2;

macro_rules! assert_str_eq {
    ($x:expr, $y:expr) => { assert_eq!($x, format!("{}", $y)) };
    ($x:expr, $y:expr, $($arg:tt)+) => { assert_eq!($x, format!("{}", $y), $($arg)+) };
}

macro_rules! mpfr {
    ($v:expr) => {
        ::mpfr::Mpfr::from_str_with_prec($v, PREC).unwrap()
    };
    ($v:expr, $p:expr) => {
        ::mpfr::Mpfr::from_str_with_prec($v, $p).unwrap()
    };
}

#[test]
fn test_from_f64() {
    use std::f64;
    assert_str_eq!("0", Mpfr::from(0f64));
    assert_str_eq!("0.9999999999999999", Mpfr::from(0.9999999999999999));
    assert_str_eq!("1.000000000000001", Mpfr::from(1.000000000000001));
    assert_str_eq!("-0.9999999999999999", Mpfr::from(-0.9999999999999999));
    assert_str_eq!("-1.000000000000001", Mpfr::from(-1.000000000000001));
    assert_str_eq!("inf", Mpfr::from(f64::INFINITY));
    assert_str_eq!("-inf", Mpfr::from(f64::NEG_INFINITY));
    assert_str_eq!("NaN", Mpfr::from(f64::NAN));
}

#[test]
fn test_from_str() {
    use std::str::FromStr;
    assert_str_eq!("0", Mpfr::from_str("0").unwrap());
    assert_str_eq!(
        "0.9999999999999999",
        Mpfr::from_str("0.9999999999999999").unwrap()
    );
    assert_str_eq!(
        "1.000000000000001",
        Mpfr::from_str("1.000000000000001").unwrap()
    );
    assert_str_eq!(
        "-0.9999999999999999",
        Mpfr::from_str("-0.9999999999999999").unwrap()
    );
    assert_str_eq!(
        "-1.000000000000001",
        Mpfr::from_str("-1.000000000000001").unwrap()
    );
    assert_str_eq!("inf", Mpfr::from_str("inf").unwrap());
    assert_str_eq!("-inf", Mpfr::from_str("-inf").unwrap());
    assert_str_eq!("NaN", Mpfr::from_str("NaN").unwrap());
}

#[test]
fn test_from_str_custom_failures() {
    assert_eq!(
        ParseMpfrError::CStringError,
        Mpfr::from_str_custom("0\00", PREC, MpfrRnd::HalfToEven)
            .err()
            .unwrap()
    );
    assert_eq!(
        ParseMpfrError::MpfrParseError,
        Mpfr::from_str_custom("0a0", PREC, MpfrRnd::HalfToEven)
            .err()
            .unwrap()
    );
}

#[test]
fn test_clone() {
    let x = mpfr!("123.456");
    assert_eq!(x, x.clone());
    let mut y = mpfr!("456.123");
    y.clone_from(&x);
    assert_eq!(x, y);
}

#[test]
fn test_into_f64() {
    use std::f64;
    assert_eq!(0f64, mpfr!("0", 113).into());
    assert_eq!(1.0, mpfr!("0.99999999999999999", 113).into());
    assert_eq!(1.0, mpfr!("1.0000000000000001", 113).into());
    assert_eq!(-1.0, mpfr!("-0.99999999999999999", 113).into());
    assert_eq!(-1.0, mpfr!("-1.0000000000000001", 113).into());
    assert_eq!(f64::INFINITY, mpfr!("inf", 113).into());
    assert_eq!(f64::NEG_INFINITY, mpfr!("-inf", 113).into());
    assert!(Into::<f64>::into(mpfr!("nan", 113)).is_nan());
}

#[test]
fn test_partial_eq() {
    assert!(mpfr!("0") == mpfr!("0"));
    assert!(mpfr!("0") == mpfr!("0", PREC + 1));
    assert!(mpfr!("0") != mpfr!("1"));
    assert!(mpfr!("inf") == mpfr!("inf"));
    assert!(mpfr!("-inf") == mpfr!("-inf"));
    assert!(mpfr!("NaN") != mpfr!("NaN"));
}

#[test]
fn test_partial_ord_cmp() {
    use std::cmp::Ordering;
    assert_eq!(
        Ordering::Equal,
        mpfr!("0").partial_cmp(&mpfr!("0")).unwrap()
    );
    assert_eq!(Ordering::Less, mpfr!("0").partial_cmp(&mpfr!("1")).unwrap());
    assert_eq!(
        Ordering::Greater,
        mpfr!("1").partial_cmp(&mpfr!("0")).unwrap()
    );
    assert_eq!(
        Ordering::Equal,
        mpfr!("-inf").partial_cmp(&mpfr!("-inf")).unwrap()
    );
    assert_eq!(
        Ordering::Equal,
        mpfr!("inf").partial_cmp(&mpfr!("inf")).unwrap()
    );
    assert_eq!(
        Ordering::Less,
        mpfr!("-inf").partial_cmp(&mpfr!("inf")).unwrap()
    );
    assert_eq!(
        Ordering::Greater,
        mpfr!("inf").partial_cmp(&mpfr!("-inf")).unwrap()
    );
    assert!(mpfr!("NaN").partial_cmp(&mpfr!("NaN")).is_none());
}

#[test]
fn test_partial_ord_rest() {
    assert!(mpfr!("0") <= mpfr!("0"));
    assert!(mpfr!("0") >= mpfr!("0"));
    assert!(mpfr!("0") < mpfr!("1"));
    assert!(mpfr!("1") > mpfr!("0"));
    assert!(mpfr!("-inf") <= mpfr!("-inf"));
    assert!(mpfr!("inf") >= mpfr!("inf"));
    assert!(mpfr!("-inf") < mpfr!("inf"));
    assert!(mpfr!("inf") > mpfr!("-inf"));
    assert!(!(mpfr!("NaN") <= mpfr!("NaN")));
    assert!(!(mpfr!("NaN") >= mpfr!("NaN")));
    assert!(!(mpfr!("NaN") < mpfr!("NaN")));
    assert!(!(mpfr!("NaN") > mpfr!("NaN")));
}

#[test]
fn test_from_lo() {
    use fp::From;
    use std::f64;
    assert_str_eq!("0", Mpfr::from_lo(0f64, PREC));
    assert_str_eq!("0.75", Mpfr::from_lo(0.9, PREC));
    assert_str_eq!("1", Mpfr::from_lo(1.1, PREC));
    assert_str_eq!("-1", Mpfr::from_lo(-0.9, PREC));
    assert_str_eq!("-1.5", Mpfr::from_lo(-1.1, PREC));
    assert_str_eq!("inf", Mpfr::from_lo(f64::INFINITY, PREC));
    assert_str_eq!("-inf", Mpfr::from_lo(f64::NEG_INFINITY, PREC));
    assert_str_eq!("NaN", Mpfr::from_lo(f64::NAN, PREC));
}

#[test]
fn test_from_hi() {
    use fp::From;
    use std::f64;
    assert_str_eq!("0", Mpfr::from_hi(0f64, PREC));
    assert_str_eq!("1", Mpfr::from_hi(0.9, PREC));
    assert_str_eq!("1.5", Mpfr::from_hi(1.1, PREC));
    assert_str_eq!("-0.75", Mpfr::from_hi(-0.9, PREC));
    assert_str_eq!("-1", Mpfr::from_hi(-1.1, PREC));
    assert_str_eq!("inf", Mpfr::from_hi(f64::INFINITY, PREC));
    assert_str_eq!("-inf", Mpfr::from_hi(f64::NEG_INFINITY, PREC));
    assert_str_eq!("NaN", Mpfr::from_hi(f64::NAN, PREC));
}

#[test]
fn test_from_str_lo() {
    use fp::FromStr;
    assert_str_eq!("0", Mpfr::from_str_lo("0", PREC).unwrap());
    assert_str_eq!("0.75", Mpfr::from_str_lo("0.9", PREC).unwrap());
    assert_str_eq!("1", Mpfr::from_str_lo("1.1", PREC).unwrap());
    assert_str_eq!("-1", Mpfr::from_str_lo("-0.9", PREC).unwrap());
    assert_str_eq!("-1.5", Mpfr::from_str_lo("-1.1", PREC).unwrap());
    assert_str_eq!("inf", Mpfr::from_str_lo("inf", PREC).unwrap());
    assert_str_eq!("-inf", Mpfr::from_str_lo("-inf", PREC).unwrap());
    assert_str_eq!("NaN", Mpfr::from_str_lo("NaN", PREC).unwrap());
}

#[test]
fn test_from_str_hi() {
    use fp::FromStr;
    assert_str_eq!("0", Mpfr::from_str_hi("0", PREC).unwrap());
    assert_str_eq!("1", Mpfr::from_str_hi("0.9", PREC).unwrap());
    assert_str_eq!("1.5", Mpfr::from_str_hi("1.1", PREC).unwrap());
    assert_str_eq!("-0.75", Mpfr::from_str_hi("-0.9", PREC).unwrap());
    assert_str_eq!("-1", Mpfr::from_str_hi("-1.1", PREC).unwrap());
    assert_str_eq!("inf", Mpfr::from_str_hi("inf", PREC).unwrap());
    assert_str_eq!("-inf", Mpfr::from_str_hi("-inf", PREC).unwrap());
    assert_str_eq!("NaN", Mpfr::from_str_hi("NaN", PREC).unwrap());
}

#[test]
fn test_into_lo_f64() {
    use fp::Into;
    use std::f64;
    assert_eq!(
        0.9999999999999999,
        mpfr!("0.99999999999999999", 113).into_lo()
    );
    assert_eq!(1.0, mpfr!("1.0000000000000001", 113).into_lo());
    assert_eq!(-1.0, mpfr!("-0.99999999999999999", 113).into_lo());
    assert_eq!(
        -1.0000000000000002,
        mpfr!("-1.0000000000000001", 113).into_lo()
    );
    assert_eq!(f64::INFINITY, mpfr!("inf", 113).into_lo());
    assert_eq!(f64::NEG_INFINITY, mpfr!("-inf", 113).into_lo());
    assert!(Into::<f64>::into_lo(mpfr!("nan", 113)).is_nan());
}

#[test]
fn test_into_hi_f64() {
    use fp::Into;
    use std::f64;
    assert_eq!(1.0, mpfr!("0.99999999999999999", 113).into_hi());
    assert_eq!(
        1.0000000000000002,
        mpfr!("1.0000000000000001", 113).into_hi()
    );
    assert_eq!(
        -0.9999999999999999,
        mpfr!("-0.99999999999999999", 113).into_hi()
    );
    assert_eq!(-1.0, mpfr!("-1.0000000000000001", 113).into_hi());
    assert_eq!(f64::INFINITY, mpfr!("inf", 113).into_hi());
    assert_eq!(f64::NEG_INFINITY, mpfr!("-inf", 113).into_hi());
    assert!(Into::<f64>::into_hi(mpfr!("nan", 113)).is_nan());
}

#[test]
fn test_min() {
    use fp::MinMax;
    assert_str_eq!("0", mpfr!("0").min(mpfr!("1")));
    assert_str_eq!("0", mpfr!("1").min(mpfr!("0")));
}

#[test]
fn test_max() {
    use fp::MinMax;
    assert_str_eq!("1", mpfr!("0").max(mpfr!("1")));
    assert_str_eq!("1", mpfr!("1").max(mpfr!("0")));
}

#[test]
fn test_neg() {
    use std::ops::Neg;
    assert_str_eq!("-0", mpfr!("0").neg());
    assert_str_eq!("0", mpfr!("-0").neg());
    assert_str_eq!("-1", mpfr!("1").neg());
    assert_str_eq!("1", mpfr!("-1").neg());
}

#[test]
fn test_abs() {
    use fp::Abs;
    assert_str_eq!("0", mpfr!("0").abs());
    assert_str_eq!("1", mpfr!("1").abs());
    assert_str_eq!("1", mpfr!("-1").abs());
}

#[test]
fn test_add_lo() {
    use fp::Add;
    assert_str_eq!("1.5", mpfr!("0.75").add_lo(mpfr!("1")));
    assert_str_eq!("-2", mpfr!("-0.75").add_lo(mpfr!("-1")));
}

#[test]
fn test_add_hi() {
    use fp::Add;
    assert_str_eq!("2", mpfr!("0.75").add_hi(mpfr!("1")));
    assert_str_eq!("-1.5", mpfr!("-0.75").add_hi(mpfr!("-1")));
}

#[test]
fn test_sub_lo() {
    use fp::Sub;
    assert_str_eq!("1.5", mpfr!("0.75").sub_lo(mpfr!("-1")));
    assert_str_eq!("-2", mpfr!("-0.75").sub_lo(mpfr!("1")));
}

#[test]
fn test_sub_hi() {
    use fp::Sub;
    assert_str_eq!("2", mpfr!("0.75").sub_hi(mpfr!("-1")));
    assert_str_eq!("-1.5", mpfr!("-0.75").sub_hi(mpfr!("1")));
}

#[test]
fn test_mul_lo() {
    use fp::Mul;
    assert_str_eq!("2", mpfr!("0.75").mul_lo(mpfr!("3")));
    assert_str_eq!("-3", mpfr!("-0.75").mul_lo(mpfr!("3")));
}

#[test]
fn test_mul_hi() {
    use fp::Mul;
    assert_str_eq!("3", mpfr!("0.75").mul_hi(mpfr!("3")));
    assert_str_eq!("-2", mpfr!("-0.75").mul_hi(mpfr!("3")));
}

#[test]
fn test_div_lo() {
    use fp::Div;
    assert_str_eq!("0.5", mpfr!("2").div_lo(mpfr!("3")));
    assert_str_eq!("-0.75", mpfr!("-2").div_lo(mpfr!("3")));
}

#[test]
fn test_div_hi() {
    use fp::Div;
    assert_str_eq!("0.75", mpfr!("2").div_hi(mpfr!("3")));
    assert_str_eq!("-0.5", mpfr!("-2").div_hi(mpfr!("3")));
}

#[test]
fn test_log_lo() {
    use fp::Transc;
    assert_str_eq!("0", mpfr!("1").log_lo());
    assert_str_eq!("0.5", mpfr!("2").log_lo());
}

#[test]
fn test_log_hi() {
    use fp::Transc;
    assert_str_eq!("0", mpfr!("1").log_hi());
    assert_str_eq!("0.75", mpfr!("2").log_hi());
}

#[test]
fn test_exp_lo() {
    use fp::Transc;
    assert_str_eq!("2", mpfr!("1").exp_lo());
    assert_str_eq!("6", mpfr!("2").exp_lo());
}

#[test]
fn test_exp_hi() {
    use fp::Transc;
    assert_str_eq!("3", mpfr!("1").exp_hi());
    assert_str_eq!("8", mpfr!("2").exp_hi());
}

#[test]
fn test_pow_lo() {
    use fp::Transc;
    assert_str_eq!("1", mpfr!("1").pow_lo(mpfr!("1.5")));
    assert_str_eq!("0.5", mpfr!("0.5").pow_lo(mpfr!("0.75")));
}

#[test]
fn test_pow_hi() {
    use fp::Transc;
    assert_str_eq!("1", mpfr!("1").pow_hi(mpfr!("1.5")));
    assert_str_eq!("0.75", mpfr!("0.5").pow_hi(mpfr!("0.75")));
}

#[test]
fn test_constants() {
    use fp::Float;
    assert_str_eq!("0", Mpfr::zero(PREC));
    assert_str_eq!("inf", Mpfr::one(PREC) / Mpfr::zero(PREC));
    assert_str_eq!("-0", Mpfr::neg_zero(PREC));
    assert_str_eq!("-inf", Mpfr::one(PREC) / Mpfr::neg_zero(PREC));
    assert_str_eq!("1", Mpfr::one(PREC));
    assert_str_eq!("inf", Mpfr::infinity(PREC));
    assert_str_eq!("-inf", Mpfr::neg_infinity(PREC));
    assert_str_eq!("NaN", Mpfr::nan(PREC));
}

#[test]
fn test_is_finite() {
    use fp::Float;
    assert!(mpfr!("0").is_finite());
    assert!(mpfr!("1").is_finite());
    assert!(!mpfr!("inf").is_finite());
    assert!(!mpfr!("-inf").is_finite());
    assert!(!mpfr!("NaN").is_finite());
}

#[test]
fn test_is_infinite() {
    use fp::Float;
    assert!(!mpfr!("0").is_infinite());
    assert!(!mpfr!("1").is_infinite());
    assert!(mpfr!("inf").is_infinite());
    assert!(mpfr!("-inf").is_infinite());
    assert!(!mpfr!("NaN").is_infinite());
}

#[test]
fn test_is_zero() {
    use fp::Float;
    assert!(mpfr!("0").is_zero());
    assert!(!mpfr!("1").is_zero());
    assert!(!mpfr!("inf").is_zero());
    assert!(!mpfr!("-inf").is_zero());
    assert!(!mpfr!("NaN").is_zero());
}

#[test]
fn test_is_infinity() {
    use fp::Float;
    assert!(!mpfr!("0").is_infinity());
    assert!(!mpfr!("1").is_infinity());
    assert!(mpfr!("inf").is_infinity());
    assert!(!mpfr!("-inf").is_infinity());
    assert!(!mpfr!("NaN").is_infinity());
}

#[test]
fn test_is_neg_infinity() {
    use fp::Float;
    assert!(!mpfr!("0").is_neg_infinity());
    assert!(!mpfr!("1").is_neg_infinity());
    assert!(!mpfr!("inf").is_neg_infinity());
    assert!(mpfr!("-inf").is_neg_infinity());
    assert!(!mpfr!("NaN").is_neg_infinity());
}

#[test]
fn test_is_nan() {
    use fp::Float;
    assert!(!mpfr!("0").is_nan());
    assert!(!mpfr!("1").is_nan());
    assert!(!mpfr!("inf").is_nan());
    assert!(!mpfr!("-inf").is_nan());
    assert!(mpfr!("NaN").is_nan());
}
