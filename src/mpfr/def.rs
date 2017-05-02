use super::capi::MpfrStruct;

/// MPFR parsing error enum.
#[derive(Debug, PartialEq)]
pub enum ParseMpfrError {
    /// Represents a `CString` conversion error.
    CStringError,
    /// Represents an MPFR parsing error.
    MpfrParseError,
}

/// MPFR struct.
#[derive(Debug)]
pub struct Mpfr {
    /// Low-level MPFR data.
    pub mpfr: MpfrStruct,
}
