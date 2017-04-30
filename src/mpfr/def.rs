use super::capi::MpfrStruct;

#[derive(Debug)]
pub enum ParseMpfrError {
    CStringError,
    MpfrError,
}

#[derive(Debug)]
pub struct Mpfr {
    pub mpfr: MpfrStruct,
}
