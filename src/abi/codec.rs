//! Low-level encoding and decoding primitives.

use core::fmt::Debug;
use core::mem::size_of;
use core::{ptr, slice};

use crate::abi::{Abi, Deserializer};
use crate::{array, Bytes, Error, Zeroable, BE, LE};

pub mod endian;

mod bytes;
pub use bytes::BytesOf;

// TODO: Add `Chunk` or array-like type to concretize output from read functions.
mod chunk;
pub use chunk::*;

// FIXME: Parse currently broken; Decode needs to be repurposed to make useful

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
/// need to work with data in an endian-aware manner, please see the [`Endianness`]
/// trait. Methods are available for reading bytes in both big and little endian byte
/// order serialization.
///
/// # ABI
///
/// [`Source`] types are simply input types that the system can safely interpret as
/// ABI-compatible types. This trait helps populate transient buffers before being
/// passing the read bytes into one of the [`Decode`] trait's methods for further
/// processing.
///
/// ABI-compatible, since implementing this for a type is the same thing as promising
/// your users that their implementation is sound. This is an extremely strong
/// guarantee and should only be implemented for types if you absolutely have to.
///
///
///
/// Any type that is [`Source`] is a valid source of readable data flowing
/// into the system. It is an abstraction for the idea of `input`, which can be
/// validated up front, parsed as needed and traversed quickly for indexing
/// operations.
///
/// # Safety
///
/// Implementing this trait for a type that violates the contract and is considered
/// abi-incompatible, and is therefore **undefined behaviour**. This trait should be
/// carefully implemented, as the soundness of your entire implementation depends on
/// the implementation of this trait being correct.
///
///
/// If an implementation were allowed
/// that failed to fulfill the contract this trait enforces, it would weaken the
/// overall soundness of the entire type system. implement [`Decodable`] must also
/// implement [`Abi`]. This ensures that the output type is layout-compatible with
/// the ABI defined by this crate.
pub unsafe trait Source {
    /// The inner type that this [`Source`] can be sliced into.
    type Slice: ?Sized + Debug + Eq + PartialEq;

    /// Gets the pointer to the head of the source bytes, returning a `*const u8`.
    fn as_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    /// Returns the alignment that must be applied to the pointer to the source bytes
    /// in order to meet the alignment requirements of `T`.
    fn align_with<T: Abi>(&self) -> usize {
        self.as_ptr().addr() & (T::ALIGN - 1)
    }

    /// Returns true if the the byte slice represented by `self` are aligned with
    /// `T`.
    fn is_aligned_with<T: Abi>(&self) -> bool {
        self.align_with::<T>() == 0
    }

    /// Returns the length of the [`Source`].
    ///
    /// This is equal to the number of bytes comprising the input source itself.
    fn source_len(&self) -> usize;

    /// Reads a slice of bytes from this [`Source`], starting at `offset` with
    /// `size`, in bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to bounds errors or any other IO
    /// failure. size or alignment invariants.
    fn read_slice(&self, offset: usize, size: usize) -> Option<(&Self::Slice, &Self)>;

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
    fn read_chunk<'chunk, const SIZE: usize, C: Chunk<'chunk, SIZE>>(
        &self,
        offset: usize,
    ) -> Option<(C, &Self)>;
}

unsafe impl Source for [u8] {
    type Slice = [u8];

    fn read_slice(&self, offset: usize, size: usize) -> Option<(&Self::Slice, &Self)> {
        // read_helper(self, Span::new(offset, size))
        let span = Span::new(offset, size);
        let size = span.size();
        if self.source_len() < size {
            None
        } else {
            let source = unsafe {
                let ptr = self.as_ptr().add(span.start());
                slice::from_raw_parts(ptr, size)
            };
            Some(source.split_at(size))
        }
    }

