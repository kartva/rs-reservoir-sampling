use reservoir_sampling::weighted::*;

mod common;
use common::weighted::run;
use std::ops::Range;

#[test]
fn test_a_res () {
    run(10, a_res::<Range<usize>, usize>);
}