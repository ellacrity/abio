//! Module related to how data is interpreted, such as its layout and endianness.

use crate::layout::endian::Endian;
use crate::{Abi, Result};

/// Trait for defining how a particular type is decoded, or deserialized, directly
/// from a slice of bytes.
///
/// This trait makes use of the [`Source`] and [`Endian`] traits to ensure values can
/// be read from the byte stream in an endian-aware manner. This allows for
/// operations on data with a specific byte order serialization type.
///
/// By default, this trait is implemented for types defined within [`abio`][crate],
/// such as its "aligned" integer types.
pub trait Decode: Abi {
    /// Offset type used to index into the source.
    type Offset;

    /// Decodes a concrete type `T` from an immutable reference to `self`.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to a size mismatch or misaligned
    /// read.
    fn decode<E: Endian>(source: &[u8], offset: Self::Offset, endian: E) -> Result<Self>;
}
