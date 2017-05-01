use super::def::{Interval, SignClass};

use fp::Float;

impl<BOUND: Float> Interval<BOUND> {
    pub fn log(self) -> Self {
        match self.sign_class() {
            SignClass::Mixed => Self::new(
                BOUND::neg_infinity(self.precision()),
                self.hi.log_hi(),
            ),
            SignClass::Zero => Self::nan(self.precision()),
            SignClass::Positive(has_zero) => if has_zero {
                Self::new(
                    BOUND::neg_infinity(self.precision()),
                    self.hi.log_hi(),
                )
            } else {
                Self::new(
                    self.lo.log_lo(),
                    self.hi.log_hi(),
                )
            },
            SignClass::Negative(_) => Self::nan(self.precision()),
        }
    }

    pub fn exp(self) -> Self {
        Interval::new(self.lo.exp_lo(), self.hi.exp_hi())
    }

    pub fn pow(self, other: Self) -> Self {
        (self.log() * other).exp()
    }
}
