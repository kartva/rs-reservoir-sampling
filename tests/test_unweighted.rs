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

//mismatched types
//expected struct `Box<(dyn Iterator<Item = usize> + 'static)>`
//   found struct `Copied<std::slice::Iter<'_, usize>>`
//for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html