use super::def::{IntervalSet, ParseIntervalSetError};

use fp::Float;
use interval::Interval;

use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

impl<BOUND: Float> IntervalSet<BOUND> {
    /// Constructs an interval set of one interval from given bounds.
    ///
    /// The result may be empty if both bounds are NaN.
    #[inline]
    pub fn new(lo: BOUND, hi: BOUND) -> Self {
        Self::from_interval(Interval::new(lo, hi))
    }

    /// Constructs an interval set of one interval from given interval.
    ///
    /// The result may be empty if the interval is NaN.
    #[inline]
    pub fn from_interval(i: Interval<BOUND>) -> Self {
        if i.is_nan() {
            Self::empty()
        } else {
            Self { intervals: vec![i] }
        }
    }

    /// Constructs an interval set from given intervals.
    ///
    /// The intervals will be sorted and the intersecting intervals will be merged.
    pub fn from_intervals(mut intervals: Vec<Interval<BOUND>>) -> Self {
        intervals.retain(|i| !i.is_nan());
        if intervals.is_empty() {
            return Self::empty();
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
        Self {
            intervals: optimized_intervals,
        }
    }

    /// Constructs an interval set of one singleton interval.
    ///
    /// The result may be empty if the value is NaN.
    #[inline]
    pub fn singleton(val: BOUND) -> Self {
        Self::from_interval(Interval::singleton(val))
    }

    /// Constructs an interval set of one interval containing only zero.
    #[inline]
    pub fn zero(precision: usize) -> Self {
        Self::from_interval(Interval::zero(precision))
    }

    /// Constructs an interval set of one interval containing only one.
    #[inline]
    pub fn one(precision: usize) -> Self {
        Self::from_interval(Interval::one(precision))
    }

    /// Constructs an empty interval set.
    #[inline]
    pub fn empty() -> Self {
        Self { intervals: vec![] }
    }

    /// Constructs an interval set of one interval containing all numbers.
    #[inline]
    pub fn whole(precision: usize) -> Self {
        Self::from_interval(Interval::whole(precision))
    }

    /// Constructs an interval set from a float with given precision.
    #[inline]
    pub fn from_with_prec(val: f64, precision: usize) -> Self {
        Self::new(
            BOUND::from_lo(val, precision),
            BOUND::from_hi(val, precision),
        )
    }

    /// Constructs an interval set by parsing a string.
    ///
    /// Accepts `INTERVAL_SET` according to the rule below.
    ///
    ///   INTERVAL_SET = INTERVAL | '{' ( INTERVAL ( ';' INTERVAL )* )? '}'
    #[inline]
    pub fn from_str_with_prec(s: &str, precision: usize) -> Result<Self, ParseIntervalSetError> {
        if let Ok(i) = Interval::from_str_with_prec(s, precision) {
            Ok(IntervalSet::from_interval(i))
        } else {
            if !s.starts_with('{') {
                return Err(ParseIntervalSetError::MissingOpeningBraces);
            }
            let s = s.trim_left_matches('{').trim_left();
            if !s.ends_with('}') {
                return Err(ParseIntervalSetError::MissingClosingBraces);
            }
            let s = s.trim_right_matches('}').trim_right();
            if s.is_empty() {
                return Ok(Self::empty());
            }
            let mut results: Vec<_> = s
                .split(';')
                .map(|v| v.trim())
                .map(|v| Interval::from_str_with_prec(v, precision))
                .collect();
            if results.iter().all(|i| i.is_ok()) {
                Ok(Self::from_intervals(
                    results.drain(..).map(|i| i.unwrap()).collect(),
                ))
            } else {
                Err(ParseIntervalSetError::IntervalsParseError)
            }
        }
    }

    /// Whether `self` contains only one interval that is singleton.
    #[inline]
    pub fn is_singleton(&self) -> bool {
        self.intervals.len() == 1 && self.intervals[0].is_singleton()
    }

    /// Whether `self` contains only one interval that contains only zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.intervals.len() == 1 && self.intervals[0].is_zero()
    }

    /// Whether `self` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    /// Whether `self` contains an interval that contains zero.
    #[inline]
    pub fn has_zero(&self) -> bool {
        self.intervals.iter().any(|i| i.has_zero())
    }

    /// Performs a binary operation by performing it on all pairs of intervals of `self` and `rhs`.
    #[inline]
    pub fn binary_op<OP>(self, rhs: Self, op: OP) -> Self
    where
        OP: Fn(Interval<BOUND>, Interval<BOUND>) -> Vec<Interval<BOUND>>,
    {
        let mut intervals = Vec::<Interval<BOUND>>::new();
        for i in &self.intervals {
            for j in &rhs.intervals {
                intervals.append(&mut op(i.clone(), j.clone()));
            }
        }
        Self::from_intervals(intervals)
    }
}

impl<BOUND: Float> From<f64> for IntervalSet<BOUND> {
    #[inline]
    fn from(val: f64) -> Self {
        Self::from_with_prec(val, 53)
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
            Display::fmt(&self.intervals[0], f)
        } else {
            if let Err(e) = f.write_char('{') {
                return Err(e);
            }
            let mut iter = self.intervals.iter();
            if let Err(e) = Display::fmt(&iter.next().unwrap(), f) {
                return Err(e);
            }
            for i in iter {
                if let Err(e) = f.write_str("; ") {
                    return Err(e);
                }
                if let Err(e) = Display::fmt(&i, f) {
                    return Err(e);
                }
            }
            f.write_char('}')
        }
    }
}

impl<BOUND: Float> Into<Vec<(BOUND, BOUND)>> for IntervalSet<BOUND> {
    fn into(mut self) -> Vec<(BOUND, BOUND)> {
        self.intervals.drain(..).map(|i| (i.lo, i.hi)).collect()
    }
}
