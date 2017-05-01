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

impl<BOUND: Float> Div<Self> for Interval<BOUND> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        match self.sign_class() {
            SignClass::Mixed => match rhs.sign_class() {
                SignClass::Mixed => Self::whole(self.precision()),
                SignClass::Zero => Self::nan(self.precision()),
                SignClass::Positive(other_has_zero) => if other_has_zero {
                    Self::whole(self.precision())
                } else {
                    Self::new(
                        self.lo.div_lo(rhs.lo.clone()),
                        self.hi.div_hi(rhs.lo),
                    )
                },
                SignClass::Negative(other_has_zero) => if other_has_zero {
                    Self::whole(self.precision())
                } else {
                    Self::new(
                        self.hi.div_lo(rhs.hi.clone()),
                        self.lo.div_hi(rhs.hi),
                    )
                },
            },
            SignClass::Zero => if rhs.is_zero() {
                Self::nan(self.precision())
            } else {
                self
            },
            SignClass::Positive(_) => match rhs.sign_class() {
                SignClass::Mixed => Self::whole(self.precision()),
                SignClass::Zero => Self::nan(self.precision()),
                SignClass::Positive(other_has_zero) => if other_has_zero {
                    let precision = self.precision();
                    Self::new(
                        self.lo.div_lo(rhs.hi),
                        BOUND::infinity(precision),
                    )
                } else {
                    Self::new(
                        self.lo.div_lo(rhs.hi),
                        self.hi.div_hi(rhs.lo),
                    )
                },
                SignClass::Negative(other_has_zero) => if other_has_zero {
                    let precision = self.precision();
                    Self::new(
                        BOUND::neg_infinity(precision),
                        self.lo.div_hi(rhs.lo),
                    )
                } else {
                    Self::new(
                        self.hi.div_lo(rhs.hi),
                        self.lo.div_hi(rhs.lo),
                    )
                },
            },
            SignClass::Negative(_) => match rhs.sign_class() {
                SignClass::Mixed => Self::whole(self.precision()),
                SignClass::Zero => Self::nan(self.precision()),
                SignClass::Positive(other_has_zero) => if other_has_zero {
                    let precision = self.precision();
                    Self::new(
                        BOUND::neg_infinity(precision),
                        self.hi.div_hi(rhs.hi),
                    )
                } else {
                    Self::new(
                        self.lo.div_lo(rhs.lo),
                        self.hi.div_hi(rhs.hi),
                    )
                },
                SignClass::Negative(other_has_zero) => if other_has_zero {
                    let precision = self.precision();
                    Self::new(
                        self.hi.div_lo(rhs.lo),
                        BOUND::infinity(precision),
                    )
                } else {
                    Self::new(
                        self.hi.div_lo(rhs.lo),
                        self.lo.div_hi(rhs.hi),
                    )
                },
            },
        }
    }
}
