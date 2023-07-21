use core::fmt::Debug;
use core::hash::Hash;

use crate::contiguous::Chunk;
use crate::integer::{I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};
use crate::util::sealed::Sealed;
use crate::{Error, Result};

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
    /// Returns `true` if this type is represented in big endian byte order.
    #[inline]
    fn is_big_endian(&self) -> bool {
        !self.is_little_endian()
    }

    /// Returns `true` if this type is represented as little endian byte order.
    fn is_little_endian(&self) -> bool;

    /// Decodes a [`u8`] from a slice of bytes.
    fn read_u8(bytes: &[u8], offset: usize) -> Result<U8>;

    fn read_u16(bytes: &[u8], offset: usize) -> Result<U16>;

    /// Decodes a [`u32`] from a slice of bytes.
    fn read_u32(bytes: &[u8], offset: usize) -> Result<U32>;

    /// Decodes a [`u64`] from a slice of bytes.
    fn read_u64(bytes: &[u8], offset: usize) -> Result<U64>;

    /// Decodes a [`u128`] from a slice of bytes.
    fn read_u128(bytes: &[u8], offset: usize) -> Result<U128>;

    /// Decodes a [`i8`] from a slice of bytes.
    fn read_i8(bytes: &[u8], offset: usize) -> Result<I8>;

    /// Decodes a [`i16`] from a slice of bytes.
    fn read_i16(bytes: &[u8], offset: usize) -> Result<I16>;

    /// Decodes a [`i32`] from a slice of bytes.
    fn read_i32(bytes: &[u8], offset: usize) -> Result<I32>;

    /// Decodes a [`i64`] from a slice of bytes.
    fn read_i64(bytes: &[u8], offset: usize) -> Result<I64>;

    /// Decodes a [`i128`] from a slice of bytes.
    fn read_i128(bytes: &[u8], offset: usize) -> Result<I128>;
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LittleEndian;

impl Endian for LittleEndian {
    #[inline]
    fn is_little_endian(&self) -> bool {
        true
    }

    #[inline]
    fn read_u8(bytes: &[u8], offset: usize) -> Result<U8> {
        Chunk::from_bytes_at(bytes, offset)
            .map(U8::from_le_chunk)
            .map_err(|_| Error::decode_failed("failed to read U8 chunk as little endian"))
    }

    #[inline]
    fn read_u16(bytes: &[u8], offset: usize) -> Result<U16> {
        Chunk::from_bytes_at(bytes, offset)
            .map(U16::from_le_chunk)
            .map_err(|_| Error::decode_failed("failed to read U16 chunk as little endian"))
    }

    #[inline]
    fn read_u32(bytes: &[u8], offset: usize) -> Result<U32> {
        Chunk::from_bytes_at(bytes, offset)
            .map(U32::from_le_chunk)
            .map_err(|_| Error::decode_failed("failed to read U32 chunk as little endian"))
    }

    #[inline]
    fn read_u64(bytes: &[u8], offset: usize) -> Result<U64> {
        Chunk::from_bytes_at(bytes, offset)
            .map(U64::from_le_chunk)
            .map_err(|_| Error::decode_failed("failed to read U64 chunk as little endian"))
    }

    #[inline]
    fn read_u128(bytes: &[u8], offset: usize) -> Result<U128> {
        Chunk::from_bytes_at(bytes, offset)
            .map(U128::from_le_chunk)
            .map_err(|_| Error::decode_failed("failed to read U128 chunk as little endian"))
    }

    fn read_i8(bytes: &[u8], offset: usize) -> Result<I8> {
        Chunk::from_bytes_at(bytes, offset)
            .map(I8::from_le_chunk)
            .map_err(|_| Error::decode_failed("failed to read I8 chunk as little endian"))
    }

    fn read_i16(bytes: &[u8], offset: usize) -> Result<I16> {
        Chunk::from_bytes_at(bytes, offset)
            .map(I16::from_le_chunk)
            .map_err(|_| Error::decode_failed("failed to read I16 chunk as little endian"))
    }

    fn read_i32(bytes: &[u8], offset: usize) -> Result<I32> {
        Chunk::from_bytes_at(bytes, offset)
            .map(I32::from_le_chunk)
            .map_err(|_| Error::decode_failed("failed to read I32 chunk as little endian"))
    }

    fn read_i64(bytes: &[u8], offset: usize) -> Result<I64> {
        Chunk::from_bytes_at(bytes, offset)
            .map(I64::from_le_chunk)
            .map_err(|_| Error::decode_failed("failed to read I64 chunk as little endian"))
    }

