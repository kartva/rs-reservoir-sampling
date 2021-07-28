### v 0.5.1
- Minor README and CHANGELOG changes.
- Documentation update to `weighted` module. 

# v 0.5.0
- `wasm32` architecture support 🎉

# v 0.4.2
- License change to `Unlicense`

### v 0.4.1
- README correction 🤦‍

# v 0.4.0
- `weighted` MVP:
	- Added `WeightedItem` trait which is used by `weighted` algorithms to sample.
	- Added `weighted` algorithm `a_res`

### v 0.3.1
- Corrected README

# v 0.3.0
- Removed `streaming-iterator` 🤦‍
- Fixed algorithm R (`unweighted::r`) and added back to unweighted API
- Separated `unweighted` into feature (which is included by default)
- Added rudimentary test suite (currently panics and prints results into console, so human can verify that results look random)
- Added advice to use algorithm L by default in README.
- Added "Quickstart" section to README

## v 0.2.3
- Fixed `streaming_iterator` support not working due to out-of-scope `use` statement.

# v 0.2.2
- Removed Algorithm R references in API due to it yielding biased results (see [issue](https://github.com/DesmondWillowbrook/rs-reservoir-sampling/issues/1#issue-771851119))

## v 0.2.1   
- Removed unneccesary dependence on `compare`

## v 0.2.0
- Added `streaming-iterator` support
