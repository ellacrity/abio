use core::ops::Range;
use core::slice;

use crate::util::{FromInner, IntoInner};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bytes<'source> {
    inner: &'source [u8],
}

impl<'a> Bytes<'a> {
    /// Creates a new [Bytes] instance from a slice of bytes.
    pub const fn new(bytes: &'a [u8]) -> Bytes<'a> {
        Bytes::_new(bytes, 0)
    }

    /// Creates a new [Bytes] instance from a given slice of bytes and offset.
    ///
    /// # Panics
    ///
    /// Panics if `bytes.len() < offset`.
    pub const fn new_with_offset(bytes: &'a [u8], offset: usize) -> Bytes<'a> {
        Bytes::_new(bytes, offset)
    }

    // Private constructor responsible for enforcing any invariants.
    #[inline]
    const fn _new(bytes: &'a [u8], offset: usize) -> Bytes<'a> {
        assert!(!bytes.is_empty(), "ZST's are currently unsupported for this type.");
        assert!(
            bytes.len() >= offset,
            "Cannot construct Bytes instance with `offset > bytes.len()`"
        );
        // "Cannot construct Bytes instance with `bytes.len() < offset`."
        let inner = unsafe {
            let data = bytes.as_ptr().add(offset).cast::<u8>();
            slice::from_raw_parts(data, bytes.len())
        };
        Bytes { inner }
    }

    /// Creates a new [`Bytes`] instance from a slice of bytes and `offset` into the
    /// slice.
    ///
    /// Returns `None` if `bytes.is_empty()` or if `bytes.len() < offset`.
    pub const fn new_offset(bytes: &'a [u8], offset: usize) -> Option<(Self, Self)> {
        if bytes.is_empty() || bytes.len() < offset {
            None
        } else {
            let (head, tail) = bytes.split_at(offset);
            Some((Bytes::_new(head, 0), Bytes::_new(tail, 0)))
        }
    }

    /// Returns the inner byte slice of this [Bytes] instance.
    #[inline]
    pub const fn chunk(&self) -> &'a [u8] {
        self.inner
    }

    #[inline]
    pub fn chunk_sized<const N: usize>(&self) -> Option<[u8; N]> {
        const_array_from_bytes::<N>(self.inner)
    }

    pub const fn as_ptr_range(&self) -> Range<*const u8> {
        self.inner.as_ptr_range()
    }

    pub fn iter(&self) -> slice::Iter<'a, u8> {
        self.inner.iter()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<'a> AsRef<[u8]> for Bytes<'a> {
    fn as_ref(&self) -> &[u8] {
        self.chunk()
    }
}

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(value: &'a [u8]) -> Bytes<'a> {
        Bytes::new(value)
    }
}

impl<'a> FromInner<&'a [u8]> for Bytes<'a> {
    fn from_inner(inner: &'a [u8]) -> Bytes<'a> {
        Bytes::from(inner)
    }
}
impl<'a> IntoInner<&'a [u8]> for Bytes<'a> {
    fn into_inner(self) -> &'a [u8] {
        self.inner
    }
}

const fn const_array_from_bytes<const CAP: usize>(bytes: &[u8]) -> Option<[u8; CAP]> {
    if bytes.len() < CAP {
        return None;
    }

    let mut buf = [0u8; CAP];
    let mut pos = 0;
    while pos < CAP {
        buf[pos] = bytes[pos];
        pos += 1;
    }
    Some(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_from_bytes() {
        let data = *b"Something\0kernel32.dll\0ntdll.dll\0";
        let array = const_array_from_bytes::<33>(&data[..]).unwrap();
        assert!(!array.is_empty() && array.len() == 33);
        assert_ne!(array, [0u8; 33]);
        let bytes = Bytes::new(&data[..]);
        assert!(!bytes.is_empty());
        assert_eq!(bytes.chunk(), data);
    }
}
