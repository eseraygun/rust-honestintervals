use libc::c_int;

macro_rules! lo {
    ($b:block) => {{
        use ::ieee::capi::*;
        let saved = get_round();
        round_down();
        let res = $b;
        set_round(saved);
        res
    }}
}

macro_rules! hi {
    ($b:block) => {{
        use ::ieee::capi::*;
        let saved = get_round();
        round_up();
        let res = $b;
        set_round(saved);
        res
    }}
}

extern {
    fn fegetround() -> c_int;
    fn fesetround(x: c_int);
}

#[inline]
pub fn round_down() {
    unsafe { fesetround(0x400); }
}

#[inline]
pub fn round_up() {
    unsafe { fesetround(0x800); }
}

#[inline]
pub fn get_round() -> c_int {
    unsafe { fegetround() }
}

#[inline]
pub fn set_round(mode: c_int) {
    unsafe { fesetround(mode) }
}
