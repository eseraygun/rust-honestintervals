use super::def::{Interval, SignClass};

use fp::Float;

use std::ops::{Add, Div, Mul, Neg, Sub};

impl<BOUND: Float> Neg for Interval<BOUND> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.hi, -self.lo)
    }
}

impl<BOUND: Float> Add<Self> for Interval<BOUND> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.lo.add_lo(rhs.lo),
            self.hi.add_hi(rhs.hi),
        )
    }
}

impl<BOUND: Float> Sub<Self> for Interval<BOUND> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.lo.sub_lo(rhs.hi),
            self.hi.sub_hi(rhs.lo),
        )
    }
}

impl<BOUND: Float> Mul<Self> for Interval<BOUND> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        match self.sign_class() {
            SignClass::Mixed => match rhs.sign_class() {
                SignClass::Mixed => Self::new(
                    self.lo.clone().mul_lo(rhs.hi.clone())
                        .min(self.hi.clone().mul_lo(rhs.lo.clone())),
                    self.lo.mul_hi(rhs.lo).max(self.hi.mul_hi(rhs.hi)),
                ),
                SignClass::Zero => rhs,
                SignClass::Positive(_) => Self::new(
                    self.lo.mul_lo(rhs.hi.clone()),
                    self.hi.mul_hi(rhs.hi),
                ),
                SignClass::Negative(_) => Self::new(
                    self.hi.mul_lo(rhs.lo.clone()),
                    self.lo.mul_hi(rhs.lo),
                ),
            },
            SignClass::Zero => if rhs.is_nan() {
                rhs
            } else {
                self
            },
            SignClass::Positive(_) => match rhs.sign_class() {
                SignClass::Mixed => Self::new(
                    self.hi.clone().mul_lo(rhs.lo),
                    self.hi.mul_hi(rhs.hi),
                ),
                SignClass::Zero => rhs,
                SignClass::Positive(_) => Self::new(
                    self.lo.mul_lo(rhs.lo),
                    self.hi.mul_hi(rhs.hi),
                ),
                SignClass::Negative(_) => Self::new(
                    self.hi.mul_lo(rhs.lo),
                    self.lo.mul_hi(rhs.hi),
                ),
            },
            SignClass::Negative(_) => match rhs.sign_class() {
                SignClass::Mixed => Self::new(
                    self.lo.clone().mul_lo(rhs.hi),
                    self.lo.mul_hi(rhs.lo),
                ),
                SignClass::Zero => rhs,
                SignClass::Positive(_) => Self::new(
                    self.lo.mul_lo(rhs.hi),
                    self.hi.mul_hi(rhs.lo),
                ),
                SignClass::Negative(_) => Self::new(
                    self.hi.mul_lo(rhs.hi),
                    self.lo.mul_hi(rhs.lo),
                ),
            },
        }
    }
}

impl<BOUND: Float> Interval<BOUND> {
    /// Divides `self` by `rhs` and returns a vector of intervals minimally covering the result.
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
            SignClass::Zero => if self.is_nan() || rhs.is_nan() || rhs.is_zero() {
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

impl<BOUND: Float> Div<Self> for Interval<BOUND> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let precision = self.precision();
        Self::minimal_cover(self.div_multi(rhs), precision)
    }
}
