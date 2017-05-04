use super::Interval;

type B = f64;
type IV = Interval<B>;

const PREC: usize = 2;

macro_rules! assert_str_eq {
    ($x:expr, $y:expr) => { assert_eq!($x, format!("{}", $y)) };
    ($x:expr, $y:expr, $($arg:tt)+) => { assert_eq!($x, format!("{}", $y), $($arg)+) };
}

macro_rules! b {
    ($s:expr) => { <B as ::std::str::FromStr>::from_str($s).unwrap() }
}

macro_rules! iv {
    ($s:expr) => { IV::from_str_with_prec($s, PREC).unwrap() }
}

#[test]
fn test_new() {
    assert_str_eq!("<0, 1>", IV::new(b!("0"), b!("1")));
    assert_str_eq!("<-1, 0>", IV::new(b!("-1"), b!("0")));
    assert_str_eq!("-1", IV::new(b!("-1"), b!("-1")));
    assert_str_eq!("NaN", IV::new(b!("NaN"), b!("NaN")));
    assert_str_eq!("<-inf, inf>", IV::new(b!("-inf"), b!("inf")));
}

#[test]
#[should_panic]
fn test_new_lo_greater_than_hi() {
    IV::new(b!("1"), b!("0"));
}

#[test]
#[should_panic]
fn test_new_exactly_one_nan_1() {
    IV::new(b!("0"), b!("NaN"));
}

#[test]
#[should_panic]
fn test_new_exactly_one_nan_2() {
    IV::new(b!("NaN"), b!("0"));
}

#[test]
#[should_panic]
fn test_new_inf() {
    IV::new(b!("inf"), b!("inf"));
}

#[test]
#[should_panic]
fn test_new_neg_inf() {
    IV::new(b!("-inf"), b!("-inf"));
}

#[test]
fn test_singleton() {
    assert_str_eq!("-1", IV::singleton(b!("-1")));
    assert_str_eq!("NaN", IV::singleton(b!("NaN")));
}

#[test]
fn test_constants() {
    assert_str_eq!("0", IV::zero(PREC));
    assert_str_eq!("1", IV::one(PREC));
    assert_str_eq!("NaN", IV::nan(PREC));
    assert_str_eq!("<-inf, inf>", IV::whole(PREC));
}

#[test]
fn test_sign_class() {
    test_unary_op(|iv| iv.sign_class(), all_sign_classes(), vec![
        ("nan", "z"),
        ("whl", "m"),
        ("m", "m"),
        ("z", "z"),
        ("p0", "p0"),
        ("p1", "p1"),
        ("n0", "n0"),
        ("n1", "n1"),
    ], false);
}

#[test]
fn test_precision() {
    use fp::Float;
    assert_eq!(53usize, IV::new(B::zero(53), B::zero(53)).precision());
}

#[test]
fn test_size() {
    test_unary_op(|iv| iv.size(), simple(), vec![
        ("nan", "NaN"),
        ("whl", "NaN"),
        ("m", "2"),
        ("z", "0"),
        ("p", "1"),
        ("n", "1"),
    ], false);
    assert_str_eq!("2", iv!("<-1.1, 0.9>").size());
}

#[test]
fn test_is_singleton() {
    test_unary_op(|iv| iv.is_singleton(), all_sign_classes(), vec![
        ("nan", "false"),
        ("whl", "false"),
        ("m", "false"),
        ("z", "true"),
        ("p0", "false"),
        ("p1", "false"),
        ("n0", "false"),
        ("n1", "false"),
    ], false);
    assert!(iv!("<-1, -1>").is_singleton());
}

#[test]
fn test_is_zero() {
    test_unary_op(|iv| iv.is_zero(), all_sign_classes(), vec![
        ("nan", "false"),
        ("whl", "false"),
        ("m", "false"),
        ("z", "true"),
        ("p0", "false"),
        ("p1", "false"),
        ("n0", "false"),
        ("n1", "false"),
    ], false);
    assert!(!iv!("<-1, -1>").is_zero());
}

#[test]
fn test_is_nan() {
    test_unary_op(|iv| iv.is_nan(), all_sign_classes(), vec![
        ("nan", "true"),
        ("whl", "false"),
        ("m", "false"),
        ("z", "false"),
        ("p0", "false"),
        ("p1", "false"),
        ("n0", "false"),
        ("n1", "false"),
    ], false);
}

