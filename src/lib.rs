//! Crate implementing reservoir sampling, a method for getting random samples
//! from a source in a single pass. Useful in situations where size of source is
//! unknown or very large.
//! Read this article for more information: https://en.wikipedia.org/wiki/Reservoir_sampling
//! All algorithms implemented here have been taken from this article only.

pub mod reservoir_sampling_core {
    use rand::{Rng, distributions::{Uniform, Distribution}};
    use std::iter::Iterator;

    /// An implementation of Algorithm *L* (https://en.wikipedia.org/wiki/Reservoir_sampling#An_optimal_algorithm)
    /// # Parameters:
    /// - Type implementing `std::iter::Iterator` as *source*,
    /// - Mutable array slice (i.e. `&mut [T]`) as *sample array*
    /// - Type implementing `rand::Rng` for random number generation.
    /// - SampleUniform type which yields (0, 1)
    /// In case iterator yields less than sample amount, sample will be filled as much as possible, and returned.

    pub fn l <R, I, T>(mut iter: I, sample: &mut [T], rng: &mut R)
    where
        R: Rng + ?Sized,
        I: Iterator<Item=T>,
    {
        // Fill the sample array from the reservoir
        for element in sample.iter_mut() {
            *element = match iter.next() {Some(n) => n, None => return};
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

    /// An implementation of algorithm *R* (https://en.wikipedia.org/wiki/Reservoir_sampling#Simple_algorithm)
    /// # Parameters:
    /// - Type implementing `std::iter::Iterator` as *source*,
    /// - Mutable array slice (i.e. `&mut [T]`) as *sample array*
    /// - Type implementing `rand::Rng` for random number generation.
    /// - SampleUniform type which yields (0, 1)
    /// In case iterator yields less than sample amount, sample will be filled as much as possible, and returned.

    pub fn r <R, I, T>(mut iter: I, sample: &mut [T], rng: &mut R)
    where
        R: Rng + ?Sized,
        I: Iterator<Item=T>,
    {
        // Fill the sample array from the reservoir
        for element in sample.iter_mut() {
            *element = match iter.next() {Some(n) => n, None => return};
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