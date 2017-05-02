use super::IntervalSet;

use mpfr::Mpfr;

type B = Mpfr;
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
    test_unary_op(|ivs| ivs.is_singleton(), all_sign_classes(), vec![
        ("nan", "false"),
        ("whl", "false"),
        ("m", "false"),
        ("z", "true"),
        ("p0", "false"),
        ("p1", "false"),
        ("n0", "false"),
        ("n1", "false"),
        ("s", "false"),
    ]);
    assert!(ivs!("<-1, -1>").is_singleton());
    assert!(!ivs!("{<-1, -1>; 0}").is_singleton());
}

#[test]
fn test_is_zero() {
    test_unary_op(|ivs| ivs.is_zero(), all_sign_classes(), vec![
        ("nan", "false"),
        ("whl", "false"),
        ("m", "false"),
        ("z", "true"),
        ("p0", "false"),
        ("p1", "false"),
        ("n0", "false"),
        ("n1", "false"),
        ("s", "false"),
    ]);
    assert!(!ivs!("<-1, -1>").is_zero());
    assert!(!ivs!("{<-1, -1>; 0}").is_zero());
}

#[test]
fn test_is_empty() {
    test_unary_op(|ivs| ivs.is_empty(), all_sign_classes(), vec![
        ("nan", "true"),
        ("whl", "false"),
        ("m", "false"),
        ("z", "false"),
        ("p0", "false"),
        ("p1", "false"),
        ("n0", "false"),
        ("n1", "false"),
        ("s", "false"),
    ]);
}

#[test]
fn test_has_zero() {
    test_unary_op(|ivs| ivs.has_zero(), all_sign_classes(), vec![
        ("nan", "false"),
        ("whl", "true"),
        ("m", "true"),
        ("z", "true"),
        ("p0", "true"),
        ("p1", "false"),
        ("n0", "true"),
        ("n1", "false"),
        ("s", "false"),
    ]);
    assert!(ivs!("{<-1, -1>; 0}").has_zero());
}

#[test]
fn test_from_f64() {
    use std::f64;
    assert_str_eq!("0", IVS::from(0f64));
    assert_str_eq!("0.9999999999999999", IVS::from(0.9999999999999999));
    assert_str_eq!("1.000000000000001", IVS::from(1.000000000000001));
    assert_str_eq!("-0.9999999999999999", IVS::from(-0.9999999999999999));
    assert_str_eq!("-1.000000000000001", IVS::from(-1.000000000000001));
    assert_str_eq!("{}", IVS::from(f64::NAN));
}

#[test]
fn test_from_str() {
    use std::str::FromStr;
    assert_str_eq!("0", IVS::from_str("0").unwrap());
    assert_str_eq!("<0.9999999999999999, 1>", IVS::from_str("0.9999999999999999").unwrap());
    assert_str_eq!("<1.0000000000000009, 1.000000000000001>", IVS::from_str("1.000000000000001").unwrap());
    assert_str_eq!("<-1, -0.9999999999999999>", IVS::from_str("-0.9999999999999999").unwrap());
    assert_str_eq!("<-1.000000000000001, -1.0000000000000009>", IVS::from_str("-1.000000000000001").unwrap());
    assert_str_eq!("{}", IVS::from_str("NaN").unwrap());
}

#[test]
fn test_clone() {
    let x = ivs!("{0; <1, 2>}");
    assert_eq!(x, x.clone());
    let mut y = ivs!("{<0, 1>; <2, 4>}");
    y.clone_from(&x);
    assert_eq!(x, y);
}

#[test]
fn test_into_vec() {
    let intervals: Vec<_> = ivs!("{0; <1, 2>}").into();
    assert_eq!(2, intervals.len());
    assert_str_eq!("0", intervals[0].0);
    assert_str_eq!("0", intervals[0].1);
    assert_str_eq!("1", intervals[1].0);
    assert_str_eq!("2", intervals[1].1);
}

