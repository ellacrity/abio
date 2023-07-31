//! Module containing a newtype wrapper for byte slices.
//!
//! This module provides an easier way to extend the API for `&[u8]` types, since the
//! [`Slice`] type is local to the crate.

use core::marker::PhantomData;
use core::ops::{Bound, Deref, Index, Range, RangeBounds, RangeFrom, RangeTo};
use core::ptr::NonNull;

use crate::shims::FromInner;
use crate::{Chunk, Error, Result, Span};

/// Contiguous region of memory containing a borrowed sequence of bytes.
///
/// # Layout
///
/// The [`Slice`] type provides an abstraction for [Dynamically Sized
/// Types][DST] (DSTs) represented as a slice of bytes. As a side effect, this type
/// must be able to safely handle [Zero Sized Types][ZST] (ZSTs).
///
/// # Safety Considerations
///
/// According to the documentation contained in the Rustnomicon, "references to ZSTs
/// (including empty slices), just like all other references, must be non-null and
/// suitably aligned. Dereferencing a null or unaligned pointer to a ZST is undefined
/// behavior, just like for any other type."
///
/// This crate chooses to use [`core::ptr::read`] to perform bitwise copies of data
/// existing in memory. The reason for this is to avoid crashes or potential
/// undefined behaviour caused by misaligned reads. This includes dereferencing of
/// raw pointers. Their alignment is always verified before performing an operation
/// that results in a dereference.
///
/// # Specialization
///
/// Although not yet implemented, future plans include specializations for common
/// slice types, such as `ByteSlice`, `WideSlice` and `NullSlice`. The addition of
/// these primitives will make it easier for higher-level crates to implement
/// primitives for working with slices of known types, each with their own unique
/// invariants.
///
/// # Zero-Cost Abstractions
///
/// Like all of the core types in [`abio`][crate], the [`Slice`] type leverages
/// Rust's zero-cost abstractions by extending the built-in `&[u8]` type. It provides
/// additional methods for parsing and validating inputs, converting slices to and
/// from arrays, and working with [`Span`] types.
///
/// [DST]: https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Slice<'data> {
    ptr: NonNull<u8>,
    len: usize,
    _lifetime: PhantomData<&'data u8>,
}

