/// Trait for transcendental functions.
pub trait Transc<RHS=Self> {
    /// Output type.
    type Output;
    /// Computes the natural logarithm of `self`.
    fn log(self) -> Self::Output;
    /// Computes the natural exponential of `self`.
    fn exp(self) -> Self::Output;
    /// Computes `self` raised to the power `rhs`.
    fn pow(self, rhs: RHS) -> Self::Output;
}
