use core::fmt::Debug;
use core::hash::Hash;

use crate::contiguous::Chunk;
use crate::integer::{I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};
use crate::{Error, Result};

mod sealed {
    //! Module containing the [`Sealed`] trait, which prevents downstream users of
    //! this crate from implementing certain items.

    use crate::Chunk;

    #[doc(hidden)]
    pub(crate) trait Sealed {}
    impl Sealed for [u8] {}
    impl Sealed for &'_ [u8] {}
    impl Sealed for core::cell::Ref<'_, u8> {}
    impl Sealed for core::cell::RefMut<'_, u8> {}

    impl<T> Sealed for T where T: super::Endian {}
    // impl Sealed for str {}
    // impl Sealed for &'_ str {}

    impl Sealed for crate::Bytes {}
    impl<const N: usize> Sealed for [u8; N] {}
    impl<const N: usize> Sealed for Chunk<N> {}
}

pub struct Config<E, N> {}

pub(crate) struct ConfigBuilder<E, N> {}

impl<E, N> ConfigBuilder<E, N> {
    pub(crate) fn builder(endian: E) -> Self {
        Self {}
    }
}

impl<E: Endian, L> Config<E, L> {
    fn builder() -> ConfigBuilder<E, L> {
        Self {}
    }
}

/// A trait that defines the endianness of a sequence of bytes. Thia trait is
/// particularly useful for reading and writing integer values to and from byte
/// slices.
///
/// # Byte Order Serialization
///
/// The [`Endian`] trait defines types with a known byte order serialization
/// type.
///
/// Most machines use [`LittleEndian`] byte order by default. However, this is
/// not absolute, as some machines use [`BigEndian`] by default.
///
/// This trait helps deal with endianness during runtime, allowing greater
/// flexibility over how data is read and interpreted.
///
/// # Network Endian
///
/// Very commonly, network protocols opt to use [`BigEndian`] byte order, also known
/// as "network endian". You will likely encounter this byte order serialization type
/// and so this trait helps convert between the types.
pub trait Endian: Sized + Copy + Default + Eq + Ord + PartialEq + PartialOrd {
    /// Decodes a [`u8`] from a slice of bytes.
    fn read_u8(bytes: &[u8]) -> Result<U8>;

    fn read_u16(bytes: &[u8]) -> Result<U16>;

    /// Decodes a [`u32`] from a slice of bytes.
    fn read_u32(bytes: &[u8]) -> Result<U32>;

    /// Decodes a [`u64`] from a slice of bytes.
    fn read_u64(bytes: &[u8]) -> Result<U64>;

    /// Decodes a [`u128`] from a slice of bytes.
    fn read_u128(bytes: &[u8]) -> Result<U128>;

    /// Decodes a [`i8`] from a slice of bytes.
    fn read_i8(bytes: &[u8]) -> Result<I8>;

    /// Decodes a [`i16`] from a slice of bytes.
    fn read_i16(bytes: &[u8]) -> Result<I16>;

    /// Decodes a [`i32`] from a slice of bytes.
    fn read_i32(bytes: &[u8]) -> Result<I32>;

    /// Decodes a [`i64`] from a slice of bytes.
    fn read_i64(bytes: &[u8]) -> Result<I64>;

    /// Decodes a [`i128`] from a slice of bytes.
    fn read_i128(bytes: &[u8]) -> Result<I128>;
}

/// Little endian byte order serialization.
///
/// This is simply a type constructor that allows implementing the [`Endian`] trait
/// for little endian data.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LittleEndian;

impl Default for LittleEndian {
    fn default() -> Self {
        unimplemented!("The `Default::default()` method should not be called for `LittleEndian`.")
    }
}

impl Endian for LittleEndian {
    #[inline]
    fn read_u8(bytes: &[u8]) -> Result<U8> {
        Chunk::from_bytes(bytes).map(U8::from_le_chunk).map_err(|_| Error::decode_failed())
    }

    #[inline]
    fn read_u16(bytes: &[u8]) -> Result<U16> {
        Chunk::from_bytes(bytes).map(U16::from_le_chunk).map_err(|_| Error::decode_failed())
    }

