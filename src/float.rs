use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum RoundingMode {
    /// Round to nearest with ties to even.
    HalfToEven,
    /// Round to nearest with ties away from zero.
    HalfAwayFromZero,
    /// Round towards positive infinity.
    Up,
    /// Round towards negative infinity.
    Down,
    /// Round towards zero.
    TowardsZero,
    /// Round away from zero.
    AwayFromZero,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Sign {
    Negative = -1,
    Zero = 0,
    Positive = 1,
}

pub trait Float : From<f64> + FromStr + Clone + Display + Into<f64> + PartialOrd
{
    fn zero(precision: usize) -> Self;
    fn one(precision: usize) -> Self;
    fn infinity(precision: usize) -> Self;
    fn neg_infinity(precision: usize) -> Self;
    fn nan(precision: usize) -> Self;

    fn custom_from_f64(val: f64, precision: usize, rounding_mode: RoundingMode) -> Self;
    fn custom_from_str(s: &str,
                       base: usize,
                       precision: usize,
                       rounding_mode: RoundingMode)
                       -> Option<Self>;

    fn to_f64(&self, rounding_mode: RoundingMode) -> f64;

    fn precision(&self) -> usize;

    fn is_finite(&self) -> bool;
    fn is_infinite(&self) -> bool;
    fn is_infinity(&self) -> bool;
    fn is_neg_infinity(&self) -> bool;
    fn is_nan(&self) -> bool;

    fn sign(&self) -> Sign;

    fn neg(&self) -> Self;
    fn abs(&self) -> Self;

    fn add(&self, other: &Self, rounding_mode: RoundingMode) -> Self;
    fn sub(&self, other: &Self, rounding_mode: RoundingMode) -> Self;
    fn mul(&self, other: &Self, rounding_mode: RoundingMode) -> Self;
    fn div(&self, other: &Self, rounding_mode: RoundingMode) -> Self;

    fn log(&self, rounding_mode: RoundingMode) -> Self;
    fn exp(&self, rounding_mode: RoundingMode) -> Self;

    fn min(&self, other: &Self, rounding_mode: RoundingMode) -> Self;
    fn max(&self, other: &Self, rounding_mode: RoundingMode) -> Self;
}
