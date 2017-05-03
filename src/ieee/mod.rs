#[macro_use]
mod capi;
mod impl_float;

use std::fmt::{Debug, Display};
use std::str::FromStr;

/// A function that is opaque to the optimizer, to allow benchmarks to
/// pretend to use outputs to assist in avoiding dead-code
/// elimination.
///
/// This function is a no-op, and does not even read from `dummy`.
pub fn black_box<E: Sized + Debug, T: Display + FromStr<Err = E>>(dummy: T) -> T {
    T::from_str(format!("{}", dummy).as_str()).unwrap()
}

#[test]
fn test_fesetround() {
    let x = black_box(1.0);
    let y = black_box(3.0);
    unsafe { println!("{}", lo!({ x / y })); }
    unsafe { println!("{}", hi!({ x / y })); }
}
