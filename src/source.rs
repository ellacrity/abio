//! Module for working with contiguous regions of memory.
//!
//! These regions may be slices, such as the [`Bytes`] type or they may represent
//! fixed size data, such as the [`Chunk`] type.

use core::fmt::Debug;
use core::slice;

use crate::layout::{Abi, BytesOf};
use crate::{Error, Result};

mod array;
pub use array::{Array, ByteArray};

// TODO: Add support for `BytesMut` and reassess `Bytes` layout (ellacrity).
mod bytes;
pub use bytes::Bytes;
// mod mutable;
// pub use mutable::BytesMut;

#[macro_use]
mod chunk;
pub use chunk::Chunk;

mod span;
pub use span::Span;

/// Types that represent valid memory for use as an input [`Source`] into the
/// abstract machine defined by this crate's [ABI], or Application Binary Interface.
///
/// # I/O
///
/// This trait provides low-level methods for higher-level reader/writer (I/O)
/// primitives to be built upon. These methods do not perform any file-based or
/// network I/O. Instead, they operate on arbitrary regions of memory represented as
/// slices of bytes.
///
/// # Endianness
///
/// Please be aware that this trait itself does not account for endianness. If you
/// need to work with data in an endian-aware manner, please see the
/// [`Endian`][endian] trait. Methods are available for reading bytes in both big
/// and little endian byte order serialization.
///
/// # ABI
///
/// [`Source`] types are input sources, such as byte slices, that the system accepts
/// as untrusted input. that the system can safely interpret as ABI-compatible types.
/// This trait helps populate transient buffers before being passing the read bytes
/// into one of the [`Decode`] trait's methods for further processing.

/// Any type that is [`Source`] is a valid source of readable data flowing
/// into the system. It is an abstraction for the idea of `input`, which can be
/// validated up front, parsed as needed and traversed quickly for indexing
/// operations.
///
/// # Safety
///
/// Implementing this trait for a type that is not compatible with
/// [`abio`][crate]'s ABI may produce unpredictable results and is therefore
/// considered **undefined behaviour**. The caller must ensure their input type
/// fulfills the contract introduced by this trait.
///
/// You are strongly advised to utilize the derive procedural macros provided by the
/// [`abio_derive`][abio_derive] crate. These macros help perform checks at
/// compile time to minimize mistakes at runtime.
///
/// [endian]: crate::endian::Endian
/// [ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
pub unsafe trait Source: BytesOf {
    /// Dynamically-sized type this [`Source`] can be sliced into.
    type Slice: ?Sized + Debug + Eq + PartialEq;

    /// Fixed-size type this [`Source`] can be sliced into with a constant size.
    type Array<const LEN: usize>: Array<LEN>;

    /// Reads a slice of bytes from this [`Source`], spanning the specified `size`.
    /// The successfully read bytes are returned as a slice.
    ///
    /// # Errors
    ///
    /// This function returns an error if `offset + size` surpasses the total byte
    /// count of the [`Source`].
    fn read_slice(&self, size: usize) -> Result<&Self::Slice>;

    /// Reads a slice of bytes from this [`Source`], starting at the `offset` and
    /// spanning the specified `size`.  The successfully read bytes are returned as a
    /// slice.
    ///
    /// # Errors
    ///
    /// This function returns an error if `offset + size` surpasses the total byte
    /// count of the [`Source`].
    fn read_slice_at(&self, offset: usize, size: usize) -> Result<&Self::Slice>;

    /// Reads a chunk of bytes with size `N` from this [`Source`], starting at the
    /// `offset`.
    ///
    /// # Return Type
    ///
    /// The crate uses the [`Chunk`] type to  A [`Chunk`] is the  a fixed size array
    /// from the [`Source`].
    ///
    /// # Errors
    ///
    /// Returns an error if the operation results in an out of bounds memory accesss.
    ///
    /// This method is the preferred method of reading from the [`Source`] as the
    /// compiler is very good at optimizing operations performed on fixed size chunks
    /// of memory.
    fn read_chunk_at<const N: usize>(&self, offset: usize) -> Result<Self::Array<N>>;

    /// Reads a chunk of bytes into a fixed size array from the [`Source`].
    ///
    /// # Errors
    ///
    /// This function returns an error if the operation leads to an out-of-bounds
    /// memory access.
    ///
    /// # Performance
    ///
    /// This method is the recommended approach for reading from the [`Source`], as
    /// the compiler is highly efficient at optimizing operations performed on fixed
    /// size chunks of memory.
    fn read_chunk<const LEN: usize>(&self) -> Result<Self::Array<LEN>>;

    /// Returns the length of the [`Source`].
    ///
    /// This is almost always going to simply be equal to the number of bytes
    /// comprising the input source itself.
    fn source_len(&self) -> usize;
}

/// Trait to define types that may be represented as a borrowed slice of bytes.
///
/// # Safety
///
/// TODO: Safety section; outline when/why it is safe or unsafe to implement for what
/// kind of types. (ellacrity)
pub unsafe trait Buf {
    /// Returns the alignment that must be applied to the pointer to the source bytes
    /// in order to meet the alignment requirements of `T`.
    ///
    /// # Algorithm
    ///
    /// The algorithm used applies a bitmask
    #[inline]
    fn align_with<T: Abi>(&self) -> usize {
        <*const Self>::from(self).cast::<u8>().addr() & (T::ALIGN - 1)
    }

    /// Returns true if the the byte slice represented by `self` are aligned with
    /// `T`.
    #[inline]
    fn is_aligned_with<T: Abi>(&self) -> bool {
        self.align_with::<T>() == 0
    }
}

