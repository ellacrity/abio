//! Shims for unstable features and compatibility helpers.
//!
//! # Purpose
//!
//! This module provides drop-in replacements for some unstable features or
//! alternative implementations to ensure compatibility with different versions of
//! the Rust compiler. The functions here act as shims, providing
//! similar functionality to unstable or features that are close to being added to
//! the language.
//!
//! # Using the Shims
//!
//! This module exists for internal use, but you can enable it via the `shims`
//! feature. Be aware that enabling these functions are
//!
//! Utilities to make certain operations within this crate easier and less
//! repetitious.
//!
//! This module is important because, as the project grows, there will be an
//! increased need for utilities that help convert between types belonging to the
//! [`abio`][crate] crate.
#![allow(dead_code)]

use core::slice;

use crate::{Bytes, Chunk, Decode, Span};

/// Converts a byte slice into an array of a specified size.
///
/// This function attempts to convert the given `bytes` slice into an array
/// of type `[u8; N]`, where `N` is the desired size of the array.
///
/// # Returns
///
/// Returns `Some([u8; N])` if the conversion succeeds, where `array` is the
/// resulting array. If the conversion fails due to a size mismatch, `None` is
/// returned.
#[inline]
pub fn to_byte_array<const N: usize>(bytes: &[u8], offset: usize) -> Option<[u8; N]> {
    assert!(bytes.len() >= offset + N);
    let range = offset..offset + N;
    if let Ok(array) = <[u8; N]>::try_from(&bytes[range]) {
        Some(array)
    } else {
        None
    }
}

/// Directly casts a slice of bytes into a `[u8; N]` by casting it to the target
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
/// target type `[u8; N]`. Additionally, the byte representation of the array must
/// be valid for the target platform. If the alignment is not guaranteed, or the byte
/// order serialization does not match the target's endianness, it may lead to
/// **undefined behaviour**.
///
/// [to_byte_array]: crate::shims::to_byte_array
#[inline]
pub const unsafe fn to_byte_array_unchecked<const N: usize>(
    bytes: &[u8],
    offset: usize,
) -> [u8; N] {
    debug_assert!(bytes.len() >= offset + N);
    let bytes = unsafe {
        let data = bytes.as_ptr().add(offset);
        slice::from_raw_parts(data, N)
    };
    Decode::decode::<[u8; N]>(bytes)

    // unsafe { bytes.as_ptr().cast::<[u8; N]>().read() }
}

/// Returns a slice of bytes with a known size. This method is useful when working
/// with data structures with explicit size and/or alignment requirements, but when
/// the source of the data is dynamically sized.
///
/// This method "pretends" that we have a fixed size type, although the returned type
/// is a [`Bytes`] instance.
#[inline]
pub const fn take_slice(bytes: &[u8], size: usize) -> crate::Result<&Bytes> {
    Bytes::new(bytes).read(size)
}

#[inline]
pub const fn first_chunk<const N: usize>(bytes: &[u8]) -> crate::Result<Chunk<N>> {
    if bytes.len() < N {
        Err(crate::Error::out_of_bounds(N, bytes.len()))
    } else {
        // SAFETY: `bytes.len()` has at least `N` bytes we can use.
        let chunk_bytes = unsafe { slice::from_raw_parts(bytes.as_ptr(), N) };

        // SAFETY: The pointer cast is valid since the size has been validated, and `Chunk`
        // has no alignment requirements.
        let array = unsafe { chunk_bytes.as_ptr().cast::<[u8; N]>().read() };
        Ok(Chunk::new(array))
    }
}

#[inline]
pub const fn read_chunk_at<const N: usize>(bytes: &[u8], offset: usize) -> crate::Result<Chunk<N>> {
    if bytes.len() < N + offset {
        Err(crate::Error::out_of_bounds(N, bytes.len()))
    } else {
        // SAFETY: `bytes.len()` has at least `N` bytes we can use.
        let chunk_bytes = unsafe { slice::from_raw_parts(bytes.as_ptr(), N) };

        // SAFETY: The pointer cast is valid since the size has been validated, and `Chunk`
        // has no alignment requirements.
        let array = unsafe { chunk_bytes.as_ptr().cast::<[u8; N]>().read() };
        Ok(Chunk::new(array))
    }
}

#[inline]
pub const fn split_chunk_at<const N: usize>(
    source: &[u8],
    offset: usize,
) -> crate::Result<(Chunk<N>, &[u8])> {
    let span = Span::new(offset, N);
    let len = source.len();
    if len < span.end() {
        Err(crate::Error::out_of_bounds(span.size(), len))
    } else {
        let offset = span.start();
        let bytes = unsafe {
            let ptr = source.as_ptr().add(offset);
            slice::from_raw_parts(ptr, len - span.start())
        };
        // SAFETY: We manually verified the bounds of the split.
        let (first, tail) = unsafe { split_at_unchecked(bytes, N) };

        // SAFETY: We explicitly check for the correct number of elements,
        //   and do not let the references outlive the slice.
        let array = unsafe { first.as_ptr().cast::<[u8; N]>().read() };
        Ok((Chunk::new(array), tail))
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

/// Compares and returns the **minimum** of two values in a `const` context.
#[inline(always)]
pub const fn const_min_value(a: usize, b: usize) -> usize {
    // NOTE: We only perform the first check `a < b` because if this is false, then `a ==
    // b` or `a > b`.
    if a < b {
        a
    } else {
        b
    }
}

/// Compares and returns the **maximum** of two values in a `const` context.
#[inline(always)]
pub const fn const_max_value(a: usize, b: usize) -> usize {
    // NOTE: We only perform the first check `a > b` because if this is false, then `a ==
    // b` or `a < b`.
    if a > b {
        a
    } else {
        b
    }
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
