use float::{Float, RoundingMode, Sign};
use interval::Interval;
use intervalset::IntervalSet;

impl<BOUND: Float> Interval<BOUND> {
    fn pow_zp_zp_multi(self, other: Self) -> Vec<Self> {
        assert!(self.lo.sign() >= Sign::Zero);
        assert!(other.lo.sign() >= Sign::Zero);

        let mut intervals = Vec::<Self>::new();
        let precision = self.precision();
        let (self_01, self_1i) = self.split(BOUND::one(precision));
        if let Some(self_1i) = self_1i {
            intervals.push(Self::new(
                (self_1i.lo.log(RoundingMode::Down).mul(&other.lo, RoundingMode::Down)).exp(RoundingMode::Down),
                (self_1i.hi.log(RoundingMode::Up).mul(&other.hi, RoundingMode::Up)).exp(RoundingMode::Up),
            ));
        }
        if let Some(self_01) = self_01 {
            intervals.push(Self::new(
                (self_01.lo.log(RoundingMode::Down).mul(&other.hi, RoundingMode::Down)).exp(RoundingMode::Down),
                (self_01.hi.log(RoundingMode::Up).mul(&other.lo, RoundingMode::Up)).exp(RoundingMode::Up),
            ));
        }
        intervals
    }

    fn pow_zp_nz_multi(self, other: Self) -> Vec<Self> {
        assert!(self.lo.sign() >= Sign::Zero);
        assert!(other.hi.sign() <= Sign::Zero);

        let mut pos_intervals = self.pow_zp_zp_multi(-other);
        let ret = pos_intervals.drain(..).flat_map(|i| Interval::one(i.precision()).div_multi(i)).collect();
        ret
    }

    fn pow_zp_a_multi(self, other: Self) -> Vec<Self> {
        assert!(self.lo.sign() >= Sign::Zero);

        let mut intervals = Vec::<Self>::new();
        let precision = other.precision();
        let (other_n, other_p) = other.split(BOUND::zero(precision));
        if let Some(other_p) = other_p {
            intervals.append(&mut self.clone().pow_zp_zp_multi(other_p));
        }
        if let Some(other_n) = other_n {
            intervals.append(&mut self.pow_zp_nz_multi(other_n));
        }
        intervals
    }

    fn pow_nz_a_multi(self, other: Self) -> Vec<Self> {
        assert!(self.hi.sign() <= Sign::Zero);

        let mut intervals = Vec::<Self>::new();
        let mut neg_intervals = (-self).pow_zp_a_multi(other);
        intervals.append(&mut neg_intervals.iter().map(|i| -i).collect());
        intervals.append(&mut neg_intervals);
        intervals
    }

    fn pow_multi(self, other: Self) -> Vec<Self> {
        let mut intervals = Vec::<Self>::new();
        let precision = self.precision();
        let (self_n, self_p) = self.split(BOUND::zero(precision));
        if let Some(self_p) = self_p {
            intervals.append(&mut self_p.pow_zp_a_multi(other.clone()));
        }
        if let Some(self_n) = self_n {
            intervals.append(&mut self_n.pow_nz_a_multi(other));
        }
        intervals
    }
}

impl<BOUND: Float> IntervalSet<BOUND> {
    pub fn log(mut self) -> Self {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.clone().log()).collect())
    }

    pub fn exp(mut self) -> Self {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.exp()).collect())
    }

    pub fn pow(self, other: Self) -> Self {
        self.binary_op(other, |i, j| i.pow_multi(j))
    }
}

#[cfg(test)]
mod test {
    use float::{Float, RoundingMode};
    use interval::Interval;
    use intervalset::IntervalSet;
    use mpfr::Mpfr;

    #[test]
    fn test_log() {
        assert_str_eq!("<0, 0.6931471805599454>", interval_set!{[0]; [1, 2]}.log());
    }

    #[test]
    fn test_exp() {
        assert_str_eq!("{1; <2.718281828459045, 7.38905609893065>}", interval_set!{[0]; [1, 2]}.exp());
    }

    #[test]
    fn test_pow() {
        assert_str_eq!("<1, 4.000000000000001>", interval_set!{[0]; [1, 2]}.pow(interval_set!{[0]; [1, 2]}));
    }

    #[test]
    fn test_pow_ba() {
        assert_str_eq!("<1, 4.000000000000001>", interval_set!{[-2, 2]}.pow(interval_set!{[-2, 2]}));
    }
}
