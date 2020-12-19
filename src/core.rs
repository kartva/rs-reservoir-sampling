/*
pub mod weighted {
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
}
*/



pub mod unweighted {
    use rand::{Rng, distributions::{Uniform, Distribution}};
    use std::iter::Iterator;

    fn fill <I, T> (iter: &mut I, sample: &mut [T]) -> Option<()>
    where
        I: Iterator<Item=T>,
    {
        for element in sample.iter_mut() {
            *element = match iter.next() {Some(n) => n, None => return None};
        }

        Some(())
    }

    /// An implementation of Algorithm `L` (https://en.wikipedia.org/wiki/Reservoir_sampling#An_optimal_algorithm)
    /// # Parameters:
    /// - Type implementing `std::iter::Iterator` as *source*,
    /// - Mutable array slice (i.e. `&mut [T]`) as *sample array (i.e. where sampled data is stored)*
    /// - Type implementing `rand::Rng` for random number generation.
    /// In case iterator yields less than sample amount, sample will be filled as much as possible, and returned.
    
    pub fn l <R, I, T>(mut iter: I, sample: &mut [T], rng: &mut R)
    where
        R: Rng + ?Sized,
        I: Iterator<Item=T>,
    {
        // Fill the sample array from the reservoir
        // And return if iterator has been already exhausted.
        if let None = fill (&mut iter, sample) {
            return;
        }

        let random_index = Uniform::from(0..sample.len());

        let mut w: f64 = (rng.gen::<f64>().ln() / sample.len() as f64).exp();
        let mut i = sample.len();

        loop {
            i += (rng.gen::<f64>().ln() / (1.0 - w).ln()).floor() as usize + 1;

            if let Some(n) = iter.nth(i) {
                sample[random_index.sample(rng)] = n;
                w *= (rng.gen::<f64>().ln() / sample.len() as f64).exp();
            } else {
                break;
            }
        };
    }

    /// An implementation of algorithm `R` (https://en.wikipedia.org/wiki/Reservoir_sampling#Simple_algorithm)
    /// # Parameters:
    /// - Type implementing `std::iter::Iterator` as *source*,
    /// - Mutable array slice (i.e. `&mut [T]`) as *sample array (i.e. where sampled data is stored)*
    /// - Type implementing `rand::Rng` for random number generation.
    /// In case iterator yields less than sample amount, sample will be filled as much as possible, and returned.

    pub fn r <R, I, T>(mut iter: I, sample: &mut [T], rng: &mut R)
    where
        R: Rng + ?Sized,
        I: Iterator<Item=T>,
    {
        // Fill the sample array from the reservoir
        // And return if iterator has been already exhausted.
        if let None = fill (&mut iter, sample) {
            return;
        }

        let random_index = Uniform::from(0..sample.len());
        let mut random_number;

        for ele in iter {
            random_number = random_index.sample(rng);
            if random_number < sample.len() {
                sample[random_number] = ele;
            }
        };
    }
}