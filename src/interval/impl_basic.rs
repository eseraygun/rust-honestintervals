use super::def::{Interval, ParseIntervalError, SignClass};

use fp::{Float, Sign};

use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

impl SignClass {
    /// Whether `self` is `SignClass::Positive(_)`.
    pub fn is_positive(&self) -> bool {
        match *self {
            SignClass::Positive(_) => true,
            _ => false,
        }
    }

    /// Whether `self` is `SignClass::Negative(_)`.
    pub fn is_negative(&self) -> bool {
        match *self {
            SignClass::Negative(_) => true,
            _ => false,
        }
    }
}

impl Display for SignClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SignClass::Mixed => f.write_str("m"),
            SignClass::Zero => f.write_str("z"),
            SignClass::Positive(has_zero) => {
                if has_zero {
                    f.write_str("p0")
                } else {
                    f.write_str("p1")
                }
            }
            SignClass::Negative(has_zero) => {
                if has_zero {
                    f.write_str("n0")
                } else {
                    f.write_str("n1")
                }
            }
        }
    }
}

impl<BOUND: Float> Interval<BOUND> {
    /// Constructs a new interval from given bounds.
    ///
    /// Lower bound must be less than or equal to upper bound. Only exception is when they are both
    /// NaNs, in which case a NaN (empty) interval is created.
    ///
    /// Cases where both bounds are negative infinity or positive infinity are not allowed as these
    /// are empty sets. If you want to represent an empty set, use `Interval::nan()`.
    #[inline]
    pub fn new(lo: BOUND, hi: BOUND) -> Self {
        assert_eq!(
            lo.precision(),
            hi.precision(),
            "inconsistent precision: {} != {}",
            lo.precision(),
            hi.precision()
        );
        assert!(
            !lo.is_nan() && !hi.is_nan() && lo <= hi || lo.is_nan() && hi.is_nan(),
            "invalid bounds: <{}, {}>",
            lo,
            hi
        );
        assert!(
            !(lo.is_infinity() && hi.is_infinity())
                && !(lo.is_neg_infinity() && hi.is_neg_infinity()),
            "invalid bounds: <{}, {}>",
            lo,
            hi
        );
        Interval { lo: lo, hi: hi }
    }

    /// Constructs the minimal interval that covers all of the given intervals.
    pub fn minimal_cover(mut intervals: Vec<Self>, precision: usize) -> Self {
        intervals.retain(|i| !i.is_nan());
        if intervals.is_empty() {
            return Self::nan(precision);
        }
        let lo = intervals
            .iter()
            .map(|i| i.lo.clone())
            .fold(BOUND::infinity(precision), |x, y| x.min(y));
        let hi = intervals
            .iter()
            .map(|i| i.hi.clone())
            .fold(BOUND::neg_infinity(precision), |x, y| x.max(y));
        Self { lo: lo, hi: hi }
    }

    /// Constructs a singleton interval (an interval with only one element).
    #[inline]
    pub fn singleton(val: BOUND) -> Self {
        Self::new(val.clone(), val)
    }

    /// Constructs an interval that contains only zero.
    #[inline]
    pub fn zero(precision: usize) -> Self {
        Self::new(BOUND::zero(precision), BOUND::zero(precision))
    }

    /// Constructs an interval that contains only one.
    #[inline]
    pub fn one(precision: usize) -> Self {
        Self::new(BOUND::one(precision), BOUND::one(precision))
    }

    /// Constructs a NaN (empty) interval.
    #[inline]
    pub fn nan(precision: usize) -> Self {
        Self::new(BOUND::nan(precision), BOUND::nan(precision))
    }

    /// Constructs an interval that contains all numbers.
    #[inline]
    pub fn whole(precision: usize) -> Self {
        Self::new(BOUND::neg_infinity(precision), BOUND::infinity(precision))
    }

    /// Constructs an interval from a float with given precision.
    #[inline]
    pub fn from_with_prec(val: f64, precision: usize) -> Self {
        Self::new(
            BOUND::from_lo(val, precision),
            BOUND::from_hi(val, precision),
        )
    }

    /// Constructs an interval by parsing a string.
    ///
    /// Accepts `INTERVAL` according to the rule below.
    ///
    ///   INTERVAL = FLOAT | '<' FLOAT ',' FLOAT '>'
    pub fn from_str_with_prec(s: &str, precision: usize) -> Result<Self, ParseIntervalError> {
        let lo = BOUND::from_str_lo(s, precision);
        let hi = BOUND::from_str_hi(s, precision);
        if let (Ok(lo), Ok(hi)) = (lo, hi) {
            Ok(Self::new(lo, hi))
        } else {
            if !s.starts_with('<') {
                return Err(ParseIntervalError::MissingOpeningBracket);
            }
            let s = s.trim_start_matches('<').trim_start();
            if !s.ends_with('>') {
                return Err(ParseIntervalError::MissingClosingBracket);
            }
            let s = s.trim_end_matches('>').trim_end();
            let p: Vec<&str> = s.split(',').collect();
            if p.len() == 2 {
                let lo = BOUND::from_str_lo(p[0].trim(), precision);
                let hi = BOUND::from_str_hi(p[1].trim(), precision);
                if let (Ok(lo), Ok(hi)) = (lo, hi) {
                    if !lo.is_nan() && !hi.is_nan() && lo <= hi || lo.is_nan() && hi.is_nan() {
                        Ok(Self::new(lo, hi))
                    } else {
                        Err(ParseIntervalError::InvalidBounds)
                    }
                } else {
                    Err(ParseIntervalError::BoundsParseError)
                }
            } else {
                Err(ParseIntervalError::InvalidNumberOfBounds)
            }
        }
    }