#[test]
fn test_partial_eq() {
    assert!(ivs!("{}") == ivs!("{}"));
    assert!(ivs!("{0}") == ivs!("{0}"));
    assert!(ivs!("{<0, 1>}") == ivs!("{<0, 1>}"));
    assert!(ivs!("{<0, 1>; 2}") == ivs!("{<0, 1>; 2}"));
    assert!(ivs!("{<0, 1>; 2}") != ivs!("{<0, 1>; 3}"));
}

#[test]
fn test_neg() {
    use std::ops::Neg;
    test_unary_op(IVS::neg, simple(), vec![
        ("nan", "{}"),
        ("whl", "<-inf, inf>"),
        ("m", "<-1, 1>"),
        ("z", "0"),
        ("p", "<-2, -1>"),
        ("n", "<1, 2>"),
        ("s", "{<-2, -1>; <1, 2>}"),
    ]);
}

#[test]
fn test_add() {
    use std::ops::Add;
    test_binary_op(IVS::add, simple(), vec![
        ("nan.*", "{}"),
        ("whl.*", "<-inf, inf>"),
        ("m.m", "<-2, 2>"),
        ("m.z", "<-1, 1>"),
        ("m.p", "<0, 3>"),
        ("m.n", "<-3, 0>"),
        ("m.s", "<-3, 3>"),
        ("z.z", "0"),
        ("z.p", "<1, 2>"),
        ("z.n", "<-2, -1>"),
        ("z.s", "{<-2, -1>; <1, 2>}"),
        ("p.p", "<2, 4>"),
        ("p.n", "<-1, 1>"),
        ("p.s", "{<-1, 1>; <2, 4>}"),
        ("n.n", "<-4, -2>"),
        ("n.s", "{<-4, -2>; <-1, 1>}"),
        ("s.s", "{<-4, -2>; <-1, 1>; <2, 4>}"),
    ], true);
}

#[test]
fn test_sub() {
    use std::ops::Sub;
    test_binary_op(IVS::sub, simple(), vec![
        ("nan.*", "{}"),
        ("*.nan", "{}"),
        ("whl.*", "<-inf, inf>"),
        ("*.whl", "<-inf, inf>"),
        ("m.m", "<-2, 2>"),
        ("m.z", "<-1, 1>"),
        ("m.p", "<-3, 0>"),
        ("m.n", "<0, 3>"),
        ("m.s", "<-3, 3>"),
        ("z.m", "<-1, 1>"),
        ("z.z", "0"),
        ("z.p", "<-2, -1>"),
        ("z.n", "<1, 2>"),
        ("z.s", "{<-2, -1>; <1, 2>}"),
        ("p.m", "<0, 3>"),
        ("p.z", "<1, 2>"),
        ("p.p", "<-1, 1>"),
        ("p.n", "<2, 4>"),
        ("p.s", "{<-1, 1>; <2, 4>}"),
        ("n.m", "<-3, 0>"),
        ("n.z", "<-2, -1>"),
        ("n.p", "<-4, -2>"),
        ("n.n", "<-1, 1>"),
        ("n.s", "{<-4, -2>; <-1, 1>}"),
        ("s.m", "<-3, 3>"),
        ("s.z", "{<-2, -1>; <1, 2>}"),
        ("s.p", "{<-4, -2>; <-1, 1>}"),
        ("s.n", "{<-1, 1>; <2, 4>}"),
        ("s.s", "{<-4, -2>; <-1, 1>; <2, 4>}"),
    ], false);
}

