use float::{Float, RoundingMode};
use interval::{Interval};

use std::fmt::{Debug, Display, Formatter, Result, Write};
use std::ops::{Add, Div, Mul, Sub};

macro_rules! interval_set {
    {$($v:tt);*} => { IntervalSet::<Mpfr>::from_intervals(vec![$(interval!$v),*]) };
}

pub struct IntervalSet<F> {
    intervals: Vec<Interval<F>>,
}

impl<F: Float + Clone + Display + PartialOrd + Ord> Display for IntervalSet<F> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.intervals.is_empty() {
            f.write_str("{}")
        } else if self.intervals.len() == 1 {
            self.intervals[0].fmt(f)
        } else {
            if let Err(e) = f.write_char('{') { return Err(e) }
            let mut iter = self.intervals.iter();
            if let Err(e) = iter.next().unwrap().fmt(f) { return Err(e) }
            for i in iter {
                if let Err(e) = f.write_str("; ") { return Err(e) }
                if let Err(e) = i.fmt(f) { return Err(e) }
            }
            f.write_char('}')
        }
    }
}

impl<F: Float + Clone + PartialOrd + Ord> IntervalSet<F> {
    #[inline]
    pub fn new(lo: F, hi: F) -> Self {
        IntervalSet {
            intervals: vec![Interval::new(lo, hi)],
        }
    }

    #[inline]
    pub fn zero(precision: usize) -> Self {
        IntervalSet {
            intervals: vec![Interval::zero(precision)],
        }
    }

    #[inline]
    pub fn whole(precision: usize) -> Self {
        IntervalSet {
            intervals: vec![Interval::whole(precision)],
        }
    }

    #[inline]
    pub fn empty() -> Self {
        IntervalSet {
            intervals: vec![],
        }
    }

    #[inline]
    pub fn from_interval(i: Interval<F>) -> Self {
        IntervalSet {
            intervals: vec![i],
        }
    }

    #[inline]
    pub fn from_intervals(mut intervals: Vec<Interval<F>>) -> Self {
        if intervals.is_empty() {
            return IntervalSet::empty()
        }

        intervals.sort_by(|i, j| {
            i.lo().cmp(j.lo())
        });
        let mut iter = intervals.drain(..);
        let first = iter.next().unwrap();
        let (mut lo, mut hi) = first.decompose();
        let mut optimized_intervals = Vec::<Interval<F>>::new();
        for i in iter {
            let (i_lo, i_hi) = i.decompose();
            if hi < i_lo {
                optimized_intervals.push(Interval::new(lo, hi));
                lo = i_lo;
                hi = i_hi;
            } else if hi < i_hi {
                hi = i_hi;
            }
        }
        optimized_intervals.push(Interval::new(lo, hi));
        IntervalSet {
            intervals: optimized_intervals,
        }
    }

    #[inline]
    pub fn from_str(val: &str, base: usize, precision: usize) -> Option<Self> {
        if let Some(i) = Interval::from_str(val, base, precision) {
            Some(IntervalSet::from_interval(i))
        } else {
            let val = val.trim_left().trim_left_matches('{').trim_left();
            let val = val.trim_right().trim_right_matches('}').trim_right();
            Some(IntervalSet::from_intervals(val.split(';')
                .filter_map(|v| Interval::from_str(v, base, precision)).collect()))
        }
    }

    #[inline]
    pub fn has_zero(&self) -> bool {
        self.intervals.iter().any(|i| i.has_zero())
    }

    #[inline]
    pub fn binary_op<O>(&self, other: &Self, op: O) -> Self
        where O: Fn(&Interval<F>, &Interval<F>) -> Vec<Interval<F>>
    {
        let mut intervals = Vec::<Interval<F>>::new();
        for i in &self.intervals {
            for j in &other.intervals {
                intervals.append(&mut op(i, j));
            }
        }
        IntervalSet::from_intervals(intervals)
    }
}

impl<F: Clone> IntervalSet<F> {
    #[inline]
    pub fn intervals(&self) -> Vec<Interval<F>> {
        self.intervals.to_vec()
    }
}

impl<'a, 'b, F: Float + Clone + Ord> Add<&'b IntervalSet<F>> for &'a IntervalSet<F> {
    type Output = IntervalSet<F>;

