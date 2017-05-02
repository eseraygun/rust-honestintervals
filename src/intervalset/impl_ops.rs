use super::def::IntervalSet;

use fp::Float;
use interval::{Interval, SignClass};

use std::ops::{Add, Div, Mul, Neg, Sub};

impl<BOUND: Float> Neg for IntervalSet<BOUND> {
    type Output = Self;

    #[inline]
    fn neg(mut self) -> Self::Output {
        IntervalSet::from_intervals(self.intervals.drain(..).rev().map(|i| -i).collect())
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
    pub fn div_multi(self, other: Self) -> Vec<Self> {
        match self.sign_class() {
            SignClass::Mixed => match other.sign_class() {
                SignClass::Mixed => vec![Self::whole(self.precision())],
                SignClass::Zero => vec![],
                SignClass::Positive(other_has_zero) => if other_has_zero {
                    vec![Self::whole(self.precision())]
                } else {
                    vec![Self::new(
                        BOUND::div(&self.lo, &other.lo, RoundingMode::Down),
                        BOUND::div(&self.hi, &other.lo, RoundingMode::Up),
                    )]
                },
                SignClass::Negative(other_has_zero) => if other_has_zero {
                    vec![Self::whole(self.precision())]
                } else {
                    vec![Self::new(
                        BOUND::div(&self.hi, &other.hi, RoundingMode::Down),
                        BOUND::div(&self.lo, &other.hi, RoundingMode::Up),
                    )]
                },
            },
            SignClass::Zero => if other.is_zero() {
                vec![]
            } else {
                vec![Self::zero(self.precision())]
            },
            SignClass::Positive(self_has_zero) => match other.sign_class() {
                SignClass::Mixed => if self_has_zero {
                    vec![Self::whole(self.precision())]
                } else {
                    vec![
                        Interval::new(
                            BOUND::neg_infinity(self.precision()),
                            BOUND::div(&self.lo, &other.lo, RoundingMode::Up),
                        ),
                        Interval::new(
                            BOUND::div(&self.lo, &other.hi, RoundingMode::Down),
                            BOUND::infinity(self.precision()),
                        ),
                    ]
                },
                SignClass::Zero => vec![],
                SignClass::Positive(other_has_zero) => if other_has_zero {
                    vec![Self::new(
                        BOUND::div(&self.lo, &other.hi, RoundingMode::Down),
                        BOUND::infinity(self.precision()),
                    )]
                } else {
                    vec![Self::new(
                        BOUND::div(&self.lo, &other.hi, RoundingMode::Down),
                        BOUND::div(&self.hi, &other.lo, RoundingMode::Up),
                    )]
                },
                SignClass::Negative(other_has_zero) => if other_has_zero {
                    vec![Self::new(
                        BOUND::neg_infinity(self.precision()),
                        BOUND::div(&self.lo, &other.lo, RoundingMode::Up),
                    )]
                } else {
                    vec![Self::new(
                        BOUND::div(&self.hi, &other.hi, RoundingMode::Down),
                        BOUND::div(&self.lo, &other.lo, RoundingMode::Up),
                    )]
                },
            },
            SignClass::Negative(self_has_zero) => match other.sign_class() {
                SignClass::Mixed => if self_has_zero {
                    vec![Self::whole(self.precision())]
                } else {
                    vec![
                        Interval::new(
                            BOUND::neg_infinity(self.precision()),
                            BOUND::div(&self.hi, &other.hi, RoundingMode::Up),
                        ),
                        Interval::new(
                            BOUND::div(&self.hi, &other.lo, RoundingMode::Down),
                            BOUND::infinity(self.precision()),
                        ),
                    ]
                },
                SignClass::Zero => vec![],
                SignClass::Positive(other_has_zero) => if other_has_zero {
                    vec![Self::new(
                        BOUND::neg_infinity(self.precision()),
                        BOUND::div(&self.hi, &other.hi, RoundingMode::Up),
                    )]
                } else {
                    vec![Self::new(
                        BOUND::div(&self.lo, &other.lo, RoundingMode::Down),
                        BOUND::div(&self.hi, &other.hi, RoundingMode::Up),
                    )]
                },
                SignClass::Negative(other_has_zero) => if other_has_zero {
                    vec![Self::new(
                        BOUND::div(&self.hi, &other.lo, RoundingMode::Down),
                        BOUND::infinity(self.precision()),
                    )]
                } else {
                    vec![Self::new(
                        BOUND::div(&self.hi, &other.lo, RoundingMode::Down),
                        BOUND::div(&self.lo, &other.hi, RoundingMode::Up),
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

#[cfg(test)]
mod test {
    use float::{Float, RoundingMode};
    use interval::Interval;
    use intervalset::IntervalSet;
    use mpfr::Mpfr;

    #[test]
    fn test_neg() {
        assert_str_eq!("{-2; <-1, 0>}", -interval_set!{[0, 1]; [2]});
    }

    #[test]
    fn test_add() {
        assert_str_eq!("{<1, 2>; 3}", interval_set!{[0, 1]; [2]} + interval_set!{[1]});
    }

    #[test]
    fn test_sub() {
        assert_str_eq!("{<-1, 0>; 1}", interval_set!{[0, 1]; [2]} - interval_set!{[1]});
    }

    #[test]
    fn test_mul() {
        assert_str_eq!("{<0, 1>; 2}", interval_set!{[0, 1]; [2]} * interval_set!{[1]});
        assert_str_eq!("{-2; <-1, 0>}", interval_set!{[0, 1]; [2]} * interval_set!{[-1]});
    }

    #[test]
    fn test_div() {
        assert_str_eq!("{<0, 1>; 2}", interval_set!{[0, 1]; [2]} * interval_set!{[1]});
        assert_str_eq!("{-2; <-1, 0>}", interval_set!{[0, 1]; [2]} * interval_set!{[-1]});
    }

    #[test]
    fn test_div_p1m() {
        assert_str_eq!("{<-inf, -0.5>; <1, inf>}", interval_set!{[1, 2]} / interval_set!{[-2, 1]});
    }
}
