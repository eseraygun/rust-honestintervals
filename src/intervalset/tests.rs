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
    ($s:expr) => {
        B::from_str_with_prec($s, PREC).unwrap()
    };
}

macro_rules! ivs {
    ($s:expr) => {
        IVS::from_str_with_prec($s, PREC).unwrap()
    };
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
    test_unary_op(
        |ivs| ivs.is_singleton(),
        all_sign_classes(),
        vec![
            ("nan", "false"),
            ("whl", "false"),
            ("m", "false"),
            ("z", "true"),
            ("p0", "false"),
            ("p1", "false"),
            ("n0", "false"),
            ("n1", "false"),
            ("s", "false"),
        ],
    );
    assert!(ivs!("<-1, -1>").is_singleton());
    assert!(!ivs!("{<-1, -1>; 0}").is_singleton());
}

#[test]
fn test_is_zero() {
    test_unary_op(
        |ivs| ivs.is_zero(),
        all_sign_classes(),
        vec![
            ("nan", "false"),
            ("whl", "false"),
            ("m", "false"),
            ("z", "true"),
            ("p0", "false"),
            ("p1", "false"),
            ("n0", "false"),
            ("n1", "false"),
            ("s", "false"),
        ],
    );
    assert!(!ivs!("<-1, -1>").is_zero());
    assert!(!ivs!("{<-1, -1>; 0}").is_zero());
}

#[test]
fn test_is_empty() {
    test_unary_op(
        |ivs| ivs.is_empty(),
        all_sign_classes(),
        vec![
            ("nan", "true"),
            ("whl", "false"),
            ("m", "false"),
            ("z", "false"),
            ("p0", "false"),
            ("p1", "false"),
            ("n0", "false"),
            ("n1", "false"),
            ("s", "false"),
        ],
    );
}

#[test]
fn test_has_zero() {
    test_unary_op(
        |ivs| ivs.has_zero(),
        all_sign_classes(),
        vec![
            ("nan", "false"),
            ("whl", "true"),
            ("m", "true"),
            ("z", "true"),
            ("p0", "true"),
            ("p1", "false"),
            ("n0", "true"),
            ("n1", "false"),
            ("s", "false"),
        ],
    );
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
    assert_str_eq!(
        "<0.9999999999999999, 1>",
        IVS::from_str("0.9999999999999999").unwrap()
    );
    assert_str_eq!(
        "<1.0000000000000009, 1.000000000000001>",
        IVS::from_str("1.000000000000001").unwrap()
    );
    assert_str_eq!(
        "<-1, -0.9999999999999999>",
        IVS::from_str("-0.9999999999999999").unwrap()
    );
    assert_str_eq!(
        "<-1.000000000000001, -1.0000000000000009>",
        IVS::from_str("-1.000000000000001").unwrap()
    );
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
    assert_eq!(ivs!("{}"), ivs!("{}"));
    assert_eq!(ivs!("{0}"), ivs!("{0}"));
    assert_eq!(ivs!("{<0, 1>}"), ivs!("{<0, 1>}"));
    assert_eq!(ivs!("{<0, 1>; 2}"), ivs!("{<0, 1>; 2}"));
    assert_ne!(ivs!("{<0, 1>; 2}"), ivs!("{<0, 1>; 3}"));
}

#[test]
fn test_neg() {
    use std::ops::Neg;
    test_unary_op(
        IVS::neg,
        simple(),
        vec![
            ("nan", "{}"),
            ("whl", "<-inf, inf>"),
            ("m", "<-1, 1>"),
            ("z", "-0"),
            ("p", "<-2, -1>"),
            ("n", "<1, 2>"),
            ("s", "{<-2, -1>; <1, 2>}"),
        ],
    );
}

#[test]
fn test_add() {
    use std::ops::Add;
    test_binary_op(
        IVS::add,
        simple(),
        simple(),
        vec![
            ("nan.*", "{}"),
            ("whl.*", "<-inf, inf>"),
            ("m.m", "<-2, 2>"),
            ("m.z", "<-1, 1>"),
            ("m.p", "<-0, 3>"),
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
        ],
        true,
    );
}

#[test]
fn test_sub() {
    use std::ops::Sub;
    test_binary_op(
        IVS::sub,
        simple(),
        simple(),
        vec![
            ("nan.*", "{}"),
            ("*.nan", "{}"),
            ("whl.*", "<-inf, inf>"),
            ("*.whl", "<-inf, inf>"),
            ("m.m", "<-2, 2>"),
            ("m.z", "<-1, 1>"),
            ("m.p", "<-3, 0>"),
            ("m.n", "<-0, 3>"),
            ("m.s", "<-3, 3>"),
            ("z.m", "<-1, 1>"),
            ("z.z", "0"),
            ("z.p", "<-2, -1>"),
            ("z.n", "<1, 2>"),
            ("z.s", "{<-2, -1>; <1, 2>}"),
            ("p.m", "<-0, 3>"),
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
        ],
        false,
    );
}