// Blanket implementation for the `Buf` trait for all types that implement `Source`
// unsafe impl<'a, T: ?Sized> Buf for T where T: Source<'a> {}

unsafe impl<T> Buf for T where T: Source {}

unsafe impl Source for &[u8] {
    type Array<const N: usize> = Chunk<N>;
    type Slice = [u8];

    #[inline]
    fn read_slice(&self, size: usize) -> Result<&Self::Slice> {
        crate::read_slice_bytes!(self, size)
    }

    #[inline]
    fn read_slice_at(&self, offset: usize, size: usize) -> Result<&Self::Slice> {
        crate::read_slice_bytes!(self, offset, size)
    }

    #[inline]
    fn read_chunk_at<const LEN: usize>(&self, offset: usize) -> Result<Self::Array<LEN>> {
        let span = Span::new(offset, LEN);
        let span_size = span.len();
        debug_assert_eq!(self[..LEN].len(), span_size);

        let source_len = self.source_len();
        if source_len < span_size {
            return Err(crate::Error::out_of_bounds(span_size, source_len));
        }

        // SAFETY: `span.start()` is within bounds of the slice, so we have both a valid
        // pointer to the start of the bytes as well as a valid length.
        let bytes = unsafe {
            let data = self.as_ptr().add(span.start());
            core::slice::from_raw_parts(data, source_len - span.start())
        };

        let Ok(chunk) = Chunk::from_bytes(bytes) else {
            return Err(Error::size_mismatch(LEN, source_len));
        };

        debug_assert!(chunk.len() == offset);
        Ok(chunk)
    }

    #[inline]
    fn read_chunk<const N: usize>(&self) -> Result<Self::Array<N>> {
        crate::read_chunk_bytes!(self, N)
    }

    #[inline]
    fn source_len(&self) -> usize {
        self.len()
    }
}

unsafe impl<const N: usize> Source for [u8; N] {
    type Slice = [u8];
    type Array<const LEN: usize> = Chunk<LEN>;

    #[inline]
    fn read_slice(&self, size: usize) -> Result<&Self::Slice> {
        crate::read_slice_bytes!(self, size)
    }

    #[inline]
    fn read_slice_at(&self, offset: usize, size: usize) -> Result<&Self::Slice> {
        crate::read_slice_bytes!(self, offset, size)
    }

    #[inline]
    fn read_chunk_at<const LEN: usize>(&self, offset: usize) -> Result<Self::Array<LEN>> {
        crate::read_chunk_bytes!(self, offset, LEN)
    }

    #[inline]
    fn read_chunk<const LEN: usize>(&self) -> Result<Self::Array<LEN>> {
        crate::read_chunk_bytes!(self, LEN)
    }

    #[inline]
    fn source_len(&self) -> usize {
        self.len()
    }
}

unsafe impl<'source> Source for Bytes<'source> {
    type Array<const LEN: usize> = Chunk<LEN>;
    type Slice = [u8];

    #[inline]
    fn read_slice_at(&self, offset: usize, size: usize) -> crate::Result<&Self::Slice> {
        if let Ok(bytes) = crate::read_slice_bytes!(self, offset, size) {
            Ok(bytes)
        } else {
            Err(Error::out_of_bounds(offset + size, self.source_len()))
        }
    }

    #[inline]
    fn read_slice(&self, size: usize) -> crate::Result<&Self::Slice> {
        crate::read_slice_bytes!(self, size)
    }

    #[inline]
    fn read_chunk_at<const N: usize>(&self, offset: usize) -> crate::Result<Self::Array<N>> {
        crate::read_chunk_bytes!(self, offset, N)
    }

    #[inline]
    fn read_chunk<const N: usize>(&self) -> crate::Result<Self::Array<N>> {
        crate::read_chunk_bytes!(self, N)
    }

    #[inline]
    fn source_len(&self) -> usize {
        self.len()
    }
}

unsafe impl<const N: usize> Source for Chunk<N> {
    type Array<const LEN: usize> = Chunk<LEN>;
    type Slice = [u8];

    #[inline]
    fn source_len(&self) -> usize {
        self.as_bytes().len()
    }

    #[inline]
    fn read_slice_at(&self, offset: usize, size: usize) -> Result<&Self::Slice> {
        let span = Span::new(offset, size);
        if self.source_len() < span.len() {
            Err(Error::out_of_bounds(span.len(), self.source_len()))
        } else {
            // SAFETY: `span.start()` is within ounds of `self`, so we havew a valid pointer to
            // this location. The resulting `bytes` slice uses this validated pointer and adjusts
            // the length to a value that is known to be within bounds.
            unsafe {
                let data = self.as_ptr_offset(span.start());
                Ok(slice::from_raw_parts(data, self.source_len() - span.start()))
            }
        }
    }

    #[inline]
    fn read_chunk_at<const LEN: usize>(&self, offset: usize) -> Result<Self::Array<LEN>> {
        Chunk::<LEN>::from_bytes_at(self.as_bytes(), offset)
    }

    #[inline]
    fn read_slice(&self, size: usize) -> Result<&Self::Slice> {
        let span = Span::new(0, size);
        if self.source_len() < span.len() {
            Err(Error::out_of_bounds(span.len(), self.source_len()))
        } else {
            // SAFETY: `span.start()` is within ounds of `self`, so we havew a valid pointer to
            // this location. The resulting `bytes` slice uses this validated pointer and adjusts
            // the length to a value that is known to be within bounds.
            Ok(unsafe { self.as_bytes().get_unchecked(span.as_range()) })
        }
    }

    #[inline]
    fn read_chunk<const LEN: usize>(&self) -> Result<Self::Array<LEN>> {
        Chunk::from_bytes(self.as_bytes())
    }
}
