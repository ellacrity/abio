use core::ptr::NonNull;

use crate::contiguous::Chunk;
use crate::{Abi, Error, Integer, Result};

/// A fixed, statically sized chunk of data that can be read from the `Source`.
///
/// This is implemented for `u8`, as well as byte arrays `&[u8; 1]` to `&[u8;
/// 32]`.
pub trait Array<'source, const N: usize>: Abi {
    /// Decodes a chunk of bytes into an ABI-compatible type.
    ///
    /// # Errors
    ///
    /// Returns an error if `ptr` is null or otherwise invalid.
    fn from_ptr<A: Abi>(ptr: *const A) -> Result<Self>;

    /// Converts an [`Integer`] type with size and alignment requirements into a
    /// fixed size [`Array`] type.
    ///
    /// The length of the returned array is the number of bytes comprising the
    /// integer value.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails.
    fn from_integer<I: Integer>(integer: I) -> Result<Self>;
}

impl<'source, const N: usize> Array<'source, N> for [u8; N] {
    #[inline]
    fn from_ptr<T: Abi>(ptr: *const T) -> Result<Self> {
        match NonNull::new(ptr.cast_mut()) {
            Some(ptr) => {
                let ptr = ptr.as_ptr().cast::<[u8; N]>();
                Ok(unsafe { ptr.read() })
            }
            None => Err(Error::incompatible_types()),
        }
    }

    fn from_integer<I: Integer>(integer: I) -> Result<Self> {
        if let Ok(chunk) = Chunk::<N>::from_bytes(integer.as_bytes()) {
            Ok(chunk.into_array())
        } else {
            Err(Error::incompatible_types())
        }
    }
}
