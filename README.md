[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/DesmondWillowbrook/rs-reservoir-sampling)

# reservoir-sampling
Crate implementing reservoir sampling, a method for getting random samples
from a source in a single pass. Useful in situations where size of source is
unknown or very large.
Read this article for more information: https://en.wikipedia.org/wiki/Reservoir_sampling
All algorithms implemented here have been taken from this article only.

# Quickstart:
```rust
use reservoir_sampling::unweighted::l;

fn main () {
    let mut sampled_arr = vec![0usize; 10];

    l(0usize..100, sampled_arr.as_mut_slice());
    println!("Sampled array: {:?}", sampled_arr);
}
```

# API Design
Functions take:
- An `Iterator` over generic type `T`, with no constraints which serves as a stream of data to sample.
- Mutable array slice (`&mut [T]`) to store sampled data

By default, functions use `rand::thread_rng` to provide RNG.
To use your own RNG which implements `rand::RNG`, use functions in `reservoir_sampling::core`.

# Future development:
Plan to implement weighted reservoir sampling as well.
