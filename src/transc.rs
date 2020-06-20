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

    /// Computer the sin of `self`.
    fn sin(self) -> Self::Output;

    /// Computes the cos of `self`.
    fn cos(self) -> Self::Output;

    /// Computes the tan of `self`.
    fn tan(self) -> Self::Output;

    /// Computes the sqrt of `self`.
    fn sqrt(self) -> Self::Output;

    /// Computes the abs of `self`.
    fn abs(self) -> Self::Output;

    /// Computes the sign of `self`.
    fn signum(self) -> Self::Output;
}