impl<'data> Slice<'data> {
    /// Create a new [`Slice`] type by wrapping a borrowed slice of bytes.
    #[inline(always)]
    pub const fn from_slice(slice: &'data [u8]) -> Slice<'data> {
        Slice {
            ptr: unsafe { NonNull::new_unchecked(slice.as_ptr().cast_mut()) },
            len: slice.len(),
            _lifetime: PhantomData,
        }
    }

    /// Creates a new [`Slice`] instance from a given slice of bytes and offset.
    ///
    /// # Errors
    ///
    /// Returns an error if `bytes.len() < offset`. Failing to perform this bounds
    /// check would cause potential UB as the offset could end up pointing past the
    /// end bound of the allocated byte slice object.
    #[inline]
    pub const fn from_slice_at(slice: &'data [u8], offset: usize) -> Result<Slice<'data>> {
        if slice.len() < offset {
            Err(Error::out_of_bounds(offset, slice.len()))
        } else {
            Ok(unsafe { Slice::from_slice_at_unchecked(slice, offset) })
        }
    }

    /// Creates a [`Slice`] instance from a slice of bytes and and offset, without
    /// checking that `offset` is within bounds of `bytes`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes.len() >= offset`. Violating this invariant
    /// is immediate **undefined behaviour**.
    #[inline]
    pub const unsafe fn from_slice_at_unchecked(slice: &'data [u8], offset: usize) -> Slice<'data> {
        debug_assert!(
            slice.len() >= offset,
            "Cannot construct Bytes instance with `offset > bytes.len()`"
        );
        Slice::from_raw_parts(slice.as_ptr().add(offset), slice.len().saturating_sub(offset))
    }

    /// Constructs a new [`Slice`] instance given a byte slice, an offset, and a
    /// size.
    ///
    /// # Errors
    ///
    /// This function returns an error if `bytes.len() < offset + size`. For
    /// additional context, see [`Bytes::new_offset`][new_offset].
    ///
    /// [new_offset]: crate::source::Bytes::new_offset
    #[inline]
    pub const fn new_offset_with_size(
        slice: &'data [u8],
        offset: usize,
        size: usize,
    ) -> Result<Slice<'data>> {
        if slice.len() < offset + size {
            Err(Error::out_of_bounds(offset, slice.len()))
        } else {
            Ok(unsafe { Slice::from_slice_at_unchecked(slice, offset) })
        }
    }

    /// Constructs a new `Slice` instance given a byte slice, an offset, and a size
    /// without first performing any bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that ensure that `offset + size` does not exceed the
    /// length of `bytes`. Violating this invariant is immediate **undefined
    /// behaviour**.
    #[inline]
    pub(crate) const unsafe fn new_offset_with_size_unchecked(
        bytes: &'data [u8],
        offset: usize,
        size: usize,
    ) -> Slice<'data> {
        Slice {
            ptr: NonNull::new_unchecked(bytes.as_ptr().add(offset).cast_mut()),
            len: size,
            _lifetime: PhantomData,
        }
    }

    /// Constructs a new [`Slice`] instance from a [`Chunk`] with size `N`.
    ///
    /// The function helps convert a chunk of bytes with a fixed size into a byte
    /// slice with the same size.
    #[inline]
    pub const fn from_chunk<const N: usize>(chunk: &'data Chunk<N>) -> Slice<'data> {
        Slice::from_slice(chunk.as_bytes())
    }

    /// Creates a [`Slice`] instance from a pointer and a length.
    ///
    /// The `len` argument is the number of **elements**, not the number of bytes.
    ///
    /// This method is a simple wrapper around [`core::ptr::from_raw_parts`].
    ///
    /// # Safety
    ///
    /// Please refer to the safety documentation for [`core::ptr::from_raw_parts`] to
    /// ensure this method is used correctly.
    #[inline]
    pub const unsafe fn from_raw_parts(data: *const u8, len: usize) -> Slice<'data> {
        Slice::from_slice(unsafe { core::slice::from_raw_parts(data, len) })
    }

    /// Reads `count` bytes from the buffer.
    ///
    /// Returns `None` if `count` is greater than the length of the buffer.
    #[inline]
    pub const fn read(&'data self, count: usize) -> Result<Slice<'data>> {
        if self.len < count {
            Err(Error::out_of_bounds(count, self.len))
        } else {
            // SAFETY: `self.len() >= size`, so we can read at least `size`
            // bytes without accessing memory out of bounds.
            unsafe { self.subslice(0..count) }
        }
    }

    /// Reads `count` bytes from the buffer, starting at `offset`.
    ///
    /// Returns `None` if `self.len() < offset + count`
    #[inline]
    pub const fn read_at(&'data self, offset: usize, count: usize) -> Result<Slice<'data>> {
        Span::span_bytes(self.as_slice(), offset, count)
    }

    /// Reads a slice of bytes indexed by `span`.
    ///
    /// # Errors
    ///
    /// This method returns an error if `self.len() != span.len()`.
    #[inline]
    pub const fn read_spanned(&'data self, span: Span) -> Result<Slice<'data>> {
        if self.len != span.len() {
            Err(Error::size_mismatch(span.len(), self.len))
        } else {
            // SAFETY: Bounds checks prove that `self.len == span.len()` && `span` is
            // not empty. This means that the result is a non-ZST byte slice.
            unsafe { self.subslice(span.as_range()) }
        }
    }

    /// Returns a subslice of this [`Slice`] instance, using a `..` range operation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the indices comprising `range` are within bounds
    /// of the byte slice. `range.start` is also expected to be <= `range.end`.
    /// Violating either of these invariants is immediate **undefined behaviour**.
    #[inline(always)]
    const unsafe fn subslice(&'data self, range: Range<usize>) -> Result<Self> {
        let size = range.end.saturating_sub(range.start);
        if self.len < size {
            Err(Error::out_of_bounds(size, self.len))
        } else {
            // SAFETY: Bounds checks ensure that the ptr to this slice is within bounds of
            // `self.inner`, and `size <= self.len`.
            Ok(Slice::new_offset_with_size_unchecked(self.as_slice(), range.start, size))
        }
    }

    /// Slices the buffer using a range expression, returning the resulting bytes.
    ///
    /// # Panics
    ///
    /// This method panics if the range is outside the bounds of the byte slice.
    #[inline]
    pub fn slice(&'data self, range: impl RangeBounds<usize>) -> &'data [u8] {
        let start = match range.start_bound() {
            Bound::Included(&idx) => idx,
            Bound::Excluded(&idx) => idx + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&idx) => idx + 1,
            Bound::Excluded(&idx) => idx,
            Bound::Unbounded => self.len(),
        };

        &self.as_slice()[start..end]
    }

    /// Returns the two raw pointers spanning the slice.
    #[inline]
    pub const fn as_ptr_range(&self) -> Range<*const u8> {
        let start = self.ptr.as_ptr().cast::<u8>().cast_const();
        // SAFETY: The indices represented by `start` and `self.len()` both fall within the
        // bounds of the slice. For additional information, refer to the official
        // documentation: https://doc.rust-lang.org/core/primitive.slice.html#method.as_ptr_range
        let end = unsafe { start.add(self.len()) };
        start..end
    }

    /// Returns the inner byte slice comprising the [`Slice`] instance.
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, u8> {
        // SAFETY: This is safe because Bytes can only be constructed with a valid slice,
        // so the pointer and length must represent a valid slice.
        self.as_slice().iter()
    }

    /// Returns the number of elements in the slice.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the slice has a length of 0.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'data> AsRef<[u8]> for Slice<'data> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'data> Deref for Slice<'data> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'data> From<&'data [u8]> for Slice<'data> {
    #[inline(always)]
    fn from(slice: &'data [u8]) -> Slice<'data> {
        Slice::from_slice(slice)
    }
}

impl<'data> Index<RangeFrom<usize>> for Slice<'data> {
    type Output = [u8];

    #[inline]
    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.as_slice()[range]
    }
}

impl<'data> Index<RangeTo<usize>> for Slice<'data> {
    type Output = [u8];

    #[inline]
    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.as_slice()[range]
    }
}

impl<'data, const N: usize> FromInner<&'data [u8; N]> for Slice<'data> {
    fn from_inner(inner: &'data [u8; N]) -> Slice<'data> {
        Slice::from_slice(&inner[..])
    }
}

impl<'data> IntoIterator for &'data Slice<'data> {
    type Item = &'data u8;

    type IntoIter = core::slice::Iter<'data, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'data> PartialEq<[u8]> for Slice<'data> {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self == other
    }
}

impl<'data> PartialEq<&'data [u8]> for Slice<'data> {
    #[inline]
    fn eq(&self, other: &&'data [u8]) -> bool {
        self.as_slice() == *other
    }
}

