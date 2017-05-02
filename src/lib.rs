#![deny(missing_docs)]

//! Honest Intervals is an interval arithmetic library with correct rounding.
//!
//! It implements elementary arithmetic (addition, subtraction, multiplication and division) as well
//! as complicated mathematical functions such as logarithm and power over intervals and interval
//! sets. Bounds of the return values are always correctly rounded up or down to ensure that all
//! possible results are contained.
//!
//! In addition to the `Interval` and `IntervalSet` structs, the library also provides the `Mpfr`
//! struct that wraps the GNU MPFR library. The `Mpfr` struct is an ideal (and currently only)
//! bound type for intervals.
//!
//! Honest Intervals tries to be a pragmatic implementation of interval arithmetic rather than an
//! abstract basis for all possible implementations. Users do not have to implement any traits; they
//! can create an arbitrary precision, correctly rounding interval right away by calling
//! `IntervalSet::<Mpfr>::new()`.

extern crate libc;

/// Finite precision module.
///
/// This module defines `fp::Float` trait and related traits where the floating point operations can
/// round up or down depending on which version of the operation is used.
pub mod fp;

/// Transcendental trait module.
///
/// This module defines transcendental functions such as `log` and `exp`.
pub mod transc;

/// MPFR wrapper module.
///
/// GNU MPFR is a C library that provides arbitrary precision floating-point functionality. This
/// module defines `mpfr::Mpfr` struct which implements `fp::Float` using GNU MPFR.
pub mod mpfr;

/// Interval module.
///
/// This module defines `Interval` struct that represents an interval bounded by two `fp::Float`s.
pub mod interval;

/// Interval module.
///
/// This module defines `IntervalSet` struct that represents a non-intersecting set of `Interval`s.
pub mod intervalset;

pub use interval::{Interval, ParseIntervalError, SignClass};
pub use intervalset::{IntervalSet, ParseIntervalSetError};

#[cfg(test)]
mod tests {
    #[test]
    fn test_quick_start() {
        use intervalset::IntervalSet;
        use mpfr::Mpfr;
        use std::str::FromStr;

        let x = IntervalSet::<Mpfr>::from_str("{0; <1, 2>}").unwrap();
        let y = IntervalSet::<Mpfr>::singleton(Mpfr::from(3.0));
        assert_eq!("{3; <4, 5>}", format!("{}", x + y));
    }
}