#[test]
fn test_is_whole() {
    test_unary_op(|iv| iv.is_whole(), all_sign_classes(), vec![
        ("nan", "false"),
        ("whl", "true"),
        ("m", "false"),
        ("z", "false"),
        ("p0", "false"),
        ("p1", "false"),
        ("n0", "false"),
        ("n1", "false"),
    ], false);
}

#[test]
fn test_has_zero() {
    test_unary_op(|iv| iv.has_zero(), all_sign_classes(), vec![
        ("nan", "true"),
        ("whl", "true"),
        ("m", "true"),
        ("z", "true"),
        ("p0", "true"),
        ("p1", "false"),
        ("n0", "true"),
        ("n1", "false"),
    ], false);
}

#[test]
fn test_split() {
    test_unary_op(|iv| {
        let (l, r) = iv.split(b!("0"));
        format!("{} | {}", l, r)
    }, all_sign_classes(), vec![
        ("nan", "NaN | NaN"),
        ("whl", "<-inf, 0> | <0, inf>"),
        ("m", "<-1, 0> | <0, 1>"),
        ("z", "NaN | 0"),
        ("p0", "NaN | <0, 1>"),
        ("p1", "NaN | <1, 2>"),
        ("n0", "<-1, 0> | NaN"),
        ("n1", "<-2, -1> | NaN"),
    ], false);
}

#[test]
fn test_from_f64() {
    use std::f64;
    assert_str_eq!("0", IV::from(0f64));
    assert_str_eq!("0.9999999999999999", IV::from(0.9999999999999999));
    assert_str_eq!("1.000000000000001", IV::from(1.000000000000001));
    assert_str_eq!("-0.9999999999999999", IV::from(-0.9999999999999999));
    assert_str_eq!("-1.000000000000001", IV::from(-1.000000000000001));
    assert_str_eq!("NaN", IV::from(f64::NAN));
}

#[test]
fn test_from_str() {
    use std::str::FromStr;
    assert_str_eq!("0", IV::from_str("0").unwrap());
    assert_str_eq!("0.9999999999999999", IV::from_str("0.9999999999999999").unwrap());
    assert_str_eq!("1.000000000000001", IV::from_str("1.000000000000001").unwrap());
    assert_str_eq!("-0.9999999999999999", IV::from_str("-0.9999999999999999").unwrap());
    assert_str_eq!("-1.000000000000001", IV::from_str("-1.000000000000001").unwrap());
    assert_str_eq!("NaN", IV::from_str("NaN").unwrap());
}

#[test]
fn test_clone() {
    let x = iv!("<0, 1>");
    assert_eq!(x, x.clone());
    let mut y = iv!("<1, 2>");
    y.clone_from(&x);
    assert_eq!(x, y);
}

#[test]
fn test_into_pair() {
    let (lo, hi) = iv!("<0, 1>").into();
    assert_str_eq!("0", lo);
    assert_str_eq!("1", hi);
}

#[test]
fn test_partial_eq() {
    assert!(iv!("NaN") == iv!("NaN"));
    assert!(iv!("0") == iv!("0"));
    assert!(iv!("<0, 1>") == iv!("<0, 1>"));
    assert!(iv!("<0, 1>") != iv!("<0, 2>"));
}

#[test]
fn test_neg() {
    use std::ops::Neg;
    test_unary_op(IV::neg, simple(), vec![
        ("nan", "NaN"),
        ("whl", "<-inf, inf>"),
        ("m", "<-1, 1>"),
        ("z", "0"),
        ("p", "<-2, -1>"),
        ("n", "<1, 2>"),
    ], false);
}

#[test]
fn test_add() {
    use std::ops::Add;
    test_binary_op(IV::add, simple(), vec![
        ("nan.*", "NaN"),
        ("whl.*", "<-inf, inf>"),
        ("m.m", "<-2, 2>"),
        ("m.z", "<-1, 1>"),
        ("m.p", "<0, 3>"),
        ("m.n", "<-3, 0>"),
        ("z.z", "0"),
        ("z.p", "<1, 2>"),
        ("z.n", "<-2, -1>"),
        ("p.p", "<2, 4>"),
        ("p.n", "<-1, 1>"),
        ("n.n", "<-4, -2>"),
    ], true);
}

