use core::fmt::Debug;
use core::hash::Hash;
use core::mem::size_of;

use crate::util::into_byte_array;
use crate::{Bytes, Result};

/// Types with a known byte order serialization.
///
/// Most machines use [`LittleEndian`] byte order, but this is not an absolute. Some
/// may use [`BigEndian`], and bytes sent across the wire are typically represented
/// in "network-endian", which is simply an alias for big endian.
pub trait ByteOrder: Sized + Copy + Debug + Eq + Ord + PartialEq + PartialOrd {
    /// Returns true if this type is represented as little endian byte order.
    fn is_little_endian(&self) -> bool;

    /// Returns true if this type is represented as big endian byte order.
    fn is_big_endian(&self) -> bool {
        !self.is_little_endian()
    }
}

macro_rules! deserialize_slice {
    ($slice:ident @ $size:item) => {
        // first code block nested
        {
            if $slice.len() < $size {
                panic!("");
            }
        }
    };
}

/// Determines how a type is deserialized from the byte stream, or [`Source`].
///
/// Primarily used to enforce subtyping on the [`Endianness`] trait.
pub trait Deserializer: ByteOrder {
    /// Deserializes a [`u8`] from a slice of bytes.
    fn deserialize_u8(self, bytes: &[u8]) -> u8;

    /// Deserializes a [`u16`] from a slice of bytes.
    fn deserialize_u16(self, bytes: &[u8]) -> u16;

    /// Deserializes a [`u32`] from a slice of bytes.
    fn deserialize_u32(self, bytes: &[u8]) -> u32;

    /// Deserializes a [`u64`] from a slice of bytes.
    fn deserialize_u64(self, bytes: &[u8]) -> u64;

    /// Deserializes a [`u128`] from a slice of bytes.
    fn deserialize_u128(self, bytes: &[u8]) -> u128;

    #[inline]
    /// Deserializes a [`i8`] from a slice of bytes.
    fn deserialize_i8(self, bytes: &[u8]) -> i8 {
        self.deserialize_u8(bytes) as i8
    }

