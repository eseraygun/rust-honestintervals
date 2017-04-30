use std::fmt::Display;
use std::convert;
use std::ops;
use std::str;

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

pub trait From<T> {
    fn from(T, precision: usize, rounding_mode: RoundingMode) -> Self;
}

pub trait FromStr: Sized {
    type Err;
    fn from_str(&str, precision: usize, rounding_mode: RoundingMode) -> Result<Self, Self::Err>;
}

pub trait Into<T> {
    fn into(self, rounding_mode: RoundingMode) -> T;
}

pub trait MinMax<RHS = Self> {
    type Output;
    fn min(self, rhs: Self) -> Self;
    fn max(self, rhs: Self) -> Self;
}

pub trait Abs {
    type Output;
    fn abs(self) -> Self::Output;
}

pub trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS, rounding_mode: RoundingMode) -> Self::Output;
}

pub trait Sub<RHS = Self> {
    type Output;
    fn sub(self, rhs: RHS, rounding_mode: RoundingMode) -> Self::Output;
}

pub trait Mul<RHS = Self> {
    type Output;
    fn mul(self, rhs: RHS, rounding_mode: RoundingMode) -> Self::Output;
}

pub trait Div<RHS = Self> {
    type Output;
    fn div(self, rhs: RHS, rounding_mode: RoundingMode) -> Self::Output;
}

pub trait Transc {
    type Output;
    fn log(self, rounding_mode: RoundingMode) -> Self::Output;
    fn exp(self, rounding_mode: RoundingMode) -> Self::Output;
}

pub trait Float: convert::From<f64> + str::FromStr +
                 Clone + Display + Into<f64> + PartialOrd +
                 From<f64> + FromStr +
                 ops::Neg + Abs + Add + Sub + Mul + Div + MinMax
{
    fn zero(precision: usize) -> Self;
    fn neg_zero(precision: usize) -> Self;
    fn one(precision: usize) -> Self;
    fn infinity(precision: usize) -> Self;
    fn neg_infinity(precision: usize) -> Self;
    fn nan(precision: usize) -> Self;

    fn sign(&self) -> Sign;
    fn precision(&self) -> usize;

    fn is_finite(&self) -> bool;
    fn is_infinite(&self) -> bool;
    fn is_infinity(&self) -> bool;
    fn is_neg_infinity(&self) -> bool;
    fn is_nan(&self) -> bool;
}