impl<'data> PartialEq<Slice<'data>> for &'data [u8] {
    #[inline]
    fn eq(&self, other: &Slice<'data>) -> bool {
        self == other
    }
}

/// Reads a subslice of `$size` bytes from a borrowed slice of bytes, with an
/// optional `$offset`.
#[doc(hidden)]
#[macro_export]
macro_rules! read_slice_bytes {
    ($bytes:ident, $size:tt) => {{
        if $bytes.len() < $size {
            return Err($crate::Error::out_of_bounds($size, $bytes.len()));
        }

        // SAFETY: The validation above tells us that `$bytes` is at least `$size` bytes in
        // length. The longest subslice this routine could take is the entire slice, which is
        // a safe operation. Additionally, the `Slice` type represents a slice of `u8`
        // elements, so alignment checks can be skipped (alignment is 1).

        let slice_bytes =
            unsafe { ::core::slice::from_raw_parts($bytes.as_ptr().cast::<u8>(), $size) };
        if slice_bytes.len() != $size {
            Err($crate::Error::size_mismatch($size, slice_bytes.len()))
        } else {
            // SAFETY: The slice is guaranteed to have a length of `$size`, and `Slice` has a
            // memory layout identical to `&[u8]`
            Ok(slice_bytes)
        }
    }};
    ($bytes:ident, $offset:ident, $size:tt) => {{
        if $bytes.len() < $offset + $size {
            return Err($crate::Error::out_of_bounds($offset + $size, $bytes.len()));
        }

        // SAFETY: The validation above tells us that `$bytes` is at least `$offset + $size`
        // bytes in length. The longest subslice this routine could take is the
        // entire slice, which is a safe operation. Additionally, the `Slice` type represents
        // a slice of `u8` elements, so alignment checks can be skipped (alignment is
        // 1).
        let slice_bytes = unsafe {
            ::core::slice::from_raw_parts($bytes.as_ptr().cast::<u8>().add($offset), $size)
        };
        if slice_bytes.len() != $size {
            Err($crate::Error::size_mismatch($size, slice_bytes.len()))
        } else {
            // SAFETY: The slice is guaranteed to have a length of `$size`, and `Slice` has a
            // memory layout identical to `&[u8]`
            Ok(slice_bytes)
        }
    }};
    () => {};
}

// TODO: Write unit tests for module

#[cfg(test)]
mod tests {
    use super::*;

    const INPUTS: &[&[u8]] = &[b"libabio.dll", b"kernel32.dll", b"ntdll.dll"];
    const STREAM: &[u8] = b"libabio.dllkernel32.dllntdll.dll";

    #[test]
    fn bytes_reader_methods() {
        let binding = Slice::from_slice(STREAM);
        let actual = binding.read(11).expect("Bytes failed to read from stream");
        assert!(!actual.is_empty());
        assert_eq!(INPUTS[0], actual.slice(..));
        assert_eq!(actual, &b"libabio.dll"[..]);
    }
}
