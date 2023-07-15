//! Module containing a newtype wrapper for byte slices.
//!
//! This module provides an easier way to extend the API for `&[u8]` types, since the
//! [`Bytes`] type is local to the crate.

use core::ops::{Bound, Range, RangeBounds};
use core::slice;

use crate::util::{AsInner, FromInner, IntoInner};
use crate::{Error, Result};

/// Newtype wrapper around a borrowed slice of bytes.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Bytes<'src> {
    inner: &'src [u8],
}

impl<'src> Bytes<'src> {
    /// Create a [`Bytes`] type by wrapping a borrowed slice of bytes.
    #[inline]
    pub const fn new(bytes: &'src [u8]) -> Self {
        Bytes { inner: bytes }
    }

    /// Creates a new [`Bytes`] instance from a given slice of bytes and offset.
    ///
    /// Returns `None` if `bytes.len() < offset`. This would cause the "cursor" to
    /// overrun the buffer, pointing to memory outside of the allocated object.
    pub const fn new_offset(bytes: &'src [u8], offset: usize) -> Option<Self> {
        if bytes.len() < offset {
            None
        } else {
            Some(unsafe { Bytes::new_offset_unchecked(bytes, offset) })
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
    pub const unsafe fn new_offset_unchecked(bytes: &'src [u8], offset: usize) -> Self {
        debug_assert!(!bytes.is_empty(), "ZST's are currently unsupported for this type.");
        debug_assert!(
            bytes.len() >= offset,
            "Cannot construct Bytes instance with `offset > bytes.len()`"
        );
        let base = bytes.as_ptr().add(offset);
        Bytes { inner: slice::from_raw_parts(base, bytes.len() - offset) }
    }

    /// Reads `size` bytes from the buffer,
    ///
    /// Returns `None` if `self.len() <
    pub fn read(&self, size: usize) -> Option<&[u8]> {
        if self.inner.len() < size {
            None
        } else {
            // SAFETY: `self.len() >= size`, so we can read at least `size`
            // bytes without accessing memory out of bounds.
            Some(unsafe { self.inner.get_unchecked(..size) })
        }
    }

    /// Reads a slice of bytes from the buffer, starting at `offset` and spanning
    /// `size` bytes.
    ///
    /// Returns `None` if `self.len() < offset + size`
    pub fn read_slice(&self, offset: usize, size: usize) -> Result<&[u8]> {
        let (start, end) = (offset, offset + size);
        if end > self.len() {
            Err(Error::out_of_bounds(offset + size, self.len()))
        } else {
            // SAFETY: `self.len() >= offset + size`, so we can read at least
            // `size` bytes, starting at `offset`, without accessing memory out of bounds.
            Ok(unsafe { self.inner.get_unchecked(start..end) })
        }
    }

    /// Returns the inner byte slice comprising the [`Bytes`] instance.
    #[inline]
    pub const fn buffer(&self) -> &'src [u8] {
        self.inner
    }

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

    /// Reads from the bytes buffer until a condition is satisifed.
    ///
    /// Returns `None` if the resulting slice is a ZST.
    pub fn read_until<F>(&mut self, predicate: F) -> Option<&[u8]>
    where
        F: Fn(&u8) -> bool,
    {
        let mut pos = 0;
        while !predicate(&self.inner[pos]) {
            pos += 1;
        }

        if pos == 0 {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(self.inner.as_ptr(), pos) })
        }
    }

    /// Returns the two raw pointers spanning the slice.
    #[inline]
    pub const fn as_ptr_range(&self) -> Range<*const u8> {
        self.inner.as_ptr_range()
    }

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    #[inline]
    pub fn iter(&self) -> slice::Iter<'src, u8> {
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

    #[inline]
    pub fn inner(&self) -> &[u8] {
        self.inner
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut &'src [u8] {
        &mut self.inner
    }
}

impl<'src> AsRef<[u8]> for Bytes<'src> {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl<'src> From<&'src [u8]> for Bytes<'src> {
    fn from(value: &'src [u8]) -> Bytes<'src> {
        Bytes::new(value)
    }
}

impl<'src> AsInner<[u8]> for Bytes<'src> {
    fn as_inner(&self) -> &'src [u8] {
        self.inner
    }
}

impl<'src, const N: usize> FromInner<&'src [u8; N]> for Bytes<'src> {
    fn from_inner(inner: &'src [u8; N]) -> Bytes<'src> {
        unsafe { Bytes::new(&inner[..]) }
    }
}
impl<'src> IntoInner<&'src [u8]> for Bytes<'src> {
    fn into_inner(self) -> &'src [u8] {
        self.inner
    }
}

impl<'src> Iterator for Bytes<'src> {
    type Item = &'src u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.iter().next()
    }
}

// TODO: Implement `Slice` for `Bytes` and other byte slices (See
// `zerocopy::ByteSlice`)
// impl<'src> Slice<'src> for Bytes<'src> {
//     fn slice(&self, range: Range<usize>) -> Result<&'src [u8]> {
//         todo!()
//     }

//     fn slice_from(&self, from: RangeFrom<usize>) -> Result<&'src [u8]> {
//         match self.inner.get(from) {
//             Some(bytes) => Ok(bytes),
//             None => Err(Error::out_of_bounds(from.start, self.len())),
//         }
//     }

//     fn slice_until<I, F>(&self, predicate: F) -> SliceUntil<&'src [u8], F>
//     where
//         F: FnMut(&u8) -> bool,
//         I: IntoIterator<Item = u8>,
//     {
//         SliceUntil::new(self.inner, predicate)
//     }

//     fn iter_stream(&self) -> Copied<Iter<'src, u8>> {
//         self.iter().copied()
//     }

//     fn enumerate_stream(&self) -> Enumerate<Copied<Iter<'src, u8>>> {
//         self.inner.iter().copied().enumerate()
//     }

//     fn slice_to(&self, to: RangeTo<usize>) -> Result<&'src [u8]> {
//         match self.inner.get(to) {
//             Some(bytes) => Ok(bytes),
//             None => Err(Error::out_of_bounds(to.end, self.len())),
//         }
//     }
// }
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::to_byte_array;

    #[test]
    fn array_from_bytes() {
        let data = *b"Something\0kernel32.dll\0ntdll.dll\0";
        let bytes = unsafe { Bytes::new(&data[..]) };
        let array = unsafe { to_byte_array(bytes.inner) };
        assert_eq!(&array, b"Something\0");

        let bytes = Bytes::new_offset(&data[..], array.len())
            .expect("offset would overflow Bytes instance");
        assert!(!bytes.is_empty());
        assert_eq!(bytes.buffer(), data);

        let mut stream = Bytes::new(&data[..]);
        let bytes_read = stream.read_until(|&b| b == b'\0');
        assert!(bytes_read.is_some());
    }
}