#[test]
fn test_mul() {
    use std::ops::Mul;
    test_binary_op(
        IVS::mul,
        all_sign_classes(),
        all_sign_classes(),
        vec![
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
            ("p0.n1", "<-2, -0>"),
            ("p0.s", "<-2, 2>"),
            ("p1.p1", "<1, 4>"),
            ("p1.n0", "<-2, 0>"),
            ("p1.n1", "<-4, -1>"),
            ("p1.s", "{<-4, -1>; <1, 4>}"),
            ("n0.n0", "<0, 1>"),
            ("n0.n1", "<-0, 2>"),
            ("n0.s", "<-2, 2>"),
            ("n1.n1", "<1, 4>"),
            ("n1.s", "{<-4, -1>; <1, 4>}"),
            ("s.s", "{<-4, -1>; <1, 4>}"),
        ],
        true,
    );
}

#[test]
fn test_div() {
    use std::ops::Div;
    test_binary_op(
        IVS::div,
        all_sign_classes(),
        all_sign_classes(),
        vec![
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
            ("p0.n0", "<-inf, -0>"),
            ("p0.n1", "<-1, -0>"),
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
            ("n0.n0", "<-0, inf>"),
            ("n0.n1", "<-0, 1>"),
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
            ("s.s", "{<-2, -0.5>; <0.5, 2>}"),
        ],
        false,
    );
}

#[test]
fn test_log() {
    use transc::Transc;
    test_unary_op(
        IVS::log,
        all_sign_classes_small_and_big(),
        vec![
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
        ],
    );
}

#[test]
fn test_exp() {
    use transc::Transc;
    test_unary_op(
        IVS::exp,
        all_sign_classes_small_and_big(),
        vec![
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
        ],
    );
}

