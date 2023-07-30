//! Module related to how data is interpreted, such as its layout and endianness.

use core::mem::size_of;

use crate::config::Codec;
use crate::{Abi, Result};

/// The Decode trait defines how a type is deserialized or decoded from a
/// slice or chunk of bytes. It provides a way to translate raw byte sequences
/// back into meaningful data in a structured manner.
///
/// # Safety and Alignment
///
/// The Decode trait prioritizes safety and correctness over speed when it comes to
/// decoding data. It intentionally avoids using ptr::read_unaligned due to potential
/// alignment issues. Correctly handling alignment is an important part of preventing
/// undefined behavior, which is a key goal of the abio crate.
///
/// # Endian-Aware Decoding
///
/// [`Decode`] leverages the [`Codec`] type to provide access to the
/// [`Endian`][crate::endian::Endian]  and Endian traits to decode byte streams in an
/// endian-aware manner. It ensures the proper interpretation of data according to
/// the specific byte order serialization type.
///
/// # Default Implementations
///
/// By default, Decode is implemented for types defined within the abio crate,
/// including its "aligned" integer types. These default implementations allow for
/// immediate use of Decode in a majority of cases where endian-aware, safe decoding
/// is required.
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
    fn decode(bytes: &[u8], codec: Codec) -> Result<Self>;
}

impl<const N: usize> Decode for [u16; N] {
    fn decode(bytes: &[u8], codec: Codec) -> Result<Self> {
        let mut pos = 0;
        let mut buf = [0u16; N];
        while pos < buf.len() {
            let offset = pos * size_of::<u16>();
            let bytes = &bytes[offset..];
            let elem = u16::decode(bytes, codec)?;
            buf[pos] = elem;
            pos += 1;
        }

        Ok(buf)
    }
}
