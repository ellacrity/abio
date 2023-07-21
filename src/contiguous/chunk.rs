//! Module containing types and primitives for working with slices of bytes with a
//! known size.
//!
//! This module provides essential utilities for converting to and from slices and
//! arrays of bytes. Fixed-size arrays module, is parameterized over the size, or
//! capacity, of its underlying backing buffer. This allows the compiler to make more
//! aggressive optimizations, since the size of the slice is explicit.

use core::slice;

use crate::bytes::Bytes;
use crate::contiguous::{Array, Source, Span};
use crate::layout::AsBytes;
use crate::{Abi, Error, Integer, Result};

/// A contiguous region of memory with a fixed size.
///
/// # Resizing
///
/// [Chunk] can be resized, at the cost of an additional allocation. However, they
/// cannot grow in size. They may only shrink.
///
/// This is the return type for many functions and provides methods for converting to
/// slices and interpreting slices as fixed size arrays. Additionally, this type is
/// very useful for converting slices of bytes to aligned integers, such as the types
/// in the [`aligned`][aligned] module.
///
/// [aligned]: crate::integer::aligned
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Chunk<const N: usize> {
    /// Array of bytes with length `N`.
    inner: [u8; N],
}

impl<const N: usize> Chunk<N> {
    /// Creates a new [`Chunk`] from an array of bytes with length `N`.
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
        if bytes.len() < N {
            return Err(Error::out_of_bounds(N, bytes.len()));
        }

        // SAFETY: The array bytes consist of valid bytes within the bounds of the slice.
        // Again, `Chunk` has no alignment requirements (its alignment is 1), so it is safe
        // to read here.
        unsafe {
            let spanned = slice::from_raw_parts(bytes.as_ptr(), N);
            if spanned.len() != N {
                Err(Error::size_mismatch(N, spanned.len()))
            } else {
                Ok(spanned.as_ptr().cast::<Self>().read())
            }
        }
    }

    /// Creates a new [`Chunk`] instance with length `N` from a bytes slice, starting
    /// at `offset`.
    ///
    /// Returns `None` if `bytes.len() < offset + N` or if the conversion from slice
    /// to array fails.
    #[inline]
    pub const fn from_bytes_at(bytes: &[u8], offset: usize) -> Result<Chunk<N>> {
        if bytes.len() < offset + N {
            return Err(Error::out_of_bounds(offset + N, bytes.len()));
        }

        // SAFETY: We have already performed bounds checking, and `Chunk` has no alignment
        // requirements.
        unsafe {
            let spanned = slice::from_raw_parts(bytes.as_ptr().add(offset), N);
            if spanned.len() != N {
                // panic!("Cannot construct Chunk<N> from byte span of wrong length");
                Err(Error::size_mismatch(N, spanned.len()))
            } else {
                Ok(spanned.as_ptr().cast::<Self>().read())
            }
        }
    }

    /// Gets a pointer to the first byte of this chunk, returning a `*const u8`.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    /// Returns the number of bytes in the chunk.
    #[inline]
    pub const fn len(&self) -> usize {
        const_min_value(self.inner.len(), N)
    }

    /// Returns `true` if the chunk has a length of 0.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub const fn from_array(bytes: [u8; N]) -> Self {
        Chunk { inner: bytes }
    }

    /// Get the array represented by this chunk as a `[u8; N]`.
    #[inline]
    pub const fn into_array(self) -> [u8; N] {
        self.inner
    }

    #[inline]
    pub const fn empty<const LEN: usize>() -> Chunk<LEN> {
        let region = [0u8; LEN];
        Chunk::<LEN>::new(region)
    }

    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    /// Returns a pointer to this chunk offset by `count` bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `count` does not exceed the size of the array, or
    /// else it is immediate **undefined behaviour**.
    #[inline(always)]
    pub const unsafe fn as_ptr_offset(&self, count: usize) -> *const u8 {
        self.as_ptr().add(count)
    }
}

/// Compares and returns the minimum of two values in a `const` context.
pub const fn const_min_value(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

impl<'source, const N: usize> Array<'source, N> for Chunk<N> {
    fn from_ptr<T: Abi>(ptr: *const T) -> Result<Self> {
        let ptr = ptr.cast::<[u8; N]>();
        let slice = unsafe { slice::from_raw_parts(ptr.cast::<u8>(), N) };
        Chunk::from_bytes(slice)
    }

    fn from_integer<I: Integer>(integer: I) -> Result<Self> {
        Chunk::<N>::from_bytes(integer.as_bytes())
    }
}

impl<const N: usize> AsRef<[u8]> for Chunk<N> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<const N: usize> From<[u8; N]> for Chunk<N> {
    fn from(array: [u8; N]) -> Self {
        Chunk::from_array(array)
    }
}

impl<'src, const N: usize> TryFrom<&'src [u8]> for Chunk<N> {
    type Error = crate::Error;

    fn try_from(bytes: &'src [u8]) -> Result<Self, Self::Error> {
        Chunk::from_bytes(bytes)
    }
}

impl<'src, const N: usize> From<Bytes<'src>> for Result<Chunk<N>> {
    fn from(bytes: Bytes<'src>) -> Result<Chunk<N>> {
        Chunk::from_bytes(bytes.as_slice())
    }
}

unsafe impl<const N: usize> Source for Chunk<N> {
    type Slice = [u8];

    #[inline]
    fn source_len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    fn read_slice(&self, offset: usize, size: usize) -> Result<(&Self::Slice, &[u8])> {
        let span = Span::new(offset, size);
        if self.source_len() < span.size() {
            Err(Error::out_of_bounds(span.size(), self.source_len()))
        } else {
            // SAFETY: `span.start()` is within ounds of `self`, so we havew a valid pointer to
            // this location. The resulting `bytes` slice uses this validated pointer and adjusts
            // the length to a value that is known to be within bounds.
            let bytes = unsafe {
                let data = self.as_ptr_offset(span.start());
                slice::from_raw_parts(data, self.source_len() - span.start())
            };
            Ok(bytes.split_at(span.size()))
        }
    }

    #[inline]
    fn read_array<'data, const SIZE: usize, A: Array<'data, SIZE>>(
        &self,
        offset: usize,
    ) -> Result<(A, &[u8])> {
        let span = Span::new(offset, SIZE);
        if self.source_len() < span.size() {
            return Err(Error::out_of_bounds(span.size(), self.source_len()));
        }

        let (head, tail) = self.as_bytes().split_at(offset);
        debug_assert!(head.len() == offset);
        debug_assert_eq!(tail[..SIZE].len(), A::SIZE);
        debug_assert_eq!(tail[..SIZE].len(), span.size());

        // SAFETY: The span above verifies that we have at least enough bytes to decode an
        // array of length `SIZE`.
        let head = &tail[..SIZE];
        let head = unsafe { head.as_ptr().cast::<A>().read() };
        Ok((head, tail))
    }
}
