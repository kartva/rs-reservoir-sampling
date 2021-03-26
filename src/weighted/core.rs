use rand::Rng;
use std::iter::Iterator;
use std::collections::BinaryHeap;

/**
Encapsulate item in this struct with `new` function, to use weighted algorithms.
The `weight` field is used to determine weight of the item. (Higher => More frequent)

### Notes:
- Only use positive values for `weight`.
- `i64` was used instead of `f64` as a heap is used in quite a few functions,
and `BinaryHeap` requires types to implement `Ord`, which `f64` does not do.
*/

pub struct WeightedItem <T>
{
    item: T,
    weight: i64, 
}

impl<T> WeightedItem<T> {
    pub fn new(item: T, weight: i64) -> Self { Self { item, weight } }
}

impl<T> Ord for WeightedItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T> PartialOrd for WeightedItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl<T> PartialEq for WeightedItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight.eq(&other.weight)
    }
}

impl<T> Eq for WeightedItem<T> {
    fn assert_receiver_is_total_eq(&self) {}
}

/// An implementation of Algorithm `A-Res` (https://en.wikipedia.org/wiki/Reservoir_sampling#Algorithm_A-Res)
/// # Parameters:
/// - Iterator of `core::WeightedItem` as *source*,
/// - `sample_len`, the size of the sampled reservoir.
/// Returns a `Vec` of length `sample_len`
/// In case iterator yields less than sample amount, sample will be filled as much as possible, and returned.

pub fn a_res <R, I, T>(iter: I, sample_len: usize, rng: &mut R) -> Vec<T>
where
    R: Rng + ?Sized,
    I: Iterator<Item=WeightedItem<T>>,
{
    let mut heap = BinaryHeap::new();

    for ele in iter {
        let r = rng.gen::<f64>().powf(1.0 / ele.weight as f64);

        if heap.len() < sample_len {
            heap.push(WeightedItem {item: ele.item, weight: -ele.weight});
        } else if r > heap.peek().unwrap().weight as f64 {
            heap.pop();
            heap.push(WeightedItem {item: ele.item, weight: -ele.weight});
        }
    }

    heap.into_iter().map(|w| w.item).collect()
}