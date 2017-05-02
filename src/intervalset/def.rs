use interval::Interval;

#[derive(Debug)]
pub enum ParseIntervalSetError {
    MissingOpeningBraces,
    MissingClosingBraces,
    IntervalsParseError,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntervalSet<F> {
    pub intervals: Vec<Interval<F>>,
}
