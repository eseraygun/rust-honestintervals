use super::{Mpfr, MpfrRnd, ParseMpfrError};

const PREC: usize = 2;

macro_rules! mpfr {
    ($v:expr) => { ::mpfr::Mpfr::from_str_with_prec($v, PREC).unwrap() };
    ($v:expr, $p:expr) => { ::mpfr::Mpfr::from_str_with_prec($v, $p).unwrap() };
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
    assert_str_eq!("0.9999999999999999", Mpfr::from_str("0.9999999999999999").unwrap());
    assert_str_eq!("1.000000000000001", Mpfr::from_str("1.000000000000001").unwrap());
    assert_str_eq!("-0.9999999999999999", Mpfr::from_str("-0.9999999999999999").unwrap());
    assert_str_eq!("-1.000000000000001", Mpfr::from_str("-1.000000000000001").unwrap());
    assert_str_eq!("inf", Mpfr::from_str("inf").unwrap());
    assert_str_eq!("-inf", Mpfr::from_str("-inf").unwrap());
    assert_str_eq!("NaN", Mpfr::from_str("NaN").unwrap());
}

#[test]
fn test_from_str_custom_failures() {
    assert_eq!(ParseMpfrError::CStringError,
               Mpfr::from_str_custom("0\00", PREC, MpfrRnd::HalfToEven).err().unwrap());
    assert_eq!(ParseMpfrError::MpfrError,
               Mpfr::from_str_custom("0a0", PREC, MpfrRnd::HalfToEven).err().unwrap());
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
    assert_eq!(Ordering::Equal, mpfr!("0").partial_cmp(&mpfr!("0")).unwrap());
    assert_eq!(Ordering::Less, mpfr!("0").partial_cmp(&mpfr!("1")).unwrap());
    assert_eq!(Ordering::Greater, mpfr!("1").partial_cmp(&mpfr!("0")).unwrap());
    assert_eq!(Ordering::Equal, mpfr!("-inf").partial_cmp(&mpfr!("-inf")).unwrap());
    assert_eq!(Ordering::Equal, mpfr!("inf").partial_cmp(&mpfr!("inf")).unwrap());
    assert_eq!(Ordering::Less, mpfr!("-inf").partial_cmp(&mpfr!("inf")).unwrap());
    assert_eq!(Ordering::Greater, mpfr!("inf").partial_cmp(&mpfr!("-inf")).unwrap());
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
    assert_eq!(0.9999999999999999, mpfr!("0.99999999999999999", 113).into_lo());
    assert_eq!(1.0, mpfr!("1.0000000000000001", 113).into_lo());
    assert_eq!(-1.0, mpfr!("-0.99999999999999999", 113).into_lo());
    assert_eq!(-1.0000000000000002, mpfr!("-1.0000000000000001", 113).into_lo());
    assert_eq!(f64::INFINITY, mpfr!("inf", 113).into_lo());
    assert_eq!(f64::NEG_INFINITY, mpfr!("-inf", 113).into_lo());
    assert!(Into::<f64>::into_lo(mpfr!("nan", 113)).is_nan());
}

#[test]
fn test_into_hi_f64() {
    use fp::Into;
    use std::f64;
    assert_eq!(1.0, mpfr!("0.99999999999999999", 113).into_hi());
    assert_eq!(1.0000000000000002, mpfr!("1.0000000000000001", 113).into_hi());
    assert_eq!(-0.9999999999999999, mpfr!("-0.99999999999999999", 113).into_hi());
    assert_eq!(-1.0, mpfr!("-1.0000000000000001", 113).into_hi());
    assert_eq!(f64::INFINITY, mpfr!("inf", 113).into_hi());
    assert_eq!(f64::NEG_INFINITY, mpfr!("-inf", 113).into_hi());
    assert!(Into::<f64>::into_hi(mpfr!("nan", 113)).is_nan());
}

