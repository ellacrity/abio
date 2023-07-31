//! Module containing types and primitives for working with slices of bytes with a
//! known size.
//!
//! This module provides essential utilities for converting to and from slices and
//! arrays of bytes. Fixed-size arrays module, is parameterized over the size, or
//! capacity, of its underlying backing buffer. This allows the compiler to make more
//! aggressive optimizations, since the size of the slice is explicit.

use core::ops::Deref;

use crate::source::Slice;
use crate::{shims, Abi, Array, Result};

/// A fixed-size array of bytes, or "chunk".
///
/// `Chunk` is a newtype wrapper around an array of bytes (`[u8; N]`). It represents
/// a fixed-size chunk of bytes that can be used to perform efficient, size-aware
/// operations.
///
/// The size of the `Chunk`, `N`, is determined at compile time via const generics.
/// This allows you to create `Chunk` instances of any size, with the size
/// information carried with the type for enhanced type safety and efficiency. The
/// maximum size of `N` depends on your particular use case and memory constraints.
///
/// This type is an integral part of the API and it is used widely to handle byte
/// arrays in a convenient and type-safe manner.
///
/// # Layout
/// This type is marked with the `#[repr(transparent)]` attribute, ensuring its
/// memory layout is identical to that of its single non-zero-sized field, `[u8; N]`.
///
/// # Type Parameters
/// - `N`: The size of the array in bytes, defined at compile time.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Chunk<const N: usize> {
    /// Array of bytes with length `N`.
    inner: [u8; N],
}

impl<const N: usize> Chunk<N> {
    /// Creates a new [`Chunk`] from an array of bytes with length `N`.
    #[inline(always)]
    pub const fn new(chunk: [u8; N]) -> Self {
        debug_assert!(!chunk.is_empty(), "Chunk types cannot represent ZST's");
        Self { inner: chunk }
    }

    /// Creates a new [`Chunk`] instance with length `N` from a bytes slice.
    ///
    /// Returns `None` if `bytes.len() < N`, or if the conversion from slice to array
    /// fails.
    #[inline]
    pub const fn from_bytes(bytes: &[u8]) -> Result<Chunk<N>> {
        crate::read_chunk_bytes!(bytes, N)
    }

    /// Creates a new [`Chunk`] instance with length `N` from a bytes slice, starting
    /// at `offset`.
    ///
    /// Returns `None` if `bytes.len() < offset + N` or if the conversion from slice
    /// to array fails.
    #[inline]
    pub const fn from_bytes_at(bytes: &[u8], offset: usize) -> Result<Chunk<N>> {
        crate::read_chunk_bytes!(bytes, offset, N)
    }

    /// Gets a pointer to the first byte of this chunk, returning a `*const u8`.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    /// Returns the number of bytes in the chunk.
    #[inline]
    pub const fn len(&self) -> usize {
        shims::const_min_value(self.inner.len(), N)
    }

    /// Returns `true` if the chunk has a length of 0.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the array represented by this chunk as a `[u8; N]`.
    #[inline(always)]
    pub const fn into_array(self) -> [u8; N] {
        self.inner
    }

    // #[inline]
    // pub const fn empty<const LEN: usize>() -> Chunk<LEN> {
    //     let region = [0u8; LEN];
    //     Chunk::<LEN>::new(region)
    // }

    /// Returns this chunk as a slice of bytes.
    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

    /// Converts this chunk of bytes into a UTF-8 encoded `&str` slice.
    ///
    /// Returns `None` if the operation fails due to malformed bytes.
    #[inline]
    pub const fn as_str(&self) -> Option<&str> {
        if let Ok(utf8) = core::str::from_utf8(self.as_bytes()) {
            Some(utf8)
        } else {
            None
        }
    }

    /// Returns a pointer to this chunk offset by `count` bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `count` does not exceed the size of the array, or
    /// else it is immediate **undefined behaviour**.
    #[inline(always)]
    pub(crate) const unsafe fn as_ptr_offset(&self, count: usize) -> *const u8 {
        self.as_ptr().add(count)
    }

    /// Returns this chunk of bytes interpreted as some type `T` where `T: Abi`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `self` is properly aligned and the bytes
    /// representing `self` have the same size as `T`.
    #[inline(always)]
    pub const unsafe fn read<T: Abi>(&self) -> T {
        self.as_ptr().cast::<T>().read()
    }
}