    #[inline]
    fn add(self, other: &'b IntervalSet<F>) -> Self::Output {
        self.binary_op(other, |i, j| { vec![i + j] })
    }
}

impl<'a, 'b, F: Float + Clone + Ord> Sub<&'b IntervalSet<F>> for &'a IntervalSet<F> {
    type Output = IntervalSet<F>;

    #[inline]
    fn sub(self, other: &'b IntervalSet<F>) -> Self::Output {
        self.binary_op(other, |i, j| { vec![i - j] })
    }
}

fn mul_pp<F: Float>(x: &Interval<F>, y: &Interval<F>) -> Interval<F> {
    Interval {
        lo: F::mul(&x.lo, &y.lo, RoundingMode::Down),
        hi: F::mul(&x.hi, &y.hi, RoundingMode::Up),
    }
}

fn mul<F: Float + Clone + PartialOrd + Ord>(x: &Interval<F>, y: &Interval<F>) -> Vec<Interval<F>> {
    let (xn, xp) = x.split_at(&F::zero(x.precision()));
    let (yn, yp) = y.split_at(&F::zero(x.precision()));
    let mut intervals = Vec::<Interval<F>>::new();
    if let Some(ref xn) = xn {
        if let Some(ref yn) = yn { intervals.push(mul_pp(&-xn, &-yn)); }
        if let Some(ref yp) = yp { intervals.push(-&mul_pp(&-xn, yp)); }
    }
    if let Some(ref xp) = xp {
        if let Some(ref yn) = yn { intervals.push(-&mul_pp(xp, &-yn)); }
        if let Some(ref yp) = yp { intervals.push(mul_pp(xp, yp)); }
    }
    intervals
}

impl<'a, 'b, F: Float + Clone + Ord> Mul<&'b IntervalSet<F>> for &'a IntervalSet<F> {
    type Output = IntervalSet<F>;

    #[inline]
    fn mul(self, other: &'b IntervalSet<F>) -> Self::Output {
        self.binary_op(other, |i, j| { mul(i, j) })
    }
}

#[inline]
fn div_pp<F: Float + Clone + PartialOrd + Ord>(x: &Interval<F>, y: &Interval<F>, negate: bool) -> Vec<Interval<F>> {
    let z = Interval {
        lo: F::div(&x.lo, &F::abs(&y.hi), RoundingMode::Down),
        hi: F::div(&x.hi, &F::abs(&y.lo), RoundingMode::Up),
    };
    if z.is_well_defined() {
        if negate {
            vec![-&z]
        } else {
            vec![z]
        }
    } else {
        vec![]
    }
}

#[inline]
fn div<F: Float + Clone + PartialOrd + Ord>(x: &Interval<F>, y: &Interval<F>) -> Vec<Interval<F>> {
    let (xn, xp) = x.split_at(&F::zero(x.precision()));
    let (yn, yp) = y.split_at(&F::zero(x.precision()));
    let mut intervals = Vec::<Interval<F>>::new();
    if let Some(ref xn) = xn {
        if let Some(ref yn) = yn { intervals.append(&mut div_pp(&-xn, &-yn, false)); }
        if let Some(ref yp) = yp { intervals.append(&mut div_pp(&-xn, yp, true)); }
    }
    if let Some(ref xp) = xp {
        if let Some(ref yn) = yn { intervals.append(&mut div_pp(xp, &-yn, true)); }
        if let Some(ref yp) = yp { intervals.append(&mut div_pp(xp, yp, false)); }
    }
    intervals
}

impl<'a, 'b, F: Float + Display + Clone + Ord> Div<&'b IntervalSet<F>> for &'a IntervalSet<F> {
    type Output = IntervalSet<F>;

    #[inline]
    fn div(self, other: &'b IntervalSet<F>) -> Self::Output {
        self.binary_op(other, |i, j| { div(i, j) })
    }
}

#[cfg(test)]
mod test {
    use mpfr::{Mpfr};
    use interval::{Interval};
    use intervalset::{IntervalSet};

    macro_rules! assert_str_eq {
        ($x:expr, $y:expr) => { assert_eq!($x, format!("{}", $y)) };
    }

