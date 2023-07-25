//! Module containing types and primitives for working with slices of bytes with a
//! known size.
//!
//! This module provides essential utilities for converting to and from slices and
//! arrays of bytes. Fixed-size arrays module, is parameterized over the size, or
//! capacity, of its underlying backing buffer. This allows the compiler to make more
//! aggressive optimizations, since the size of the slice is explicit.

use core::ops::Deref;
use core::slice;

use crate::contiguous::{Bytes, Source, Span};
use crate::layout::BytesOf;
use crate::{shims, Abi, Array, Decode, Error, Integer, Result};

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
        read_chunk_at_inner::<N>(bytes, 0)
    }

    /// Creates a new [`Chunk`] instance with length `N` from a bytes slice, starting
    /// at `offset`.
    ///
    /// Returns `None` if `bytes.len() < offset + N` or if the conversion from slice
    /// to array fails.
    #[inline]
    pub const fn from_bytes_at(bytes: &[u8], offset: usize) -> Result<Chunk<N>> {
        read_chunk_at_inner::<N>(bytes, offset)
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
    pub const fn as_bytes(&self) -> &[u8] {
        &self.inner
    }

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
    pub const unsafe fn as_ptr_offset(&self, count: usize) -> *const u8 {
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
        unsafe { self.as_ptr().cast::<T>().read() }
    }
}

#[inline]
const fn read_chunk_at_inner<const N: usize>(bytes: &[u8], offset: usize) -> Result<Chunk<N>> {
    if bytes.len() < offset + N {
        return Err(Error::out_of_bounds(offset + N, bytes.len()));
    }

    //       SAFETY: The array bytes consist of valid bytes within the bounds of the
    // slice.       Again, `Chunk` has no alignment requirements (its alignment is
    // 1), so it is safe    to read here.
    unsafe {
        let bytes = slice::from_raw_parts(bytes.as_ptr(), N);
        if bytes.len() != N {
            Err(Error::size_mismatch(N, bytes.len()))
        } else {
            Ok(bytes.as_ptr().cast::<Chunk<N>>().read())
        }
    }

    // SAFETY: Bounds checks above prove that `bytes.len() >= offset + N`.
    // Chunk::
    // let bytes = unsafe { slice::from_raw_parts(bytes.as_ptr().add(offset), N) };
    // if bytes.len() != N {
    //     Err(Error::size_mismatch(N, bytes.len()))
    // } else if let Ok(array) = bytes.try_into() {
    //     Ok(Chunk::from_array(array))
    // } else {
    //     Err(Error::size_mismatch(N, bytes.len()))
    // }
}

impl<const N: usize> Array<N> for Chunk<N> {
    #[inline]
    fn from_ptr<T: Abi>(ptr: *const T) -> Result<Self> {
        let ptr = ptr.cast::<[u8; N]>();
        let slice = unsafe { slice::from_raw_parts(ptr.cast::<u8>(), N) };
        Chunk::from_bytes(slice)
    }

    #[inline]
    fn from_integer<I: Integer>(integer: I) -> Result<Self> {
        read_chunk_at_inner::<N>(integer.bytes_of(), 0)
    }
}

impl<const N: usize> AsRef<[u8]> for Chunk<N> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const N: usize> Deref for Chunk<N> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
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
        Chunk::<N>::from_bytes(bytes)
    }
}

impl<'src, const N: usize> From<&'src Bytes> for Result<Chunk<N>> {
    fn from(bytes: &'src Bytes) -> Result<Chunk<N>> {
        Chunk::<N>::from_bytes(bytes.as_slice())
    }
}

impl<'a, const N: usize> From<&'a Bytes> for Option<Chunk<N>> {
    fn from(value: &'a Bytes) -> Option<Chunk<N>> {
        shims::first_chunk(value.as_slice()).ok()
    }
}

unsafe impl<const N: usize> Source for Chunk<N> {
    type Array<const LEN: usize> = Chunk<LEN>;
    type Slice = [u8];

    #[inline]
    fn source_len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    fn read_slice_at(&self, offset: usize, size: usize) -> Result<&Self::Slice> {
        let span = Span::new(offset, size);
        if self.source_len() < span.size() {
            Err(Error::out_of_bounds(span.size(), self.source_len()))
        } else {
            // SAFETY: `span.start()` is within ounds of `self`, so we havew a valid pointer to
            // this location. The resulting `bytes` slice uses this validated pointer and adjusts
            // the length to a value that is known to be within bounds.
            unsafe {
                let data = self.as_ptr_offset(span.start());
                Ok(slice::from_raw_parts(data, self.source_len() - span.start()))
            }
        }
    }

    #[inline]
    fn read_chunk_at<const LEN: usize>(&self, offset: usize) -> Result<Self::Array<LEN>> {
        Chunk::<LEN>::from_bytes_at(self.as_bytes(), offset)
    }

    fn read_slice(&self, size: usize) -> Result<&Self::Slice> {
        let span = Span::new(0, size);
        if self.source_len() < span.size() {
            return Err(Error::out_of_bounds(span.size(), self.source_len()));
        }
        // SAFETY: `span.start()` is within ounds of `self`, so we havew a valid pointer to
        // this location. The resulting `bytes` slice uses this validated pointer and adjusts
        // the length to a value that is known to be within bounds.
        unsafe {
            let data = self.as_ptr_offset(span.start());
            Ok(slice::from_raw_parts(data, self.source_len() - span.start()))
        }
    }

    fn read_chunk<const LEN: usize>(&self) -> Result<Self::Array<LEN>> {
        Chunk::from_bytes(self.as_bytes())
    }
}

impl Decode for u8 {
    fn decode<U: Abi>(bytes: &[u8]) -> Result<U> {
        match bytes.read_slice(U::SIZE) {
            Ok(bytes) => {
                if !bytes.is_aligned_with::<U>() {
                    Err(Error::misaligned_access(bytes.bytes_of()))
                } else {
                    Ok(unsafe { bytes.as_ptr().cast::<U>().read() })
                }
            }
            Err(err) => Err(err),
        }
    }
}

impl Decode for u16 {
    fn decode<T: Abi, const N: usize>(chunk: Chunk<N>) -> Result<T> {
        // comparing to an associated const so are optimized away.
        if chunk.size() != T::SIZE || Self::SIZE != T::SIZE {
            return Err(Error::size_mismatch(Self::SIZE, chunk.size()));
        } else if !chunk.is_aligned_with::<T>() {
            return Err(Error::misaligned_access(chunk.as_bytes()));
        } else {
            // got a chunk with same size and alignment as `Self` so read ptr.
            Ok(unsafe { (*chunk).read::<T>() })
        }
    }
}
