use transc;

use std::convert;
use std::fmt::Display;
use std::ops;
use std::str;

/// Represents the sign of a float.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Sign {
    /// Sign of numbers that are strictly less than zero.
    Negative = -1,
    /// Sign of zero (including negative zero).
    Zero = 0,
    /// Sign of numbers that are strictly greater than zero.
    Positive = 1,
}

/// Finite precision version of the `std::convert::From` trait.
pub trait From<T> {
    /// Converts `T` into `Self` and rounds down inexact representations.
    fn from_lo(_: T, precision: usize) -> Self;
    /// Converts `T` into `Self` and rounds up inexact representations.
    fn from_hi(_: T, precision: usize) -> Self;
}

/// Finite precision version of the `std::str::FromStr` trait.
pub trait FromStr: Sized + str::FromStr {
    /// Parses `Self` and rounds down inexact representations.
    fn from_str_lo(s: &str, _: usize) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
    /// Parses `Self` and rounds up inexact representations.
    fn from_str_hi(s: &str, _: usize) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

/// Finite precision version of the `std::convert::Into` trait.
pub trait Into<T> {
    /// Converts `Self` into `T` and rounds down inexact representations.
    fn into_lo(self) -> T;
    /// Converts `Self` into `T` and rounds up inexact representations.
    fn into_hi(self) -> T;
}

/// Trait for binary `min` and `max` operations.
pub trait MinMax {
    /// Returns `self` if `self <= rhs`; returns `rhs` otherwise.
    ///
    /// If both `self` and `rhs` are NaN, it returns NaN. If either `self` or `rhs` is NaN, it
    /// returns the non-NaN value.
    fn min(self, rhs: Self) -> Self;
    /// Returns `self` if `self >= rhs`; returns `rhs` otherwise.
    ///
    /// If both `self` and `rhs` are NaN, it returns NaN. If either `self` or `rhs` is NaN, it
    /// returns the non-NaN value.
    fn max(self, rhs: Self) -> Self;
}

/// Trait for `abs` operation.
pub trait Abs {
    /// Returns `self` if `self >= 0`; returns `-self` otherwise.
    fn abs(self) -> Self;
}

/// Finite precision version of the `std::ops::Add` trait.
pub trait Add: Sized + ops::Add<Output = Self> {
    /// Adds `self` to `rhs` and rounds down the result.
    fn add_lo(self, rhs: Self) -> Self {
        self + rhs
    }
    /// Adds `self` to `rhs` and rounds up the result.
    fn add_hi(self, rhs: Self) -> Self {
        self + rhs
    }
}

/// Finite precision version of the `std::ops::Sub` trait.
pub trait Sub: Sized + ops::Sub<Output = Self> {
    /// Subtracts `rhs` from `self` and rounds down the result.
    fn sub_lo(self, rhs: Self) -> Self {
        self - rhs
    }
    /// Subtracts `rhs` from `self` and rounds up the result.
    fn sub_hi(self, rhs: Self) -> Self {
        self - rhs
    }
}

/// Finite precision version of the `std::ops::Mul` trait.
pub trait Mul: Sized + ops::Mul<Output = Self> {
    /// Multiplies `self` by `rhs` and rounds down the result.
    fn mul_lo(self, rhs: Self) -> Self {
        self * rhs
    }
    /// Multiplies `self` by `rhs` and rounds up the result.
    fn mul_hi(self, rhs: Self) -> Self {
        self * rhs
    }
}

/// Finite precision version of the `std::ops::Div` trait.
pub trait Div: Sized + ops::Div<Output = Self> {
    /// Divides `self` by `rhs` and rounds down the result.
    fn div_lo(self, rhs: Self) -> Self {
        self / rhs
    }
    /// Divides `self` by `rhs` and rounds up the result.
    fn div_hi(self, rhs: Self) -> Self {
        self / rhs
    }
}

/// Finite precision version of the `transc::Transc` trait.
pub trait Transc: Sized + transc::Transc<Output = Self> {
    /// Computes the natural logarithm of `self` and rounds down the result.
    fn log_lo(self) -> Self {
        self.log()
    }
    /// Computes the natural logarithm of `self` and rounds up the result.
    fn log_hi(self) -> Self {
        self.log()
    }
    /// Computes the natural exponential of `self` and rounds down the result.
    fn exp_lo(self) -> Self {
        self.exp()
    }
    /// Computes the natural exponential of `self` and rounds up the result.
    fn exp_hi(self) -> Self {
        self.exp()
    }
    /// Computes `self` raised to the power `rhs` and rounds down the result.
    fn pow_lo(self, rhs: Self) -> Self {
        self.pow(rhs)
    }
    /// Computes `self` raised to the power `rhs` and rounds up the result.
    fn pow_hi(self, rhs: Self) -> Self {
        self.pow(rhs)
    }
}

/// All-encapsulating trait for finite precision floats.
pub trait Float:
    convert::From<f64>
    + str::FromStr
    + Clone
    + Display
    + Into<f64>
    + PartialOrd
    + From<f64>
    + FromStr
    + ops::Neg<Output = Self>
    + Abs
    + Add
    + Sub
    + Mul
    + Div
    + MinMax
    + Transc
{
    /// Constructs a float representing zero.
    fn zero(precision: usize) -> Self;
    /// Constructs a float representing negative zero.
    fn neg_zero(precision: usize) -> Self;
    /// Constructs a float representing one.
    fn one(precision: usize) -> Self;
    /// Constructs a float representing positive infinity.
    fn infinity(precision: usize) -> Self;
    /// Constructs a float representing negative infinity.
    fn neg_infinity(precision: usize) -> Self;
    /// Constructs a float representing NaN.
    fn nan(precision: usize) -> Self;

    /// Returns the sign of `self`.
    fn sign(&self) -> Sign;
    /// Returns the precision of `self`.
    fn precision(&self) -> usize;

    /// Whether `self` is a regular number (non-infinity and non-NaN).
    fn is_finite(&self) -> bool;
    /// Whether `self` is either positive infinity or negative infinity.
    fn is_infinite(&self) -> bool;
    /// Whether `self` is zero (or negative zero).
    fn is_zero(&self) -> bool;
    /// Whether `self` is positive infinity.
    fn is_infinity(&self) -> bool;
    /// Whether `self` is negative infinity.
    fn is_neg_infinity(&self) -> bool;
    /// Whether `self` is NaN.
    fn is_nan(&self) -> bool;
}
