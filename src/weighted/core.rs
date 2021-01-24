use rand::{Rng};
use std::iter::Iterator;
use std::collections::BinaryHeap;

pub struct WeightedItem <T, W>
where W: PartialOrd + ops::Div + ops::Neg
{
    item: T,
    weight: W, 
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
    I: Iterator<Item=WeightedItem<T, W>>,
    W: WeightedItem<T, W>,
{
    let mut heap = BinaryHeap::with_capacity (sample_size);

    for ele in iter {
        let r = rng.gen::<f64>().powf(1.0 / ele.weight);

        if heap.len() < sample_size {
            heap.push(WeightedItem {item: ele.item, weight: -ele.weight});
        } else if r > heap.top().weight {
            heap.pop();
            heap.push(WeightedItem {item: ele.item, weight: -ele.weight});
        }
    }

    heap.into_vec()
}