#[test]
fn test_min() {
    use fp::MinMax;
    test_binary_op(Mpfr::min, all_signs(), vec![
        ("*.ninf", "-inf"),
        ("ninf.*", "-inf"),
        ("nan.nan", "NaN"),
        ("nan.inf", "inf"),
        ("nan.z", "0"),
        ("nan.p", "1"),
        ("nan.n", "-1"),
        ("inf.nan", "inf"),
        ("inf.inf", "inf"),
        ("inf.z", "0"),
        ("inf.p", "1"),
        ("inf.n", "-1"),
        ("z.nan", "0"),
        ("z.inf", "0"),
        ("z.z", "0"),
        ("z.p", "0"),
        ("z.n", "-1"),
        ("p.nan", "1"),
        ("p.inf", "1"),
        ("p.z", "0"),
        ("p.p", "1"),
        ("p.n", "-1"),
        ("n.nan", "-1"),
        ("n.inf", "-1"),
        ("n.z", "-1"),
        ("n.p", "-1"),
        ("n.n", "-1"),
    ], false);
}

#[test]
fn test_max() {
    use fp::MinMax;
    test_binary_op(Mpfr::max, all_signs(), vec![
        ("*.inf", "inf"),
        ("inf.*", "inf"),
        ("nan.nan", "NaN"),
        ("nan.ninf", "-inf"),
        ("nan.z", "0"),
        ("nan.p", "1"),
        ("nan.n", "-1"),
        ("ninf.nan", "-inf"),
        ("ninf.inf", "inf"),
        ("ninf.ninf", "-inf"),
        ("ninf.z", "0"),
        ("ninf.p", "1"),
        ("ninf.n", "-1"),
        ("z.nan", "0"),
        ("z.ninf", "0"),
        ("z.z", "0"),
        ("z.p", "1"),
        ("z.n", "0"),
        ("p.nan", "1"),
        ("p.ninf", "1"),
        ("p.z", "1"),
        ("p.p", "1"),
        ("p.n", "1"),
        ("n.nan", "-1"),
        ("n.ninf", "-1"),
        ("n.z", "0"),
        ("n.p", "1"),
        ("n.n", "-1"),
    ], false);
}

#[test]
fn test_neg() {
    use std::ops::Neg;
    test_unary_op(Mpfr::neg, all_signs(), vec![
        ("nan", "NaN"),
        ("inf", "-inf"),
        ("ninf", "inf"),
        ("z", "0"),
        ("p", "-1"),
        ("n", "1"),
    ], false);
}

#[test]
fn test_abs() {
    use fp::Abs;
    test_unary_op(Mpfr::abs, all_signs(), vec![
        ("nan", "NaN"),
        ("inf", "inf"),
        ("ninf", "inf"),
        ("z", "0"),
        ("p", "1"),
        ("n", "1"),
    ], false);
}

#[test]
fn test_add_lo() {
    use fp::Add;
    test_binary_op(Mpfr::add_lo, all_signs_and_roundings(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("inf.ninf", "NaN"),
        ("ninf.inf", "NaN"),
        ("inf.*", "inf"),
        ("*.inf", "inf"),
        ("ninf.*", "-inf"),
        ("*.ninf", "-inf"),
        ("z.z", "0"),
        ("z.ps", "0.75"),
        ("z.pl", "1.5"),
        ("z.ns", "-0.75"),
        ("z.nl", "-1.5"),
        ("ps.z", "0.75"),
        ("ps.ps", "1.5"),
        ("ps.pl", "2"),
        ("ps.ns", "0"),
        ("ps.nl", "-0.75"),
        ("pl.z", "1.5"),
        ("pl.ps", "2"),
        ("pl.pl", "3"),
        ("pl.ns", "0.75"),
        ("pl.nl", "0"),
        ("ns.z", "-0.75"),
        ("ns.ps", "0"),
        ("ns.pl", "0.75"),
        ("ns.ns", "-1.5"),
        ("ns.nl", "-3"),
        ("nl.z", "-1.5"),
        ("nl.ps", "-0.75"),
        ("nl.pl", "0"),
        ("nl.ns", "-3"),
        ("nl.nl", "-3"),
    ], false);
}

