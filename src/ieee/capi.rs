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
    fn fesetround(x: c_int) -> c_int;
}

#[inline]
pub fn get_round() -> c_int {
    unsafe { fegetround() }
}

#[inline]
pub fn set_round(mode: c_int) {
    let ret = unsafe { fesetround(mode) };
    if ret != 0 {
        panic!("unable to set rounding mode to {}", mode);
    }
}

#[inline]
pub fn round_down() {
    set_round(0x400);
}

#[inline]
pub fn round_up() {
    set_round(0x800);
}
