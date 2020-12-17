//! Crate implementing reservoir sampling, a method for getting random samples
//! from a source in a single pass. Useful in situations where size of source is
//! unknown or very large.
//! Read this article for more information: https://en.wikipedia.org/wiki/Reservoir_sampling
//! All algorithms implemented here have been taken from this article only.

//! # API Design
//! Functions take:
//! - An iterator of generic type T, with no constraints which serves as a stream of data to sample.
//! - Mutable array slice to store sampled data
//! - RNG 
pub mod core;

pub mod unweighted {
    use crate::core::unweighted;
    use rand::thread_rng;
    
    /// An implementation of Algorithm `L` (https://en.wikipedia.org/wiki/Reservoir_sampling#An_optimal_algorithm)
    /// # Parameters:
    /// - Type implementing `std::iter::Iterator` as *source*,
    /// - Mutable array slice (i.e. `&mut [T]`) as *sample array*
    /// # Notes:
    /// - In case iterator yields less than sample amount, sample will be filled as much as possible, and returned.
    /// - This uses `rand::thread_rng` to provide RNG. To use your own RNG which implements `rand::RNG`, see `reservoir_sampling::core`
    
    pub fn l <I, T>(stream: I, sample: &mut [T])
    where
        I: Iterator<Item=T>,
    {
        let mut rng = thread_rng();
        unweighted::l(stream, sample, &mut rng);
    }

    pub fn r <I, T>(stream: I, sample: &mut [T])
    where
        I: Iterator<Item=T>,
    {
        let mut rng = thread_rng();
        unweighted::r(stream, sample, &mut rng);
    }
}