#[test]
fn test_pow() {
    use transc::Transc;
    test_binary_op(
        IVS::pow,
        all_sign_classes_small_and_big(),
        all_sign_classes_with_singletons(),
        vec![
            ("nan.*", "{}"),
            ("*.nan", "{}"),
            ("*.z", "1"),
            ("z.whl", "{0; 1}"),
            ("z.m", "{0; 1}"),
            ("z.p0", "{0; 1}"),
            ("z.n0", "{0; 1}"),
            ("z.m1", "{}"),
            ("z.m2", "{}"),
            ("z.mh", "{}"),
            ("z.*", "0"),
            ("whl.2", "<0, inf>"),
            ("whl.m2", "<0, inf>"),
            ("whl.h", "<0, inf>"),
            ("whl.mh", "<0, inf>"),
            ("whl.*", "<-inf, inf>"),
            ("m.whl", "<-inf, inf>"),
            ("m.m", "<-inf, inf>"),
            ("m.p0", "<-1, 1>"),
            ("m.p1", "<-1, 1>"),
            ("m.n0", "{<-inf, -1>; <1, inf>}"),
            ("m.n1", "{<-inf, -1>; <1, inf>}"),
            ("m.s", "<-inf, inf>"),
            ("m.1", "<-1, 1>"),
            ("m.m1", "{<-inf, -1>; <1, inf>}"),
            ("m.2", "<0, 1>"),
            ("m.m2", "<1, inf>"),
            ("m.h", "<0, 1>"),
            ("m.mh", "<1, inf>"),
            ("p0s.whl", "<0, inf>"),
            ("p0s.m", "<0, inf>"),
            ("p0s.p0", "<0, 1>"),
            ("p0s.p1", "<0, 0.75>"),
            ("p0s.n0", "<1, inf>"),
            ("p0s.n1", "<1, inf>"),
            ("p0s.s", "{<0, 0.75>; <1, inf>}"),
            ("p0s.1", "<0, 0.75>"),
            ("p0s.m1", "<1, inf>"),
            ("p0s.2", "<0, 0.75>"),
            ("p0s.m2", "<1, inf>"),
            ("p0s.h", "<0, 1>"),
            ("p0s.mh", "<1, inf>"),
            ("p0b.whl", "<0, inf>"),
            ("p0b.m", "<0, inf>"),
            ("p0b.p0", "<0, 1.5>"),
            ("p0b.p1", "<0, 3>"),
            ("p0b.n0", "<0.5, inf>"),
            ("p0b.n1", "<0.25, inf>"),
            ("p0b.s", "<0, inf>"),
            ("p0b.1", "<0, 1.5>"),
            ("p0b.m1", "<0.5, inf>"),
            ("p0b.2", "<0, 3>"),
            ("p0b.m2", "<0.25, inf>"),
            ("p0b.h", "<0, 1.5>"),
            ("p0b.mh", "<0.5, inf>"),
            ("p1s.whl", "<0, inf>"),
            ("p1s.m", "<0.5, 2>"),
            ("p1s.p0", "<0.75, 2>"),
            ("p1s.p1", "<0.5, 4>"),
            ("p1s.n0", "<0.5, 1.5>"),
            ("p1s.n1", "<0.25, 2>"),
            ("p1s.s", "<0.25, 4>"),
            ("p1s.1", "<0.75, 2>"),
            ("p1s.m1", "<0.5, 1.5>"),
            ("p1s.2", "<0.5, 4>"),
            ("p1s.m2", "<0.25, 2>"),
            ("p1s.h", "<0.75, 1.5>"),
            ("p1s.mh", "<0.5, 1.5>"),
            ("p1b.whl", "<0, inf>"),
            ("p1b.m", "<0.5, 2>"),
            ("p1b.p0", "<1, 2>"),
            ("p1b.p1", "<1.5, 4>"),
            ("p1b.n0", "<0.5, 1>"),
            ("p1b.n1", "<0.25, 0.75>"),
            ("p1b.s", "{<0.25, 0.75>; <1.5, 4>}"),
            ("p1b.1", "<1.5, 2>"),
            ("p1b.m1", "<0.5, 0.75>"),
            ("p1b.2", "<2, 4>"),
            ("p1b.m2", "<0.25, 0.5>"),
            ("p1b.h", "<1, 1.5>"),
            ("p1b.mh", "<0.5, 1>"),
            ("n0s.whl", "<-inf, inf>"),
            ("n0s.m", "<-inf, inf>"),
            ("n0s.p0", "<-1, 1>"),
            ("n0s.p1", "<-0.75, 0.75>"),
            ("n0s.n0", "{<-inf, -1>; <1, inf>}"),
            ("n0s.n1", "{<-inf, -1>; <1, inf>}"),
            ("n0s.s", "{<-inf, -1>; <-0.75, 0.75>; <1, inf>}"),
            ("n0s.1", "<-0.75, 0>"),
            ("n0s.m1", "<-inf, -1>"),
            ("n0s.2", "<0, 0.75>"),
            ("n0s.m2", "<1, inf>"),
            ("n0s.h", "0"),
            ("n0s.mh", "{}"),
            ("n0b.whl", "<-inf, inf>"),
            ("n0b.m", "<-inf, inf>"),
            ("n0b.p0", "<-1.5, 1.5>"),
            ("n0b.p1", "<-3, 3>"),
            ("n0b.n0", "{<-inf, -0.5>; <0.5, inf>}"),
            ("n0b.n1", "{<-inf, -0.25>; <0.25, inf>}"),
            ("n0b.s", "<-inf, inf>"),
            ("n0b.1", "<-1.5, 0>"),
            ("n0b.m1", "<-inf, -0.5>"),
            ("n0b.2", "<0, 3>"),
            ("n0b.m2", "<0.25, inf>"),
            ("n0b.h", "0"),
            ("n0b.mh", "{}"),
            ("n1s.whl", "<-inf, inf>"),
            ("n1s.m", "{<-2, -0.5>; <0.5, 2>}"),
            ("n1s.p0", "{<-2, -0.75>; <0.75, 2>}"),
            ("n1s.p1", "{<-4, -0.5>; <0.5, 4>}"),
            ("n1s.n0", "{<-1.5, -0.5>; <0.5, 1.5>}"),
            ("n1s.n1", "{<-2, -0.25>; <0.25, 2>}"),
            ("n1s.s", "{<-4, -0.25>; <0.25, 4>}"),
            ("n1s.1", "<-2, -0.75>"),
            ("n1s.m1", "<-1.5, -0.5>"),
            ("n1s.2", "<0.5, 4>"),
            ("n1s.m2", "<0.25, 2>"),
            ("n1s.h", "{}"),
            ("n1s.mh", "{}"),
            ("n1b.whl", "<-inf, inf>"),
            ("n1b.m", "{<-2, -0.5>; <0.5, 2>}"),
            ("n1b.p0", "{<-2, -1>; <1, 2>}"),
            ("n1b.p1", "{<-4, -1.5>; <1.5, 4>}"),
            ("n1b.n0", "{<-1, -0.5>; <0.5, 1>}"),
            ("n1b.n1", "{<-0.75, -0.25>; <0.25, 0.75>}"),
            ("n1b.s", "{<-4, -1.5>; <-0.75, -0.25>; <0.25, 0.75>; <1.5, 4>}"),
            ("n1b.1", "<-2, -1.5>"),
            ("n1b.m1", "<-0.75, -0.5>"),
            ("n1b.2", "<2, 4>"),
            ("n1b.m2", "<0.25, 0.5>"),
            ("n1b.h", "{}"),
            ("n1b.mh", "{}"),
            ("ss.whl", "<-inf, inf>"),
            ("ss.m", "{<-2, -0.5>; <0.5, 2>}"),
            ("ss.p0", "{<-2, -0.75>; <0.75, 2>}"),
            ("ss.p1", "{<-4, -0.5>; <0.5, 4>}"),
            ("ss.n0", "{<-1.5, -0.5>; <0.5, 1.5>}"),
            ("ss.n1", "{<-2, -0.25>; <0.25, 2>}"),
            ("ss.s", "{<-4, -0.25>; <0.25, 4>}"),
            ("ss.1", "{<-2, -0.75>; <0.75, 2>}"),
            ("ss.m1", "{<-1.5, -0.5>; <0.5, 1.5>}"),
            ("ss.2", "<0.5, 4>"),
            ("ss.m2", "<0.25, 2>"),
            ("ss.h", "<0.75, 1.5>"),
            ("ss.mh", "<0.5, 1.5>"),
            ("sb.whl", "<-inf, inf>"),
            ("sb.m", "{<-2, -0.5>; <0.5, 2>}"),
            ("sb.p0", "{<-2, -1>; <1, 2>}"),
            ("sb.p1", "{<-4, -1.5>; <1.5, 4>}"),
            ("sb.n0", "{<-1, -0.5>; <0.5, 1>}"),
            ("sb.n1", "{<-0.75, -0.25>; <0.25, 0.75>}"),
            ("sb.s", "{<-4, -1.5>; <-0.75, -0.25>; <0.25, 0.75>; <1.5, 4>}"),
            ("sb.1", "{<-2, -1.5>; <1.5, 2>}"),
            ("sb.m1", "{<-0.75, -0.5>; <0.5, 0.75>}"),
            ("sb.2", "<2, 4>"),
            ("sb.m2", "<0.25, 0.5>"),
            ("sb.h", "<1, 1.5>"),
            ("sb.mh", "<0.5, 1>"),
        ],
        false,
    );
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

fn all_sign_classes_with_singletons<'a>() -> Vec<(&'a str, IVS)> {
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
        ("1", ivs!("1")),
        ("m1", ivs!("-1")),
        ("2", ivs!("2")),
        ("m2", ivs!("-2")),
        ("h", ivs!("0.5")),
        ("mh", ivs!("-0.5")),
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

fn find_binary_case<'x, 'y, 'z>(
    cases: &'z Vec<(&str, &str)>,
    cx: &'x str,
    cy: &'y str,
    commutative: bool,
) -> Option<&'z str> {
    for &(ref cz, ref z) in cases {
        let cz = String::from(cz.clone());
        let mut is_match = format!("{}.{}", cx, cy) == cz
            || format!("{}.*", cx).as_str() == cz
            || format!("*.{}", cy).as_str() == cz
            || format!("*.*").as_str() == cz;
        if commutative {
            is_match = is_match
                || format!("{}.{}", cy, cx) == cz
                || format!("{}.*", cy).as_str() == cz
                || format!("*.{}", cx).as_str() == cz;
        }
        if is_match {
            return Some(z);
        }
    }
    None
}

