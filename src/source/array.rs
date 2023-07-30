use core::ptr::NonNull;

use crate::source::Chunk;
use crate::{Abi, Error, Integer, Result, Zeroable};

/// A fixed, statically sized chunk of data that can be read from the `Source`.
pub trait Array<const N: usize>: Abi {
    /// Decodes a chunk of bytes into an ABI-compatible type.
    ///
    /// # Errors
    ///
    /// Returns an error if `ptr` is null or otherwise invalid.
    fn from_ptr<A: Abi>(ptr: *const A) -> Result<Self>;
}

#[doc(hidden)]
pub trait ByteArray<const N: usize> {
    /// Converts an array of bytes with length `N` to `Self`.
    fn from_byte_array(bytes: [u8; N]) -> Self;
}

impl<T: Abi + Integer + Zeroable, const N: usize> Array<N> for T {
    #[inline]
    fn from_ptr<A: Abi>(ptr: *const A) -> Result<Self> {
        match NonNull::new(ptr.cast::<u8>().cast_mut()) {
            Some(ptr) => Ok(unsafe { ptr.as_ptr().cast::<Self>().read() }),
            None => Err(Error::incompatible_types()),
        }
    }
}

impl<const N: usize> ByteArray<N> for [u8; N] {
    #[inline(always)]
    fn from_byte_array(bytes: [u8; N]) -> Self {
        bytes
    }
}

impl<const N: usize> ByteArray<N> for Chunk<N> {
    #[inline(always)]
    fn from_byte_array(array: [u8; N]) -> Self {
        Chunk::new(array)
    }
}