#[test]
fn test_add_hi() {
    use fp::Add;
    test_binary_op(Mpfr::add_hi, all_signs_and_roundings(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("inf.ninf", "NaN"),
        ("ninf.inf", "NaN"),
        ("inf.*", "inf"),
        ("*.inf", "inf"),
        ("ninf.*", "-inf"),
        ("*.ninf", "-inf"),
        ("z.z", "0"),
        ("z.ps", "0.75"),
        ("z.pl", "1.5"),
        ("z.ns", "-0.75"),
        ("z.nl", "-1.5"),
        ("ps.z", "0.75"),
        ("ps.ps", "1.5"),
        ("ps.pl", "3"),
        ("ps.ns", "0"),
        ("ps.nl", "-0.75"),
        ("pl.z", "1.5"),
        ("pl.ps", "3"),
        ("pl.pl", "3"),
        ("pl.ns", "0.75"),
        ("pl.nl", "0"),
        ("ns.z", "-0.75"),
        ("ns.ps", "0"),
        ("ns.pl", "0.75"),
        ("ns.ns", "-1.5"),
        ("ns.nl", "-2"),
        ("nl.z", "-1.5"),
        ("nl.ps", "-0.75"),
        ("nl.pl", "0"),
        ("nl.ns", "-2"),
        ("nl.nl", "-3"),
    ], false);
}

#[test]
fn test_sub_lo() {
    use fp::Sub;
    test_binary_op(Mpfr::sub_lo, all_signs_and_roundings(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("inf.inf", "NaN"),
        ("ninf.ninf", "NaN"),
        ("inf.*", "inf"),
        ("*.inf", "-inf"),
        ("ninf.*", "-inf"),
        ("*.ninf", "inf"),
        ("z.z", "0"),
        ("z.ps", "-0.75"),
        ("z.pl", "-1.5"),
        ("z.ns", "0.75"),
        ("z.nl", "1.5"),
        ("ps.z", "0.75"),
        ("ps.ps", "0"),
        ("ps.pl", "-0.75"),
        ("ps.ns", "1.5"),
        ("ps.nl", "2"),
        ("pl.z", "1.5"),
        ("pl.ps", "0.75"),
        ("pl.pl", "0"),
        ("pl.ns", "2"),
        ("pl.nl", "3"),
        ("ns.z", "-0.75"),
        ("ns.ps", "-1.5"),
        ("ns.pl", "-3"),
        ("ns.ns", "0"),
        ("ns.nl", "0.75"),
        ("nl.z", "-1.5"),
        ("nl.ps", "-3"),
        ("nl.pl", "-3"),
        ("nl.ns", "-0.75"),
        ("nl.nl", "0"),
    ], false);
}

#[test]
fn test_sub_hi() {
    use fp::Sub;
    test_binary_op(Mpfr::sub_hi, all_signs_and_roundings(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("inf.inf", "NaN"),
        ("ninf.ninf", "NaN"),
        ("inf.*", "inf"),
        ("*.inf", "-inf"),
        ("ninf.*", "-inf"),
        ("*.ninf", "inf"),
        ("z.z", "0"),
        ("z.ps", "-0.75"),
        ("z.pl", "-1.5"),
        ("z.ns", "0.75"),
        ("z.nl", "1.5"),
        ("ps.z", "0.75"),
        ("ps.ps", "0"),
        ("ps.pl", "-0.75"),
        ("ps.ns", "1.5"),
        ("ps.nl", "3"),
        ("pl.z", "1.5"),
        ("pl.ps", "0.75"),
        ("pl.pl", "0"),
        ("pl.ns", "3"),
        ("pl.nl", "3"),
        ("ns.z", "-0.75"),
        ("ns.ps", "-1.5"),
        ("ns.pl", "-2"),
        ("ns.ns", "0"),
        ("ns.nl", "0.75"),
        ("nl.z", "-1.5"),
        ("nl.ps", "-2"),
        ("nl.pl", "-3"),
        ("nl.ns", "-0.75"),
        ("nl.nl", "0"),
    ], false);
}

