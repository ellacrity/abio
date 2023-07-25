//! Module containing a newtype wrapper for byte slices.
//!
//! This module provides an easier way to extend the API for `&[u8]` types, since the
//! [`Bytes`] type is local to the crate.

use core::ops::{Bound, Deref, Index, Range, RangeBounds, RangeFrom, RangeTo};
use core::slice;

use crate::shims::{self, FromInner};
use crate::{Abi, BytesOf, Chunk, Decode, Error, Result, Source, Span};

/// Wrapper type for a borrowed slice of bytes.
///
/// This type extends the `&[u8]` type, providing additional methods  wrapper around
/// a borrowed slice of bytes.
///
/// The [`Bytes`] type provides zero-cost abstractions, extending the `&[u8]` type by
/// providing  `#[repr(transparent)]` layout,
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Bytes {
    inner: [u8],
}

impl Bytes {
    /// Create a new [`Bytes`] type by wrapping a borrowed slice of bytes.
    #[inline]
    pub const fn new(bytes: &[u8]) -> &Self {
        Bytes::parse_bytes(bytes)
    }

    /// Create a [`Bytes`] instance from a slice of bytes.
    #[inline]
    pub(crate) const fn parse_bytes<T: Decode>(bytes: &[u8]) -> &Bytes {
        unsafe {
            let bytes = slice::from_raw_parts(bytes.as_ptr().cast::<u8>(), bytes.len());
            T::is_aligned
        }
    }

    /// Returns this [`Bytes`] instance interpreted as some type `T` where `T: Abi`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the bytes comprising `self` do not
    /// match the size requirements of `T`. This function also returns an error if
    /// the alignment requirements of `T` are not fulfilled..
    #[inline]
    pub fn interpret<T: Abi>(&self) -> Result<T> {
        if self.inner.len() != T::SIZE {
            Err(Error::size_mismatch(T::SIZE, self.inner.len()))
        } else if self.is_aligned_with::<T>() {
            Ok(unsafe { self.inner.as_ptr().cast::<T>().read() })
        } else {
            Err(Error::misaligned_access(self.bytes_of()))
        }
    }
}

impl Bytes {
    /// Creates a new [`Bytes`] instance from a given slice of bytes and offset.
    ///
    /// Returns `None` if `bytes.len() < offset`. This would cause the "cursor" to
    /// overrun the buffer, pointing to memory outside of the allocated object.
    #[inline]
    pub const fn new_offset(bytes: &[u8], offset: usize) -> Result<&Self> {
        if bytes.len() < offset {
            Err(Error::out_of_bounds(offset, bytes.len()))
        } else {
            Ok(unsafe { Bytes::new_offset_unchecked(bytes, offset) })
        }
    }

    /// Creates a [`Bytes`] instance from a slice of bytes and and offset, without
    /// checking that `offset` is within bounds of `bytes`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes.len() >= offset`. Violating this invariant
    /// is immediate **undefined behaviour**.
    #[inline(always)]
    pub const unsafe fn new_offset_unchecked(bytes: &[u8], offset: usize) -> &Self {
        debug_assert!(!bytes.is_empty(), "ZST's are currently unsupported for this type.");
        debug_assert!(
            bytes.len() >= offset,
            "Cannot construct Bytes instance with `offset > bytes.len()`"
        );
        let base = bytes.as_ptr().add(offset);
        let from_raw_parts = slice::from_raw_parts(base, bytes.len() - offset);
        Bytes::parse_bytes(from_raw_parts)
    }

    /// Slices the buffer using a range expression, returning the resulting bytes.
    ///
    /// # Panics
    ///
    /// This method panics if the range is outside the bounds of the byte slice.
    #[inline]
    pub fn slice(&self, range: impl RangeBounds<usize>) -> &[u8] {
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

        &self.inner[start..end]
    }

    /// Returns the two raw pointers spanning the slice.
    #[inline]
    pub const fn as_ptr_range(&self) -> Range<*const u8> {
        self.inner.as_ptr_range()
    }

    /// Returns the inner byte slice comprising the [`Bytes`] instance.
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    /// Returns the inner byte slice comprising the [`Bytes`] instance.
    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, u8> {
        self.inner.iter()
    }

    /// Returns the number of elements in the slice.
    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the slice has a length of 0.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn with_offset(&self, new_start: usize) -> Option<&Bytes> {
        if self.inner.len() < new_start {
            None
        } else {
            // SAFETY: Bounds checks prove that `self.inner.len() >= new_start`.
            Some(unsafe { self.subslice(new_start..self.inner.len() - new_start) })
        }
    }
}

impl Bytes {
    /// Reads `count` bytes from the buffer.
    ///
    /// Returns `None` if `count` is greater than the length of the buffer.
    #[inline]
    pub const fn read(&self, count: usize) -> Result<&Bytes> {
        let remaining = self.inner.len();
        if remaining < count {
            Err(Error::out_of_bounds(count, remaining))
        } else {
            // SAFETY: `self.len() >= size`, so we can read at least `size`
            // bytes without accessing memory out of bounds.
            Ok(unsafe { self.subslice(0..count) })
        }
    }

    /// Reads `count` bytes from the buffer, starting at `offset`.
    ///
    /// Returns `None` if `self.len() < offset + count`
    #[inline]
    pub const fn read_at(&self, offset: usize, count: usize) -> Result<&Bytes> {
        let (start, end) = (offset, offset + count);
        if end > self.len() {
            Err(Error::out_of_bounds(count, self.inner.len()))
        } else {
            // SAFETY: `self.len() >= offset + size`, so we can read at least
            // `size` bytes, starting at `offset`, without accessing memory out of bounds.
            Ok(unsafe { self.subslice(start..end) })
        }
    }