#[test]
fn test_mul() {
    use std::ops::Mul;
    test_binary_op(IVS::mul, all_sign_classes(), vec![
        ("nan.*", "{}"),
        ("z.*", "0"),
        ("whl.*", "<-inf, inf>"),
        ("m.m", "<-1, 1>"),
        ("m.p0", "<-1, 1>"),
        ("m.p1", "<-2, 2>"),
        ("m.n0", "<-1, 1>"),
        ("m.n1", "<-2, 2>"),
        ("m.s", "<-2, 2>"),
        ("p0.p0", "<0, 1>"),
        ("p0.p1", "<0, 2>"),
        ("p0.n0", "<-1, 0>"),
        ("p0.n1", "<-2, 0>"),
        ("p0.s", "<-2, 2>"),
        ("p1.p1", "<1, 4>"),
        ("p1.n0", "<-2, 0>"),
        ("p1.n1", "<-4, -1>"),
        ("p1.s", "{<-4, -1>; <1, 4>}"),
        ("n0.n0", "<0, 1>"),
        ("n0.n1", "<0, 2>"),
        ("n0.s", "<-2, 2>"),
        ("n1.n1", "<1, 4>"),
        ("n1.s", "{<-4, -1>; <1, 4>}"),
        ("s.s", "{<-4, -1>; <1, 4>}"),
    ], true);
}

#[test]
fn test_div() {
    use std::ops::Div;
    test_binary_op(IVS::div, all_sign_classes(), vec![
        ("nan.*", "{}"),
        ("*.nan", "{}"),
        ("*.z", "{}"),
        ("z.*", "0"),
        ("whl.*", "<-inf, inf>"),
        ("*.whl", "<-inf, inf>"),
        ("m.m", "<-inf, inf>"),
        ("m.p0", "<-inf, inf>"),
        ("m.p1", "<-1, 1>"),
        ("m.n0", "<-inf, inf>"),
        ("m.n1", "<-1, 1>"),
        ("m.s", "<-1, 1>"),
        ("p0.m", "<-inf, inf>"),
        ("p0.p0", "<0, inf>"),
        ("p0.p1", "<0, 1>"),
        ("p0.n0", "<-inf, 0>"),
        ("p0.n1", "<-1, 0>"),
        ("p0.s", "<-1, 1>"),
        ("p1.m", "{<-inf, -1>; <1, inf>}"),
        ("p1.p0", "<1, inf>"),
        ("p1.p1", "<0.5, 2>"),
        ("p1.n0", "<-inf, -1>"),
        ("p1.n1", "<-2, -0.5>"),
        ("p1.s", "{<-2, -0.5>; <0.5, 2>}"),
        ("n0.m", "<-inf, inf>"),
        ("n0.p0", "<-inf, 0>"),
        ("n0.p1", "<-1, 0>"),
        ("n0.n0", "<0, inf>"),
        ("n0.n1", "<0, 1>"),
        ("n0.s", "<-1, 1>"),
        ("n1.m", "{<-inf, -1>; <1, inf>}"),
        ("n1.p0", "<-inf, -1>"),
        ("n1.p1", "<-2, -0.5>"),
        ("n1.n0", "<1, inf>"),
        ("n1.n1", "<0.5, 2>"),
        ("n1.s", "{<-2, -0.5>; <0.5, 2>}"),
        ("s.m", "{<-inf, -1>; <1, inf>}"),
        ("s.p0", "{<-inf, -1>; <1, inf>}"),
        ("s.p1", "{<-2, -0.5>; <0.5, 2>}"),
        ("s.n0", "{<-inf, -1>; <1, inf>}"),
        ("s.n1", "{<-2, -0.5>; <0.5, 2>}"),
        ("s.s", "{<-2, -0.5>; <0.5, 2>}")
    ], false);
}

#[test]
fn test_log() {
    test_unary_op(IVS::log, all_sign_classes_small_and_big(), vec![
        ("nan", "{}"),
        ("whl", "<-inf, inf>"),
        ("m", "<-inf, 0>"),
        ("z", "{}"),
        ("p0s", "<-inf, -0.25>"),
        ("p0b", "<-inf, 0.5>"),
        ("p1s", "<-0.375, 0.75>"),
        ("p1b", "<0.375, 0.75>"),
        ("n0s", "{}"),
        ("n0b", "{}"),
        ("n1s", "{}"),
        ("n1b", "{}"),
        ("ss", "<-0.375, 0.75>"),
        ("sb", "<0.375, 0.75>"),
    ]);
}