// #[inline]
// const fn read_chunk_at_inner<const N: usize>(bytes: &[u8], offset: usize) ->
// Result<Chunk<N>> {     if bytes.len() < offset + N {
//         return Err(Error::out_of_bounds(offset + N, bytes.len()));
//     }

//     // SAFETY: The subslice operation below relies on the validations performed
// above,     // which ensure `offset + N` are within bounds of the slice.
// Additionally, the     // `Chunk` type represents an array of `u8`, so it has no
// alignment requirements.     unsafe {
//         let bytes = core::core::slice::from_raw_parts(bytes.as_ptr(), N);
//         if bytes.len() != N {
//             Err(Error::size_mismatch(N, bytes.len()))
//         } else {
//             // Final size checks ensure that `bytes.len() == N`, and Chunk has the
// same layout             // internally as a `[u8; N]`.
//             Ok(bytes.as_ptr().cast::<Chunk<N>>().read())
//         }
//     }
// }

impl<const N: usize> Array<N> for Chunk<N> {
    #[inline]
    fn from_ptr<T: Abi>(ptr: *const T) -> Result<Self> {
        let ptr = ptr.cast::<[u8; N]>();
        let slice = unsafe { core::slice::from_raw_parts(ptr.cast::<u8>(), N) };
        Chunk::from_bytes(slice)
    }
}

impl<const N: usize> AsRef<[u8]> for Chunk<N> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const N: usize> Deref for Chunk<N> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl<const N: usize> From<[u8; N]> for Chunk<N> {
    #[inline]
    fn from(array: [u8; N]) -> Self {
        Chunk::new(array)
    }
}

impl<'data, const N: usize> TryFrom<&'data [u8]> for Chunk<N> {
    type Error = crate::Error;

    #[inline]
    fn try_from(bytes: &'data [u8]) -> Result<Self, Self::Error> {
        crate::read_chunk_bytes!(bytes, N)
    }
}

impl<'data, const N: usize> From<Slice<'data>> for Result<Chunk<N>> {
    #[inline]
    fn from(bytes: Slice<'data>) -> Result<Chunk<N>> {
        crate::read_chunk_bytes!(bytes, N)
    }
}

/// Reads a chunk of `$size` bytes from a borrowed slice of bytes, with an
/// optional `$offset`.
#[doc(hidden)]
#[macro_export]
macro_rules! read_chunk_bytes {
    ($bytes:ident, $size:tt) => {{
        if $bytes.len() < $size {
            return Err($crate::Error::out_of_bounds($size, $bytes.len()));
        }

        // SAFETY: The validation above tells us that `$bytes` is at least `$size` bytes in
        // length. The longest subslice this routine could take is the entire slice, which is
        // a safe operation. Additionally, the `Chunk` type represents a slice of `u8`
        // elements, so alignment checks can be skipped (alignment is 1).
        let chunk_bytes = unsafe { ::core::slice::from_raw_parts($bytes.as_ptr(), $size) };
        if chunk_bytes.len() != $size {
            Err($crate::Error::size_mismatch($size, chunk_bytes.len()))
        } else {
            // SAFETY: The slice is guaranteed to have a length of `$size`, and `Chunk` has a
            // memory layout identical to `[u8; $size]`.
            Ok(unsafe { chunk_bytes.as_ptr().cast::<Chunk<$size>>().read() })
        }
    }};
    ($bytes:ident, $offset:ident, $size:tt) => {{
        if $bytes.len() < $offset + $size {
            return Err($crate::Error::out_of_bounds($offset + $size, $bytes.len()));
        }

        // SAFETY: The validation above tells us that `$bytes` is at least `$offset + $size`
        // bytes in length. The longest subslice this routine could take is the
        // entire slice, which is a safe operation. Additionally, the `Chunk` type represents
        // a slice of `u8` elements, so alignment checks can be skipped (alignment is
        // 1).
        let chunk_bytes =
            unsafe { ::core::slice::from_raw_parts($bytes.as_ptr().add($offset), $size) };
        if chunk_bytes.len() != $size {
            Err($crate::Error::size_mismatch($size, chunk_bytes.len()))
        } else {
            // SAFETY: The slice is guaranteed to have a length of `$size`, and `Chunk` has a
            // memory layout identical to `[u8; $size]`.
            Ok(unsafe { chunk_bytes.as_ptr().cast::<Chunk<$size>>().read() })
        }
    }};
    () => {};
}
