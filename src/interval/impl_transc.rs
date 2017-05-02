use super::def::{Interval, SignClass};

use fp::Float;

impl<BOUND: Float> Interval<BOUND> {
    fn pow_p_p_multi(self, rhs: Self) -> Vec<Self> {
        assert!(self.sign_class().is_positive());
        assert!(rhs.sign_class().is_positive());

        let precision = self.precision();
        let mut intervals = Vec::<Self>::new();
        let (self_01, self_1i) = self.split(BOUND::one(precision));
        if !self_1i.is_nan() {
            intervals.push(Self::new(
                (self_1i.lo.log_lo().mul_lo(rhs.lo.clone())).exp_lo(),
                (self_1i.hi.log_hi().mul_hi(rhs.hi.clone())).exp_hi(),
            ));
        }
        if !self_01.is_nan() {
            intervals.push(Self::new(
                (self_01.lo.log_lo().mul_lo(rhs.hi)).exp_lo(),
                (self_01.hi.log_hi().mul_hi(rhs.lo)).exp_hi(),
            ));
        }
        intervals
    }

    fn pow_p_n_multi(self, rhs: Self) -> Vec<Self> {
        assert!(self.sign_class().is_positive());
        assert!(rhs.sign_class().is_negative());

        let mut pos_intervals = self.pow_p_p_multi(-rhs);
        let res = pos_intervals.drain(..)
            .flat_map(|i| Interval::one(i.precision()).div_multi(i))
            .collect();
        res
    }

    fn pow_p_a_multi(self, rhs: Self) -> Vec<Self> {
        assert!(self.sign_class().is_positive());

        let mut intervals = Vec::<Self>::new();
        let precision = rhs.precision();
        let (other_n, other_p) = rhs.split(BOUND::zero(precision));
        if !other_p.is_nan() {
            intervals.append(&mut self.clone().pow_p_p_multi(other_p));
        }
        if !other_n.is_nan() {
            intervals.append(&mut self.pow_p_n_multi(other_n));
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

    pub fn pow_multi(self, rhs: Self) -> Vec<Self> {
        let precision = self.precision();

        if self.is_nan() {
            return vec![]
        }
        if rhs.is_nan() {
            return vec![]
        }
        if rhs.is_zero() {
            return vec![Self::one(precision)];
        }
        if self.is_zero() {
            return vec![self];
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

impl<BOUND: Float> Interval<BOUND> {
    pub fn log(self) -> Self {
        match self.sign_class() {
            SignClass::Mixed => Self::new(
                BOUND::neg_infinity(self.precision()),
                self.hi.log_hi(),
            ),
            SignClass::Zero => Self::nan(self.precision()),
            SignClass::Positive(has_zero) => if has_zero {
                Self::new(
                    BOUND::neg_infinity(self.precision()),
                    self.hi.log_hi(),
                )
            } else {
                Self::new(
                    self.lo.log_lo(),
                    self.hi.log_hi(),
                )
            },
            SignClass::Negative(_) => Self::nan(self.precision()),
        }
    }

    pub fn exp(self) -> Self {
        Interval::new(self.lo.exp_lo(), self.hi.exp_hi())
    }

    pub fn pow(self, rhs: Self) -> Self {
        let precision = self.precision();
        Self::minimal_cover(self.pow_multi(rhs), precision)
    }
}
