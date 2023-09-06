//! Module containing types and primitives for working with slices of bytes with a
//! known size.
//!
//! This module provides essential utilities for converting to and from slices and
//! arrays of bytes. Fixed-size arrays module, is parameterized over the size, or
//! capacity, of its underlying backing buffer. This allows the compiler to make more
//! aggressive optimizations, since the size of the slice is explicit.

use core::mem::{self};
use core::ops::{Range, RangeTo};
use core::{ptr, slice};

use crate::source::Bytes;
use crate::{util, Abi, Alignment, Endian, Endianness, Error, LittleEndian, Result};

/// A fixed-size array of bytes, or "chunk" guaranteed to contain bytes in native
/// endian order.
///
/// `Chunk` is a newtype wrapper around an array of bytes (`[u8; N]`). It represents
/// a fixed-size chunk of bytes that can be used to perform efficient, size-aware
/// operations.
///
/// The size of the `Chunk`, `N`, is determined at compile time via const generics.
/// This allows you to create `Chunk` instances of any size, with the size
/// information carried with the type for enhanced type safety and efficiency. The
/// maximum size of `N` depends on your particular use case and memory constraints.
///
/// This type is an integral part of the API and it is used widely to handle byte
/// arrays in a convenient and type-safe manner.
///
/// # Layout
/// This type is marked with the `#[repr(transparent)]` attribute, ensuring its
/// memory layout is identical to that of its single non-zero-sized field, `[u8; N]`.
///
/// # Type Parameters
/// - `N`: The size of the array in bytes, defined at compile time.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Chunk<const N: usize> {
    // TODO: Consider adding a `len` here and using it like a conventional "stack".
    /// Array of bytes with length `N`.
    inner: [u8; N],
}

impl<const N: usize> Chunk<N> {
    /// Creates a new [`Chunk`] from an array of bytes with length `N`.
    #[inline(always)]
    pub const fn new<E: Endianness>(array: [u8; N]) -> Self {
        let bytes = util::read_endian_bytes::<E, N>(&array)
            .expect("Chunk types cannot be used to represent ZST's");
        Self { inner: bytes }
    }

    /// Creates a new [`Chunk`] instance with length `N` from a bytes slice.
    ///
    /// # Errors
    ///
    /// Returns an `OutOfBoundsError` if
    /// Returns `None` if `bytes.len() < N`, or if the conversion from slice to array
    /// fails.
    #[inline]
    pub const fn from_slice<E: Endianness>(bytes: &[u8]) -> Result<Self> {
        // SAFETY: The validation above tells us that `bytes` is at least `N` bytes in
        // length. The longest subslice this routine could take is the entire slice, which is
        // a safe operation. Additionally, the `Chunk` type represents a slice of `u8`
        // elements, so alignment checks can be skipped (alignment is 1).
        match unsafe { util::read_endian_bytes::<E, N>(bytes) } {
            Ok(array) => {
                if array.len() != N {
                    Err(Error::size_mismatch(N, array.len()))
                } else {
                    match E::ENDIAN {
                        Endian::Big => Ok(Self::from_be_bytes(array)),
                        Endian::Little => Ok(Self::from_le_bytes(array)),
                    }
                }
            }
            Err(e) => return Err(e),
        }
    }

    /// Creates a new [`Chunk`] instance with length `N` from a bytes slice, starting
    /// at `offset`.
    ///
    /// Returns `None` if `bytes.len() < offset + N` or if the conversion from slice
    /// to array fails.
    #[inline]
    pub const fn read_bytes_offset<E: Endianness>(bytes: &[u8], offset: usize) -> Result<Self> {
        // check that bytes are not empty
        if bytes.is_empty() {
            return Err(Error::null_reference());
        }

        // declare needed number of bytes
        let needed = offset + N;
        // `bytes` must have at least `offset + N` bytes
        if bytes.len() < needed {
            Err(Error::out_of_bounds(needed, bytes.len()))
        }

        // SAFETY: The validation above tells us that `bytes` is at least `$offset + $size`
        // bytes in length. The longest subslice this routine could take is the
        // entire slice, which is a safe operation. Additionally, the `Chunk` type represents
        // a slice of `u8` elements, so alignment checks can be skipped (alignment is
        // 1).
        Ok(unsafe {
            let bytes = slice::from_raw_parts(bytes.as_ptr().add(offset), N);
            Self::make_chunk::<E>(bytes)
        })
    }

    /// Gets a pointer to the first byte of this chunk, returning a `*const u8`.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    /// Returns the number of bytes in the chunk.
    #[inline]
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns `true` if the chunk has a length of 0.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Gets the "address" portion of the pointer.
    #[inline(always)]
    pub fn addr(self) -> usize {
        self.as_ptr().addr()
    }

    /// Return the chunk as a reference to a sized array of bytes, or `&[u8; N]`.
    #[inline(always)]
    pub const fn as_byte_array(&self) -> &[u8; N] {
        &self.inner
    }