#[test]
fn test_mul_lo() {
    use fp::Mul;
    test_binary_op(Mpfr::mul_lo, all_signs_and_roundings(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("inf.inf", "inf"),
        ("inf.ninf", "-inf"),
        ("inf.z", "NaN"),
        ("inf.ps", "inf"),
        ("inf.pl", "inf"),
        ("inf.ns", "-inf"),
        ("inf.nl", "-inf"),
        ("ninf.inf", "-inf"),
        ("ninf.ninf", "inf"),
        ("ninf.z", "NaN"),
        ("ninf.ps", "-inf"),
        ("ninf.pl", "-inf"),
        ("ninf.ns", "inf"),
        ("ninf.nl", "inf"),
        ("z.inf", "NaN"),
        ("z.ninf", "NaN"),
        ("z.*", "0"),
        ("*.z", "0"),
        ("ps.inf", "inf"),
        ("ps.ninf", "-inf"),
        ("ps.ps", "0.5"),
        ("ps.pl", "1"),
        ("ps.ns", "-0.75"),
        ("ps.nl", "-1.5"),
        ("pl.inf", "inf"),
        ("pl.ninf", "-inf"),
        ("pl.ps", "1"),
        ("pl.pl", "2"),
        ("pl.ns", "-1.5"),
        ("pl.nl", "-3"),
        ("ns.inf", "-inf"),
        ("ns.ninf", "inf"),
        ("ns.ps", "-0.75"),
        ("ns.pl", "-1.5"),
        ("ns.ns", "0.5"),
        ("ns.nl", "1"),
        ("nl.inf", "-inf"),
        ("nl.ninf", "inf"),
        ("nl.ps", "-1.5"),
        ("nl.pl", "-3"),
        ("nl.ns", "1"),
        ("nl.nl", "2"),
    ], false);
}

#[test]
fn test_mul_hi() {
    use fp::Mul;
    test_binary_op(Mpfr::mul_hi, all_signs_and_roundings(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("inf.inf", "inf"),
        ("inf.ninf", "-inf"),
        ("inf.z", "NaN"),
        ("inf.ps", "inf"),
        ("inf.pl", "inf"),
        ("inf.ns", "-inf"),
        ("inf.nl", "-inf"),
        ("ninf.inf", "-inf"),
        ("ninf.ninf", "inf"),
        ("ninf.z", "NaN"),
        ("ninf.ps", "-inf"),
        ("ninf.pl", "-inf"),
        ("ninf.ns", "inf"),
        ("ninf.nl", "inf"),
        ("z.nan", "NaN"),
        ("z.inf", "NaN"),
        ("z.ninf", "NaN"),
        ("z.*", "0"),
        ("*.z", "0"),
        ("ps.inf", "inf"),
        ("ps.ninf", "-inf"),
        ("ps.ps", "0.75"),
        ("ps.pl", "1.5"),
        ("ps.ns", "-0.5"),
        ("ps.nl", "-1"),
        ("pl.inf", "inf"),
        ("pl.ninf", "-inf"),
        ("pl.ps", "1.5"),
        ("pl.pl", "3"),
        ("pl.ns", "-1"),
        ("pl.nl", "-2"),
        ("ns.inf", "-inf"),
        ("ns.ninf", "inf"),
        ("ns.ps", "-0.5"),
        ("ns.pl", "-1"),
        ("ns.ns", "0.75"),
        ("ns.nl", "1.5"),
        ("nl.inf", "-inf"),
        ("nl.ninf", "inf"),
        ("nl.ps", "-1"),
        ("nl.pl", "-2"),
        ("nl.ns", "1.5"),
        ("nl.nl", "3"),
    ], false);
}

