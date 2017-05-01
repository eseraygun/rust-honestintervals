use std::fmt::Display;
use std::convert;
use std::ops;
use std::str;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Sign {
    Negative = -1,
    Zero = 0,
    Positive = 1,
}

pub trait From<T> {
    fn from_lo(T, precision: usize) -> Self;
    fn from_hi(T, precision: usize) -> Self;
}

pub trait FromStr: Sized {
    type Err;
    fn from_str_lo(&str, precision: usize) -> Result<Self, Self::Err>;
    fn from_str_hi(&str, precision: usize) -> Result<Self, Self::Err>;
}

pub trait Into<T> {
    fn into_lo(self) -> T;
    fn into_hi(self) -> T;
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
    fn add_lo(self, rhs: RHS) -> Self::Output;
    fn add_hi(self, rhs: RHS) -> Self::Output;
}

pub trait Sub<RHS = Self> {
    type Output;
    fn sub_lo(self, rhs: RHS) -> Self::Output;
    fn sub_hi(self, rhs: RHS) -> Self::Output;
}

pub trait Mul<RHS = Self> {
    type Output;
    fn mul_lo(self, rhs: RHS) -> Self::Output;
    fn mul_hi(self, rhs: RHS) -> Self::Output;
}

pub trait Div<RHS = Self> {
    type Output;
    fn div_lo(self, rhs: RHS) -> Self::Output;
    fn div_hi(self, rhs: RHS) -> Self::Output;
}

pub trait Transc {
    type Output;
    fn log_lo(self) -> Self::Output;
    fn log_hi(self) -> Self::Output;
    fn exp_lo(self) -> Self::Output;
    fn exp_hi(self) -> Self::Output;
}

pub trait Float: convert::From<f64> + str::FromStr +
                 Clone + Display + Into<f64> + PartialOrd +
                 From<f64> + FromStr +
                 ops::Neg<Output=Self> + Abs<Output=Self> +
                 Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
                 MinMax<Output=Self>
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
