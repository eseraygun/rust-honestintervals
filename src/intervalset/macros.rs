macro_rules! interval_set {
    {$($v:tt);*} => { IntervalSet::<Mpfr>::from_intervals(vec![$(interval!$v),*]) };
}
