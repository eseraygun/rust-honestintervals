use super::def::Interval;

use fp::Float;

impl<BOUND: Float> PartialEq for Interval<BOUND> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.is_nan() && rhs.is_nan() || self.lo == rhs.lo && self.hi == rhs.hi
    }
}
