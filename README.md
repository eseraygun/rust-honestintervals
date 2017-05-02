# Honest Intervals

[![Crate](https://img.shields.io/crates/v/honestintervals.svg)](https://crates.io/crates/honestintervals)
[![Documentation](https://docs.rs/honestintervals/badge.svg)](https://docs.rs/honestintervals/)
[![License](https://img.shields.io/crates/l/honestintervals.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/honestintervals.svg)](https://crates.io/crates/honestintervals)

Honest Intervals is an arbitrary precision interval arithmetic library with correct rounding.

It implements elementary arithmetic (addition, subtraction, multiplication and division) as well
as complicated mathematical functions such as logarithm and power over intervals and interval
sets. Bounds of the return values are always correctly rounded up or down to ensure that all
possible results are contained.

In addition to the `Interval` and `IntervalSet` structs, the library also provides the `Mpfr`
struct that wraps the GNU MPFR library. The `Mpfr` struct is an ideal (and currently only)
bound type for intervals.

Honest Intervals tries to be a pragmatic implementation of interval arithmetic rather than an
abstract basis for all possible implementations. Users do not have to implement any traits; they
can create a correctly rounding interval right away by calling `IntervalSet::<Mpfr>::new()`.

## Quick Start

Here is a code that creates two interval sets and adds them.

```rust
use intervalset::IntervalSet;
use mpfr::Mpfr;
use std::str::FromStr;

let x = IntervalSet::<Mpfr>::from_str("{0; <1, 2>}").unwrap();
let y = IntervalSet::<Mpfr>::singleton(Mpfr::from(3.0));
assert_eq!("{3; <4, 5>}", format!("{}", x + y));
```

Here is a tricky case of interval division handled correctly by Honest Intervals.

```rust
let x = IntervalSet::<Mpfr>::from_str("<1, 2>").unwrap();
let y = IntervalSet::<Mpfr>::from_str("<-1, 1>").unwrap();
assert_eq!("{<-inf, -1>; <1, inf>}", format!("{}", x / y));
```

See the [documentation](https://docs.rs/honestintervals/) for more details.

## License

Honest Intervals is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](LICENSE) for details.