    /// Returns the sign class of `self`.
    #[inline]
    pub fn sign_class(&self) -> SignClass {
        match self.lo.sign() {
            Sign::Negative => match self.hi.sign() {
                Sign::Negative => SignClass::Negative(false),
                Sign::Zero => SignClass::Negative(true),
                Sign::Positive => SignClass::Mixed,
            },
            Sign::Zero => match self.hi.sign() {
                Sign::Negative => unreachable!(),
                Sign::Zero => SignClass::Zero,
                Sign::Positive => SignClass::Positive(true),
            },
            Sign::Positive => match self.hi.sign() {
                Sign::Negative => unreachable!(),
                Sign::Zero => unreachable!(),
                Sign::Positive => SignClass::Positive(false),
            },
        }
    }

    /// Returns the precision of `self`.
    #[inline]
    pub fn precision(&self) -> usize {
        assert!(self.lo.precision() == self.hi.precision());
        self.lo.precision()
    }

    /// Returns the difference between the upper bound and the lower bound of `self`.
    ///
    /// As the result is not always exactly representable as `BOUND`, an interval is returned
    /// instead.
    #[inline]
    pub fn size(&self) -> Self {
        if self.is_whole() {
            Self::nan(self.precision())
        } else {
            Self::singleton(self.hi.clone()) - Self::singleton(self.lo.clone())
        }
    }

    /// Whether `self` is a singleton interval (an interval containing only one element).
    #[inline]
    pub fn is_singleton(&self) -> bool {
        self.lo == self.hi
    }

    /// Whether `self` contains only zero.
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.lo.sign() == Sign::Zero && self.hi.sign() == Sign::Zero && !self.is_nan()
    }

    /// Whether `self` is NaN (empty).
    #[inline]
    pub fn is_nan(&self) -> bool {
        assert!(self.lo.is_nan() == self.hi.is_nan());
        self.lo.is_nan()
    }

    /// Whether `self` contains all numbers.
    #[inline]
    pub fn is_whole(&self) -> bool {
        self.lo.is_neg_infinity() && self.hi.is_infinity()
    }

    /// Whether `self` contains zero.
    #[inline]
    pub fn has_zero(&self) -> bool {
        self.lo.sign() <= Sign::Zero && self.hi.sign() >= Sign::Zero
    }

    /// Cuts `self` into two at `val` and returns the left and right pieces as a pair.
    ///
    /// If `self` lies on only one side of `val`, the non-existent side will be a NaN interval.
    #[inline]
    pub fn split(self, val: BOUND) -> (Self, Self) {
        let precision = self.precision();
        if self.lo >= val {
            (Self::nan(precision), self)
        } else if self.hi <= val {
            (self, Self::nan(precision))
        } else {
            if self.is_nan() {
                let precision = self.precision();
                (Self::nan(precision), Self::nan(precision))
            } else {
                (Self::new(self.lo, val.clone()), Self::new(val, self.hi))
            }
        }
    }
}

impl<BOUND: Float> From<f64> for Interval<BOUND> {
    #[inline]
    fn from(val: f64) -> Self {
        Self::from_with_prec(val, 53)
    }
}

impl<BOUND: Float> FromStr for Interval<BOUND> {
    type Err = ParseIntervalError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_with_prec(s, 53)
    }
}

impl<BOUND: Float> Display for Interval<BOUND> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_singleton() || self.is_nan() {
            // `self.lo` may be printed as -0, so we prefer `self.hi`.
            Display::fmt(&self.hi, f)
        } else {
            if let Err(e) = f.write_char('<') {
                return Err(e);
            }
            if let Err(e) = Display::fmt(&self.lo, f) {
                return Err(e);
            }
            if let Err(e) = f.write_str(", ") {
                return Err(e);
            }
            if let Err(e) = Display::fmt(&self.hi, f) {
                return Err(e);
            }
            f.write_char('>')
        }
    }
}

impl<BOUND: Float> Into<(BOUND, BOUND)> for Interval<BOUND> {
    fn into(self) -> (BOUND, BOUND) {
        (self.lo, self.hi)
    }
}
