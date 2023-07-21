//! Low-level encoding and decoding primitives.

use core::fmt::Debug;

use crate::layout::{Abi, AsBytes};
use crate::Result;

mod array;
pub use array::Array;

mod chunk;
pub use chunk::Chunk;

mod span;
pub use span::Span;

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
pub unsafe trait Source: AsBytes {
    /// The inner type that this [`Source`] can be sliced into.
    type Slice: ?Sized + Debug + Eq + PartialEq;

    /// Gets the pointer to the head of the source bytes, returning a `*const u8`.
    #[inline(always)]
    fn as_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    /// Returns the alignment that must be applied to the pointer to the source bytes
    /// in order to meet the alignment requirements of `T`.
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

    /// Reads a slice of bytes from this [`Source`], starting at `offset` with
    /// `size`, in bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to bounds errors or any other IO
    /// failure. size or alignment invariants.
    fn read_slice(&self, offset: usize, size: usize) -> Result<(&Self::Slice, &[u8])>;

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
    fn read_array<'data, const SIZE: usize, A: Array<'data, SIZE>>(
        &self,
        offset: usize,
    ) -> Result<(A, &[u8])>;

    /// Returns the length of the [`Source`].
    ///
    /// This is almost always going to simply be equal to the number of bytes
    /// comprising the input source itself.
    fn source_len(&self) -> usize;
}

unsafe impl Source for [u8] {
    type Slice = [u8];

    #[inline]
    fn source_len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn read_slice(&self, offset: usize, size: usize) -> Result<(&Self::Slice, &[u8])> {
        let span = Span::new(offset, size);
        if self.source_len() < span.size() {
            Err(crate::Error::out_of_bounds(span.size(), self.source_len()))
        } else {
            // SAFETY: `span.start()` is within ounds of `self`, so we havew a valid pointer to
            // this location. The resulting `bytes` slice uses this validated pointer and adjusts
            // the length to a value that is known to be within bounds.
            let bytes = unsafe {
                let data = self.as_ptr().add(span.start());
                core::slice::from_raw_parts(data, self.source_len() - span.start())
            };
            Ok(bytes.split_at(span.size()))
        }
    }

    #[inline]
    fn read_array<'data, const SIZE: usize, A: Array<'data, SIZE>>(
        &self,
        offset: usize,
    ) -> Result<(A, &[u8])> {
        let span = Span::new(offset, SIZE);
        if self.source_len() < span.size() {
            return Err(crate::Error::out_of_bounds(span.size(), self.source_len()));
        }

        let (head, tail) = self.as_bytes().split_at(offset);
        debug_assert!(head.len() == offset);
        debug_assert_eq!(tail[..SIZE].len(), A::SIZE);
        debug_assert_eq!(tail[..SIZE].len(), span.size());

        // SAFETY: The span above verifies that we have at least enough bytes to decode an
        // array of length `SIZE`.
        let head = &tail[..SIZE];
        let head = unsafe { head.as_ptr().cast::<A>().read() };
        Ok((head, tail))
    }
}