#[test]
fn test_div_lo() {
    use fp::Div;
    test_binary_op(Mpfr::div_lo, all_signs_and_roundings(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("inf.inf", "NaN"),
        ("inf.ninf", "NaN"),
        ("inf.z", "inf"),
        ("inf.ps", "inf"),
        ("inf.pl", "inf"),
        ("inf.ns", "-inf"),
        ("inf.nl", "-inf"),
        ("ninf.inf", "NaN"),
        ("ninf.ninf", "NaN"),
        ("ninf.z", "-inf"),
        ("ninf.ps", "-inf"),
        ("ninf.pl", "-inf"),
        ("ninf.ns", "inf"),
        ("ninf.nl", "inf"),
        ("z.nan", "NaN"),
        ("z.z", "NaN"),
        ("z.*", "0"),
        ("*.inf", "0"),
        ("*.ninf", "0"),
        ("ps.nan", "NaN"),
        ("ps.z", "inf"),
        ("ps.ps", "1"),
        ("ps.pl", "0.5"),
        ("ps.ns", "-1"),
        ("ps.nl", "-0.5"),
        ("pl.z", "inf"),
        ("pl.ps", "2"),
        ("pl.pl", "1"),
        ("pl.ns", "-2"),
        ("pl.nl", "-1"),
        ("ns.z", "-inf"),
        ("ns.ps", "-1"),
        ("ns.pl", "-0.5"),
        ("ns.ns", "1"),
        ("ns.nl", "0.5"),
        ("nl.z", "-inf"),
        ("nl.ps", "-2"),
        ("nl.pl", "-1"),
        ("nl.ns", "2"),
        ("nl.nl", "1"),
    ], false);
}

#[test]
fn test_div_hi() {
    use fp::Div;
    test_binary_op(Mpfr::div_hi, all_signs_and_roundings(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("inf.inf", "NaN"),
        ("inf.ninf", "NaN"),
        ("inf.z", "inf"),
        ("inf.ps", "inf"),
        ("inf.pl", "inf"),
        ("inf.ns", "-inf"),
        ("inf.nl", "-inf"),
        ("ninf.inf", "NaN"),
        ("ninf.ninf", "NaN"),
        ("ninf.z", "-inf"),
        ("ninf.ps", "-inf"),
        ("ninf.pl", "-inf"),
        ("ninf.ns", "inf"),
        ("ninf.nl", "inf"),
        ("z.nan", "NaN"),
        ("z.z", "NaN"),
        ("z.*", "0"),
        ("*.inf", "0"),
        ("*.ninf", "0"),
        ("ps.nan", "NaN"),
        ("ps.z", "inf"),
        ("ps.ps", "1"),
        ("ps.pl", "0.5"),
        ("ps.ns", "-1"),
        ("ps.nl", "-0.5"),
        ("pl.z", "inf"),
        ("pl.ps", "2"),
        ("pl.pl", "1"),
        ("pl.ns", "-2"),
        ("pl.nl", "-1"),
        ("ns.z", "-inf"),
        ("ns.ps", "-1"),
        ("ns.pl", "-0.5"),
        ("ns.ns", "1"),
        ("ns.nl", "0.5"),
        ("nl.z", "-inf"),
        ("nl.ps", "-2"),
        ("nl.pl", "-1"),
        ("nl.ns", "2"),
        ("nl.nl", "1"),
    ], false);
}

#[test]
fn test_log_lo() {
    use fp::Transc;
    test_unary_op(Mpfr::log_lo, all_signs_and_roundings(), vec![
        ("nan", "NaN"),
        ("inf", "inf"),
        ("ninf", "NaN"),
        ("z", "-inf"),
        ("ps", "-0.375"),
        ("pl", "0.375"),
        ("ns", "NaN"),
        ("nl", "NaN"),
    ], false);
}

#[test]
fn test_log_hi() {
    use fp::Transc;
    test_unary_op(Mpfr::log_hi, all_signs_and_roundings(), vec![
        ("nan", "NaN"),
        ("inf", "inf"),
        ("ninf", "NaN"),
        ("z", "-inf"),
        ("ps", "-0.25"),
        ("pl", "0.5"),
        ("ns", "NaN"),
        ("nl", "NaN"),
    ], false);
}

