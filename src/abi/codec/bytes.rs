//! This module contains extensions to types that are represented as raw bytes.
//!
//! Note that this includes `&str`, as the inner layout of `&str` is the same as
//! `&[u8]`.

use crate::Aligned;

// TODO: Improve documentation of this module and the `BytesExt` methods.

/// Extension methods for types representable as byte slices.
///
/// This trait extends functionality of these types by providing methods for working
/// with data that has specific alignment requirements.
pub trait BytesExt: private::Sealed {
    /// Returns the alignment of the bytes relative to `T`.
    fn align_of_bytes<T: Aligned>(&self) -> usize;

    /// Returns true iff the the byte slice represented by `self` are aligned with
    /// `T`.
    fn is_aligned_with<T: Aligned>(&self) -> bool;
}

impl BytesExt for [u8] {
    fn align_of_bytes<T: Aligned>(&self) -> usize {
        self.as_ptr().addr() & (core::mem::align_of::<T>() - 1)
    }

    fn is_aligned_with<T: Aligned>(&self) -> bool {
        self.align_of_bytes::<T>() == 0
    }
}

impl<'a> BytesExt for &'a [u8] {
    fn align_of_bytes<T: Aligned>(&self) -> usize {
        self.as_ptr().addr() & (core::mem::align_of::<T>() - 1)
    }

    fn is_aligned_with<T: Aligned>(&self) -> bool {
        self.align_of_bytes::<T>() == 0
    }
}

impl BytesExt for str {
    fn align_of_bytes<T: Aligned>(&self) -> usize {
        self.as_ptr().addr() & (core::mem::align_of::<T>() - 1)
    }

    fn is_aligned_with<T: Aligned>(&self) -> bool {
        self.align_of_bytes::<T>() == 0
    }
}

impl<'a> BytesExt for &'a str {
    fn align_of_bytes<T: Aligned>(&self) -> usize {
        self.as_ptr().addr() & (core::mem::align_of::<T>() - 1)
    }

    fn is_aligned_with<T: Aligned>(&self) -> bool {
        self.align_of_bytes::<T>() == 0
    }
}

impl<const N: usize> BytesExt for [u8; N] {
    fn align_of_bytes<T: Aligned>(&self) -> usize {
        self.as_ptr().addr() & (core::mem::align_of::<T>() - 1)
    }

    fn is_aligned_with<T: Aligned>(&self) -> bool {
        self.align_of_bytes::<T>() == 0
    }
}

mod private {
    //! Sealed, private traits to prevent downstream users of the crate from
    //! implementing this trait.
    pub trait Sealed {}

    impl Sealed for [u8] {}
    impl<'a> Sealed for &'a [u8] {}

    impl Sealed for str {}
    impl<'a> Sealed for &'a str {}

    impl<const N: usize> Sealed for [u8; N] {}
}