#[test]
fn test_exp() {
    test_unary_op(IVS::exp, all_sign_classes_small_and_big(), vec![
        ("nan", "{}"),
        ("whl", "<0, inf>"),
        ("m", "<0.25, 3>"),
        ("z", "1"),
        ("p0s", "<1, 3>"),
        ("p0b", "<1, 6>"),
        ("p1s", "<2, 8>"),
        ("p1b", "<4, 8>"),
        ("n0s", "<0.375, 1>"),
        ("n0b", "<0.1875, 1>"),
        ("n1s", "<0.125, 0.5>"),
        ("n1b", "<0.125, 0.25>"),
        ("ss", "{<0.125, 0.5>; <2, 8>}"),
        ("sb", "{<0.125, 0.25>; <4, 8>}"),
    ]);
}

#[test]
fn test_pow() {
    test_binary_op(IVS::pow, all_sign_classes_small_and_big(), vec![
        ("nan.*", "{}"),
        ("*.nan", "{}"),
        ("*.z", "1"),
        ("z.*", "0"),
        ("whl.*", "<-inf, inf>"),
        ("m.whl", "<-inf, inf>"),
        ("m.m", "<-inf, inf>"),
        ("m.p0s", "<-1, 1>"),
        ("m.p0b", "<-1, 1>"),
        ("m.p1s", "<-1, 1>"),
        ("m.p1b", "<-1, 1>"),
        ("m.n0s", "{<-inf, -1>; <1, inf>}"),
        ("m.n0b", "{<-inf, -1>; <1, inf>}"),
        ("m.n1s", "{<-inf, -1>; <1, inf>}"),
        ("m.n1b", "{<-inf, -1>; <1, inf>}"),
        ("m.ss", "<-inf, inf>"),
        ("m.sb", "<-inf, inf>"),
        ("p0s.whl", "<0, inf>"),
        ("p0s.m", "<0, inf>"),
        ("p0s.p0s", "<0, 1>"),
        ("p0s.p0b", "<0, 1>"),
        ("p0s.p1s", "<0, 1>"),
        ("p0s.p1b", "<0, 0.75>"),
        ("p0s.n0s", "<1, inf>"),
        ("p0s.n0b", "<1, inf>"),
        ("p0s.n1s", "<1, inf>"),
        ("p0s.n1b", "<1, inf>"),
        ("p0s.ss", "<0, inf>"),
        ("p0s.sb", "{<0, 0.75>; <1, inf>}"),
        ("p0b.whl", "<0, inf>"),
        ("p0b.m", "<0, inf>"),
        ("p0b.p0s", "<0, 1.5>"),
        ("p0b.p0b", "<0, 3>"),
        ("p0b.p1s", "<0, 3>"),
        ("p0b.p1b", "<0, 3>"),
        ("p0b.n0s", "<0.5, inf>"),
        ("p0b.n0b", "<0.25, inf>"),
        ("p0b.n1s", "<0.25, inf>"),
        ("p0b.n1b", "<0.25, inf>"),
        ("p0b.ss", "<0, inf>"),
        ("p0b.sb", "<0, inf>"),
        ("p1s.whl", "<0, inf>"),
        ("p1s.m", "<0.25, 3>"),
        ("p1s.p0s", "<0.5, 3>"),
        ("p1s.p0b", "<0.375, 6>"),
        ("p1s.p1s", "<0.375, 6>"),
        ("p1s.p1b", "<0.375, 6>"),
        ("p1s.n0s", "<0.25, 2>"),
        ("p1s.n0b", "<0.125, 3>"),
        ("p1s.n1s", "<0.125, 3>"),
        ("p1s.n1b", "<0.125, 3>"),
        ("p1s.ss", "<0.125, 6>"),
        ("p1s.sb", "<0.125, 6>"),
        ("p1b.whl", "<0, inf>"),
        ("p1b.m", "<0.25, 3>"),
        ("p1b.p0s", "<1, 3>"),
        ("p1b.p0b", "<1, 6>"),
        ("p1b.p1s", "<1, 6>"),
        ("p1b.p1b", "<1.5, 6>"),
        ("p1b.n0s", "<0.25, 1>"),
        ("p1b.n0b", "<0.125, 1>"),
        ("p1b.n1s", "<0.125, 1>"),
        ("p1b.n1b", "<0.125, 0.75>"),
        ("p1b.ss", "<0.125, 6>"),
        ("p1b.sb", "{<0.125, 0.75>; <1.5, 6>}"),
        ("n0s.whl", "<-inf, inf>"),
        ("n0s.m", "<-inf, inf>"),
        ("n0s.p0s", "<-1, 1>"),
        ("n0s.p0b", "<-1, 1>"),
        ("n0s.p1s", "<-1, 1>"),
        ("n0s.p1b", "<-0.75, 0.75>"),
        ("n0s.n0s", "{<-inf, -1>; <1, inf>}"),
        ("n0s.n0b", "{<-inf, -1>; <1, inf>}"),
        ("n0s.n1s", "{<-inf, -1>; <1, inf>}"),
        ("n0s.n1b", "{<-inf, -1>; <1, inf>}"),
        ("n0s.ss", "<-inf, inf>"),
        ("n0s.sb", "{<-inf, -1>; <-0.75, 0.75>; <1, inf>}"),
        ("n0b.whl", "<-inf, inf>"),
        ("n0b.m", "<-inf, inf>"),
        ("n0b.p0s", "<-1.5, 1.5>"),
        ("n0b.p0b", "<-3, 3>"),
        ("n0b.p1s", "<-3, 3>"),
        ("n0b.p1b", "<-3, 3>"),
        ("n0b.n0s", "{<-inf, -0.5>; <0.5, inf>}"),
        ("n0b.n0b", "{<-inf, -0.25>; <0.25, inf>}"),
        ("n0b.n1s", "{<-inf, -0.25>; <0.25, inf>}"),
        ("n0b.n1b", "{<-inf, -0.25>; <0.25, inf>}"),
        ("n0b.ss", "<-inf, inf>"),
        ("n0b.sb", "<-inf, inf>"),
        ("n1s.whl", "<-inf, inf>"),
        ("n1s.m", "{<-3, -0.25>; <0.25, 3>}"),
        ("n1s.p0s", "{<-3, -0.5>; <0.5, 3>}"),
        ("n1s.p0b", "{<-6, -0.375>; <0.375, 6>}"),
        ("n1s.p1s", "{<-6, -0.375>; <0.375, 6>}"),
        ("n1s.p1b", "{<-6, -0.375>; <0.375, 6>}"),
        ("n1s.n0s", "{<-2, -0.25>; <0.25, 2>}"),
        ("n1s.n0b", "{<-3, -0.125>; <0.125, 3>}"),
        ("n1s.n1s", "{<-3, -0.125>; <0.125, 3>}"),
        ("n1s.n1b", "{<-3, -0.125>; <0.125, 3>}"),
        ("n1s.ss", "{<-6, -0.125>; <0.125, 6>}"),
        ("n1s.sb", "{<-6, -0.125>; <0.125, 6>}"),
        ("n1b.whl", "<-inf, inf>"),
        ("n1b.m", "{<-3, -0.25>; <0.25, 3>}"),
        ("n1b.p0s", "{<-3, -1>; <1, 3>}"),
        ("n1b.p0b", "{<-6, -1>; <1, 6>}"),
        ("n1b.p1s", "{<-6, -1>; <1, 6>}"),
        ("n1b.p1b", "{<-6, -1.5>; <1.5, 6>}"),
        ("n1b.n0s", "{<-1, -0.25>; <0.25, 1>}"),
        ("n1b.n0b", "{<-1, -0.125>; <0.125, 1>}"),
        ("n1b.n1s", "{<-1, -0.125>; <0.125, 1>}"),
        ("n1b.n1b", "{<-0.75, -0.125>; <0.125, 0.75>}"),
        ("n1b.ss", "{<-6, -0.125>; <0.125, 6>}"),
        ("n1b.sb", "{<-6, -1.5>; <-0.75, -0.125>; <0.125, 0.75>; <1.5, 6>}"),
        ("ss.whl", "<-inf, inf>"),
        ("ss.m", "{<-3, -0.25>; <0.25, 3>}"),
        ("ss.p0s", "{<-3, -0.5>; <0.5, 3>}"),
        ("ss.p0b", "{<-6, -0.375>; <0.375, 6>}"),
        ("ss.p1s", "{<-6, -0.375>; <0.375, 6>}"),
        ("ss.p1b", "{<-6, -0.375>; <0.375, 6>}"),
        ("ss.n0s", "{<-2, -0.25>; <0.25, 2>}"),
        ("ss.n0b", "{<-3, -0.125>; <0.125, 3>}"),
        ("ss.n1s", "{<-3, -0.125>; <0.125, 3>}"),
        ("ss.n1b", "{<-3, -0.125>; <0.125, 3>}"),
        ("ss.ss", "{<-6, -0.125>; <0.125, 6>}"),
        ("ss.sb", "{<-6, -0.125>; <0.125, 6>}"),
        ("sb.whl", "<-inf, inf>"),
        ("sb.m", "{<-3, -0.25>; <0.25, 3>}"),
        ("sb.p0s", "{<-3, -1>; <1, 3>}"),
        ("sb.p0b", "{<-6, -1>; <1, 6>}"),
        ("sb.p1s", "{<-6, -1>; <1, 6>}"),
        ("sb.p1b", "{<-6, -1.5>; <1.5, 6>}"),
        ("sb.n0s", "{<-1, -0.25>; <0.25, 1>}"),
        ("sb.n0b", "{<-1, -0.125>; <0.125, 1>}"),
        ("sb.n1s", "{<-1, -0.125>; <0.125, 1>}"),
        ("sb.n1b", "{<-0.75, -0.125>; <0.125, 0.75>}"),
        ("sb.ss", "{<-6, -0.125>; <0.125, 6>}"),
        ("sb.sb", "{<-6, -1.5>; <-0.75, -0.125>; <0.125, 0.75>; <1.5, 6>}"),
    ], false);
}

