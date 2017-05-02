use super::def::IntervalSet;

use fp::Float;
use interval::{Interval, SignClass};

use std::ops::{Add, Div, Mul, Neg, Sub};

impl<BOUND: Float> Neg for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self::Output {
        Self { intervals: self.intervals.drain(..).rev().map(|i| -i).collect() }
    }
}

impl<BOUND: Float> Add<Self> for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        self.binary_op(other, |i, j| { vec![i + j] })
    }
}

impl<BOUND: Float> Sub<Self> for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        self.binary_op(other, |i, j| { vec![i - j] })
    }
}

impl<BOUND: Float> Mul<Self> for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        self.binary_op(other, |i, j| { vec![i * j] })
    }
}

impl<BOUND: Float> Interval<BOUND> {
    pub fn div_multi(self, rhs: Self) -> Vec<Self> {
        let precision = self.precision();
        match self.sign_class() {
            SignClass::Mixed => match rhs.sign_class() {
                SignClass::Mixed => vec![Self::whole(precision)],
                SignClass::Zero => vec![],
                SignClass::Positive(other_has_zero) => if other_has_zero {
                    vec![Self::whole(precision)]
                } else {
                    vec![Self::new(
                        self.lo.div_lo(rhs.lo.clone()),
                        self.hi.div_hi(rhs.lo),
                    )]
                },
                SignClass::Negative(other_has_zero) => if other_has_zero {
                    vec![Self::whole(precision)]
                } else {
                    vec![Self::new(
                        self.hi.div_lo(rhs.hi.clone()),
                        self.lo.div_hi(rhs.hi),
                    )]
                },
            },
            SignClass::Zero => if rhs.is_zero() {
                vec![]
            } else {
                vec![self]
            },
            SignClass::Positive(self_has_zero) => match rhs.sign_class() {
                SignClass::Mixed => if self_has_zero {
                    vec![Self::whole(precision)]
                } else {
                    vec![
                        Interval::new(
                            BOUND::neg_infinity(precision),
                            self.lo.clone().div_hi(rhs.lo),
                        ),
                        Interval::new(
                            self.lo.div_lo(rhs.hi),
                            BOUND::infinity(precision),
                        ),
                    ]
                },
                SignClass::Zero => vec![],
                SignClass::Positive(other_has_zero) => if other_has_zero {
                    vec![Self::new(
                        self.lo.clone().div_lo(rhs.hi),
                        BOUND::infinity(precision),
                    )]
                } else {
                    vec![Self::new(
                        self.lo.div_lo(rhs.hi),
                        self.hi.div_hi(rhs.lo),
                    )]
                },
                SignClass::Negative(other_has_zero) => if other_has_zero {
                    vec![Self::new(
                        BOUND::neg_infinity(precision),
                        self.lo.div_hi(rhs.lo),
                    )]
                } else {
                    vec![Self::new(
                        self.hi.div_lo(rhs.hi),
                        self.lo.div_hi(rhs.lo),
                    )]
                },
            },
            SignClass::Negative(self_has_zero) => match rhs.sign_class() {
                SignClass::Mixed => if self_has_zero {
                    vec![Self::whole(precision)]
                } else {
                    vec![
                        Interval::new(
                            BOUND::neg_infinity(precision),
                            self.hi.clone().div_hi(rhs.hi),
                        ),
                        Interval::new(
                            self.hi.div_lo(rhs.lo),
                            BOUND::infinity(precision),
                        ),
                    ]
                },
                SignClass::Zero => vec![],
                SignClass::Positive(other_has_zero) => if other_has_zero {
                    vec![Self::new(
                        BOUND::neg_infinity(precision),
                        self.hi.div_hi(rhs.hi),
                    )]
                } else {
                    vec![Self::new(
                        self.lo.div_lo(rhs.lo),
                        self.hi.div_hi(rhs.hi),
                    )]
                },
                SignClass::Negative(other_has_zero) => if other_has_zero {
                    vec![Self::new(
                        self.hi.div_lo(rhs.lo),
                        BOUND::infinity(precision),
                    )]
                } else {
                    vec![Self::new(
                        self.hi.div_lo(rhs.lo),
                        self.lo.div_hi(rhs.hi),
                    )]
                },
            },
        }
    }
}

impl<BOUND: Float> Div<Self> for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self::Output {
        self.binary_op(other, |i, j| { i.div_multi(j) })
    }
}

//#[cfg(test)]
//mod test {
//    use float::{Float, RoundingMode};
//    use interval::Interval;
//    use intervalset::IntervalSet;
//    use mpfr::Mpfr;
//
//    #[test]
//    fn test_neg() {
//        assert_str_eq!("{-2; <-1, 0>}", -interval_set!{[0, 1]; [2]});
//    }
//
//    #[test]
//    fn test_add() {
//        assert_str_eq!("{<1, 2>; 3}", interval_set!{[0, 1]; [2]} + interval_set!{[1]});
//    }
//
//    #[test]
//    fn test_sub() {
//        assert_str_eq!("{<-1, 0>; 1}", interval_set!{[0, 1]; [2]} - interval_set!{[1]});
//    }
//
//    #[test]
//    fn test_mul() {
//        assert_str_eq!("{<0, 1>; 2}", interval_set!{[0, 1]; [2]} * interval_set!{[1]});
//        assert_str_eq!("{-2; <-1, 0>}", interval_set!{[0, 1]; [2]} * interval_set!{[-1]});
//    }
//
//    #[test]
//    fn test_div() {
//        assert_str_eq!("{<0, 1>; 2}", interval_set!{[0, 1]; [2]} * interval_set!{[1]});
//        assert_str_eq!("{-2; <-1, 0>}", interval_set!{[0, 1]; [2]} * interval_set!{[-1]});
//    }
//
//    #[test]
//    fn test_div_p1m() {
//        assert_str_eq!("{<-inf, -0.5>; <1, inf>}", interval_set!{[1, 2]} / interval_set!{[-2, 1]});
//    }
//}
