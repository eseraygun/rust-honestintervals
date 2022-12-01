/// Struct and functions definitions for the GNU MPFR library. See
/// http://www.mpfr.org/mpfr-current/mpfr.html for details.
use libc::{c_char, c_double, c_int, c_long, c_ulong};

/// MPFR rounding mode enum.
#[repr(C)]
pub enum MpfrRnd {
    /// Round to the nearest representable number. Ties goes to the even number.
    HalfToEven = 0,
    /// Round towards zero.
    TowardsZero,
    /// Round towards positive infinity.
    Up,
    /// Round towards negative infinity.
    Down,
    /// Round away from zero.
    AwayFromZero,
    /// Round to the nearest representable number. Ties goes away from zero.
    HalfAwayFromZero = -1,
}

/// MPFR precision type.
pub type MpfrPrec = c_long;
/// MPFR sign type.
pub type MpfrSign = c_int;
/// MPFR exponent type.
pub type MpfrExp = c_long;
/// MPFR limb type.
pub type MpLimb = c_ulong;

/// Low-level MPFR struct.
#[repr(C)]
#[derive(Debug)]
pub struct MpfrStruct {
    pub _mpfr_prec: MpfrPrec,
    pub _mpfr_sign: MpfrSign,
    pub _mpfr_exp: MpfrExp,
    pub _mpfr_d: *mut MpLimb,
}

type MpfrPtr = *mut MpfrStruct;
type MpfrConstPtr = *const MpfrStruct;

#[allow(missing_docs)]
#[link(name = "mpfr")]
extern "C" {
    // Initialization functions.
    pub fn mpfr_init2(x: MpfrPtr, prec: MpfrPrec);
    pub fn mpfr_clear(x: MpfrPtr);

    // Assignment functions.
    pub fn mpfr_set(rop: MpfrPtr, op: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_set_nan(x: MpfrPtr);
    pub fn mpfr_set_inf(x: MpfrPtr, sign: c_int);
    pub fn mpfr_set_zero(x: MpfrPtr, sign: c_int);
    pub fn mpfr_set_d(rop: MpfrPtr, op: c_double, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_set_str(rop: MpfrPtr, s: *const c_char, base: c_int, rnd: MpfrRnd) -> c_int;

    // Conversion functions.
    pub fn mpfr_get_d(op: MpfrConstPtr, rnd: MpfrRnd) -> c_double;

    // Comparison functions.
    pub fn mpfr_cmp(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_nan_p(op: MpfrConstPtr) -> c_int;
    pub fn mpfr_inf_p(op: MpfrConstPtr) -> c_int;
    pub fn mpfr_number_p(op: MpfrConstPtr) -> c_int;
    pub fn mpfr_zero_p(op: MpfrConstPtr) -> c_int;
    pub fn mpfr_sgn(op: MpfrConstPtr) -> c_int;
    pub fn mpfr_greater_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_greaterequal_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_less_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_lessequal_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_equal_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_unordered_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;

    // Integer and remainder related functions.
    pub fn mpfr_integer_p(op: MpfrConstPtr) -> c_int;

    // Basic arithmetic functions.
    pub fn mpfr_neg(rop: MpfrPtr, op: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_abs(rop: MpfrPtr, op: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_add(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_sub(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_mul(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_div(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_pow(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;

    // Special functions.
    pub fn mpfr_log(rop: MpfrPtr, op: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_exp(rop: MpfrPtr, op: MpfrConstPtr, rnd: MpfrRnd) -> c_int;

    // Miscellaneous Functions
    pub fn mpfr_min(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_max(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
}
