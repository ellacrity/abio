//! This module contains extensions to types that are represented as raw bytes.
//!
//! Note that this includes `&str`, as the inner layout of `&str` is the same as
//! `&[u8]`.

use core::mem::{self};
use core::slice;

use crate::{Abi, Zeroable};

/// Types that can be represented as slices of raw bytes.
///
/// Any type that may potentially serve as an input [`Source`] must implement this
/// trait.
///
/// # Derive
///
/// To ensure your type is compatible with [`abio`][crate], is it recommended that
/// you use the derivezeroed macro. The derive macro will verify, at compile time,
/// that your type meets the requirements of this trait and that any safety invarants
/// are upheld.
///
/// # Safety
///
/// // TODO: Safety docs; explain why it is unsafe to implement this trait
/// Implementors of this trait must ensure that their type fulfills the contract
/// defined by the trait. You are strongly encouraged to derive this trait for your
/// type if possible.
pub unsafe trait AsBytes {
    fn as_bytes(&self) -> &[u8] {
        // We have a reference already, so we know `self` is a valid reference. We can create
        // a pointer to a reference. The size of the slice is determined via the
        // `mem::size_of_val` function to get the number of bytes comprising `Self`.
        unsafe {
            let data = <*const Self>::from(self).cast::<u8>();
            slice::from_raw_parts(data, mem::size_of_val(self))
        }
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        // We have a reference already, so we know `self` is a valid reference. We can create
        // a pointer to a reference. The size of the slice is determined via the
        // `mem::size_of_val` function to get the number of bytes comprising `Self`.
        unsafe {
            let bptr = <*mut Self>::from(self).cast::<u8>();
            slice::from_raw_parts_mut(bptr, mem::size_of_val(self))
        }
    }
}

unsafe impl AsBytes for [u8] {
    fn as_bytes(&self) -> &[u8] {
        self
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        self
    }
}

unsafe impl<T: Abi + Zeroable> AsBytes for T {}
