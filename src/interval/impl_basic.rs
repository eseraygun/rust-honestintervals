use super::def::{Interval, ParseIntervalError, SignClass};

use fp::{Float, Sign};

use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

impl SignClass {
    pub fn is_positive(&self) -> bool {
        match *self {
            SignClass::Positive(_) => true,
            _ => false,
        }
    }

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
            SignClass::Positive(has_zero) => if has_zero {
                f.write_str("p0")
            } else {
                f.write_str("p1")
            },
            SignClass::Negative(has_zero) => if has_zero {
                f.write_str("n0")
            } else {
                f.write_str("n1")
            },
        }
    }
}

impl<BOUND: Float> Interval<BOUND> {
    #[inline]
    pub fn new(lo: BOUND, hi: BOUND) -> Self {
        assert_eq!(lo.precision(), hi.precision(),
                   "inconsistent precision: {} != {}", lo.precision(), hi.precision());
        assert!(!lo.is_nan() && !hi.is_nan() && lo <= hi || lo.is_nan() && hi.is_nan(),
                "invalid bounds: <{}, {}>", lo, hi);
        assert!(!(lo.is_infinity() && hi.is_infinity()) && !(lo.is_neg_infinity() && hi.is_neg_infinity()),
                "invalid bounds: <{}, {}>", lo, hi);
        Interval { lo: lo, hi: hi }
    }

    pub fn minimal_cover(mut intervals: Vec<Self>, precision: usize) -> Self {
        intervals.retain(|i| !i.is_nan());
        if intervals.is_empty() {
            return Self::nan(precision)
        }
        let lo = intervals.iter()
            .map(|i| i.lo.clone())
            .fold(BOUND::infinity(precision), |x, y| x.min(y));
        let hi = intervals.iter()
            .map(|i| i.hi.clone())
            .fold(BOUND::neg_infinity(precision), |x, y| x.max(y));
        Self { lo: lo, hi: hi }
    }

    #[inline]
    pub fn singleton(val: BOUND) -> Self {
        Self::new(val.clone(), val)
    }

    #[inline]
    pub fn zero(precision: usize) -> Self {
        Self::new(BOUND::zero(precision), BOUND::zero(precision))
    }

    #[inline]
    pub fn one(precision: usize) -> Self {
        Self::new(BOUND::one(precision), BOUND::one(precision))
    }

    #[inline]
    pub fn nan(precision: usize) -> Self {
        Self::new(BOUND::nan(precision), BOUND::nan(precision))
    }

    #[inline]
    pub fn whole(precision: usize) -> Self {
        Self::new(BOUND::neg_infinity(precision), BOUND::infinity(precision))
    }

    pub fn from_str_with_prec(s: &str, precision: usize) -> Result<Self, ParseIntervalError> {
        let lo = BOUND::from_str_lo(s, precision);
        let hi = BOUND::from_str_hi(s, precision);
        if let (Ok(lo), Ok(hi)) = (lo, hi) {
            Ok(Self::new(lo, hi))
        } else {
            if !s.starts_with('<') { return Err(ParseIntervalError::MissingOpeningBracket) }
            let s = s.trim_left_matches('<').trim_left();
            if !s.ends_with('>') { return Err(ParseIntervalError::MissingClosingBracket) }
            let s = s.trim_right_matches('>').trim_right();
            let p: Vec<&str> = s.split(',').collect();
            if p.len() == 2 {
                let lo = BOUND::from_str_lo(p[0], precision);
                let hi = BOUND::from_str_hi(p[1], precision);
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

    #[inline]
    pub fn precision(&self) -> usize {
        assert!(self.lo.precision() == self.hi.precision());
        self.lo.precision()
    }

    #[inline]
    pub fn size(&self) -> Self {
        if self.is_whole() {
            Self::nan(self.precision())
        } else {
            Self::singleton(self.hi.clone()) - Self::singleton(self.lo.clone())
        }
    }

    #[inline]
    pub fn is_singleton(&self) -> bool {
        self.lo == self.hi
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.lo.sign() == Sign::Zero && self.hi.sign() == Sign::Zero && !self.is_nan()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        assert!(self.lo.is_nan() == self.hi.is_nan());
        self.lo.is_nan()
    }

    #[inline]
    pub fn is_whole(&self) -> bool {
        self.lo.is_neg_infinity() && self.hi.is_infinity()
    }

    #[inline]
    pub fn has_zero(&self) -> bool {
        self.lo.sign() <= Sign::Zero && self.hi.sign() >= Sign::Zero
    }

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
        Self::singleton(BOUND::from(val))
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
            Display::fmt(&self.lo, f)
        } else {
            if let Err(e) = f.write_char('<') { return Err(e) }
            if let Err(e) = Display::fmt(&self.lo, f) { return Err(e) }
            if let Err(e) = f.write_str(", ") { return Err(e) }
            if let Err(e) = Display::fmt(&self.hi, f) { return Err(e) }
            f.write_char('>')
        }
    }
}

impl<BOUND> Into<(BOUND, BOUND)> for Interval<BOUND> {
    fn into(self) -> (BOUND, BOUND) {
        (self.lo, self.hi)
    }
}
