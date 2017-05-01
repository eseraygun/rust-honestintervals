macro_rules! assert_str_eq {
    ($x:expr, $y:expr) => { assert_eq!($x, format!("{}", $y)) };
    ($x:expr, $y:expr, $($arg:tt)+) => { assert_eq!($x, format!("{}", $y), $($arg)+) };
}

macro_rules! mpfr {
    ($v:expr) => { ::mpfr::Mpfr::from($v as f64) }
}

macro_rules! mpfr_lo {
    ($v:expr) => {
        {
            use ::fp::From;
            ::mpfr::Mpfr::from_lo($v as f64, 53)
        }
    }
}

macro_rules! mpfr_hi {
    ($v:expr) => {
        {
            use ::fp::From;
            ::mpfr::Mpfr::from_hi($v as f64, 53)
        }
    }
}

macro_rules! mpfr_inf {
    () => { ::mpfr::Mpfr::infinity(53) }
}

macro_rules! mpfr_neg_inf {
    () => { ::mpfr::Mpfr::neg_infinity(53) }
}

macro_rules! mpfr_nan {
    () => { ::mpfr::Mpfr::nan(53) }
}
