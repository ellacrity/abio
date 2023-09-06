//! Module containing a newtype wrapper for byte slices.
//!
//! This module provides an easier way to extend the API for `&[u8]` types, since the
//! [`Bytes`] type is local to the crate.

use core::borrow::Borrow;
use core::marker::PhantomData;
use core::ops::{Bound, Deref, Index, Range, RangeBounds, RangeFrom, RangeTo};
use core::slice;

use crate::util::{self, FromInner};
use crate::{Chunk, Error, Result};

/// Contiguous region of memory containing a borrowed sequence of bytes.
///
/// # Layout
///
/// The [`Bytes`] type provides an abstraction for [Dynamically Sized
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
/// [DST]: https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts
#[derive(Eq, Hash, Ord, PartialOrd)]
pub struct Bytes<'data> {
    ptr: *const u8,
    len: usize,
    _lifetime: PhantomData<&'data u8>,
}

impl Copy for Bytes<'_> {}
impl Clone for Bytes<'_> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'data> Bytes<'data> {
    /// Create a new [`Bytes`] type by wrapping a borrowed slice of bytes.
    #[inline(always)]
    pub const fn new(bytes: &'data [u8]) -> Bytes<'data> {
        // TODO: Figure out how exactly we want to handle ZST's, because they may be
        // desirable in certain situations. I would prefer to be able to somehow support
        // certain ZSTs. (ellacrity)
        assert!(
            !bytes.is_empty(),
            "Illegal `Bytes` construction. Cannot construct a `Bytes` instance from a ZST."
        );

        Bytes {
            ptr: bytes.as_ptr(),
            len: bytes.len(),
            _lifetime: PhantomData,
        }
    }

    /// Creates a new [`Bytes`] instance from a given slice of bytes and offset.
    ///
    /// # Errors
    ///
    /// Returns an error if `bytes.len() < offset`. Failing to perform this bounds
    /// check would cause potential UB as the offset could end up pointing past the
    /// end bound of the allocated byte slice object.
    #[inline]
    pub const fn new_with_offset(bytes: &'data [u8], offset: usize) -> Result<Bytes<'data>> {
        if bytes.len() < offset {
            Err(Error::out_of_bounds(offset, bytes.len()))
        } else {
            Ok(unsafe { Bytes::new_with_offset_unchecked(bytes, offset) })
        }
    }

    /// Splits a byte slice at `offset` into two parts, using the given `offset`.
    /// [`Bytes`] instance given a byte slice, an offset, and a size.
    ///
    /// # Errors
    ///
    /// This function returns an error if `bytes.len() < offset + size`.
    #[inline]
    pub const fn new_from_split_at(
        bytes: &'data [u8],
        offset: usize,
    ) -> Result<(Bytes<'data>, Bytes<'data>)> {
        let needed = offset;
        if bytes.len() < needed {
            Err(Error::out_of_bounds(needed, bytes.len()))
        } else {
            // SAFETY: The returned slices are bound by the lifetime `data, so they are valid as
            // long as the data `bytes` references is live. The bounds check above ensures that
            // `offset` is a valid offset and so this is a safe operation.
            let (head, tail) = unsafe { util::split_at_unchecked(bytes, offset) };
            Ok((Bytes::new(head), Bytes::new(tail)))
        }
    }

    /// Creates a [`Bytes`] instance from a slice of bytes and and offset, without
    /// checking that `offset` is within bounds of `bytes`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes.len() >= offset`. Violating this invariant
    /// is immediate **undefined behaviour**.
    #[inline]
    const unsafe fn new_with_offset_unchecked(bytes: &'data [u8], offset: usize) -> Bytes<'data> {
        debug_assert!(
            bytes.len() >= offset,
            "Cannot construct Bytes instance with `offset > bytes.len()`"
        );
        let data = bytes.as_ptr().add(offset);
        let len = bytes
            .len()
            .saturating_sub(offset);
        Bytes::from_raw_parts(data, len)
    }

    /// Constructs a new [`Bytes`] instance from a [`Chunk`] with size `N`.
    ///
    /// The function helps convert a chunk of bytes with a fixed size into a byte
    /// slice with the same size.
    #[inline]
    pub const fn from_chunk<'chunk, const N: usize>(chunk: &'chunk Chunk<N>) -> Bytes<'data> {
        Bytes { ptr: chunk.as_ptr(), len: N, _lifetime: PhantomData }
    }

    /// Creates a [`Bytes`] instance from a pointer and a length.
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
    pub(crate) const unsafe fn from_raw_parts(data: *const u8, len: usize) -> Bytes<'data> {
        // SAFETY: Please refer to the above safety documentation. This constructor is
        // subject to the same safety invariants as `core::ptr::from_raw_parts`.
        Bytes::new(unsafe { slice::from_raw_parts(data, len) })
    }

    /// Acquires a pointer to the first byte of the underlying slice.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    /// Acquires a pointer to the last byte of the underlying slice.
    #[inline(always)]
    pub const fn as_end_ptr(&self) -> *const u8 {
        // SAFETY: A valid instance of this type means that any pointer within bounds of the
        // allocated object is valid.
        unsafe { self.as_ptr().add(self.len) }
    }

    /// Slices the buffer using a range expression, returning the resulting bytes.
    ///
    /// This is a convenience method for performing indexing operations. Note that
    /// this method is not `const`, like many of the other methods in this module.
    ///
    /// # Panics
    ///
    /// This method panics if the range is outside the bounds of the byte slice.
    #[inline]
    pub fn slice_range(&'data self, range: impl RangeBounds<usize>) -> &'data [u8] {
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
        let start = self.ptr.cast::<u8>();
        // SAFETY: The indices represented by `start` and `self.len()` both fall within the
        // bounds of the slice. For additional information, refer to the official
        // documentation: https://doc.rust-lang.org/core/primitive.slice.html#method.as_ptr_range
        let end = unsafe { start.add(self.len()) };
        start..end
    }

    /// Returns the inner byte slice comprising the [`Bytes`] instance.
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        // SAFETY: The byte slice comprising this instance was constructed using a validated
        // pointer and length.
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    /// Returns an iterator over the slice.
    ///
    /// The iterator yields all items from start to end.
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, u8> {
        // SAFETY: This is safe because Bytes can only be constructed with a valid slice,
        // so the pointer and length must represent a valid slice.
        self.as_slice().iter()
    }

    pub fn as_chunk<const N: usize>(&self) -> Option<&Chunk<N>> {
        if self.len != N {
            None
        } else {
            Chunk::from_ptr(self.ptr)
        }
    }

    /// Returns the number of available bytes in the slice.
    ///
    /// This function is equivalent to all of the bytes that have not yet been
    /// consumed.
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

    /// Returns a subslice of this [`Bytes`] instance, using a [`Range<usize>`] type
    /// to perform the indexing operation.
    ///
    /// # Errors
    ///
    /// Returns an error if `range.start >= range.end` (resulting in a ZST) or if the
    /// range is out of bounds of the underlying byte slice.
    #[inline]
    #[allow(dead_code)]
    const fn subslice(&self, range: Range<usize>) -> Result<&[u8]> {
        debug_assert!(
            range.start < range.end,
            "Illegal range value. Cannot construct a `Range` type where `start >= end`."
        );

        if self.len < range.end {
            Err(Error::out_of_bounds(range.end, self.len))
        } else {
            // SAFETY: Bounds checks ensure that the ptr to this slice is within bounds of
            // `self`, and `size <= self.len`.
            Ok(unsafe {
                slice::from_raw_parts(
                    self.as_slice()
                        .as_ptr()
                        .add(range.start),
                    range
                        .end
                        .saturating_sub(range.start),
                )
            })
        }
    }

    /// Returns a subslice of the input
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub(crate) const fn slice_at(&self, offset: usize, len: usize) -> Result<Bytes<'data>> {
        Ok(unsafe {
            let data = self.as_ptr().add(offset);
            Bytes::from_raw_parts(data, len)
        })
    }
}

