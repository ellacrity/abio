use core::borrow::BorrowMut;
use core::ops::{Bound, Deref, Range, RangeBounds};
use core::slice;

use crate::util::{AsInner, FromInner, IntoInner};

/// Wrapper type for a borrowed slice of bytes.
///
/// This type extends the `&[u8]` type, providing additional methods  wrapper around
/// a borrowed slice of bytes.
///
/// The [`Bytes`] type provides zero-cost abstractions, extending the `&[u8]` type by
/// providing  `#[repr(transparent)]` layout,
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
    #[inline]
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
    pub const fn as_slice(&self) -> &'src [u8] {
        self.inner
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
}

impl<'src> Bytes<'src> {
    /// Reads `count` bytes from the buffer.
    ///
    /// Returns `None` if `count` is greater than the length of the buffer.
    pub fn read(&self, count: usize) -> Option<&[u8]> {
        if self.inner.len() < count {
            None
        } else {
            // SAFETY: `self.len() >= size`, so we can read at least `size`
            // bytes without accessing memory out of bounds.
            Some(unsafe { self.inner.get_unchecked(..count) })
        }
    }

    /// Reads `count` bytes from the buffer, starting at `offset`.
    ///
    /// Returns `None` if `self.len() < offset + count`
    pub fn read_at(&self, offset: usize, count: usize) -> Option<&[u8]> {
        let (start, end) = (offset, offset + count);
        if end > self.len() {
            None
        } else {
            // SAFETY: `self.len() >= offset + size`, so we can read at least
            // `size` bytes, starting at `offset`, without accessing memory out of bounds.
            Some(unsafe { self.inner.get_unchecked(start..end) })
        }
    }

    /// Reads from the bytes buffer until a condition is satisifed.
    ///
    /// Returns `None` if the resulting slice is a ZST.
    pub fn read_until<F>(&self, predicate: F) -> Option<&[u8]>
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
            None
        } else {
            Some(unsafe { slice::from_raw_parts(self.inner.as_ptr(), pos) })
        }
    }
}

impl<'src> AsRef<[u8]> for Bytes<'src> {
    fn as_ref(&self) -> &[u8] {
        self.inner
    }
}

impl<'src> Deref for Bytes<'src> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_inner()
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
        Bytes::new(&inner[..])
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

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &[u8] = b"NtAllocateVirtualMemory\0kernel32.dll\0ntdll.dll\0";

    #[test]
    fn array_from_bytes() {
        let expected = SOURCE.get(0..23);

        let reader = Bytes::new(SOURCE);
        let actual = reader.read(23);
        assert_eq!(expected, actual);
        assert_eq!(actual, Some(&b"NtAllocateVirtualMemory"[..]));
    }

    #[test]
    fn reading_subslice_in_middle() {
        let expected = SOURCE.get(13 + 24..SOURCE.len() - 1);
        let reader =
            Bytes::new_offset(SOURCE, 13 + 24).expect("offset would overflow Bytes instance");
        let actual = reader.read_until(|&b| b == b'\0');

        assert_eq!(actual, expected);
        assert_eq!(actual, Some(&b"ntdll.dll"[..]));
    }
}
