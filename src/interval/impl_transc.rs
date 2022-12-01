use super::def::{Interval, SignClass};

use fp::{Float, Sign};
use transc::Transc;

impl<BOUND: Float> Interval<BOUND> {
    fn pow_a_sp_multi(self, rhs: BOUND) -> Vec<Self> {
        assert_eq!(rhs.sign(), Sign::Positive);

        let precision = self.precision();
        let mut intervals = Vec::<Self>::new();
        let (self_n, self_p) = self.split(BOUND::zero(precision));
        if !self_n.is_nan() {
            intervals.push(Self::new(
                BOUND::min(self_n.lo.clone().pow_lo(rhs.clone()), self_n.hi.clone().pow_lo(rhs.clone())),
                BOUND::max(self_n.lo.pow_hi(rhs.clone()), self_n.hi.pow_hi(rhs.clone())),
            ));
        }
        if !self_p.is_nan() {
            intervals.push(Self::new(
                self_p.lo.pow_lo(rhs.clone()),
                self_p.hi.pow_hi(rhs),
            ));
        }
        intervals
    }

    fn pow_a_sn_multi(self, rhs: BOUND) -> Vec<Self> {
        assert_eq!(rhs.sign(), Sign::Negative);

        let mut pos_intervals = self.pow_a_sp_multi(-rhs);
        let res = pos_intervals
            .drain(..)
            .flat_map(|i| Interval::one(i.precision()).div_multi(i))
            .collect();
        res
    }

    fn pow_a_s_multi(self, rhs: BOUND) -> Vec<Self> {
        match rhs.sign() {
            Sign::Negative => self.pow_a_sn_multi(rhs),
            Sign::Zero => vec![Self::one(self.precision())],
            Sign::Positive => self.pow_a_sp_multi(rhs),
        }
    }

    fn pow_p_p_multi(self, rhs: Self) -> Vec<Self> {
        assert!(self.sign_class().is_positive());
        assert!(rhs.sign_class().is_positive());

        let precision = self.precision();
        let mut intervals = Vec::<Self>::new();
        if rhs.has_zero() {
            intervals.push(Self::one(self.precision()));
        }
        let (self_01, self_1i) = self.split(BOUND::one(precision));
        if !self_1i.is_nan() {
            intervals.push(Self::new(
                self_1i.lo.pow_lo(rhs.lo.clone()),
                self_1i.hi.pow_hi(rhs.hi.clone()),
            ));
        }
        if !self_01.is_nan() {
            intervals.push(Self::new(
                self_01.lo.pow_lo(rhs.hi),
                self_01.hi.pow_hi(rhs.lo),
            ));
        }
        intervals
    }

    fn pow_p_n_multi(self, rhs: Self) -> Vec<Self> {
        assert!(self.sign_class().is_positive());
        assert!(rhs.sign_class().is_negative());

        let mut pos_intervals = self.pow_p_p_multi(-rhs);
        let res = pos_intervals
            .drain(..)
            .flat_map(|i| Interval::one(i.precision()).div_multi(i))
            .collect();
        res
    }

    fn pow_p_a_multi(self, rhs: Self) -> Vec<Self> {
        assert!(self.sign_class().is_positive());

        let mut intervals = Vec::<Self>::new();
        let precision = rhs.precision();
        let (rhs_n, rhs_p) = rhs.split(BOUND::zero(precision));
        if !rhs_p.is_nan() {
            intervals.append(&mut self.clone().pow_p_p_multi(rhs_p));
        }
        if !rhs_n.is_nan() {
            intervals.append(&mut self.pow_p_n_multi(rhs_n));
        }
        intervals
    }

    fn pow_n_a_multi(self, rhs: Self) -> Vec<Self> {
        assert!(self.sign_class().is_negative());

        let mut intervals = Vec::<Self>::new();
        let mut neg_intervals = (-self).pow_p_a_multi(rhs);
        intervals.append(&mut neg_intervals.iter().map(|i| -i.clone()).collect());
        intervals.append(&mut neg_intervals);
        intervals
    }

    /// Computes `self` raised to the power `rhs` and returns a vector of intervals minimally
    /// covering the result.
    pub fn pow_multi(self, rhs: Self) -> Vec<Self> {
        let precision = self.precision();

        if rhs.is_nan() || self.is_nan() {
            return vec![];
        }
        if rhs.is_zero() {
            return vec![Self::one(precision)];
        }
        if self.is_zero() {
            return if rhs.has_zero() {
                vec![Self::zero(self.precision()), Self::one(self.precision())]
            } else {
                vec![self]
            };
        }
        if rhs.is_singleton() {
            return self.pow_a_s_multi(rhs.hi);
        }

        let mut intervals = Vec::<Self>::new();
        let (self_n, self_p) = self.split(BOUND::zero(precision));
        if !self_p.is_nan() {
            intervals.append(&mut self_p.pow_p_a_multi(rhs.clone()));
        }
        if !self_n.is_nan() {
            intervals.append(&mut self_n.pow_n_a_multi(rhs));
        }
        intervals
    }
}

impl<BOUND: Float> Transc for Interval<BOUND> {
    type Output = Self;

    fn log(self) -> Self::Output {
        match self.sign_class() {
            SignClass::Mixed => Self::new(BOUND::neg_infinity(self.precision()), self.hi.log_hi()),
            SignClass::Zero => Self::nan(self.precision()),
            SignClass::Positive(has_zero) => {
                if has_zero {
                    Self::new(BOUND::neg_infinity(self.precision()), self.hi.log_hi())
                } else {
                    Self::new(self.lo.log_lo(), self.hi.log_hi())
                }
            }
            SignClass::Negative(_) => Self::nan(self.precision()),
        }
    }

    fn exp(self) -> Self::Output {
        Interval::new(self.lo.exp_lo(), self.hi.exp_hi())
    }

    fn pow(self, rhs: Self) -> Self::Output {
        let precision = self.precision();
        Self::minimal_cover(self.pow_multi(rhs), precision)
    }
}
