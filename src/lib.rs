//! Crate implementing reservoir sampling, a method for getting random samples
//! from a source in a single pass. Useful in situations where size of source is
//! unknown or very large.
//! Read this article for more information: https://en.wikipedia.org/wiki/Reservoir_sampling
//! All algorithms implemented here have been taken from this article only.

//! # API Design
//! Functions take:
//! - An iterator of generic type T, with no constraints which serves as a stream of data to sample.
//! - Mutable array slice to store sampled data
//! <br>
//! By default, functions use `rand::thread_rng` to provide RNG.
//! To use your own RNG which implements `rand::RNG`, use functions in `reservoir_sampling::core`
pub mod core;

#[cfg(feature = "streaming_iterator_support",)]
pub mod streaming_iterator_sampling;

pub mod unweighted {
    use crate::core::unweighted;
    use rand::thread_rng;
    
    /// An implementation of Algorithm `L` (https://en.wikipedia.org/wiki/Reservoir_sampling#An_optimal_algorithm)
    /// # Parameters:
    /// - Type implementing `std::iter::Iterator` as *source*,
    /// - Mutable array slice (i.e. `&mut [T]`) as *sample array (i.e. where sampled data is stored)*
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

    /// An implementation of algorithm `R` (https://en.wikipedia.org/wiki/Reservoir_sampling#Simple_algorithm)
    /// # Parameters:
    /// - Type implementing `std::iter::Iterator` as *source*,
    /// - Mutable array slice (i.e. `&mut [T]`) as *sample array (i.e. where sampled data is stored)*
    /// - Type implementing `rand::Rng` for random number generation.
    /// In case iterator yields less than sample amount, sample will be filled as much as possible, and returned.

    pub fn r <I, T>(stream: I, sample: &mut [T])
    where
        I: Iterator<Item=T>,
    {
        let mut rng = thread_rng();
        unweighted::r(stream, sample, &mut rng);
    }
}
