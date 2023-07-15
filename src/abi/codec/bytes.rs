//! This module contains extensions to types that are represented as raw bytes.
//!
//! Note that this includes `&str`, as the inner layout of `&str` is the same as
//! `&[u8]`.

use core::mem::size_of_val;
use core::slice;

mod sealed;

pub unsafe trait BytesOf {
    fn as_bytes_of(&self) -> &[u8] {
        unsafe {
            let this = self as *const Self;
            let data = <*const _>::from(self).cast::<u8>();
            let len = size_of_val(self);
            assert_eq!(data.cast::<u8>().addr(), data.cast::<u8>().addr());
            slice::from_raw_parts(data, len)
        }
    }
}

unsafe impl<T: ?Sized> BytesOf for T {
    fn as_bytes_of(&self) -> &[u8] {
        unsafe {
            let data = <*const T>::from(self).cast::<u8>();
            let other = self as *const Self;
            let len = size_of_val(self);
            assert_eq!(data.addr(), other.cast::<u8>().addr());
            slice::from_raw_parts(data, len)
        }
    }
}
