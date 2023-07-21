//! Utilities to make certain operations within this crate easier and less
//! repetitious.
//!
//! This module is important because, as the project grows, there will be an
//! increased need for utilities that help convert between types belonging to the
//! [`abio`][crate] crate.
#![allow(dead_code)]

use core::slice;

pub(crate) mod sealed;

use crate::{Chunk, Span};

/// Converts a byte slice into an array of a specified size.
///
/// This function attempts to convert the given `bytes` slice into an array
/// of type `[u8; SIZE]`, where `SIZE` is the desired size of the array.
///
/// # Returns
///
/// Returns `Some(array)` if the conversion succeeds, where `array` is the
/// resulting array. If the conversion fails due to a size mismatch, `None` is
/// returned.
#[inline]
pub fn to_byte_array<const SIZE: usize>(bytes: &[u8], offset: usize) -> Option<[u8; SIZE]> {
    assert!(bytes.len() >= offset + SIZE);
    let range = offset..offset + SIZE;
    if let Ok(array) = <[u8; SIZE]>::try_from(&bytes[range]) {
        Some(array)
    } else {
        None
    }
}

/// Directly casts a slice of bytes into a `[u8; SIZE]` by casting it to the target
/// type and reading the value.
///
/// This method is unsafe as the size of the byte slice is not validated before
/// performing the conversion. As such, you should favor the [safe
/// version][to_byte_array] instead. If you absolutely certain that you have a valid
/// byte slice, then this method offers slight performance gains.
///
/// # Safety
///
/// The caller must ensure that the provided slice is correctly aligned for the
/// target type `[u8; SIZE]`. Additionally, the byte representation of the array must
/// be valid for the target platform. If the alignment is not guaranteed, or the byte
/// order serialization does not match the target's endianness, it may lead to
/// **undefined behaviour**.
///
/// [to_byte_array]: crate::util::to_byte_array
#[inline]
pub const unsafe fn to_byte_array_unchecked<const SIZE: usize>(
    bytes: &[u8],
    offset: usize,
) -> [u8; SIZE] {
    debug_assert!(bytes.len() >= offset + SIZE);
    let bytes = unsafe {
        let data = bytes.as_ptr().add(offset);
        slice::from_raw_parts(data, SIZE)
    };

    unsafe { bytes.as_ptr().cast::<[u8; SIZE]>().read() }
}

#[inline]
pub const fn split_first_chunk<const N: usize>(bytes: &[u8]) -> Option<(Chunk<N>, &[u8])> {
    if bytes.len() < N {
        None
    } else {
        // SAFETY: We manually verified the bounds of the split.
        let (first, tail) = unsafe { split_at_unchecked(bytes, N) };

        // SAFETY: We explicitly check for the correct number of elements,
        //   and do not let the references outlive the slice.
        let array = unsafe { first.as_ptr().cast::<[u8; N]>().read() };
        Some((Chunk::new(array), tail))
    }
}

#[inline]
pub const fn split_chunk_at<const N: usize>(
    bytes: &[u8],
    offset: usize,
) -> Option<(Chunk<N>, &[u8])> {
    let span = Span::new(offset, N);
    let len = bytes.len();
    if len < span.end() {
        None
    } else {
        let bytes = unsafe {
            let ptr = bytes.as_ptr().add(span.start());
            slice::from_raw_parts(ptr, len - span.start())
        };
        // SAFETY: We manually verified the bounds of the split.
        let (first, tail) = unsafe { split_at_unchecked(bytes, N) };

        // SAFETY: We explicitly check for the correct number of elements,
        //   and do not let the references outlive the slice.
        let array = unsafe { first.as_ptr().cast::<[u8; N]>().read() };
        Some((Chunk::new(array), tail))
    }
}

/// Splits a slice of bytes in two at `offset`, returning a pair of byte slices.
///
/// # Hack
///
/// This is a temporary hack to make this operation `const`. This will be removed
/// when the feature is stabilized.
#[inline]
#[must_use]
pub const unsafe fn split_at_unchecked(bytes: &[u8], offset: usize) -> (&[u8], &[u8]) {
    // FIXME: Remove `const` hack function when feature is stabilized
    debug_assert!(bytes.len() >= offset);
    let range = bytes.as_ptr()..bytes.as_ptr().add(offset);
    (
        slice::from_raw_parts(range.start, offset),
        slice::from_raw_parts(range.end, bytes.len() - offset),
    )
}

#[doc(hidden)]
pub trait IntoInner<T> {
    fn into_inner(self) -> T;
}

#[doc(hidden)]
pub trait AsInner<T: ?Sized> {
    fn as_inner(&self) -> &T;
}

#[doc(hidden)]
pub trait AsInnerMut<T: ?Sized> {
    fn as_inner_mut(&mut self) -> &mut T;
}

#[doc(hidden)]
pub trait FromInner<T: ?Sized> {
    fn from_inner(inner: T) -> Self;
}

#[doc(hidden)]
pub trait FromInnerMut<T: ?Sized> {
    fn from_inner_mut(inner: &mut T) -> &mut Self;
}

#[doc(hidden)]
pub trait FromInnerRef<T: ?Sized> {
    fn from_inner_ref(inner: &T) -> &Self;
}
