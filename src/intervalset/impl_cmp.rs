#[cfg(test)]
mod test {
    use float::{Float, RoundingMode};
    use interval::Interval;
    use intervalset::IntervalSet;
    use mpfr::Mpfr;

    #[test]
    fn test_partial_eq() {
        assert!(interval_set!{[0]} == interval_set!{[0]});
        assert!(interval_set!{[0, 1]} == interval_set!{[0, 1]});
        assert!(interval_set!{[0, 1]; [2]} == interval_set!{[0, 1]; [2]});
        assert!(interval_set!{[0, 1]} != interval_set!{[0, 123]});
    }
}
