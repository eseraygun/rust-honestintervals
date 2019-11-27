use crate::interval::Interval;

/// Interval set parsing error enum.
#[derive(Debug)]
pub enum ParseIntervalSetError {
    /// The first character of the strict is anything other than `'{'`.
    MissingOpeningBraces,
    /// The first character of the strict is anything other than `'}'`.
    MissingClosingBraces,
    /// There was an error while parsing an interval.
    IntervalsParseError,
}

/// Interval set struct.
///
/// Represents a set of non-intersecting intervals.
#[derive(Clone, Debug)]
pub struct IntervalSet<BOUND: PartialEq + PartialOrd> {
    /// Non-intersecting intervals in strictly ascending order.
    pub intervals: Vec<Interval<BOUND>>,
}
