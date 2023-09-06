use core::mem;

use crate::{Abi, Chunk};

/// A fixed, statically sized chunk of data that can be read from the `Source`.
pub trait Array<'data>: Sized + Copy + PartialEq + Eq {
    type Item: Abi;

    /// Associated constant defining the size of the array.
    const SIZE: usize;

    fn read_le(self) -> Self;

    fn read_be(self) -> Self;

    fn read_ne(self) -> Self;

    /// Encodes the data behind a pointer to some allocated region of memory,
    /// returning `Self` as an array.
    ///
    /// # Errors
    ///
    /// Returns an error if the pointer is invalid, points to guarded or inaccessible
    /// memory, or if the resulting length of data overflows the allocated object
    /// derived from `ptr`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `ptr` is valid, pointing to an existing slice of
    /// memory and that `Self` does not outlive this reference.
    fn try_read_ptr(ptr: *const u8) -> crate::Result<&'data Self>;
}

const ONE_BYTE: usize = mem::size_of::<u8>();

impl<'data> Array<'data> for u8 {
    const SIZE: usize = ONE_BYTE;

    #[inline]
    fn try_read_ptr(ptr: *const u8) -> crate::Result<&'data Self> {
        // SAFETY: We have a pointer that is not null and the alignment requirement is 1,
        // so no alignment checks are required.
        if ptr.is_null() {
            Err(crate::Error::null_reference())
        } else {
            Ok(unsafe { &*ptr })
        }
    }
}

impl<'data, const N: usize> Array<'data> for &'data [u8; N] {
    const SIZE: usize = N;

    #[inline]
    fn try_read_ptr(ptr: *const u8) -> crate::Result<&'data Self> {
        // SAFETY: We have a pointer that is not null and the alignment requirement is 1,
        // so no alignment checks are required.
        // (ptr as *const [u8; N]).as_ref()
        if ptr.is_null() {
            Err(crate::Error::null_reference())
        } else {
            unsafe { *(&ptr as *const [u8; N]).cast::<Self>() }
        }
    }
}

impl<'data, const N: usize> Array<'data> for &'data Chunk<N> {
    const SIZE: usize = N;

    #[inline]
    fn try_read_ptr(ptr: *const u8) -> crate::Result<&'data Chunk<N>> {
        #[cfg(target_endian = "little")]
        {
            // SAFETY: We have a pointer that is not null and the alignment requirement is 1,
            // so no alignment checks are required. See the `core::ptr::as_ref` safety
            // documentation for additional information.
            let raw_chunk = unsafe { Chunk::try_from_raw_parts(ptr, Self::SIZE) };
            match raw_chunk {
                Ok(chunk) => chunk.to_le(),
                Err(e) => return Err(e),
            }
        }
        #[cfg(not(target_endian = "little"))]
        {
            // SAFETY: We have a pointer that is not null and the alignment requirement is 1,
            // so no alignment checks are required. See the `core::ptr::as_ref` safety
            // documentation for additional information.
            let raw_chunk = unsafe { Chunk::try_from_raw_parts(ptr, Self::SIZE) };
            match raw_chunk {
                Ok(chunk) => chunk.to_be(),
                Err(e) => return Err(e),
            }
        }
    }
}