    /// Consumes this `Chunk`, returning the inner `[u8; N]` byte array.
    #[inline(always)]
    pub const fn into_array(self) -> [u8; N] {
        // These types have an identical in memory layout, so this will likely be optimized
        // away by the compiler.
        self.inner
    }

    const fn erase_bytes(mut self) -> Self {
        unsafe { ptr::write_bytes(self.as_ptr().cast_mut(), 0, N) };
        assert_eq!(self.inner, [0u8; N], "Chunk should have had its memory zeroed out forcefully.");
        self
    }

    /// Returns this `Chunk` as a dynamically-sized byte slice.
    #[inline(always)]
    pub const fn as_slice(&self) -> &[u8] {
        &self.inner
    }

    /// Get a subslice of this chunk from `range.start..range.end`.
    #[inline]
    pub const fn subslice(&self, range: Range<usize>) -> &[u8] {
        debug_assert!(!self.is_empty() && N >= range.end);
        unsafe {
            let data = self
                .inner
                .as_ptr()
                .add(range.start);
            let len = range
                .end
                .saturating_sub(range.start);
            core::slice::from_raw_parts(data, len)
        }
    }

    /// Get a subslice of this chunk starting at `range.from`, spanning until the
    /// end.
    #[inline]
    pub const fn slice_to(&self, to: RangeTo<usize>) -> &[u8] {
        let end_offset = to.end;
        debug_assert!(N >= end_offset);
        debug_assert!(
            end_offset > 0,
            "The `slice_to(0)` operation on this Chunk produces an illegal type (ZST)."
        );
        unsafe {
            let data = self
                .inner
                .as_ptr()
                .add(end_offset);
            let len = self.inner.len() - end_offset;
            core::slice::from_raw_parts(data, len)
        }
    }

    /// Converts this chunk of bytes into a UTF-8 encoded `&str` slice.
    ///
    /// Returns `None` if the operation fails due to malformed bytes.
    #[inline]
    pub const fn as_utf8_str(&self) -> Option<&str> {
        if let Ok(utf8) = core::str::from_utf8(self.as_slice()) {
            Some(utf8)
        } else {
            None
        }
    }

    /// Interprets this chunk of bytes as some type `T` where `T` implements the
    /// [`Abi`] trait.
    #[inline(always)]
    pub unsafe fn read<T: Abi>(self) -> Result<T> {
        if self.inner.len() != T::SIZE {
            Err(Error::size_mismatch(T::SIZE, self.inner.len()))
        } else if self
            .as_ptr()
            .is_aligned_with::<T>()
        {
            Err(Error::misaligned_access::<T>(self.addr()))
        } else {
            // SAFETY: `self` has the same size as `T`, fulfills its alignment requirements and
            // is thus safe to read. Note that we return a bitwise copy of `T`, and not a
            // reference.
            Ok(unsafe { self.as_ptr().cast::<T>().read() })
        }
    }

    pub(crate) fn is_abi_compatible<T: Abi>(&self) -> bool {
        (self.inner.as_ptr() as usize) & (T::MIN_ALIGN.saturating_sub(1)) == 0
    }

    const fn _read_endian_inner<E: Endianness>(bytes: &[u8]) -> Result<Self> {
        Chunk::from_slice::<E>(bytes)
    }

    const fn copy_reversed(&self) -> Self {
        let mut buf = [0u8; N];
        let mut pos = N;

        while pos < N {
            // Use the index of the last element, then as we increment, we decrement the pointer
            // int othe slice.
            let idx = N - 1 - pos;
            buf[idx] = self.inner[idx];
            pos += 1;
        }
        Self::new(buf)
    }

    const fn copy_slice_reversed(bytes: &[u8]) -> Self {
        assert_eq!(
            bytes.len(),
            N,
            "Cannot copy bytes from a slice to create a Chunk where `bytes.len() != N`."
        );

        let mut buf = [0u8; N];
        let mut pos = N;

        while pos < N {
            // Use the index of the last element, then as we increment, we decrement the pointer
            // int othe slice.
            let idx = N - 1 - pos;
            buf[idx] = bytes[idx];
            pos += 1;
        }
        Self::new(buf)
    }

    /// Convenience function for creating [`Chunk<N>`] instances using the given byte
    /// order serialization defined by the type [`Context`] parameter.
    const unsafe fn make_chunk<E: Endianness>(bytes: &[u8]) -> Self {
        let chunk = Chunk::_read_endian_inner(bytes);
        match util::read_aligned_bytes::<Self, E, N>(chunk) {
            Ok(array) => array,
            Err(err) => return Err(err),
        }
    }

    pub(crate) fn try_from_raw_parts<'data>(ptr: *const u8, size: usize) -> Result<&'data Self> {
        let bytes = unsafe { slice::from_raw_parts(ptr, size) };
        debug_assert_eq!(bytes.len(), N);
        #[cfg(target_endian = "little")]
        {
            use crate::LE;
            Chunk::from_slice::<LE>(bytes).as_ref()
        }
        #[cfg(not(target_endian = "little"))]
        {
            use crate::BE;
            Chunk::from_slice::<BE>(bytes).as_ref()
        }
    }