    /// Reads a slice of bytes indexed by `span`.
    ///
    /// # Errors
    ///
    /// This method returns an error if `self.len() != span.size()`.
    #[inline]
    pub const fn read_spanned(&self, span: Span) -> Result<&Bytes> {
        if span.size() == 0 || self.inner.len() != span.size() {
            Err(Error::size_mismatch(span.size(), self.inner.len()))
        } else {
            // SAFETY: Bounds checks prove that `self.inner.len() >= span.size()` && `span` is
            // not empty. This means that the result is a non-ZST byte slice.
            Ok(unsafe { self.subslice(span.start()..span.end()) })
        }
    }

    /// Reads from the bytes buffer until a condition is satisifed.
    ///
    /// Returns `None` if the resulting slice is a ZST.
    pub fn read_until<F>(&self, predicate: F) -> Result<&[u8]>
    where
        F: Fn(&u8) -> bool,
    {
        let mut pos = 0;
        while pos < self.inner.len() {
            if predicate(&self.inner[pos]) {
                break;
            }

            pos += 1;
        }

        if pos == 0 {
            Err(Error::verbose(
                "cannot call `read_until` with a result that yields `0` consumed bytes.",
            ))
        } else {
            // SAFETY: Bounds checks above prove that `pos <= self.inner.len()`.
            Ok(unsafe { self.subslice(0..pos) })
        }
    }

    /// Returns a subslice of this [`Bytes`] instance, using a `..` range operation.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `range` is within bounds of the byte slice
    /// represented by this instance. Violating this constraint is immediate
    /// **undefined behaviour**.
    #[inline(always)]
    const unsafe fn subslice(&self, range: Range<usize>) -> &Bytes {
        let data = self.inner.as_ptr().add(range.start);
        let len = range.end.saturating_sub(range.start);
        Bytes::parse_bytes(slice::from_raw_parts(data, len))
    }
}

impl AsRef<[u8]> for Bytes {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.bytes_of()
    }
}

impl AsMut<[u8]> for Bytes {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a> From<&'a [u8]> for &'a Bytes {
    fn from(bytes: &'a [u8]) -> &'a Bytes {
        Bytes::new(bytes)
    }
}

impl<'a> Index<RangeFrom<usize>> for &'a Bytes {
    type Output = [u8];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.inner[range]
    }
}

impl<'a> Index<RangeTo<usize>> for &'a Bytes {
    type Output = [u8];

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.inner[range]
    }
}

impl<'a, const N: usize> FromInner<&'a [u8; N]> for &'a Bytes {
    fn from_inner(inner: &'a [u8; N]) -> &'a Bytes {
        Bytes::new(&inner[..])
    }
}

impl<'a> Iterator for &'a Bytes {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.iter().next()
    }
}

unsafe impl Source for Bytes {
    type Array<const LEN: usize> = Chunk<LEN>;
    type Slice = Bytes;

    fn read_slice_at(&self, offset: usize, size: usize) -> crate::Result<&Self::Slice> {
        let span = Span::new(offset, size);
        if self.source_len() < span.size() {
            Err(Error::out_of_bounds(span.size(), self.source_len()))
        } else {
            // SAFETY: `span.start()` is within ounds of `self`, so we havew a valid pointer to
            // this location. The resulting `bytes` slice uses this validated pointer and adjusts
            // the length to a value that is known to be within bounds.
            unsafe { Ok(self.subslice(span.start()..span.end())) }
        }
    }

    fn read_slice(&self, size: usize) -> crate::Result<&Self::Slice> {
        if let Ok(bytes) = self.read(size) {
            Ok(Bytes::new(bytes.as_ref()))
        } else {
            Err(Error::out_of_bounds(size, self.inner.len()))
        }
    }

    fn read_chunk_at<const N: usize>(&self, offset: usize) -> crate::Result<Self::Array<N>> {
        if let Ok(bytes) = self.read_at(offset, N) {
            Ok(Chunk::from(unsafe { shims::to_byte_array_unchecked::<N>(bytes, offset) }))
        } else {
            Err(Error::out_of_bounds(offset + N, self.len()))
        }
    }

    fn read_chunk<const N: usize>(&self) -> crate::Result<Self::Array<N>> {
        match shims::first_chunk::<N>(self.bytes_of()) {
            Ok(chunk) => Ok(chunk),
            Err(err) => Err(err),
        }
    }

    fn source_len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &[u8] = b"NtAllocateVirtualAbi\0kernel32.dll\0ntdll.dll\0";

    #[test]
    fn array_from_bytes() {
        let expected = SOURCE.get(0..23).unwrap();

        let reader = Bytes::new(SOURCE);
        let actual = reader.read(23).unwrap();
        assert_eq!(expected, actual.slice(..));
        assert_eq!(Some(&actual.inner), Some(&b"NtAllocateVirtualAbi"[..]));
    }

    #[test]
    fn reading_subslice_in_middle() {
        let expected = SOURCE.get(13 + 24..SOURCE.len() - 1).unwrap();
        let reader =
            Bytes::new_offset(SOURCE, 13 + 24).expect("offset would overflow Bytes instance");
        let actual = reader.read_until(|&b| b == b'\0').unwrap();

        assert_eq!(actual, expected);
        assert_eq!(actual, &b"ntdll.dll"[..]);
    }
}