    #[test]
    fn test_from_intervals() {
        assert_str_eq!("{}", interval_set!{});
        assert_str_eq!("<-2, -1>", interval_set!{[-2, -1]});
        assert_str_eq!("{<-2, -1>; <1, 2>}", interval_set!{[-2, -1]; [1, 2]});
        assert_str_eq!("<-2, 2>", interval_set!{[-2, 1]; [1, 2]});
        assert_str_eq!("<-2, 2>", interval_set!{[-1, 1]; [-2, 2]});
    }

    #[test]
    fn test_add() {
        assert_str_eq!("<1, 2>", &interval_set!{[0, 1]} + &interval_set!{[1]});
        assert_str_eq!("{<-2, 0>; <1, 2>}", &interval_set!{[0, 1]} + &interval_set!{[1]; [-2, -1]});
    }

    #[test]
    fn test_sub() {
        assert_str_eq!("<-1, 0>", &interval_set!{[0, 1]} - &interval_set!{[1]});
        assert_str_eq!("{<-1, 0>; <1, 3>}", &interval_set!{[0, 1]} - &interval_set!{[1]; [-2, -1]});
    }

    #[test]
    fn test_mul_mm() {
        let i = interval_set!{[-1, 2]; [-3, -2]};
        let j = interval_set!{[-2, 1]; [3, 4]};
        let k = &i * &j;
        assert_str_eq!("{<-12, -6>; <-4, 8>}", k);
    }

    #[test]
    fn test_mul_mn() {
        let i = interval_set!{[-1, 2]};
        let j = interval_set!{[-2, -1]};
        let k = &i * &j;
        assert_str_eq!("<-4, 2>", k);
    }

    #[test]
    fn test_mul_mp() {
        let i = interval_set!{[-1, 2]};
        let j = interval_set!{[1, 2]};
        let k = &i * &j;
        assert_str_eq!("<-2, 4>", k);
    }

    #[test]
    fn test_mul_mz() {
        let i = interval_set!{[-1, 2]};
        let j = interval_set!{[0]};
        let k = &i * &j;
        assert_str_eq!("0", k);
    }

    #[test]
    fn test_mul_nm() {
        let i = interval_set!{[-2, -1]};
        let j = interval_set!{[-2, 1]};
        let k = &i * &j;
        assert_str_eq!("<-2, 4>", k);
    }

    #[test]
    fn test_mul_nn() {
        let i = interval_set!{[-2, -1]};
        let j = interval_set!{[-2, -1]};
        let k = &i * &j;
        assert_str_eq!("<1, 4>", k);
    }

    #[test]
    fn test_mul_np() {
        let i = interval_set!{[-2, -1]};
        let j = interval_set!{[1, 2]};
        let k = &i * &j;
        assert_str_eq!("<-4, -1>", k);
    }

    #[test]
    fn test_mul_nz() {
        let i = interval_set!{[-2, -1]};
        let j = interval_set!{[0]};
        let k = &i * &j;
        assert_str_eq!("0", k);
    }

    #[test]
    fn test_mul_pm() {
        let i = interval_set!{[1, 2]};
        let j = interval_set!{[-2, 1]};
        let k = &i * &j;
        assert_str_eq!("<-4, 2>", k);
    }

    #[test]
    fn test_mul_pn() {
        let i = interval_set!{[1, 2]};
        let j = interval_set!{[-2, -1]};
        let k = &i * &j;
        assert_str_eq!("<-4, -1>", k);
    }

    #[test]
    fn test_mul_pp() {
        let i = interval_set!{[1, 2]};
        let j = interval_set!{[1, 2]};
        let k = &i * &j;
        assert_str_eq!("<1, 4>", k);
    }

    #[test]
    fn test_mul_pz() {
        let i = interval_set!{[1, 2]};
        let j = interval_set!{[0]};
        let k = &i * &j;
        assert_str_eq!("0", k);
    }

    #[test]
    fn test_mul_za() {
        let i = interval_set!{[0]};
        let j = interval_set!{[-2, 1]};
        let k = &i * &j;
        assert_str_eq!("0", k);
    }

    #[test]
    fn test_div_mm() {
        let i = interval_set!{[-1, 2]};
        let j = interval_set!{[-2, 1]};
        let k = &i / &j;
        assert_str_eq!("<-inf, inf>", k);
    }

    #[test]
    fn test_div_mn() {
        let i = interval_set!{[-1, 2]};
        let j = interval_set!{[-2, -1]};
        let k = &i / &j;
        assert_str_eq!("<-2, 1>", k);
    }