#[test]
fn test_exp_lo() {
    use fp::Transc;
    test_unary_op(Mpfr::exp_lo, all_signs_and_roundings(), vec![
        ("nan", "NaN"),
        ("inf", "inf"),
        ("ninf", "0"),
        ("z", "1"),
        ("ps", "2"),
        ("pl", "4"),
        ("ns", "0.375"),
        ("nl", "0.1875"),
    ], false);
}

#[test]
fn test_exp_hi() {
    use fp::Transc;
    test_unary_op(Mpfr::exp_hi, all_signs_and_roundings(), vec![
        ("nan", "NaN"),
        ("inf", "inf"),
        ("ninf", "0"),
        ("z", "1"),
        ("ps", "3"),
        ("pl", "6"),
        ("ns", "0.5"),
        ("nl", "0.25"),
        ("nl", "0.25"),
    ], false);
}

fn all_signs<'a>() -> Vec<(&'a str, Mpfr)> {
    vec![
        ("nan", mpfr!("NaN")),
        ("inf", mpfr!("inf")),
        ("ninf", mpfr!("-inf")),
        ("z", mpfr!("0")),
        ("p", mpfr!("1")),
        ("n", mpfr!("-1")),
    ]
}

fn all_signs_and_roundings<'a>() -> Vec<(&'a str, Mpfr)> {
    vec![
        ("nan", mpfr!("NaN")),
        ("inf", mpfr!("inf")),
        ("ninf", mpfr!("-inf")),
        ("z", mpfr!("0")),
        ("ps", mpfr!("0.75")),
        ("pl", mpfr!("1.5")),
        ("ns", mpfr!("-0.75")),
        ("nl", mpfr!("-1.5")),
    ]
}

fn find_unary_case<'x, 'z>(cases: &'z Vec<(&str, &str)>, cx: &'x str) -> &'z str {
    for &(ref cz, ref z) in cases {
        let is_match = &format!("{}", cx).as_str() == cz || &format!("*").as_str() == cz;
        if is_match {
            return z;
        }
    }
    panic!("unmatched case: {}", cx);
}

fn find_binary_case<'x, 'y, 'z>(cases: &'z Vec<(&str, &str)>, cx: &'x str, cy: &'y str) -> &'z str {
    for &(ref cz, ref z) in cases {
        let cz = String::from(cz.clone());
        let is_match = format!("{}.{}", cx, cy) == cz ||
            format!("{}.*", cx).as_str() == cz ||
            format!("*.{}", cy).as_str() == cz ||
            format!("*.*").as_str() == cz;
        if is_match {
            return z;
        }
    }
    panic!("unmatched case: {}.{}", cx, cy);
}

pub fn test_unary_op<'a, OP, R>(op: OP,
                                cases: Vec<(&'a str, Mpfr)>,
                                expected: Vec<(&str, &str)>,
                                print: bool)
    where OP: Fn(Mpfr) -> R, R: ::std::fmt::Display + Sized
{
    for (cx, x) in cases {
        let z = op(x.clone());
        if print {
            println!("    (\"{}\", \"{}\"),", cx, z);
        } else {
            assert_str_eq!(String::from(find_unary_case(&expected, cx)), z,
                           "{} ({})", cx, x);
        }
    }
}

pub fn test_binary_op<'a, OP, R>(op: OP,
                                 cases: Vec<(&'a str, Mpfr)>,
                                 expected: Vec<(&str, &str)>,
                                 print: bool)
    where OP: Fn(Mpfr, Mpfr) -> R, R: ::std::fmt::Display + Sized
{
    for (cx, x) in cases.clone() {
        for (cy, y) in cases.clone() {
            let z = op(x.clone(), y.clone());
            if print {
                println!("    (\"{}.{}\", \"{}\"),", cx, cy, z);
            } else {
                assert_str_eq!(String::from(find_binary_case(&expected, cx, cy)), z,
                               "{}.{} ({} . {})", cx, cy, x, y);
            }
        }
    }
}
