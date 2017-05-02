use super::def::IntervalSet;

use fp::Float;

impl<BOUND: Float> PartialEq for IntervalSet<BOUND> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        for (i, j) in self.intervals.iter().zip(&rhs.intervals) {
            if !(i.lo == j.lo && i.hi == j.hi) {
                return false;
            }
        }
        true
    }
}
