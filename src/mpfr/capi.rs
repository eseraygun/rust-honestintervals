use libc::{c_char, c_double, c_int, c_long, c_longlong};

#[repr(C)]
pub enum MpfrRnd {
    HalfToEven = 0,
    TowardsZero,
    Up,
    Down,
    AwayFromZero,
    HalfAwayFromZero = -1,
}

pub type MpfrPrec = c_long;
pub type MpfrSign = c_int;
pub type MpfrExp = c_long;
pub type MpLimb = c_longlong;

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
    pub fn mpfr_sgn(op: MpfrConstPtr) -> c_int;
    pub fn mpfr_greater_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_greaterequal_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_less_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_lessequal_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_equal_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;
    pub fn mpfr_unordered_p(op1: MpfrConstPtr, op2: MpfrConstPtr) -> c_int;

    // Basic arithmetic functions.
    pub fn mpfr_neg(rop: MpfrPtr, op: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_abs(rop: MpfrPtr, op: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_add(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_sub(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_mul(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_div(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;

    // Special functions.
    pub fn mpfr_log(rop: MpfrPtr, op: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_exp(rop: MpfrPtr, op: MpfrConstPtr, rnd: MpfrRnd) -> c_int;

    // Miscellaneous Functions
    pub fn mpfr_min(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
    pub fn mpfr_max(rop: MpfrPtr, op1: MpfrConstPtr, op2: MpfrConstPtr, rnd: MpfrRnd) -> c_int;
}