pub fn test_unary_op<'a, OP, R>(op: OP, cases: Vec<(&'a str, IVS)>, expected: Vec<(&str, &str)>)
where
    OP: Fn(IVS) -> R,
    R: std::fmt::Display + Sized,
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

pub fn test_binary_op<'a, OP, R>(
    op: OP,
    left_cases: Vec<(&'a str, IVS)>,
    right_cases: Vec<(&'a str, IVS)>,
    mut expected: Vec<(&str, &str)>,
    commutative: bool,
) where
    OP: Fn(IVS, IVS) -> R,
    R: std::fmt::Display + Sized,
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

    let mut failed_cases = Vec::<String>::new();
    let mut unmatched_cases = Vec::<String>::new();
    for (cx, x) in left_cases.clone() {
        for (cy, y) in right_cases.clone() {
            let z = op(x.clone(), y.clone());
            if let Some(cz) = find_binary_case(&expected, cx, cy, commutative) {
                if String::from(cz) != format!("{}", z) {
                    failed_cases.push(format!("    (\"{}.{}\", \"{}\"),", cx, cy, z));
                }
            } else {
                unmatched_cases.push(format!("    (\"{}.{}\", \"{}\"),", cx, cy, z));
            }
        }
    }
    if !failed_cases.is_empty() {
        panic!("failed case(s):\n{}", failed_cases.join("\n"));
    }
    if !unmatched_cases.is_empty() {
        panic!("unmatched case(s):\n{}", unmatched_cases.join("\n"));
    }
}
