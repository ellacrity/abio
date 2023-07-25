//! Module for working with contiguous regions of memory.
//!
//! These regions may be slices, such as the [`Bytes`] type or they may represent
//! fixed size data, such as the [`Chunk`] type.

// TODO: Determine whether we want to move this module to its own separate crate. Is
// it within the scope of this project to provide wrapper types like `Bytes` and
// `Chunk`? (ellacrity)

use core::fmt::Debug;
use core::ops::Deref;

use crate::layout::{Abi, BytesOf};
use crate::{shims, Error, Result};

mod array;
pub use array::Array;

// FIXME: Add support for `BytesMut` and reassess `Bytes` layout (ellacrity).
mod bytes;
pub use bytes::Bytes;
// mod mutable;
// pub use mutable::BytesMut;

mod chunk;
pub use chunk::Chunk;

mod span;
pub use span::Span;

// type ChunkArray<'data, const N: usize> = <Chunk<N> as Source>::Array<'data, N>;
// type BytesArray<'data, const N: usize> = <&'data Bytes as Source>::Array<'data,
// N>;

/// Types that represent valid memory for use as an input [`Source`] into the
/// abstract machine defined by this crate's ABI, or Application Binary Interface.
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
/// Implementing this trait for a type that is not compatible with [`abio`][crate]'s
/// ABI may produce unpredictable results and is therefore considered **undefined
/// behaviour**. The caller must ensure their input type fulfills the contract
/// introduced by this trait.
///
/// You are strongly advised to utilize the derive procedural macros provided by the
/// [`abio_derive`][abio_derive] crate. These macros help perform checks at compile
/// time to minimize mistakes at runtime.
///
/// [endian]: crate::endian::Endian
pub unsafe trait Source: BytesOf {
    /// Dynamically-sized type this [`Source`] can be sliced into.
    type Slice: ?Sized + Debug + Eq + PartialEq + Source;

    /// Fixed-size type this [`Source`] can be sliced into with a constant size.
    type Array<const LEN: usize>: Array<LEN>;

    /// Gets the pointer to the head of the source bytes, returning a `*const u8`.
    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    /// Returns the alignment that must be applied to the pointer to the source bytes
    /// in order to meet the alignment requirements of `T`.
    ///
    /// # Algorithm
    ///
    /// The algorithm used applies a bitmask
    #[inline]
    fn align_with<T: Abi>(&self) -> usize {
        self.as_ptr().addr() & (T::ALIGN - 1)
    }

    /// Returns true if the the byte slice represented by `self` are aligned with
    /// `T`.
    #[inline]
    fn is_aligned_with<T: Abi>(&self) -> bool {
        self.align_with::<T>() == 0
    }

    /// Reads a slice of bytes from this [`Source`], starting at `offset` with the
    /// specified `size`. The slice is returned along with the remaining [`Source`]
    /// bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if `offset + size` exceeds the number of bytes comprising
    /// the [`Source`].
    fn read_slice_at(&self, offset: usize, size: usize) -> Result<&Self::Slice>;

    fn read_slice(&self, size: usize) -> Result<&Self::Slice>;

    /// Reads a chunk of bytes into a fixed size array, returning the chunk along
    /// with the remaining [`Source`] bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation results in an out of bounds memory accesss.
    ///
    /// This method is the preferred method of reading from the [`Source`] as the
    /// compiler is very good at optimizing operations performed on fixed size chunks
    /// of memory.
    fn read_chunk_at<const LEN: usize>(&self, offset: usize) -> Result<Self::Array<LEN>>;

    fn read_chunk<const LEN: usize>(&self) -> Result<Self::Array<LEN>>;

    /// Returns the length of the [`Source`].
    ///
    /// This is almost always going to simply be equal to the number of bytes
    /// comprising the input source itself.
    fn source_len(&self) -> usize;
}

unsafe impl Source for [u8] {
    type Array<const N: usize> = Chunk<N>;
    type Slice = [u8];

    #[inline]
    fn source_len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn read_slice_at(&self, offset: usize, size: usize) -> Result<&Self::Slice> {
        Ok(Bytes::new(self.bytes_of()).read_at(offset, size)?)
    }

    #[inline]
    fn read_chunk_at<const LEN: usize>(&self, offset: usize) -> Result<Self::Array<LEN>> {
        let span = Span::new(offset, LEN);
        let span_size = span.size();
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

    fn read_slice(&self, size: usize) -> Result<&Self::Slice> {
        match shims::take_slice(self.bytes_of(), size) {
            Ok(bytes) => Ok(bytes),
            Err(err) => Err(err),
        }
    }

    fn read_chunk<const N: usize>(&self) -> Result<Self::Array<N>> {
        match shims::first_chunk::<N>(self.bytes_of()) {
            Ok(chunk) => Ok(chunk),
            Err(err) => Err(err),
        }
    }
}
