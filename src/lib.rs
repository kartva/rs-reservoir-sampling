#[cfg(feature = "unweighted")]
pub mod unweighted {
    pub mod core;

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
        core::l(stream, sample, &mut rng);
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
        core::r(stream, sample, &mut rng);
    }
}

#[cfg(feature = "weighted")]
pub mod weighted {
    pub mod core;

    use rand::thread_rng;
    use self::core::WeightedItem;

    /// An implementation of Algorithm `A-Res` (https://en.wikipedia.org/wiki/Reservoir_sampling#Algorithm_A-Res)
    /// # Parameters:
    /// - Iterator of `core::WeightedItem` as *source*,
    /// - `sample_len`, the size of the sampled reservoir.
    /// Returns a `Vec` of length `sample_len`
    /// In case iterator yields less than sample amount, sample will be filled as much as possible, and returned.

    pub fn a_res <I, T> (stream: I, sample_len: usize) -> Vec<T>
    where
        I: Iterator<Item=WeightedItem<T>>
    {
        let mut rng = thread_rng();
        core::a_res(stream, sample_len, &mut rng)
    }
}