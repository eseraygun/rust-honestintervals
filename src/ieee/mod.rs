#[macro_use]
mod capi;
mod impl_float;

#[cfg(test)]
mod tests;

#[inline(never)]
fn black_box<T>(dummy: T) -> T {
    dummy
}

#[test]
fn test_fesetround() {
    let x = black_box(1.0);
    let y = black_box(3.0);
    println!("{}", lo!({ x / y }));
    println!("{}", hi!({ x / y }));
}
