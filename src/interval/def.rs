/// Interval parsing error enum.
#[derive(Debug)]
pub enum ParseIntervalError {
    /// The first character of the strict is anything other than `'<'`.
    MissingOpeningBracket,
    /// The first character of the strict is anything other than `'>'`.
    MissingClosingBracket,
    /// There are zero, one or more than two bounds between brackets.
    InvalidNumberOfBounds,
    /// There was an error while parsing a bound.
    BoundsParseError,
    /// Bounds do not satisfy interval criteria. See `interval::Interval::new` for details.
    InvalidBounds,
}

/// Represents the sign class of an interval.
///
/// See http://fab.cba.mit.edu/classes/S62.12/docs/Hickey_interval.pdf for details.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SignClass {
    /// Contains both negative and positive numbers.
    Mixed,
    /// Contains only zero.
    Zero,
    /// Contains only positive numbers and maybe zero.
    Positive(bool),
    /// Contains only negative numbers and maybe zero.
    Negative(bool),
}

/// Interval struct.
///
/// Represents a set where each element `x` satisfies `lo <= x && x <= hi`.
#[derive(Clone, Debug)]
pub struct Interval<BOUND: PartialOrd> {
    /// Inclusive lower bound.
    pub lo: BOUND,
    /// Inclusive upper bound.
    pub hi: BOUND,
}