    pub(crate) fn read_native_bytes(bytes: &[u8]) -> Result<Self> {
        Chunk::from_ne_bytes(bytes.try_into().unwrap())
    }
}

pub const fn with_endianness<E: Endianness>() -> Endian {
    if E::endian().is_little_endian() {
        Endian::Little
    } else {
        Endian::Big
    }
}

/*
 * Endian-aware `Chunk` constructors, conversion methods and related utilities.
 */

impl<const N: usize> Chunk<N> {
    /// Converts a chunk from big endian to the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    #[inline(always)]
    pub const fn from_be(chunk: Self) -> Self {
        #[cfg(target_endian = "big")]
        {
            chunk
        }
        #[cfg(not(target_endian = "big"))]
        {
            chunk.swap_bytes()
        }
    }

    /// Converts a chunk from little endian to the target's endianness.
    ///
    /// On little endian this is a no-op. On big endian the bytes are swapped.
    pub const fn from_le(chunk: Self) -> Self {
        #[cfg(target_endian = "little")]
        {
            chunk
        }
        #[cfg(not(target_endian = "little"))]
        {
            chunk.swap_bytes()
        }
    }

    /// Converts self to big endian from the target's endianness.
    ///
    /// On big endian this is a no-op. On little endian the bytes are swapped.
    #[inline(always)]
    pub const fn to_be(self) -> Self {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            self.swap_bytes()
        }
    }

    /// Consumes this `Chunk`, returning a new instance with little endian byte order
    /// serialization.
    ///
    /// This method is a no-op on little endian architecture.
    #[inline(always)]
    pub const fn to_le(self) -> Chunk<N> {
        #[cfg(target_endian = "little")]
        {
            self
        }
        #[cfg(not(target_endian = "little"))]
        {
            self.swap_bytes()
        }
    }

    /// Create a native endian integer value from its representation
    /// as a byte array in little endian.
    #[inline]
    pub const fn from_be_bytes(bytes: [u8; N]) -> Self {
        Self::from_be(Self::from_ne_bytes(bytes))
    }

    /// Create a native endian [`Chunk<N>`] from its representation as a byte array
    /// in little endian.
    #[inline]
    pub const fn from_le_bytes(bytes: [u8; N]) -> Self {
        Self::from_le(Self::from_ne_bytes(bytes))
    }

    /// Create a native endian [`Chunk`] value from its memory representation
    /// as a byte array in native endianness.
    #[inline]
    pub const fn from_ne_bytes(bytes: [u8; N]) -> Self {
        unsafe { mem::transmute(bytes) }
    }

    /// Consumes this `Chunk`, returning the underlying byte array in big endian byte
    /// order.
    ///
    /// On big endian this is a no-op. On little endian the bytes are
    /// swapped.
    #[inline(always)]
    pub const fn to_be_bytes(self) -> [u8; N] {
        self.to_be().to_ne_bytes()
    }

    /// Return the memory representation of this [`Chunk<N>`] as a byte array in
    /// little-endian byte order.
    ///
    /// /// Converts this `Chunk` into an array of `N` bytes in little endian byte
    /// order.
    ///
    /// On **little endian** this is a no-op. On big endian the bytes are swapped.
    #[inline]
    pub const fn to_le_bytes(self) -> [u8; N] {
        self.to_le().to_ne_bytes()
    }

    /// Return the memory representation of this [`Chunk<N>`] as a byte array in
    /// native byte order.
    #[inline]
    pub const fn to_ne_bytes(self) -> [u8; N] {
        unsafe { mem::transmute(self) }
    }

    /// Converts a slice of bytes to an array, reading the first `LEN` bytes. If the
    /// `REVERSE` flag is set, the bytes will be returned in reverse order.
    ///
    /// # Const
    ///
    /// This function is essentially a hack and work-around to `constify` this
    /// function, allowing it to be evaluated at compile time rather than runtime.
    #[inline(always)]
    pub const fn swap_bytes(self) -> Self {
        let mut buf = [0u8; N];
        let mut pos = 0;

        while pos < N {
            let idx = (N - 1) - pos;
            buf[idx] = self.inner[idx];
            pos += 1;
        }

        Self { inner: buf }
    }
}

impl<const N: usize> AsRef<[u8; N]> for Chunk<N> {
    #[inline]
    fn as_ref(&self) -> &[u8; N] {
        self.as_byte_array()
    }
}

impl<const N: usize> Eq for Chunk<N> {}

impl<const N: usize> PartialEq for Chunk<N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_byte_array()
            .eq(other.as_byte_array())
    }
}

impl<const N: usize> From<[u8; N]> for Chunk<N> {
    #[inline]
    fn from(array: [u8; N]) -> Self {
        Chunk::<N>::new(array)
    }
}

impl<'data, const N: usize> TryFrom<Bytes<'data>> for Chunk<N> {
    type Error = crate::Error;

    #[inline]
    fn try_from(slice: Bytes<'data>) -> Result<Self, Self::Error> {
        Chunk::from_slice::<LittleEndian>(slice.as_slice())
    }
}