    #[inline]
    /// Deserializes a [`i16`] from a slice of bytes.
    fn deserialize_i16(self, bytes: &[u8]) -> i16 {
        self.deserialize_u16(bytes) as i16
    }
    #[inline]
    /// Deserializes a [`i32`] from a slice of bytes.
    fn deserialize_i32(self, bytes: &[u8]) -> i32 {
        self.deserialize_u32(bytes) as i32
    }
    #[inline]
    /// Deserializes a [`i64`] from a slice of bytes.
    fn deserialize_i64(self, bytes: &[u8]) -> i64 {
        self.deserialize_u64(bytes) as i64
    }
    #[inline]
    /// Deserializes a [`i128`] from a slice of bytes.
    fn deserialize_i128(self, bytes: &[u8]) -> i128 {
        self.deserialize_u128(bytes) as i128
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LittleEndian;
impl LittleEndian {
    fn read_slice(&self, bytes: &[u8]) -> Result<&[u8]> {
        if self.is_big_endian() {
            let mut ret = bytes.clone();
            ret.reverse();
            Ok(ret)
        } else {
            Ok(bytes)
        }
    }
}

pub type LE = LittleEndian;

impl ByteOrder for LittleEndian {
    fn is_little_endian(&self) -> bool {
        true
    }
}

impl Deserializer for LittleEndian {
    fn deserialize_u8(self, bytes: &[u8]) -> u8 {
        let array = into_byte_array::<1>(bytes).expect(
            "into_byte_array should be an infallible operation if `bytes.len() > Self::SIZE`.",
        );
        u8::from_le_bytes(array)
    }

    fn deserialize_u16(self, bytes: &[u8]) -> u16 {
        let array = into_byte_array::<2>(bytes).expect(
            "into_byte_array should be an infallible operation if `bytes.len() > Self::SIZE`.",
        );
        u16::from_le_bytes(array)
    }

    fn deserialize_u32(self, bytes: &[u8]) -> u32 {
        let size = size_of::<u32>();
        // let array = Bytes::new_array::<[u8; 4]>(bytes).expect("");
        u32::from_le_bytes(into_byte_array::<4>(bytes).expect(
            "into_byte_array should be an infallible operation if `bytes.len() > Self::SIZE`.",
        ))
    }

    fn deserialize_u64(self, bytes: &[u8]) -> u64 {
        let array = into_byte_array(bytes).expect(
            "into_byte_array should be an infallible operation if `bytes.len() > Self::SIZE`.",
        );
        u64::from_le_bytes(array)
    }

    fn deserialize_u128(self, bytes: &[u8]) -> u128 {
        let array = into_byte_array(bytes).expect(
            "into_byte_array should be an infallible operation if `bytes.len() > Self::SIZE`.",
        );
        u128::from_le_bytes(array)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BigEndian;

pub type BE = BigEndian;

impl ByteOrder for BigEndian {
    fn is_little_endian(&self) -> bool {
        false
    }
}

impl Deserializer for BigEndian {
    fn deserialize_u8(self, bytes: &[u8]) -> u8 {
        let array = into_byte_array(bytes).expect(
            "into_byte_array should be an infallible operation if `bytes.len() > Self::SIZE`.",
        );
        u8::from_be_bytes(array)
    }

    fn deserialize_u16(self, bytes: &[u8]) -> u16 {
        let array = into_byte_array(bytes).expect(
            "into_byte_array should be an infallible operation if `bytes.len() > Self::SIZE`.",
        );
        u16::from_be_bytes(array)
    }

    fn deserialize_u32(self, bytes: &[u8]) -> u32 {
        let array = into_byte_array(bytes).expect(
            "into_byte_array should be an infallible operation if `bytes.len() > Self::SIZE`.",
        );
        u32::from_be_bytes(array)
    }

    fn deserialize_u64(self, bytes: &[u8]) -> u64 {
        let array = into_byte_array(bytes).expect(
            "into_byte_array should be an infallible operation if `bytes.len() > Self::SIZE`.",
        );
        u64::from_be_bytes(array)
    }

    fn deserialize_u128(self, bytes: &[u8]) -> u128 {
        let array = into_byte_array(bytes).expect(
            "into_byte_array should be an infallible operation if `bytes.len() > Self::SIZE`.",
        );
        u128::from_be_bytes(array)
    }
}

// impl From<BigEndian> for Endian {
//     fn from(endianness: BigEndian) -> Endian {
//         read_endianness(endianness)
//     }
// }

// fn read_endianness<E: Endianness>(endianness: E) -> Endian {
//     if endianness.is_big_endian() {
//         Endian::Big
//     } else {
//         Endian::Little
//     }
// }

// impl From<LittleEndian> for Endian {
//     fn from(value: LittleEndian) -> Endian {
//         read_endianness(value)
//     }
// }

// /// Byte order of the serialized data.
// #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
// pub enum Endian {
//     Big,
//     #[default]
//     Little,
// }
// impl Endian {
//     pub fn as_big(&self) -> Option<Endian> {
//         match self {
//             Endian::Big => Some(Endian::Big),
//             _ => None,
//         }
//     }

//     pub fn as_little(&self) -> Option<Endian> {
//         match self {
//             Endian::Little => Some(Endian::Little),
//             _ => None,
//         }
//     }
// }

// impl Endianness for Endian {
//     #[inline]
//     fn is_little_endian(&self) -> bool {
//         matches!(self, Endian::Little)
//     }

//     #[inline]
//     fn is_big_endian(&self) -> bool {
//         matches!(self, Endian::Big)
//     }

//     #[inline]
//     fn deserialize_u8(self, array: &[u8]) -> u8 {
//         if self.is_big_endian() {
//             u8::from_be_bytes(array)
//         } else {
//             u8::from_le_bytes(array)
//         }
//     }

//     #[inline]
//     fn deserialize_u16(self, array: [u8; 2]) -> u16 {
//         if self.is_big_endian() {
//             u16::from_be_bytes(array)
//         } else {
//             u16::from_le_bytes(array)
//         }
//     }

//     #[inline]
//     fn deserialize_u32(self, array: [u8; 4]) -> u32 {
//         if self.is_big_endian() {
//             u32::from_be_bytes(array)
//         } else {
//             u32::from_le_bytes(array)
//         }
//     }

//     #[inline]
//     fn deserialize_u64(self, array: [u8; 8]) -> u64 {
//         if self.is_big_endian() {
//             u64::from_be_bytes(array)
//         } else {
//             u64::from_le_bytes(array)
//         }
//     }

//     #[inline]
//     fn deserialize_u128(self, array: [u8; 16]) -> u128 {
//         if self.is_big_endian() {
//             u128::from_be_bytes(array)
//         } else {
//             u128::from_le_bytes(array)
//         }
//     }

//     fn deserialize_i8(self, array: [u8; 1]) -> i8 {
//         if self.is_big_endian() {
//             i8::from_be_bytes(array)
//         } else {
//             i8::from_le_bytes(array)
//         }
//     }

//     fn deserialize_i16(self, array: [u8; 2]) -> i16 {
//         if self.is_big_endian() {
//             i16::from_be_bytes(array)
//         } else {
//             i16::from_le_bytes(array)
//         }
//     }

//     fn deserialize_i32(self, array: [u8; 4]) -> i32 {
//         if self.is_big_endian() {
//             i32::from_be_bytes(array)
//         } else {
//             i32::from_le_bytes(array)
//         }
//     }

//     fn deserialize_i64(self, array: [u8; 8]) -> i64 {
//         if self.is_big_endian() {
//             i64::from_be_bytes(array)
//         } else {
//             i64::from_le_bytes(array)
//         }
//     }

//     fn deserialize_i128(self, array: [u8; 16]) -> i128 {
//         if self.is_big_endian() {
//             i128::from_be_bytes(array)
//         } else {
//             i128::from_le_bytes(array)
//         }
//     }
// }