    fn read_chunk<'chunk, const SIZE: usize, C: Chunk<'chunk, SIZE>>(
        &self,
        offset: usize,
    ) -> Option<(C, &Self)> {
        if self.len() < SIZE + offset {
            None
        } else {
            // SAFETY: We manually verified the bounds of the split.
            let (chunk, tail) = self.split_at(SIZE);

            debug_assert_eq!(chunk.len(), SIZE, "Invalid read_chunk operation: `chunk.len() != N`");

            // SAFETY: We explicitly check for the correct number of elements, and do not let the
            // references outlive the slice.
            let chunk = unsafe { ptr::read(chunk.as_ptr().cast::<C>()) };
            Some((chunk, tail))
        }
    }

    fn source_len(&self) -> usize {
        self.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// Creates a new [`Span`] from a `start and `end` offset.
    ///
    /// # Panics
    ///
    /// This contructor method will panic if `start < end`.
    #[inline(always)]
    pub const fn new(offset: usize, size: usize) -> Self {
        let end = offset.saturating_add(size);
        assert!(
            offset >= end,
            "Illegal construction of Span. Cannot create a Span where `self < end`."
        );
        Span { start: offset, end }
    }

    /// Creates a new [`Span`] from an existing instance of this type and a `source`.
    pub fn from_slice<S: Source>(&self, source: &S) -> Option<Self> {
        let slice = source.read_slice(self.start, self.size());
        let Some((head, tail)) = slice else {
            return None;
        };
        Some(Span { start: 0, end: head.as_bytes_of().len() })
    }

    pub fn from_chunk<S: Source, const N: usize>(&self, source: &S) -> Option<Span> {
        let Some((head, tail)) = source.read_chunk::<N, ByteChunk<N>>(self.start) else {
            return None;
        };
        Some(Span { start: 0, end: head.as_slice().len() })
    }

    /// Returns the size of this [`Span`].
    #[inline]
    pub const fn size(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Returns the start of this span, also referred to as its `offset`.
    #[doc(alias = "offset")]
    #[inline]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the end the span, represented as its upper bound, or index.
    #[inline]
    pub const fn end(&self) -> usize {
        self.end
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ByteChunk<const N: usize> {
    chunk: [u8; N],
}

impl<const N: usize> ByteChunk<N> {
    pub fn new(chunk: [u8; N]) -> Self {
        Self { chunk }
    }
}

unsafe impl<const N: usize> Abi for ByteChunk<N> {}
unsafe impl<const N: usize> Zeroable for ByteChunk<N> {}

impl<'source, const N: usize> Chunk<'source, N> for ByteChunk<N> {
    unsafe fn from_ptr(ptr: *const u8) -> Self {
        todo!()
    }

    fn as_slice(&self) -> Bytes<'source> {
        todo!()
    }

    fn from_bytes(bytes: [u8; N]) -> Self {
        todo!()
    }

    fn into_bytes(self) -> [u8; N] {
        todo!()
    }
}

impl<const N: usize> AsRef<[u8]> for ByteChunk<N> {
    fn as_ref(&self) -> &[u8] {
        self.chunk.as_slice()
    }
}

fn read_helper<S: Source>(source: &S, span: Span) -> Option<(&[u8], &[u8])> {
    let size = span.size();
    if source.source_len() < size {
        None
    } else {
        let source = unsafe {
            let ptr = source.as_ptr().add(span.start());
            slice::from_raw_parts(ptr, size)
        };
        Some(source.split_at(size))
    }
}

fn read_chunk_helper<'a, S: Source, const SIZE: usize, C: Chunk<'a, SIZE>>(
    source: &S,
    span: Span,
) -> Option<(C, &[u8])> {
    let size = span.size();
    if source.source_len() < size {
        None
    } else {
        let source = unsafe {
            let ptr = source.as_ptr().add(span.start());
            slice::from_raw_parts(ptr, size)
        };
        let (head, tail) = source.split_at(size);
        let chunk = unsafe { Chunk::from_ptr(head.as_ptr()) };
        Some((chunk, tail))
    }
}

/// Reads `N` bytes from the source, returning a chunk, or `[u8; N]` array type
/// where `N` represents the length of the array.
///
/// # ZSTs
///
/// This function may return a ZST if `N` is equal to 0.
///
/// # Panics
///
/// This method will panic if you attempt to read a chunk where `N > self.len()`.
/// It is the callers responsibility to should check the length of your result
/// when working with this method.
#[inline]
fn read_inner<'chunk, C, const N: usize>(bytes: &[u8], offset: usize) -> Option<(C, &[u8])>
where
    C: Chunk<'chunk, N>,
{
    read_chunk_inner::<N, C>(bytes, offset)
}

#[inline]
fn read_chunk_inner<'chunk, const N: usize, C>(bytes: &[u8], offset: usize) -> Option<(C, &[u8])>
where
    C: Chunk<'chunk, N>,
{
    if bytes.len() < N + offset {
        None
    } else {
        // SAFETY: We manually verified the bounds of the split.
        let (chunk, tail) = bytes.split_at(N);

        debug_assert_eq!(chunk.len(), N, "Invalid read_chunk_inner operation: `chunk.len() != N`");

        // SAFETY: We explicitly check for the correct number of elements,
        //   and do not let the references outlive the slice.
        let chunk = unsafe { ptr::read(chunk.as_ptr().cast::<C>()) };
        Some((chunk, tail))
    }
}

macro_rules! print_type {
    ($ty:ty) => {
        // inner scope
        {
            ::core::any::type_name::<$ty>()
        }
    };
}

fn read_source_slice<T, const CHUNK_SIZE: usize, S, D>(
    source: &S,
    deserializer: D,
) -> Result<T, Error>
where
    T: Sized,
    S: Source,
    D: Deserializer,
{
    let size = size_of::<T>();
    assert!(
        source.source_len() >= size,
        "cannot read `{}` from `bytes` with length {}",
        print_type!(T),
        source.source_len(),
    );
    if source.source_len() < size {
        return Err(Error::incompatible_types());
    }
    let bytes = source.as_bytes_of();
    let array = array!(bytes with CHUNK_SIZE).map_err(|_| Error::incompatible_types())?;
    if deserializer.is_little_endian() {
        Ok(deserializer.deserialize_slice::<LE>(array))
    } else {
        Ok(deserializer.deserialize_slice::<BE>(array))
    }
}

/// Trait for defining how a particular type is decoded, or deserialized, from the
/// underlying byte slice.
///
/// This trait extends the [`Parse`] trait by allowing endian-aware operations to be
/// performed on the parsed bytes.
///
/// # Configuration
///
/// The behaviour of the [`Decode`] trait is determined by the [`Codec`] used to
/// interpret the bytes read from any arbitrary source. The default provided
/// [`Config`] struct is used to configure the codec. However, if you need to
/// further customize how data is decoded, you can implement [`Codec`] for your own
/// custom type. Implementing [`Codec`] for a custom type allows complete control
/// over how the actual bytes are decoded and interpreted.
pub trait Deserialize: Abi {
    /// Decodes a concrete type `T` from an immutable reference to `self`.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to a size mismatch or misaligned
    /// read.
    fn deserialize<D: Deserializer>(bytes: &[u8], deserializer: D) -> crate::Result<Self>;
}

/// A fixed, statically sized chunk of data that can be read from the `Source`.
///
/// This is implemented for `u8`, as well as byte arrays `&[u8; 1]` to `&[u8; 32]`.
pub trait Chunk<'source, const LEN: usize>: AsRef<[u8]> {
    /// Deserializes a chunk of bytes into an ABI-compatible type.
    ///
    /// # Safety
    ///
    /// Raw byte pointer should point to a valid location in source.
    unsafe fn from_ptr(ptr: *const u8) -> Self;

    fn as_slice(&self) -> Bytes<'source>;

    fn from_bytes(bytes: [u8; LEN]) -> Self;

    fn into_bytes(self) -> [u8; LEN];

    // fn deserialize<D: Deserializer>(self, deserializer: D) -> Option<&'source Self>;
}

impl<'source, const N: usize> Chunk<'source, N> for [u8; N] {
    #[inline]
    unsafe fn from_ptr(ptr: *const u8) -> Self {
        ptr.cast::<Self>().read()
    }

    fn as_slice(&self) -> Bytes<'source> {
        Bytes::new(self)
    }

    fn from_bytes(bytes: [u8; N]) -> Self {
        bytes
    }

    fn into_bytes(self) -> [u8; N] {
        self
    }
}
