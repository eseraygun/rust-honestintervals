use super::def::Interval;
use std::cmp::Ordering;

use crate::fp::Float;

impl<BOUND: Float> PartialEq for Interval<BOUND> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.is_nan() && rhs.is_nan() || self.lo == rhs.lo && self.hi == rhs.hi
    }
}

impl<BOUND: Float> PartialOrd for Interval<BOUND> {
    #[inline]
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        if self.lo > rhs.hi {
            return Some(Ordering::Greater);
        }
        if self.hi < rhs.lo {
            return Some(Ordering::Less);
        }
        None
    }
}