# Project Roadmap

## Future Releases

Futures releases are expected to come with them a refined public API, a more performant and safer private API (implementation detail), and enhancements to the existing type and trait system.

### Enhance `Array` trait

Add ability to construct an `Array` type from any type that implements `Integer`. The primary benefit of this is to allow easier and more ergonomic conversions between commonly-used types.

/// Converts an [`Integer`] type into its equivalent [`Array`] type.
///
/// The length of the returned array is the number of bytes comprising the
/// integer value.
///
/// # Errors
///
/// Returns an error if the conversion fails.
fn from_integer<I: Integer>(integer: I) -> Result<Self>;