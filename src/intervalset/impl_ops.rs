use super::def::IntervalSet;

use fp::Float;

use std::ops::{Add, Div, Mul, Neg, Sub};

impl<BOUND: Float> Neg for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self::Output {
        Self {
            intervals: self.intervals.drain(..).rev().map(|i| -i).collect(),
        }
    }
}

impl<BOUND: Float> Add<Self> for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        self.binary_op(other, |i, j| vec![i + j])
    }
}

impl<BOUND: Float> Sub<Self> for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        self.binary_op(other, |i, j| vec![i - j])
    }
}

impl<BOUND: Float> Mul<Self> for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        self.binary_op(other, |i, j| vec![i * j])
    }
}

impl<BOUND: Float> Div<Self> for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        self.binary_op(other, |i, j| i.div_multi(j))
    }
}
