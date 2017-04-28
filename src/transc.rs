pub trait Transc {
    type Output;

    fn log(self) -> Self::Output;
    fn exp(self) -> Self::Output;
}
