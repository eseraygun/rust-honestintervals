use super::def::IntervalSet;

use fp::Float;

impl<BOUND: Float> IntervalSet<BOUND> {
    pub fn log(mut self) -> Self {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.log()).collect())
    }

    pub fn exp(mut self) -> Self {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.exp()).collect())
    }

    pub fn pow(self, rhs: Self) -> Self {
        self.binary_op(rhs, |i, j| i.pow_multi(j))
    }
}