    fn read_i128(bytes: &[u8], offset: usize) -> Result<I128> {
        Chunk::from_bytes_at(bytes, offset)
            .map(I128::from_le_chunk)
            .map_err(|_| Error::decode_failed("failed to read I128 chunk as little endian"))
    }
}

impl Sealed for LittleEndian {}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BigEndian;

impl Endian for BigEndian {
    #[inline]
    fn is_little_endian(&self) -> bool {
        false
    }

    #[inline]
    fn read_u8(bytes: &[u8], offset: usize) -> Result<U8> {
        Chunk::from_bytes_at(bytes, offset)
            .map(U8::from_be_chunk)
            .map_err(|_| Error::decode_failed("failed to read U8 chunk as little endian"))
    }

    #[inline]
    fn read_u16(bytes: &[u8], offset: usize) -> Result<U16> {
        Chunk::from_bytes_at(bytes, offset)
            .map(U16::from_be_chunk)
            .map_err(|_| Error::decode_failed("failed to read U16 chunk as little endian"))
    }

    #[inline]
    fn read_u32(bytes: &[u8], offset: usize) -> Result<U32> {
        Chunk::from_bytes_at(bytes, offset)
            .map(U32::from_be_chunk)
            .map_err(|_| Error::decode_failed("failed to read U32 chunk as little endian"))
    }

    #[inline]
    fn read_u64(bytes: &[u8], offset: usize) -> Result<U64> {
        Chunk::from_bytes_at(bytes, offset)
            .map(U64::from_be_chunk)
            .map_err(|_| Error::decode_failed("failed to read U64 chunk as little endian"))
    }

    #[inline]
    fn read_u128(bytes: &[u8], offset: usize) -> Result<U128> {
        Chunk::from_bytes_at(bytes, offset)
            .map(U128::from_be_chunk)
            .map_err(|_| Error::decode_failed("failed to read U128 chunk as little endian"))
    }

    fn read_i8(bytes: &[u8], offset: usize) -> Result<I8> {
        Chunk::from_bytes_at(bytes, offset)
            .map(I8::from_be_chunk)
            .map_err(|_| Error::decode_failed("failed to read I8 chunk as little endian"))
    }

    fn read_i16(bytes: &[u8], offset: usize) -> Result<I16> {
        Chunk::from_bytes_at(bytes, offset)
            .map(I16::from_be_chunk)
            .map_err(|_| Error::decode_failed("failed to read I16 chunk as little endian"))
    }

    fn read_i32(bytes: &[u8], offset: usize) -> Result<I32> {
        Chunk::from_bytes_at(bytes, offset)
            .map(I32::from_be_chunk)
            .map_err(|_| Error::decode_failed("failed to read I32 chunk as little endian"))
    }

    fn read_i64(bytes: &[u8], offset: usize) -> Result<I64> {
        Chunk::from_bytes_at(bytes, offset)
            .map(I64::from_be_chunk)
            .map_err(|_| Error::decode_failed("failed to read I64 chunk as little endian"))
    }

    fn read_i128(bytes: &[u8], offset: usize) -> Result<I128> {
        Chunk::from_bytes_at(bytes, offset)
            .map(I128::from_be_chunk)
            .map_err(|_| Error::decode_failed("failed to read I128 chunk as little endian"))
    }
}

impl Sealed for BigEndian {}

pub type BE = BigEndian;
pub type LE = LittleEndian;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Endianness {
    Big,
    #[default]
    Little,
}

impl From<Endianness> for BigEndian {
    fn from(endian: Endianness) -> Self {
        if matches!(endian, Endianness::Big) {
            BigEndian
        } else {
            panic!("cannot create `BigEndian` from `Endian::Little`")
        }
    }
}

impl From<Endianness> for LittleEndian {
    fn from(endian: Endianness) -> Self {
        if matches!(endian, Endianness::Little) {
            LittleEndian
        } else {
            panic!("cannot create `LittleEndian` from `Endian::Big`")
        }
    }
}

impl From<BigEndian> for Endianness {
    fn from(value: BigEndian) -> Self {
        if value.is_big_endian() {
            Endianness::Big
        } else {
            panic!("Conversion from BigEndian to Endian failed.");
        }
    }
}

impl From<LittleEndian> for Endianness {
    fn from(value: LittleEndian) -> Self {
        if value.is_little_endian() {
            Endianness::Little
        } else {
            panic!("Conversion from LittleEndian to Endian failed.");
        }
    }
}
