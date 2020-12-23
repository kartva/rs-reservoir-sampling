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

        match iter.nth(i) {
            Some(n) => {
                sample[random_index.sample(rng)] = n;
                w *= (rng.gen::<f64>().ln() / sample.len() as f64).exp();
            }
            None => break,
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

    let mut random_number;

    let mut i = sample.len();
    for ele in iter {
        random_number = rng.gen_range(0..i);
        if random_number < sample.len() {
            sample[random_number] = ele;
        }
        i += 1;
    };
}
