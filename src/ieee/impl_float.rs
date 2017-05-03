//use fp;
//use fp::{Float, Sign};
//
//use std::f64;
//use std::num::ParseFloatError;
//
//use libc::c_int;
//
//macro_rules! lo {
//    ($b:block) => {{
//        round_down();
//        let res = $b;
//        round_restore();
//        res
//    }}
//}
//
//macro_rules! hi {
//    {$b:block} => {{
//        round_down();
//        let res = { $b };
//        round_restore();
//        res
//    }}
//}
//
//#[repr(C)]
//enum RoundingMode {
//    ToNearest = 0,
//    Downward = 0x400,
//    Upward = 0x800,
//    TowardZero = 0xc00,
//}
//
//extern {
//    fn fegetround() -> c_int;
//    fn fesetround(x: c_int);
//}
//
//thread_local! {
//    static ROUNDING_MODE_STACK: Vec<RoundingMode> = Vec::new();
//}
//
//fn round_up() {
//    ROUNDING_MODE_STACK.push(fegetround());
//    fesetround(RoundingMode::Upward);
//}
//
//fn round_down() {
//    ROUNDING_MODE_STACK.push(fegetround());
//    fesetround(RoundingMode::Downward);
//}
//
//fn round_restore() {
//    fesetround(ROUNDING_MODE_STACK.pop());
//}
//
//impl fp::From<f64> for f64 {
//    #[inline]
//    fn from_lo(val: f64, precision: usize) -> Self { val }
//
//    #[inline]
//    fn from_hi(val: f64, precision: usize) -> Self { val }
//}
//
//impl fp::FromStr for f64 {
//    type Err = ParseFloatError;
//
//    #[inline]
//    fn from_str_lo(s: &str, precision: usize) -> Result<Self, Self::Err> {
//        lo!({ f64::from_str(s) })
//    }
//
//    #[inline]
//    fn from_str_hi(s: &str, precision: usize) -> Result<Self, Self::Err> {
//        hi!({ f64::from_str(s) })
//    }
//}
//
//impl fp::Into<f64> for f64 {
//    #[inline]
//    fn into_lo(self) -> f64 { self }
//
//    #[inline]
//    fn into_hi(self) -> f64 { self }
//}
//
//impl fp::MinMax<Mpfr> for f64 {
//    type Output = Self;
//
//    #[inline]
//    fn min(mut self, rhs: Self) -> Self { self.min(rhs) }
//
//    #[inline]
//    fn max(mut self, rhs: Self) -> Self { self.max(rhs) }
//}
//
//impl Float for f64 {
//    fn zero(precision: usize) -> Self {
//        unimplemented!()
//    }
//
//    fn neg_zero(precision: usize) -> Self {
//        unimplemented!()
//    }
//
//    fn one(precision: usize) -> Self {
//        unimplemented!()
//    }
//
//    fn infinity(precision: usize) -> Self {
//        unimplemented!()
//    }
//
//    fn neg_infinity(precision: usize) -> Self {
//        unimplemented!()
//    }
//
//    fn nan(precision: usize) -> Self {
//        unimplemented!()
//    }
//
//    fn sign(&self) -> Sign {
//        unimplemented!()
//    }
//
//    fn precision(&self) -> usize {
//        unimplemented!()
//    }
//
//    fn is_finite(&self) -> bool {
//        unimplemented!()
//    }
//
//    fn is_infinite(&self) -> bool {
//        unimplemented!()
//    }
//
//    fn is_zero(&self) -> bool {
//        unimplemented!()
//    }
//
//    fn is_infinity(&self) -> bool {
//        unimplemented!()
//    }
//
//    fn is_neg_infinity(&self) -> bool {
//        unimplemented!()
//    }
//
//    fn is_nan(&self) -> bool {
//        unimplemented!()
//    }
//}