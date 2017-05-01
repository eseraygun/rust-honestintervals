extern crate libc;

#[macro_use]
mod macros;
pub mod fp;
pub mod transc;
#[macro_use]
pub mod mpfr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
