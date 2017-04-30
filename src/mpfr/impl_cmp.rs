use super::capi::*;
use super::def::Mpfr;

use std::cmp::Ordering;

impl PartialEq<Mpfr> for Mpfr {
    #[inline]
    fn eq(&self, other: &Mpfr) -> bool {
        if unsafe { mpfr_unordered_p(&self.mpfr, &other.mpfr) } == 0 {
            unsafe { mpfr_equal_p(&self.mpfr, &other.mpfr) != 0 }
        } else {
            false  // self or other is NaN and they cannot be compared
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
            None  // self or other is NaN and they cannot be compared
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

#[cfg(test)]
mod test {
    use super::super::def::Mpfr;

    use float::{Float, RoundingMode};

    use std::cmp::Ordering;

    #[test]
    fn test_partial_eq() {
        assert!(mpfr!(0) == mpfr!(0));
        assert!(mpfr!(0) == Mpfr::zero(123));
        assert!(mpfr!(0) != mpfr!(1));
        assert!(mpfr_inf!() == mpfr_inf!());
        assert!(mpfr_neg_inf!() == mpfr_neg_inf!());
        assert!(mpfr_nan!() != mpfr_nan!());
    }

    #[test]
    fn test_partial_ord_cmp() {
        assert_eq!(Ordering::Equal, mpfr!(0).partial_cmp(&mpfr!(0)).unwrap());
        assert_eq!(Ordering::Less, mpfr!(0).partial_cmp(&mpfr!(1)).unwrap());
        assert_eq!(Ordering::Greater, mpfr!(1).partial_cmp(&mpfr!(0)).unwrap());
        assert_eq!(Ordering::Equal, mpfr_neg_inf!().partial_cmp(&mpfr_neg_inf!()).unwrap());
        assert_eq!(Ordering::Equal, mpfr_inf!().partial_cmp(&mpfr_inf!()).unwrap());
        assert_eq!(Ordering::Less, mpfr_neg_inf!().partial_cmp(&mpfr_inf!()).unwrap());
        assert_eq!(Ordering::Greater, mpfr_inf!().partial_cmp(&mpfr_neg_inf!()).unwrap());
        assert!(mpfr_nan!().partial_cmp(&mpfr_nan!()).is_none());
    }

    #[test]
    fn test_partial_ord_rest() {
        assert!(mpfr!(0) <= mpfr!(0));
        assert!(mpfr!(0) >= mpfr!(0));
        assert!(mpfr!(0) < mpfr!(1));
        assert!(mpfr!(1) > mpfr!(0));
        assert!(mpfr_neg_inf!() <= mpfr_neg_inf!());
        assert!(mpfr_inf!() >= mpfr_inf!());
        assert!(mpfr_neg_inf!() < mpfr_inf!());
        assert!(mpfr_inf!() > mpfr_neg_inf!());
        assert!(!(mpfr_nan!() <= mpfr_nan!()));
        assert!(!(mpfr_nan!() >= mpfr_nan!()));
        assert!(!(mpfr_nan!() < mpfr_nan!()));
        assert!(!(mpfr_nan!() > mpfr_nan!()));
    }
}
