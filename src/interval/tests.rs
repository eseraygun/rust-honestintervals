use fp::Float;

type B = ::mpfr::Mpfr;
type IV = ::interval::Interval<::mpfr::Mpfr>;

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

macro_rules! iv_fn {
    ($n:tt) => { IV::$n };
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
fn test_new_different_precisions() {
    IV::new(B::zero(24), B::zero(53));
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
    assert_eq!(2usize, IV::new(B::zero(2), B::zero(2)).precision());
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
    assert_str_eq!("<2, 3>", iv!("<-1.1, 0.9>").size());
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
fn test_partial_eq() {
    test_binary_op(|x, y| x == y, simple(), vec![
        ("nan.*", "false"),
        ("whl.whl", "true"),
        ("whl.*", "false"),
        ("m.m", "true"),
        ("m.*", "false"),
        ("z.z", "true"),
        ("z.*", "false"),
        ("p.p", "true"),
        ("p.*", "false"),
        ("n.n", "true"),
        ("n.*", "false"),
    ], false);
}

#[test]
fn test_neg() {
    use std::ops::Neg;
    test_unary_op(iv_fn!(neg), simple(), vec![
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
    test_binary_op(iv_fn!(add), simple(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("whl.*", "<-inf, inf>"),
        ("*.whl", "<-inf, inf>"),
        ("m.m", "<-2, 2>"),
        ("m.z", "<-1, 1>"),
        ("m.p", "<0, 3>"),
        ("m.n", "<-3, 0>"),
        ("z.m", "<-1, 1>"),
        ("z.z", "0"),
        ("z.p", "<1, 2>"),
        ("z.n", "<-2, -1>"),
        ("p.m", "<0, 3>"),
        ("p.z", "<1, 2>"),
        ("p.p", "<2, 4>"),
        ("p.n", "<-1, 1>"),
        ("n.m", "<-3, 0>"),
        ("n.z", "<-2, -1>"),
        ("n.p", "<-1, 1>"),
        ("n.n", "<-4, -2>"),
    ], false);
}

#[test]
fn test_mul() {
    use std::ops::Mul;
    test_binary_op(iv_fn!(mul), all_sign_classes(), vec![
        ("nan.*", "NaN"),
        ("*.nan", "NaN"),
        ("z.*", "0"),
        ("*.z", "0"),
        ("whl.*", "<-inf, inf>"),
        ("*.whl", "<-inf, inf>"),
        ("m.m", "<-1, 1>"),
        ("m.p0", "<-1, 1>"),
        ("m.p1", "<-2, 2>"),
        ("m.n0", "<-1, 1>"),
        ("m.n1", "<-2, 2>"),
        ("p0.m", "<-1, 1>"),
        ("p0.p0", "<0, 1>"),
        ("p0.p1", "<0, 2>"),
        ("p0.n0", "<-1, 0>"),
        ("p0.n1", "<-2, 0>"),
        ("p1.m", "<-2, 2>"),
        ("p1.p0", "<0, 2>"),
        ("p1.p1", "<1, 4>"),
        ("p1.n0", "<-2, 0>"),
        ("p1.n1", "<-4, -1>"),
        ("n0.m", "<-1, 1>"),
        ("n0.p0", "<-1, 0>"),
        ("n0.p1", "<-2, 0>"),
        ("n0.n0", "<0, 1>"),
        ("n0.n1", "<0, 2>"),
        ("n1.m", "<-2, 2>"),
        ("n1.p0", "<-2, 0>"),
        ("n1.p1", "<-4, -1>"),
        ("n1.n0", "<0, 2>"),
        ("n1.n1", "<1, 4>"),
    ], false);
}

#[test]
fn test_div() {
    use std::ops::Div;
    test_binary_op(iv_fn!(div), all_sign_classes(), vec![
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

#[test]
fn test_log() {
    test_unary_op(iv_fn!(log), all_sign_classes(), vec![
        ("nan", "NaN"),
        ("whl", "<-inf, inf>"),
        ("m", "<-inf, 0>"),
        ("z", "NaN"),
        ("p0", "<-inf, 0>"),
        ("p1", "<0, 0.75>"),
        ("n0", "NaN"),
        ("n1", "NaN"),
    ], false);
}

#[test]
fn test_exp() {
    test_unary_op(iv_fn!(exp), all_sign_classes(), vec![
        ("nan", "NaN"),
        ("whl", "<0, inf>"),
        ("m", "<0.25, 3>"),
        ("z", "1"),
        ("p0", "<1, 3>"),
        ("p1", "<2, 8>"),
        ("n0", "<0.25, 1>"),
        ("n1", "<0.125, 0.375>"),
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
                                 expected: Vec<(&str, &str)>,
                                 print: bool)
    where OP: Fn(IV, IV) -> R, R: ::std::fmt::Display + Sized
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
