#[derive(Debug)]
pub enum ParseIntervalError {
    MissingOpeningBracket,
    MissingClosingBracket,
    InvalidNumberOfBounds,
    BoundsParseError,
    InvalidBounds,
}

/// See http://fab.cba.mit.edu/classes/S62.12/docs/Hickey_interval.pdf
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SignClass {
    Mixed,
    Zero,
    Positive(bool),
    Negative(bool),
}

#[derive(Clone, Debug)]
pub struct Interval<BOUND> {
    pub lo: BOUND,
    pub hi: BOUND,
}
