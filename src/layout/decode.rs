//! Module related to how data is interpreted, such as its layout and endianness.

use crate::{Abi, Error, Result};

/// Trait for defining how a particular type is decoded, or deserialized, directly
/// from a slice of bytes.
///
/// This trait makes use of the [`Source`] and [`Endian`] traits to ensure values can
/// be read from the byte stream in an endian-aware manner. This allows for
/// operations on data with a specific byte order serialization type.
///
/// By default, this trait is implemented for types defined within [`abio`][crate],
/// such as its "aligned" integer types.
#[const_trait]
pub trait Decode: Abi {
    /// Decodes a slice of bytes as a concrete type `T`, where `T` implements
    /// [`Abi`].
    ///
    ///
    /// trait and has an identical in memory representation as the type `T`. The
    /// slice of bytes has the same size and alignment requirements.
    ///
    /// # Decoding
    ///
    /// The first performs a bitwise copy to create an instance of `T`, while the
    /// latter dereferences the raw pointer before taking a reference to the data as
    /// `&T`. `&T`.
    ///
    /// This crate uses the [`ptr::read`][`core::ptr::read`] method whenever possible
    /// because [`abio`][crate] is focused on soundness and correctness before
    /// performance. This helps us avoid **undefined behaviour**, and strikes a
    /// healthy balance between safety and efficiency.
    ///
    /// # Miri
    ///
    ///
    /// related methods. Miri is able to detect misaligned accesses due to
    /// pointer misalignment issues and this allows [`abio`][crate] to read raw
    /// bytes as safely as possible.
    ///
    /// # Performance
    ///
    /// This operation provides a balance between performance and safety. It favors
    /// safety whenever it can only have one of the two. In most cases, it can be
    /// safe without sacrificing on speed.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to a size mismatch, misaligned
    /// memory access, or if the result represents a type with an invalid byte
    /// pattern.
    fn decode<T: Abi>(bytes: &[u8]) -> Result<T>;

    fn is_access_aligned<T: Abi>(&self) -> bool {
        self.bytes_of() & (Self::ALIGN - 1) == 0
    }
}