impl<'data> AsRef<[u8]> for Bytes<'data> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'data> Borrow<[u8]> for Bytes<'data> {
    fn borrow(&self) -> &[u8] {
        self.deref()
    }
}

impl<'data> Deref for Bytes<'data> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'data> From<&'data [u8]> for Bytes<'data> {
    #[inline(always)]
    fn from(slice: &'data [u8]) -> Bytes<'data> {
        Bytes::new(slice)
    }
}

impl<'data> Index<RangeFrom<usize>> for Bytes<'data> {
    type Output = [u8];

    #[inline]
    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.as_slice()[range]
    }
}

impl<'data> Index<RangeTo<usize>> for Bytes<'data> {
    type Output = [u8];

    #[inline]
    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.as_slice()[range]
    }
}

impl<'data, const N: usize> FromInner<&'data [u8; N]> for Bytes<'data> {
    fn from_inner(inner: &'data [u8; N]) -> Bytes<'data> {
        Bytes::new(&inner[..])
    }
}

impl<'data> IntoIterator for &'data Bytes<'data> {
    type Item = &'data u8;

    type IntoIter = slice::Iter<'data, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'data> PartialEq<[u8]> for Bytes<'data> {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self == other
    }
}

impl<'data, S: AsRef<[u8]>> PartialEq<S> for Bytes<'data> {
    #[inline]
    fn eq(&self, other: &S) -> bool {
        self.as_slice()
            .eq(other.as_ref())
    }
}

impl<'data> PartialEq<Bytes<'data>> for &'data [u8] {
    #[inline]
    fn eq(&self, other: &Bytes<'data>) -> bool {
        self == other
    }
}
