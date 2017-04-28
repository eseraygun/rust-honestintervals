macro_rules! mpfr {
    ($v:expr) => { Mpfr::custom_from_f64($v.into(), 53, RoundingMode::HalfToEven) }
}

macro_rules! mpfr_inf {
    () => { Mpfr::infinity(53) }
}

macro_rules! mpfr_neg_inf {
    () => { Mpfr::neg_infinity(53) }
}

macro_rules! mpfr_nan {
    () => { Mpfr::nan(53) }
}
