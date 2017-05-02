use super::def::{IntervalSet, ParseIntervalSetError};

use fp::Float;
use interval::{Interval, ParseIntervalError};

use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

impl<BOUND: Float> IntervalSet<BOUND> {
    #[inline]
    pub fn new(lo: BOUND, hi: BOUND) -> Self {
        Self { intervals: vec![Interval::new(lo, hi)] }
    }

    #[inline]
    pub fn from_interval(i: Interval<BOUND>) -> Self {
        if i.is_nan() {
            Self::empty()
        } else {
            Self { intervals: vec![i] }
        }
    }

    pub fn from_intervals(mut intervals: Vec<Interval<BOUND>>) -> Self {
        intervals.retain(|i| !i.is_nan());
        if intervals.is_empty() {
            return Self::empty()
        }
        intervals.sort_by(|i, j| {
            if i.lo < j.lo {
                Ordering::Less
            } else if i.lo > j.lo {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        let mut iter = intervals.drain(..);
        let first = iter.next().unwrap();
        let (mut lo, mut hi) = first.into();
        let mut optimized_intervals = Vec::<Interval<BOUND>>::new();
        for i in iter {
            let (i_lo, i_hi) = i.into();
            if hi < i_lo {
                optimized_intervals.push(Interval::new(lo, hi));
                lo = i_lo;
                hi = i_hi;
            } else if hi < i_hi {
                hi = i_hi;
            }
        }
        optimized_intervals.push(Interval::new(lo, hi));
        Self { intervals: optimized_intervals }
    }

    #[inline]
    pub fn singleton(val: BOUND) -> Self {
        Self::from_interval(Interval::singleton(val))
    }

    #[inline]
    pub fn zero(precision: usize) -> Self {
        Self::from_interval(Interval::zero(precision))
    }

    #[inline]
    pub fn one(precision: usize) -> Self {
        Self::from_interval(Interval::one(precision))
    }

    #[inline]
    pub fn empty() -> Self {
        Self { intervals: vec![] }
    }

    #[inline]
    pub fn whole(precision: usize) -> Self {
        Self::from_interval(Interval::whole(precision))
    }

    #[inline]
    pub fn from_str_with_prec(s: &str, precision: usize) -> Result<Self, ParseIntervalSetError> {
        if let Ok(i) = Interval::from_str_with_prec(s, precision) {
            Ok(IntervalSet::from_interval(i))
        } else {
            if !s.starts_with('{') { return Err(ParseIntervalSetError::MissingOpeningBraces) }
            let s = s.trim_left_matches('{').trim_left();
            if !s.ends_with('}') { return Err(ParseIntervalSetError::MissingClosingBraces) }
            let s = s.trim_right_matches('}').trim_right();
            let mut intervals = s.split(';').map(|v| Interval::from_str_with_prec(v, precision))
                .collect::<Vec<Result<Interval<BOUND>, ParseIntervalError>>>();
            if intervals.iter().all(|i| i.is_ok()) {
                Ok(IntervalSet::from_intervals(intervals.drain(..).map(|i| i.unwrap()).collect()))
            } else {
                Err(ParseIntervalSetError::IntervalsParseError)
            }
        }
    }

    #[inline]
    pub fn is_singleton(&self) -> bool {
        self.intervals.len() == 1 && self.intervals[0].is_singleton()
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.intervals.len() == 1 && self.intervals[0].is_zero()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    #[inline]
    pub fn has_zero(&self) -> bool {
        self.intervals.iter().any(|i| i.has_zero())
    }

    #[inline]
    pub fn binary_op<OP>(self, other: Self, op: OP) -> Self
        where OP: Fn(Interval<BOUND>, Interval<BOUND>) -> Vec<Interval<BOUND>>
    {
        let mut intervals = Vec::<Interval<BOUND>>::new();
        for i in &self.intervals {
            for j in &other.intervals {
                intervals.append(&mut op(i.clone(), j.clone()));
            }
        }
        Self::from_intervals(intervals)
    }
}

impl<BOUND: Float> From<f64> for IntervalSet<BOUND> {
    #[inline]
    fn from(val: f64) -> Self {
        Self::singleton(BOUND::from(val))
    }
}

impl<BOUND: Float> FromStr for IntervalSet<BOUND> {
    type Err = ParseIntervalSetError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, ParseIntervalSetError> {
        Self::from_str_with_prec(s, 53)
    }
}

impl<BOUND: Float> Display for IntervalSet<BOUND> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
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