fn simple<'a>() -> Vec<(&'a str, IVS)> {
    vec![
        ("nan", ivs!("{}")),
        ("whl", ivs!("<-inf, inf>")),
        ("m", ivs!("<-1, 1>")),
        ("z", ivs!("0")),
        ("p", ivs!("<1, 2>")),
        ("n", ivs!("<-2, -1>")),
        ("s", ivs!("{<-2, -1>; <1, 2>}")),
    ]
}

fn all_sign_classes<'a>() -> Vec<(&'a str, IVS)> {
    vec![
        ("nan", ivs!("{}")),
        ("whl", ivs!("<-inf, inf>")),
        ("m", ivs!("<-1, 1>")),
        ("z", ivs!("0")),
        ("p0", ivs!("<0, 1>")),
        ("p1", ivs!("<1, 2>")),
        ("n0", ivs!("<-1, 0>")),
        ("n1", ivs!("<-2, -1>")),
        ("s", ivs!("{<-2, -1>; <1, 2>}")),
    ]
}

fn all_sign_classes_small_and_big<'a>() -> Vec<(&'a str, IVS)> {
    vec![
        ("nan", ivs!("{}")),
        ("whl", ivs!("<-inf, inf>")),
        ("m", ivs!("<-1, 1>")),
        ("z", ivs!("0")),
        ("p0s", ivs!("<0, 0.75>")),
        ("p0b", ivs!("<0, 1.5>")),
        ("p1s", ivs!("<0.75, 2>")),
        ("p1b", ivs!("<1.5, 2>")),
        ("n0s", ivs!("<-0.75, 0>")),
        ("n0b", ivs!("<-1.5, 0>")),
        ("n1s", ivs!("<-2, -0.75>")),
        ("n1b", ivs!("<-2, -1.5>")),
        ("ss", ivs!("{<-2, -0.75>; <0.75, 2>}")),
        ("sb", ivs!("{<-2, -1.5>; <1.5, 2>}")),
    ]
}

