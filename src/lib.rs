extern crate libc;


/// Finite precision module.
///
/// This module defines `fp::Float` trait and related traits where the floating point operations can
/// round up or down depending on which version of the operation is used.
#[deny(missing_docs)]
pub mod fp;

/// Transcendental trait module.
#[deny(missing_docs)]
pub mod transc;

/// MPFR wrapper module.
#[deny(missing_docs)]
pub mod mpfr;

/// Interval module.
#[deny(missing_docs)]
pub mod interval;

/// Interval set module.
#[deny(missing_docs)]
pub mod intervalset;
