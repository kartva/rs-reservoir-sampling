# reservoir-sampling
Crate implementing reservoir sampling, a method for getting random samples
from a source in a single pass. Useful in situations where size of source is
unknown or very large.
Read this article for more information: https://en.wikipedia.org/wiki/Reservoir_sampling
All algorithms implemented here have been taken from this article only.

# Note:
- `Algorithm R` is currently yielding broken values, refrain from using.

# Features:
- Support for `streaming-iterator::StreamingIterator` type.

# API Design
Functions take:
- An `Iterator` of generic type `T`, with no constraints which serves as a stream of data to sample.
- Mutable array slice (`&mut [T]`) to store sampled data

By default, functions use `rand::thread_rng` to provide RNG.
To use your own RNG which implements `rand::RNG`, use functions in `reservoir_sampling::core`.

# Contributing:
Feel free to implement more sampling algorithms! (Also reduce code duplication, that's a big issue right now)
Remember to add algorithm to:
- `reservoir_sampling::core::`
- `reservoir_sampling::`
- `reservoir_sampling::streaming_iterator_sampling::core::`
- `reservoir_sampling::streaming_iterator_sampling::`

# Future development:
Plan to implement weighted reservoir sampling as well.
Feel free to make a pull request or an issue!
