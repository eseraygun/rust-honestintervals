macro_rules! mpfr {
    ($v:expr) => { Mpfr::from($v as f64) }
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
