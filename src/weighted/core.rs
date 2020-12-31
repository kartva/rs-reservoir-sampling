/*
use rand::{Rng};
use std::{iter::Iterator, ops::Div};
use binary_heap_plus::{BinaryHeap, MinComparator};
use compare::Compare;

pub trait WeightedItem {
    type Item;
    type Weight: Div;

    fn curr(self) -> Self::Item;

    fn weight(self) -> Self::Weight;
}

impl<W: WeightedItem> Compare<W> for W
{

}

/// An implementation of Algorithm `A-Res` (https://en.wikipedia.org/wiki/Reservoir_sampling#Algorithm_A-Res)
/// # Parameters:
/// - Type implementing `std::iter::Iterator` as *source*,
/// - Mutable array slice (i.e. `&mut [T]`) as *sample array*
/// - Type implementing `rand::Rng` for random number generation.
/// In case iterator yields less than sample amount, sample will be filled as much as possible, and returned.

pub fn a_res <R, W, I, T>(mut iter: I, sample_size: usize, rng: &mut R) -> Vec<T>
where
    R: Rng + ?Sized,
    I: Iterator<Item=W>,
    W: WeightedItem<Item=T> + Compare<W> + Ord,
{
    let mut heap: BinaryHeap<W, MinComparator> = BinaryHeap::with_capacity_min(sample_size);

    for ele in iter {
        let r = rng.gen::<f64>().powf(1.0 / ele.weight());

        if heap.len() < sample_size {
            heap.push()
        }
    }

    heap.into_vec()
}
*/