use reservoir_sampling::unweighted::*;

mod common;
use common::unweighted::run;
use std::ops::Range;

#[test]
fn test_l () {
    run(10, l::<Range<usize>, usize>);
}

#[test]
fn test_r () {
    run(10, r::<Range<usize>, usize>);
}