    #[test]
    fn test_div_mn0() {
        let i = interval_set!{[-1, 2]};
        let j = interval_set!{[-2, 0]};
        let k = &i / &j;
        assert_str_eq!("<-inf, inf>", k);
    }

    #[test]
    fn test_div_mp() {
        let i = interval_set!{[-1, 2]};
        let j = interval_set!{[1, 2]};
        let k = &i / &j;
        assert_str_eq!("<-1, 2>", k);
    }

    #[test]
    fn test_div_mp0() {
        let i = interval_set!{[-1, 2]};
        let j = interval_set!{[0, 2]};
        let k = &i / &j;
        assert_str_eq!("<-inf, inf>", k);
    }

    #[test]
    fn test_div_mz() {
        let i = interval_set!{[-1, 2]};
        let j = interval_set!{[0]};
        let k = &i / &j;
        assert_str_eq!("{}", k);
    }

    #[test]
    fn test_div_nn() {
        let i = interval_set!{[-2, -1]};
        let j = interval_set!{[-2, -1]};
        let k = &i / &j;
        assert_str_eq!("<0.5, 2>", k);
    }

    #[test]
    fn test_div_nn0() {
        let i = interval_set!{[-2, -1]};
        let j = interval_set!{[-2, 0]};
        let k = &i / &j;
        assert_str_eq!("<0.5, inf>", k);
    }

//    #[test]
//    fn test_div_nz() {
//        let i = interval_set!{[-2, -1]};
//        let j = interval_set!{[0]};
//        let k = &i / &j;
//        assert!(k.is_nan());
//    }
//
//    #[test]
//    fn test_div_np() {
//        let i = interval_set!{[-2, -1]};
//        let j = interval_set!{[1, 2]};
//        let k = &i / &j;
//        assert_eq!(-2.0, k.lo.into());
//        assert_eq!(-0.5, k.hi.into());
//    }
//
//    #[test]
//    fn test_div_np0() {
//        let i = interval_set!{[-2, -1]};
//        let j = interval_set!{[0, 2]};
//        let k = &i / &j;
//        assert!(k.lo.is_neg_infinity());
//        assert_eq!(-0.5, k.hi.into());
//    }

    #[test]
    fn test_div_p1m() {
        let i = interval_set!{[1, 2]};
        let j = interval_set!{[-2, 1]};
        let k = &i / &j;
        assert_str_eq!("{<-inf, -0.5>; <1, inf>}", k);
    }

//    #[test]
//    fn test_div_pn() {
//        let i = interval_set!{[1, 2]};
//        let j = interval_set!{[-2, -1]};
//        let k = &i / &j;
//        assert_eq!(-2.0, k.lo.into());
//        assert_eq!(-0.5, k.hi.into());
//    }
//
//    #[test]
//    fn test_div_pn0() {
//        let i = interval_set!{[1, 2]};
//        let j = interval_set!{[-2, 0]};
//        let k = &i / &j;
//        assert!(k.lo.is_neg_infinity());
//        assert_eq!(-0.5, k.hi.into());
//    }
//
//    #[test]
//    fn test_div_pz() {
//        let i = interval_set!{[1, 2]};
//        let j = interval_set!{[0]};
//        let k = &i / &j;
//        assert!(k.is_nan());
//    }
//
//    #[test]
//    fn test_div_pp() {
//        let i = interval_set!{[1, 2]};
//        let j = interval_set!{[1, 2]};
//        let k = &i / &j;
//        assert_eq!(0.5, k.lo.into());
//        assert_eq!(2.0, k.hi.into());
//    }
//
//    #[test]
//    fn test_div_pp0() {
//        let i = interval_set!{[1, 2]};
//        let j = interval_set!{[0, 2]};
//        let k = &i / &j;
//        assert_eq!(0.5, k.lo.into());
//        assert!(k.hi.is_pos_infinity());
//    }
//
//    #[test]
//    fn test_div_zm() {
//        let i = interval_set!{[0]};
//        let j = interval_set!{[-2, 1]};
//        let k = &i / &j;
//        assert!(k.is_zero());
//    }
//
//    #[test]
//    fn test_div_zz() {
//        let i = interval_set!{[0]};
//        let j = interval_set!{[0]};
//        let k = &i / &j;
//        assert!(k.is_nan());
//    }
}
