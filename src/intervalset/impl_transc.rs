use super::def::IntervalSet;

use fp::Float;
use transc::Transc;

impl<BOUND: Float> Transc for IntervalSet<BOUND> {
    type Output = Self;

     fn log(mut self) -> Self::Output {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.log()).collect())
    }

     fn exp(mut self) -> Self::Output {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.exp()).collect())
    }

     fn pow(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, |i, j| i.math_pow(j))
    }
}