    #[inline]
    fn read_u32(bytes: &[u8]) -> Result<U32> {
        Chunk::from_bytes(bytes).map(U32::from_le_chunk).map_err(|_| Error::decode_failed())
    }

    #[inline]
    fn read_u64(bytes: &[u8]) -> Result<U64> {
        Chunk::from_bytes(bytes).map(U64::from_le_chunk).map_err(|_| Error::decode_failed)
    }

    #[inline]
    fn read_u128(bytes: &[u8]) -> Result<U128> {
        Chunk::from_bytes(bytes).map(U128::from_le_chunk).map_err(|_| Error::decode_failed())
    }

    fn read_i8(bytes: &[u8]) -> Result<I8> {
        Chunk::from_bytes(bytes).map(I8::from_le_chunk).map_err(|_| Error::decode_failed())
    }

    fn read_i16(bytes: &[u8]) -> Result<I16> {
        Chunk::from_bytes(bytes).map(I16::from_le_chunk).map_err(|_| Error::decode_failed())
    }

    fn read_i32(bytes: &[u8]) -> Result<I32> {
        Chunk::from_bytes(bytes).map(I32::from_le_chunk).map_err(|_| Error::decode_failed())
    }

    fn read_i64(bytes: &[u8]) -> Result<I64> {
        Chunk::from_bytes(bytes).map(I64::from_le_chunk).map_err(|_| Error::decode_failed())
    }

    fn read_i128(bytes: &[u8]) -> Result<I128> {
        Chunk::from_bytes(bytes).map(I128::from_le_chunk).map_err(|e| Error::decode_failed())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BigEndian;

impl Default for BigEndian {
    fn default() -> Self {
        unimplemented!("The `Default::default()` method should not be called for `BigEndian`.")
    }
}

impl Endian for BigEndian {
    #[inline]
    fn read_u8(bytes: &[u8]) -> Result<U8> {
        Chunk::from_bytes(bytes).map(U8::from_be_chunk).map_err(|_| Error::decode_failed())
    }

    #[inline]
    fn read_u16(bytes: &[u8]) -> Result<U16> {
        Chunk::from_bytes(bytes).map(U16::from_be_chunk).map_err(|_| Error::decode_failed())
    }

    #[inline]
    fn read_u32(bytes: &[u8]) -> Result<U32> {
        Chunk::from_bytes(bytes).map(U32::from_be_chunk).map_err(|_| Error::decode_failed())
    }

    #[inline]
    fn read_u64(bytes: &[u8]) -> Result<U64> {
        Chunk::from_bytes(bytes).map(U64::from_be_chunk).map_err(|_| Error::decode_failed())
    }

    #[inline]
    fn read_u128(bytes: &[u8]) -> Result<U128> {
        Chunk::from_bytes(bytes).map(U128::from_be_chunk).map_err(|_| Error::decode_failed())
    }

    fn read_i8(bytes: &[u8]) -> Result<I8> {
        Chunk::from_bytes(bytes).map(I8::from_be_chunk).map_err(|_| Error::decode_failed())
    }

    fn read_i16(bytes: &[u8]) -> Result<I16> {
        Chunk::from_bytes(bytes).map(I16::from_be_chunk).map_err(|_| Error::decode_failed())
    }

    fn read_i32(bytes: &[u8]) -> Result<I32> {
        Chunk::from_bytes(bytes).map(I32::from_be_chunk).map_err(|_| Error::decode_failed())
    }

    fn read_i64(bytes: &[u8]) -> Result<I64> {
        Chunk::from_bytes(bytes).map(I64::from_be_chunk).map_err(|_| Error::decode_failed())
    }

    fn read_i128(bytes: &[u8]) -> Result<I128> {
        Chunk::from_bytes(bytes).map(I128::from_be_chunk).map_err(|_| Error::decode_failed())
    }
}

pub type BE = BigEndian;
pub type LE = LittleEndian;

// #[repr(u8)]
// #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
// pub enum Endianness {
//     Big = 0x00,
//     #[default]
//     Little,
// }
