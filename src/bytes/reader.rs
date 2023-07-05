//! Byte reading primitives.

use core::mem::size_of;
use core::ops::Range;

use crate::bytes::BytePos;
use crate::{Bytes, FromBytes, Pod};

/// Stateful parser for traversing a byte slice.
#[derive(Clone, Copy, Debug)]
pub struct Parser<'source> {
    buf: Bytes<'source>,
    pos: BytePos,
}

impl<'source> Parser<'source> {
    /// Creates a new [`Parser`] from a byte slice.
    pub const fn new(bytes: &'source [u8]) -> Self {
        Parser { buf: Bytes::new(bytes), pos: BytePos::new(0) }
    }

    /// Construct a new [Parser] given a byte slice and start `offset`.
    ///
    /// Returns `None` if `offset > bytes.len()`.
    pub const fn new_at_offset(bytes: &'source [u8], offset: usize) -> Option<Parser<'source>> {
        if offset > bytes.len() {
            None
        } else {
            Some(Parser { buf: Bytes::new(bytes), pos: BytePos::new(offset) })
        }
    }

    /// Advances the internal cursor by `count` bytes.
    pub fn advance(&mut self, count: usize) {
        self.pos += count;
    }

    /// Returns the remaining bytes in the buffer relative to the position of the
    /// cursor.
    pub fn chunk(&self) -> &[u8] {
        &self.buf.0[self.pos.get()..]
    }

    pub const fn bytes(&self) -> Bytes<'source> {
        self.buf
    }

    pub const fn position(&self) -> BytePos {
        self.pos
    }

    /// Get the remaining number of bytes in the buffer.
    pub const fn remaining(&self) -> usize {
        self.len().saturating_sub(self.pos.get())
    }

    /// Returns the total length of the input buffer.
    pub const fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns true iff the buffer is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn slice(&self, range: Range<usize>) -> Option<&[u8]> {
        self.chunk().get(range)
    }

    #[inline]
    pub fn read_sized<T: Pod, const SIZE: usize>(&mut self) -> Option<T> {
        if self.remaining() < T::SIZE.min(SIZE) {
            return None;
        }

        // `bytes` contains `T` and `tail` is the remaining bytes
        let bytes = &self.chunk()[..SIZE];
        debug_assert_eq!(bytes.len(), T::SIZE);
        debug_assert_eq!(bytes.len(), SIZE);
        debug_assert_eq!(bytes.len(), size_of::<T>());
        if bytes.is_aligned::<T>() {
            // SAFETY: Size and alignment checks have been performed.
            let res = unsafe { bytes.as_ptr().cast::<T>().read() };
            self.pos += T::SIZE;
            Some(res)
        } else {
            None
        }
    }
}

impl<'a> Parser<'a> {
    #[inline]
    pub fn read_ref<T: Pod>(&mut self) -> Option<&'a T> {
        let size = T::SIZE;
        if self.remaining() < size {
            return None;
        }

        // `bytes` contains `T` and `tail` is the remaining bytes
        let bytes = &self.chunk()[..size];
        debug_assert_eq!(bytes.len(), T::SIZE);
        debug_assert_eq!(bytes.len(), size_of::<T>());
        if bytes.is_aligned::<T>() {
            // SAFETY: Size and alignment checks have been performed.
            let res = unsafe { &*bytes.as_ptr().cast::<T>() };
            self.pos += size;
            Some(res)
        } else {
            None
        }
    }

    #[inline]
    pub fn read_ref_at<T: Pod>(&mut self, offset: usize) -> Option<&'a T> {
        let size = size_of::<T>();
        if self.remaining() < size + offset {
            return None;
        }

        // `bytes` contains `T` and `tail` is the remaining bytes
        let bytes = &self.chunk()[offset..size + offset];
        debug_assert_eq!(
            bytes.len(),
            size_of::<T>(),
            "length of bytes must be equial to size_of::<T>()"
        );
        if bytes.is_aligned::<T>() {
            return None;
        }

        // SAFETY: Size and alignment checks have been performed.
        let res = unsafe { &*bytes.as_ptr().cast::<T>() };
        self.pos += size;
        Some(res)
    }

    /// Returns the read of this [Parser].
    #[inline]
    pub fn read<T: FromBytes>(&mut self) -> Option<T> {
        let size = size_of::<T>();
        if self.remaining() < size {
            return None;
        }

        // `bytes` contains `T` and `tail` is the remaining bytes
        let Some(bytes) = self.chunk().get(..size) else { return None };
        if bytes.is_aligned::<T>() {
            // SAFETY: size and alignment have both been verified. This call
            // is sound so long as the bytes represent what the user claims they do.
            let value = unsafe { bytes.as_ptr().cast::<T>().read() };
            self.pos += size;
            Some(value)
        } else {
            None
        }
    }

    #[inline]
    fn fun_name<T: Pod>(&mut self, size: usize) -> Option<T> {
        let bytes = self.chunk().get(..T::SIZE)?;
        debug_assert_eq!(bytes.len(), T::SIZE);
        debug_assert_eq!(bytes.len(), size_of::<T>());
        if bytes.is_aligned::<T>() {
            // SAFETY: Size and alignment checks have been performed.
            let res = unsafe { bytes.as_ptr().cast::<T>().read() };
            self.pos += size;
            Some(res)
        } else {
            None
        }
    }
}

/// Extension methods for byte slices.
pub trait BytesExt {
    fn align_of<T: FromBytes>(&self) -> usize;

    fn is_aligned<T: FromBytes>(&self) -> bool;
}

impl BytesExt for [u8] {
    fn align_of<T: FromBytes>(&self) -> usize {
        self.as_ptr().addr() & (core::mem::align_of::<T>() - 1)
    }

    fn is_aligned<T: FromBytes>(&self) -> bool {
        self.align_of::<T>() == 0
    }
}

impl<'a> BytesExt for &'a [u8] {
    fn align_of<T: FromBytes>(&self) -> usize {
        self.as_ptr().addr() & (core::mem::align_of::<T>() - 1)
    }

    fn is_aligned<T: FromBytes>(&self) -> bool {
        self.align_of::<T>() == 0
    }
}
