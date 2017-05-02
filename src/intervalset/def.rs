use interval::Interval;

#[derive(Debug)]
pub enum ParseIntervalSetError {
    MissingOpeningBraces,
    MissingClosingBraces,
    IntervalsParseError,
}

#[derive(Clone, Debug)]
pub struct IntervalSet<BOUND: PartialEq> {
    pub intervals: Vec<Interval<BOUND>>,
}
