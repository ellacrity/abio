use core::ptr::NonNull;

use crate::contiguous::Chunk;
use crate::{Abi, BytesOf, Error, Integer, Result, Zeroable};

/// A fixed, statically sized chunk of data that can be read from the `Source`.
pub trait Array<const N: usize>: Abi {
    /// Decodes a chunk of bytes into an ABI-compatible type.
    ///
    /// # Errors
    ///
    /// Returns an error if `ptr` is null or otherwise invalid.
    fn from_ptr<A: Abi>(ptr: *const A) -> Result<Self>;

    /// Converts an [`Integer`] type into its equivalent [`Array`] type.
    ///
    /// The length of the returned array is the number of bytes comprising the
    /// integer value.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails.
    fn from_integer<I: Integer>(integer: I) -> Result<Self>;
}

impl<T: Abi + Integer + Zeroable, const N: usize> Array<N> for T {
    #[inline]
    fn from_ptr<A: Abi>(ptr: *const A) -> Result<Self> {
        match NonNull::new(ptr.cast::<u8>().cast_mut()) {
            Some(ptr) => Ok(unsafe { ptr.as_ptr().cast::<Self>().read() }),
            None => Err(Error::incompatible_types()),
        }
    }

    fn from_integer<I: Abi + BytesOf + Integer>(integer: I) -> Result<Self> {
        if let Ok(chunk) = Chunk::<N>::from_bytes(integer.bytes_of()) {
            Ok(unsafe { chunk.read::<Self>() })
        } else {
            Err(Error::incompatible_types())
        }
    }
}

// impl<const N: usize> Array<N> for u8 {
//     fn from_ptr<A: Abi>(ptr: *const A) -> Result<Self> {
//         match NonNull::new(ptr.cast_mut()) {
//             Some(ptr) => {
//                 let ptr = ptr.as_ptr().cast::<Self>();
//             }
//             None => todo!(),
//         }
//         unsafe { slice::from_raw_parts(ptr, Self::SIZE) }
//     }

//     fn from_integer<I: Integer>(integer: I) -> Result<Self> {
//         todo!()
//     }
// }