#[test]
fn test_sub() {
    use std::ops::Sub;
    test_binary_op(IV::sub, simple(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("whl.*", "<-inf, inf>"),
        ("*.whl", "<-inf, inf>"),
        ("m.m", "<-2, 2>"),
        ("m.z", "<-1, 1>"),
        ("m.p", "<-3, 0>"),
        ("m.n", "<0, 3>"),
        ("z.m", "<-1, 1>"),
        ("z.z", "0"),
        ("z.p", "<-2, -1>"),
        ("z.n", "<1, 2>"),
        ("p.m", "<0, 3>"),
        ("p.z", "<1, 2>"),
        ("p.p", "<-1, 1>"),
        ("p.n", "<2, 4>"),
        ("n.m", "<-3, 0>"),
        ("n.z", "<-2, -1>"),
        ("n.p", "<-4, -2>"),
        ("n.n", "<-1, 1>"),
    ], false);
}

#[test]
fn test_mul() {
    use std::ops::Mul;
    test_binary_op(IV::mul, all_sign_classes(), vec![
        ("nan.*", "NaN"),
        ("z.*", "0"),
        ("whl.*", "<-inf, inf>"),
        ("m.m", "<-1, 1>"),
        ("m.p0", "<-1, 1>"),
        ("m.p1", "<-2, 2>"),
        ("m.n0", "<-1, 1>"),
        ("m.n1", "<-2, 2>"),
        ("p0.p0", "<0, 1>"),
        ("p0.p1", "<0, 2>"),
        ("p0.n0", "<-1, 0>"),
        ("p0.n1", "<-2, 0>"),
        ("p1.p1", "<1, 4>"),
        ("p1.n0", "<-2, 0>"),
        ("p1.n1", "<-4, -1>"),
        ("n0.n0", "<0, 1>"),
        ("n0.n1", "<0, 2>"),
        ("n1.n1", "<1, 4>"),
    ], true);
}

#[test]
fn test_div() {
    use std::ops::Div;
    test_binary_op(IV::div, all_sign_classes(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("*.z", "NaN"),
        ("z.*", "0"),
        ("whl.*", "<-inf, inf>"),
        ("*.whl", "<-inf, inf>"),
        ("m.m", "<-inf, inf>"),
        ("m.p0", "<-inf, inf>"),
        ("m.p1", "<-1, 1>"),
        ("m.n0", "<-inf, inf>"),
        ("m.n1", "<-1, 1>"),
        ("p0.m", "<-inf, inf>"),
        ("p0.p0", "<0, inf>"),
        ("p0.p1", "<0, 1>"),
        ("p0.n0", "<-inf, 0>"),
        ("p0.n1", "<-1, 0>"),
        ("p1.m", "<-inf, inf>"),
        ("p1.p0", "<1, inf>"),
        ("p1.p1", "<0.5, 2>"),
        ("p1.n0", "<-inf, -1>"),
        ("p1.n1", "<-2, -0.5>"),
        ("n0.m", "<-inf, inf>"),
        ("n0.p0", "<-inf, 0>"),
        ("n0.p1", "<-1, 0>"),
        ("n0.n0", "<0, inf>"),
        ("n0.n1", "<0, 1>"),
        ("n1.m", "<-inf, inf>"),
        ("n1.p0", "<-inf, -1>"),
        ("n1.p1", "<-2, -0.5>"),
        ("n1.n0", "<1, inf>"),
        ("n1.n1", "<0.5, 2>"),
    ], false);
}

fn simple<'a>() -> Vec<(&'a str, IV)> {
    vec![
        ("nan", iv!("NaN")),
        ("whl", iv!("<-inf, inf>")),
        ("m", iv!("<-1, 1>")),
        ("z", iv!("0")),
        ("p", iv!("<1, 2>")),
        ("n", iv!("<-2, -1>")),
    ]
}

fn all_sign_classes<'a>() -> Vec<(&'a str, IV)> {
    vec![
        ("nan", iv!("NaN")),
        ("whl", iv!("<-inf, inf>")),
        ("m", iv!("<-1, 1>")),
        ("z", iv!("0")),
        ("p0", iv!("<0, 1>")),
        ("p1", iv!("<1, 2>")),
        ("n0", iv!("<-1, 0>")),
        ("n1", iv!("<-2, -1>")),
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
                                cases: Vec<(&'a str, IV)>,
                                expected: Vec<(&str, &str)>,
                                print: bool)
    where OP: Fn(IV) -> R, R: ::std::fmt::Display + Sized
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
                                 cases: Vec<(&'a str, IV)>,
                                 mut expected: Vec<(&str, &str)>,
                                 commutative: bool)
    where OP: Fn(IV, IV) -> R, R: ::std::fmt::Display + Sized
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
