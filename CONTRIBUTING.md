# Internal Structure
- There are two top-level modules, `weighted` and `unweighted`
- `self::core` houses algorithms.
  Eg, for `crate::unweighted`, algorithms are housed in `crate::unweighted::core`
- A module itself houses thin wrappers over `self::core`.
  Eg, `unweighted::l` is a wrapper which provides RNG over `unweighted::core::l`