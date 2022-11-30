use super::capi::*;
use super::def::Mpfr;

use std::cmp::Ordering;

impl PartialEq<Mpfr> for Mpfr {
    #[inline]
    fn eq(&self, other: &Mpfr) -> bool {
        if unsafe { mpfr_unordered_p(&self.mpfr, &other.mpfr) } == 0 {
            unsafe { mpfr_equal_p(&self.mpfr, &other.mpfr) != 0 }
        } else {
            false // self or other is NaN and they cannot be compared
        }
    }
}

impl PartialOrd<Mpfr> for Mpfr {
    #[inline]
    fn partial_cmp(&self, other: &Mpfr) -> Option<Ordering> {
        if unsafe { mpfr_unordered_p(&self.mpfr, &other.mpfr) } == 0 {
            let cmp = unsafe { mpfr_cmp(&self.mpfr, &other.mpfr) };
            if cmp < 0 {
                Some(Ordering::Less)
            } else if cmp > 0 {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        } else {
            None // self or other is NaN and they cannot be compared
        }
    }

    #[inline]
    fn lt(&self, other: &Mpfr) -> bool {
        unsafe { mpfr_less_p(&self.mpfr, &other.mpfr) != 0 }
    }

    #[inline]
    fn le(&self, other: &Mpfr) -> bool {
        unsafe { mpfr_lessequal_p(&self.mpfr, &other.mpfr) != 0 }
    }

    #[inline]
    fn gt(&self, other: &Mpfr) -> bool {
        unsafe { mpfr_greater_p(&self.mpfr, &other.mpfr) != 0 }
    }

    #[inline]
    fn ge(&self, other: &Mpfr) -> bool {
        unsafe { mpfr_greaterequal_p(&self.mpfr, &other.mpfr) != 0 }
    }
}
