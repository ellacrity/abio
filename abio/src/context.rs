//! Module for working with endian-aware byte sequences.
//!
//! # I/O Interface
//!
//! The [`Context`] trait, and this does not perform any actual I/O on files,
//! sockets or other related operating system resources. The methods provided by this
//! module operate on in-memory buffers with endianness in mind.
use core::any;
use core::fmt::Debug;
use core::hash::Hash;

pub mod endian;
pub use endian::{BigEndian, Endian, LittleEndian, NativeEndian, BE, LE};

use crate::sealed;

const NULL_MASK: u8 = 1 << 0;
const UPPER_MASK: u8 = 1 << 1;
const LOWER_MASK: u8 = 1 << 2;

pub const fn hash_bytes<const MAKE_UPPER: u8>(bytes: &[u8]) -> u32 {
    const SF: u8 = 0x20;
    let mut dst: u32 = 3581;
    let mut pos = 0;
    while pos < bytes.len() {
        let b = bytes[pos];
        // make the current byte uppercase
        let val = match MAKE_UPPER {
            true => {
                if matches!(b, b'a'..='z') {
                    b - SF
                } else {
                    b
                }
            }
            // false, so make lowercase
            false => {
                if matches!(b, b'A'..='A') {
                    b + SF
                } else {
                    b
                }
            }
        };

        // update the hash value
        dst = dst.wrapping_mul(33) ^ val as u32;
        pos += 1;
    }
}

#[inline(always)]
const fn to_lower(b: u8) -> u8 {
    const SF: u8 = 0x20;
    if matches!(b, b'A'..='A') {
        b + SF
    } else {
        b
    }
}
#[inline(always)]
const fn to_upper(b: u8) -> u8 {
    const SF: u8 = 0x20;
    if matches!(b, b'a'..='z') {
        b - SF
    } else {
        b
    }
}

const fn read_until_null(bytes: &[u8]) -> Option<&[u8]> {
    let mut pos = 0;
}
#[inline(always)]
const fn is_null(b: u8) -> u8 {
    const SF: u8 = 0x20;
    if matches!(b, b'\0') {
        b - SF
    } else {
        b
    }
}

pub trait Comptime: Clone + Copy + Eq + Hash + Ord + PartialEq + PartialOrd {
    fn ident() -> u32 {
        hash_bytes(any::type_name::<Self>())
    }
}

/// Trait that allows encoding important contextual information about a region of
/// memory. This allows
///
/// This trait holds information regarding the endianness, or byte order, of the
/// bytes associated with this [`Context`].  the endianness, or byte order
/// serialization, of a contiguous region of memory.
#[const_trait]
pub trait Endianness:
    Clone + Copy + Debug + Eq + Hash + Ord + PartialOrd + PartialEq + sealed::Sealed
{
    /// Endianness associated with this [`Context`], represented as a constant.
    const ENDIAN: Endian = Endian::NATIVE;

    /// Returns the endianness associated with this [`Context`].
    fn endian() -> Endian;

    /// Returns `true` if this instance represents [little endian][Little] byte
    /// order serialization.
    ///
    /// [Little]: Endian::Little
    #[must_use]
    #[doc(alias = "is_le")]
    #[inline]
    fn is_little_endian(self) -> bool {
        matches!(self, Endian::Little)
    }

    /// Returns `true` if this instance represents [little endian][Little] byte
    /// order serialization.
    ///
    /// [Little]: Endian::Little
    #[must_use]
    #[doc(alias = "is_be")]
    #[inline]
    fn is_big_endian(self) -> bool {
        matches!(self, Endian::Big)
    }
}