fn find_unary_case<'x, 'z>(cases: &'z Vec<(&str, &str)>, cx: &'x str) -> Option<&'z str> {
    for &(ref cz, ref z) in cases {
        let is_match = &format!("{}", cx).as_str() == cz || &format!("*").as_str() == cz;
        if is_match {
            return Some(z);
        }
    }
    None
}

fn find_binary_case<'x, 'y, 'z>(cases: &'z Vec<(&str, &str)>,
                                cx: &'x str, cy: &'y str,
                                commutative: bool)
                                -> Option<&'z str> {
    for &(ref cz, ref z) in cases {
        let cz = String::from(cz.clone());
        let mut is_match = format!("{}.{}", cx, cy) == cz ||
            format!("{}.*", cx).as_str() == cz ||
            format!("*.{}", cy).as_str() == cz ||
            format!("*.*").as_str() == cz;
        if commutative {
            is_match = is_match || format!("{}.{}", cy, cx) == cz ||
                format!("{}.*", cy).as_str() == cz ||
                format!("*.{}", cx).as_str() == cz;
        }
        if is_match {
            return Some(z)
        }
    }
    None
}

pub fn test_unary_op<'a, OP, R>(op: OP,
                                cases: Vec<(&'a str, IVS)>,
                                expected: Vec<(&str, &str)>)
    where OP: Fn(IVS) -> R, R: ::std::fmt::Display + Sized
{
    let mut unmatched_cases = Vec::<String>::new();
    for (cx, x) in cases {
        let z = op(x.clone());
        if let Some(cz) = find_unary_case(&expected, cx) {
            assert_str_eq!(String::from(cz), z, "{} ({})", cx, x);
        } else {
            unmatched_cases.push(format!("    (\"{}\", \"{}\"),", cx, z));
        }
    }
    if !unmatched_cases.is_empty() {
        panic!("unmatched case(s):\n{}", unmatched_cases.join("\n"));
    }
}

