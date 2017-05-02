use super::IntervalSet;

use interval::Interval;
use mpfr::Mpfr;

type B = Mpfr;
type IV = Interval<B>;
type IVS = IntervalSet<B>;

const PREC: usize = 2;

macro_rules! assert_str_eq {
    ($x:expr, $y:expr) => { assert_eq!($x, format!("{}", $y)) };
    ($x:expr, $y:expr, $($arg:tt)+) => { assert_eq!($x, format!("{}", $y), $($arg)+) };
}

macro_rules! b {
    ($s:expr) => { B::from_str_with_prec($s, PREC).unwrap() }
}

macro_rules! iv {
    ($s:expr) => { IV::from_str_with_prec($s, PREC).unwrap() }
}

macro_rules! ivs {
    ($s:expr) => { IVS::from_str_with_prec($s, PREC).unwrap() }
}

#[test]
fn test_new() {
    assert_str_eq!("<0, 1>", IVS::new(b!("0"), b!("1")));
    assert_str_eq!("<-1, 0>", IVS::new(b!("-1"), b!("0")));
    assert_str_eq!("-1", IVS::new(b!("-1"), b!("-1")));
    assert_str_eq!("{}", IVS::new(b!("NaN"), b!("NaN")));
    assert_str_eq!("<-inf, inf>", IVS::new(b!("-inf"), b!("inf")));
}

#[test]
fn test_singleton() {
    assert_str_eq!("-1", IVS::singleton(b!("-1")));
    assert_str_eq!("{}", IVS::singleton(b!("NaN")));
}

#[test]
fn test_constants() {
    assert_str_eq!("0", IVS::zero(PREC));
    assert_str_eq!("1", IVS::one(PREC));
    assert_str_eq!("{}", IVS::empty());
    assert_str_eq!("<-inf, inf>", IVS::whole(PREC));
}

#[test]
fn test_is_singleton() {
    assert!(!ivs!("{}").is_singleton());
    assert!(!ivs!("<0, 1>").is_singleton());
    assert!(ivs!("<1, 1>").is_singleton());
    assert!(!ivs!("{0; <1, 2>}").is_singleton());
}

#[test]
fn test_from_f64() {
    use std::f64;
    assert_str_eq!("-123.456", IVS::from(-123.456));
    assert_str_eq!("{}", IVS::from(f64::NAN));
}

#[test]
fn test_from_str() {
    use std::str::FromStr;
    assert_str_eq!("{<-123, 123>; 456}", IVS::from_str("{<-123, 123>; 456}").unwrap());
    assert_str_eq!("<-123, 456>", IVS::from_str("<-123, 456>").unwrap());
    assert_str_eq!("<-123.456, -123.45599999999999>", IVS::from_str("-123.456").unwrap());
    assert_str_eq!("<-inf, inf>", IVS::from_str("<-inf, inf>").unwrap());
    assert_str_eq!("{}", IVS::from_str("NaN").unwrap());
    assert_str_eq!("{}", IVS::from_str("<NaN, NaN>").unwrap());
    assert!(IVS::from_str("-123.45x").is_err());
    assert!(IVS::from_str("<1, 0>").is_err());
    assert!(IVS::from_str("<NaN, 0>").is_err());
    assert!(IVS::from_str("<0, NaN>").is_err());
}

#[test]
fn test_clone() {
    let x = ivs!("{<-123, 123>; 456}");
    assert_eq!(x, x.clone());
    let mut y = ivs!("{<-456, 789>}");
    y.clone_from(&x);
    assert_eq!(x, y);
}
