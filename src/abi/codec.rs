//! Low-level encoding and decoding primitives.

use core::mem::size_of;

use crate::{Aligned, Error, Result};

mod bytes;
pub use bytes::BytesExt;

/// Concrete types that can be decoded from any arbitrary source of bytes, regardless
/// of their endianness.
///
/// # ABI
///
/// All types that implement [`Decodable`] must also implement [`Aligned`]. This
/// ensures that the output type is layout-compatible with the ABI defined by this
/// crate.
pub trait Decodable: Aligned {
    /// Reads an instance of `Self` from a slice of bytes where `Self: Aligned`.
    ///
    /// The remaining input from `bytes` is returned as the "tail", after a split
    /// operation is performed on the slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to violating size or alignment
    /// invariants.
    fn read_bytes(bytes: &[u8]) -> Result<(Self, &[u8])> {
        let size = size_of::<Self>();
        if size > bytes.len() {
            return Err(Error::buffer_overflow(size, bytes.len()));
        }

        let (bytes, remaining) = bytes.split_at(size);
        assert!(bytes.len() == size);
        assert!(
            bytes.len() == size,
            "`bytes.len()` ({}) must be same size as `size_of::<Self>()` {}",
            bytes.len(),
            size
        );
        assert!(bytes.len() == size);

        if !bytes.is_aligned_with::<Self>() {
            // `T` cannot safely be decoded from bytes; alignment check failed.
            return Err(Error::misaligned_access(bytes));
        }

        // SAFETY: Size requirements have been verified, the bytes we are reading from meet
        // the alignment requirements of the output type, and the output type is safe to cast
        // to as it implements Decodable.
        let res = unsafe { bytes.as_ptr().cast::<Self>().read() };
        assert!(res.validate_alignment());
        let min_align = res.min_align();
        assert_eq!(min_align, Self::MIN_ALIGN);
        Ok((res, remaining))
    }
}

/// Specialized trait for defining the logic required to handle converting a type
/// read from a byte stream into a concrete type `T` where `T: ReadBytes`.
///
/// This trait is used to allow customized decoding logic for individual types. This
/// provides additional control over how the type is decoded from the byte source.
pub trait Decode<T: Decodable>: Aligned {
    /// Decodes a concrete type `T` from an immutable reference to `self`.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to a size mismatch or misaligned
    /// read.
    fn decode(&self) -> crate::Result<T>;
}