pub fn test_binary_op<'a, OP, R>(op: OP,
                                 cases: Vec<(&'a str, IVS)>,
                                 mut expected: Vec<(&str, &str)>,
                                 commutative: bool)
    where OP: Fn(IVS, IVS) -> R, R: ::std::fmt::Display + Sized
{
    let mut expected_up_to_now = Vec::<(&str, &str)>::new();
    for (cz, z) in expected.drain(..) {
        let parts = cz.split(".").collect::<Vec<_>>();
        let (cx, cy) = (parts[0], parts[1]);
        if find_binary_case(&expected_up_to_now, cx, cy, commutative).is_some() {
            panic!("redundant case: {}.{}", cx, cy);
        }
        expected_up_to_now.push((cz, z));
    }
    expected = expected_up_to_now;

    let mut unmatched_cases = Vec::<String>::new();
    for (cx, x) in cases.clone() {
        for (cy, y) in cases.clone() {
            let z = op(x.clone(), y.clone());
            if let Some(cz) = find_binary_case(&expected, cx, cy, commutative) {
                assert_str_eq!(String::from(cz), z, "{}.{} ({} . {})", cx, cy, x, y);
            } else {
                unmatched_cases.push(format!("    (\"{}.{}\", \"{}\"),", cx, cy, z));
            }
        }
    }
    if !unmatched_cases.is_empty() {
        panic!("unmatched case(s):\n{}", unmatched_cases.join("\n"));
    }
}
