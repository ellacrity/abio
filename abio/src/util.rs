//! Utilities for common functionality, unstable features and compatibility helpers.
//!
//! # Purpose
//!
//! This module provides drop-in replacements for some unstable features or
//! alternative implementations to ensure compatibility with different versions of
//! the Rust compiler. Some of the functions here act as shims, providing
//! similar functionality to unstable or features that are close to being added to
//! the language.

use crate::{Abi, Alignment, Endianness, Error, Result};

#[macro_use]
mod macros;

#[doc(hidden)]
mod derive;
pub use derive::*;

#[doc(hidden)]
mod internal;
#[allow(unused_imports)]
pub(crate) use internal::{array_assume_init, array_assume_init_reversed, split_at_unchecked};

/// Interprets a slice of bytes as a reference of type `&T` where `T` is [`Abi`].
///
/// Types that implement the `Abi` trait have a known, predictable layout and can
/// be decoded directly from a slice of bytes.
///
/// # Safety
///
/// The caller must ensure that their custom type fulfills the contract enforced by
/// the `Abi` trait. Although this function performs bounds checking and layout
/// validations, it is still possible to introduce **undefined behaviour** into an
/// application. This function should be used with caution, preferring the
/// `Encode` trait instead if possible.
#[inline]
#[allow(dead_code)]
pub(crate) fn read_aligned_bytes<T, E, const N: usize>(bytes: &[u8]) -> Result<T, Error>
where
    T: Abi,
    E: Endianness,
{
    debug_assert_eq!(T::SIZE, N);

    if bytes.len() != N {
        Err(Error::size_mismatch(N, bytes.len()))
    } else if T::IS_ZST {
        Err(Error::null_reference())
    } else {
        // Error::read_failed("failed to read from bytes into array")
        let array = if E::ENDIAN.is_native_endian() {
            match internal::array_assume_init::<N>(bytes) {
                Ok(array) => array,
                Err(e) => Err(e),
            }
        } else {
            match internal::array_assume_init_reversed::<N>(bytes) {
                Ok(array) => array,
                Err(e) => Err(e),
            }
        };

        let array_ptr = array.as_ptr();
        if !array_ptr.is_aligned_with::<T>() {
            Err(Error::misaligned_access::<T>(array_ptr))
        } else {
            // SAFETY: The checks performed above ensure that the bytes slice contains memory
            // that matches the layout of the ABI-compatible type `T`. It is safe to create a
            // reference to `T` since the following invariants are satisifed:
            // * `bytes.len() == T::SIZE`
            // * `T` is not a ZST
            // * `bytes.as_ptr()` lies at an address compatible with the alignment requirements
            //   of `T`
            Ok(unsafe { array_ptr.cast::<T>().read() })
        }
    }
}

fn fun_name<'data, T, E, const N: usize>(array: [u8; N]) -> Result<&'data T>
where
    T: Abi,
    E: Endianness,
{
}

/// Converts a slice of bytes to an array, reading the first `LEN` bytes. If the
/// `REVERSE` flag is set, the bytes will be returned in reverse order.
///
/// # Const
///
/// This function is essentially a hack and work-around to `constify` this function,
/// allowing it to be evaluated at compile time rather than runtime.
pub(crate) const fn copy_to_array<const N: usize, const REVERSE: bool>(bytes: &[u8]) -> [u8; N] {
    assert!(bytes.len() >= N);
    if REVERSE {
        let mut buf = [0u8; N];
        let mut pos = 0;
        let idx = N - 1 - pos;
        while pos < N {
            buf[pos] = bytes[idx];
            pos += 1;
        }
        buf
    } else {
        let mut pos = 0;
        let mut buf = [0u8; N];
        let idx = pos;
        while pos < N {
            buf[pos] = bytes[pos];
            pos += 1;
        }
        buf
    }
}

/// Converts a slice of bytes to an array, reading the first `LEN` bytes. If the
/// `REVERSE` flag is set, the bytes will be returned in reverse order.
///
/// # Const
///
/// This function is essentially a hack and work-around to `constify` this function,
/// allowing it to be evaluated at compile time rather than runtime.
pub const fn read_endian_bytes<E: Endianness, const LEN: usize>(
    bytes: &[u8],
) -> crate::Result<[u8; LEN]> {
    if E::ENDIAN.is_native_endian() {
        internal::array_assume_init::<LEN>(bytes)
    } else {
        internal::array_assume_init_reversed::<LEN>(bytes)
    }
}

#[doc(alias = "from_ne_bytes")]
#[inline(always)]
pub const fn read_ne_bytes<const LEN: usize>(src: &[u8]) -> [u8; LEN] {
    internal::array_assume_init(src)
}

#[doc(alias = "from_le_bytes")]
#[inline(always)]
pub const fn read_le_bytes<const LEN: usize>(src: &[u8]) -> [u8; LEN] {
    #[cfg(target_endian = "little")]
    {
        internal::array_assume_init::<LEN>(src)
    }
    #[cfg(not(target_endian = "little"))]
    {
        internal::array_assume_init_reversed::<LEN>(src)
    }
}

#[doc(alias = "from_be_bytes")]
#[inline(always)]
pub const fn read_be_bytes<const LEN: usize>(src: &[u8]) -> [u8; LEN] {
    #[cfg(target_endian = "big")]
    {
        internal::array_assume_init::<LEN>(src)
    }
    #[cfg(not(target_endian = "big"))]
    {
        internal::array_assume_init_reversed::<LEN>(src)
    }
}

/// Compares and returns the **minimum** value between the two.
///
/// # CTFE
///
/// This is a hack, essentially, to allow calculating the minimum value within a
/// `const` context. Unfortunately, the API provided by [`core::cmp::min`] does not
/// currently support this.
#[allow(dead_code)]
#[inline(always)]
pub const fn const_min_value(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

/// Compares and returns the **maximum** value between the two.
///
/// # CTFE
///
/// This is a hack, essentially, to allow calculating the maximum value within a
/// `const` context. Unfortunately, the API provided by [`core::cmp::max`] does not
/// currently support this.
#[allow(dead_code)]
#[inline(always)]
pub const fn const_max_value(lhs: usize, rhs: usize) -> usize {
    if lhs > rhs {
        lhs
    } else {
        rhs
    }
}

#[doc(hidden)]
#[const_trait]
pub trait IntoInner<T> {
    fn into_inner(self) -> T;
}

#[doc(hidden)]
#[const_trait]
pub trait AsInner<T: ?Sized> {
    fn as_inner(&self) -> &T;
}

#[doc(hidden)]
#[const_trait]
pub trait AsInnerMut<T: ?Sized> {
    fn as_inner_mut(&mut self) -> &mut T;
}

#[doc(hidden)]
#[const_trait]
pub trait FromInner<T: ?Sized> {
    fn from_inner(inner: T) -> Self;
}
