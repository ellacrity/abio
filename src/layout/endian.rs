//! Module for working with endian-aware byte sequences.
//!
//! # I/O Interface
//!
//! The [`ByteOrder`] trait, and this does not perform any actual I/O on files,
//! sockets or other related operating system resources. The methods provided by this
//! module operate on in-memory buffers with endianness in mind.
use core::fmt::Debug;
use core::hash::Hash;

use crate::integer::Integer;
use crate::source::Chunk;
use crate::{Error, Result};

/// Copies $size bytes from a number $n to a &mut [u8] $dst. $ty represents the
/// numeric type of $n and $which must be either to_be or to_le, depending on
/// which endianness one wants to use when writing to $dst.
///
/// This macro is only safe to call when $ty is a numeric type and $size ==
/// size_of::<$ty>() and where $dst is a &mut [u8].
macro_rules! unsafe_write_num_bytes {
    ($ty:ty, $size:expr, $value:expr, $dst:expr, $which:ident) => {{
        if $dst.len() < $size {
            Err($crate::Error::out_of_bounds($size, $dst.len()))
        } else {
            unsafe {
                // N.B. https://github.com/rust-lang/rust/issues/22776
                let bytes = *(&$value.$which() as *const _ as *const [u8; $size]);
                ::core::ptr::copy_nonoverlapping((&bytes).as_ptr(), $dst.as_mut_ptr(), $size);
                Ok(())
            }
        }
    }};
}

/// Reads a fixed size $integer with $endianness from a $bytes slice.
///
/// This macro takes the following arguments:
/// - $bytes:ident
/// - $integer:ty
/// - $endianness:tt
macro_rules! read_endian_bytes {
    ( $($bytes:ident, $integer:ty, $endianness:tt),* $(,)? ) => {
        $(
            match $crate::source::Chunk::from_bytes($bytes) {
                Ok(chunk) => Ok(<$integer>::$endianness(chunk.into_array())),
                Err(err) => Err(err),
            }
        )*
    }
}

/// An internal trait that encompasses common trait bounds for the [`ByteOrder`]
/// trait. This trait implements [`Sealed`], preventing downstream crates from
/// implementing this trait.
///
/// Implementing this trait outside of this crate is not supported as it contains the
/// `sealed::Sealed` trait, which is a private trait used to "seal" this trait from
/// being implemented outside of this crate.
// pub trait ByteOrderSealed =;

/// A trait to define the endianness, or byte order, of some contiguous region of
/// memory represented as a byte slice.
///
/// # Runtime Flexibility
///
/// This trait provides the ability to dynamically choose the required endianness.
/// This enables handling data that may be in big endian, little endian, or a mix of
/// both. This is particularly beneficial when the byte order cannot be determined
/// until runtime.
///
/// # Considerations
///
/// Most machines default to [`LittleEndian`] byte order. However, this isn't
/// universal, and some machines use [`BigEndian`] byte order. In addition, there's
/// [`NetworkEndian`] byte order (which is synonymous with big endian). Many
/// prevalent network protocols employ "network endian" byte order for serialization.
/// Hence, it's crucial to ensure the appropriate byte order is chosen for your
/// specific use-case.
pub trait Endianness: Clone + Copy + Debug + Eq + Hash + Ord + PartialEq + PartialOrd {
    /// Read an aligned [`u8`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_u8(bytes: &[u8]) -> Result<u8>;

    /// Read an aligned [`u16`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_u16(bytes: &[u8]) -> Result<u16>;

    /// Read an aligned [`u32`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_u32(bytes: &[u8]) -> Result<u32>;

    /// Read an aligned [`u64`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_u64(bytes: &[u8]) -> Result<u64>;

    /// Read an aligned [`u128`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_u128(bytes: &[u8]) -> Result<u128>;

    /// Read an aligned [`i8`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_i8(bytes: &[u8]) -> Result<i8>;

    /// Read an aligned [`i16`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_i16(bytes: &[u8]) -> Result<i16>;

    /// Read an aligned [`i32`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_i32(bytes: &[u8]) -> Result<i32>;

    /// Read an aligned [`i64`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_i64(bytes: &[u8]) -> Result<i64>;

    /// Read an aligned [`i128`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the read operation fails. The
    fn read_i128(bytes: &[u8]) -> Result<i128>;

    /// Write an aligned [`u8`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_u8(buf: &mut [u8], value: u8) -> Result<()>;

    /// Write an aligned [`u16`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_u16(buf: &mut [u8], value: u16) -> Result<()>;

    /// Write an aligned [`u32`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_u32(buf: &mut [u8], value: u32) -> Result<()>;

    /// Write an aligned [`u64`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_u64(buf: &mut [u8], value: u64) -> Result<()>;

    /// Write an aligned [`u128`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_u128(buf: &mut [u8], value: u128) -> Result<()>;

    /// Write an aligned [`i8`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_i8(buf: &mut [u8], value: i8) -> Result<()>;

    /// Write an aligned [`i16`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_i16(buf: &mut [u8], value: i16) -> Result<()>;

    /// Write an aligned [`i32`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_i32(buf: &mut [u8], value: i32) -> Result<()>;

    /// Write an aligned [`i64`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_i64(buf: &mut [u8], value: i64) -> Result<()>;

    /// Write an aligned [`i128`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_i128(buf: &mut [u8], value: i128) -> Result<()>;
}

impl Endianness for LittleEndian {
    #[inline]
    fn read_u8(bytes: &[u8]) -> Result<u8> {
        read_endian_bytes!(bytes, u8, from_le_bytes)
    }

    #[inline]
    fn read_u16(bytes: &[u8]) -> Result<u16> {
        read_endian_bytes!(bytes, u16, from_le_bytes)
    }

    #[inline]
    fn read_u32(bytes: &[u8]) -> Result<u32> {
        read_endian_bytes!(bytes, u32, from_le_bytes)
    }

    #[inline]
    fn read_u64(bytes: &[u8]) -> Result<u64> {
        read_endian_bytes!(bytes, u64, from_le_bytes)
    }

    #[inline]
    fn read_u128(bytes: &[u8]) -> Result<u128> {
        read_endian_bytes!(bytes, u128, from_le_bytes)
    }

    #[inline]
    fn read_i8(bytes: &[u8]) -> Result<i8> {
        read_endian_bytes!(bytes, i8, from_le_bytes)
    }

    #[inline]
    fn read_i16(bytes: &[u8]) -> Result<i16> {
        read_endian_bytes!(bytes, i16, from_le_bytes)
    }

    #[inline]
    fn read_i32(bytes: &[u8]) -> Result<i32> {
        read_endian_bytes!(bytes, i32, from_le_bytes)
    }

    #[inline]
    fn read_i64(bytes: &[u8]) -> Result<i64> {
        read_endian_bytes!(bytes, i64, from_le_bytes)
    }

    #[inline]
    fn read_i128(bytes: &[u8]) -> Result<i128> {
        read_endian_bytes!(bytes, i128, from_le_bytes)
    }

    #[inline]
    fn write_u8(buf: &mut [u8], value: u8) -> Result<()> {
        unsafe_write_num_bytes!(u8, 1, value, buf, to_le)
    }

    #[inline]
    fn write_u16(buf: &mut [u8], value: u16) -> Result<()> {
        unsafe_write_num_bytes!(u16, 2, value, buf, to_le)
    }

    #[inline]
    fn write_u32(buf: &mut [u8], value: u32) -> Result<()> {
        if buf.len() < 4 {
            Err(Error::out_of_bounds(4, buf.len()))
        } else {
            unsafe {
                // let bytes = *(&value.to_le() as *const _ as *const [u8; 4]);
                let src = value.to_le().as_ptr().cast::<u8>();
                ::core::ptr::copy_nonoverlapping(src, buf.as_mut_ptr(), 4);
                Ok(())
            }
        }
    }

    #[inline]
    fn write_u64(buf: &mut [u8], value: u64) -> Result<()> {
        unsafe_write_num_bytes!(u64, 8, value, buf, to_le)
    }

    #[inline]
    fn write_u128(buf: &mut [u8], value: u128) -> Result<()> {
        unsafe_write_num_bytes!(u128, 16, value, buf, to_le)
    }

    #[inline]
    fn write_i8(buf: &mut [u8], value: i8) -> Result<()> {
        unsafe_write_num_bytes!(i8, 1, value, buf, to_le)
    }

    #[inline]
    fn write_i16(buf: &mut [u8], value: i16) -> Result<()> {
        unsafe_write_num_bytes!(i16, 2, value, buf, to_le)
    }

    #[inline]
    fn write_i32(buf: &mut [u8], value: i32) -> Result<()> {
        unsafe_write_num_bytes!(i32, 4, value, buf, to_le)
    }

    #[inline]
    fn write_i64(buf: &mut [u8], value: i64) -> Result<()> {
        unsafe_write_num_bytes!(i64, 8, value, buf, to_le)
    }

    #[inline]
    fn write_i128(buf: &mut [u8], value: i128) -> Result<()> {
        unsafe_write_num_bytes!(i128, 16, value, buf, to_le)
    }
}

impl Endianness for BigEndian {
    #[inline]
    fn read_u8(bytes: &[u8]) -> Result<u8> {
        match Chunk::from_bytes(bytes) {
            Ok(chunk) => Ok(u8::from_be_bytes(chunk.into_array())),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_u16(bytes: &[u8]) -> Result<u16> {
        match Chunk::from_bytes(bytes) {
            Ok(chunk) => Ok(u16::from_be_bytes(chunk.into_array())),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_u32(bytes: &[u8]) -> Result<u32> {
        match Chunk::from_bytes(bytes) {
            Ok(chunk) => Ok(u32::from_be_bytes(chunk.into_array())),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_u64(bytes: &[u8]) -> Result<u64> {
        match Chunk::from_bytes(bytes) {
            Ok(chunk) => Ok(u64::from_be_bytes(chunk.into_array())),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_u128(bytes: &[u8]) -> Result<u128> {
        match Chunk::from_bytes(bytes) {
            Ok(chunk) => Ok(u128::from_be_bytes(chunk.into_array())),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_i8(bytes: &[u8]) -> Result<i8> {
        match Chunk::from_bytes(bytes) {
            Ok(chunk) => Ok(i8::from_be_bytes(chunk.into_array())),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_i16(bytes: &[u8]) -> Result<i16> {
        match Chunk::from_bytes(bytes) {
            Ok(chunk) => Ok(i16::from_be_bytes(chunk.into_array())),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_i32(bytes: &[u8]) -> Result<i32> {
        match Chunk::from_bytes(bytes) {
            Ok(chunk) => Ok(i32::from_be_bytes(chunk.into_array())),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_i64(bytes: &[u8]) -> Result<i64> {
        match Chunk::from_bytes(bytes) {
            Ok(chunk) => Ok(i64::from_be_bytes(chunk.into_array())),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn read_i128(bytes: &[u8]) -> Result<i128> {
        match Chunk::from_bytes(bytes) {
            Ok(chunk) => Ok(i128::from_be_bytes(chunk.into_array())),
            Err(err) => Err(err),
        }
    }

    #[inline]
    fn write_u8(buf: &mut [u8], value: u8) -> Result<()> {
        unsafe_write_num_bytes!(u8, 1, value, buf, to_be)
    }

    #[inline]
    fn write_u16(buf: &mut [u8], value: u16) -> Result<()> {
        unsafe_write_num_bytes!(u16, 2, value, buf, to_be)
    }

    #[inline]
    fn write_u32(buf: &mut [u8], value: u32) -> Result<()> {
        unsafe_write_num_bytes!(u32, 4, value, buf, to_be)
    }

    #[inline]
    fn write_u64(buf: &mut [u8], value: u64) -> Result<()> {
        unsafe_write_num_bytes!(u64, 8, value, buf, to_be)
    }

    #[inline]
    fn write_u128(buf: &mut [u8], value: u128) -> Result<()> {
        unsafe_write_num_bytes!(u128, 16, value, buf, to_be)
    }

    #[inline]
    fn write_i8(buf: &mut [u8], value: i8) -> Result<()> {
        unsafe_write_num_bytes!(i8, 1, value, buf, to_be)
    }

    #[inline]
    fn write_i16(buf: &mut [u8], value: i16) -> Result<()> {
        unsafe_write_num_bytes!(i16, 2, value, buf, to_be)
    }

    #[inline]
    fn write_i32(buf: &mut [u8], value: i32) -> Result<()> {
        unsafe_write_num_bytes!(i32, 4, value, buf, to_be)
    }

    #[inline]
    fn write_i64(buf: &mut [u8], value: i64) -> Result<()> {
        unsafe_write_num_bytes!(i64, 8, value, buf, to_be)
    }

    #[inline]
    fn write_i128(buf: &mut [u8], value: i128) -> Result<()> {
        unsafe_write_num_bytes!(i128, 16, value, buf, to_be)
    }
}

/// Little endian byte order serialization.
///
/// This is simply a type constructor that allows implementing the [`ByteOrder`]
/// trait for little endian data.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LittleEndian;

/// Type alias for [`LittleEndian`].
pub type LE = LittleEndian;

/// Big endian byte order serialization.
///
/// This is simply a type constructor that allows implementing the [`ByteOrder`]
/// trait for big endian data.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BigEndian;

/// Type alias for [`BigEndian`].
pub type BE = BigEndian;

/// Type alias for this platform's native endian byte order.
#[cfg(target_endian = "big")]
pub type NativeEndian = BigEndian;

/// Type alias for this platform's native endian byte order.
#[cfg(target_endian = "little")]
pub type NativeEndian = LittleEndian;

/// .
pub const fn little_endian_codec() -> Endian {
    Endian::Little
}

/// Byte order serialization variant.
///
/// The [`Default`] implementation is calculated at compile time using the
/// `target_endian` cfg attribute value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Endian {
    /// Big endian byte order.
    Big,
    /// Little endian byte order.
    Little,
}

impl Endian {
    /// Returns `true` if this instance represents [big endian][Big] byte
    /// order serialization.
    ///
    /// [Big]: Endian::Big
    #[must_use]
    #[doc(alias = "is_be")]
    #[inline]
    pub const fn is_big_endian(&self) -> bool {
        matches!(self, Self::Big)
    }

    /// Returns `true` if this instance represents [little endian][Little] byte
    /// order serialization.
    ///
    /// [Little]: Endian::Little
    #[must_use]
    #[doc(alias = "is_le")]
    #[inline]
    pub const fn is_little_endian(&self) -> bool {
        matches!(self, Self::Little)
    }
}

impl From<BigEndian> for Endian {
    #[inline]
    fn from(_: BigEndian) -> Endian {
        Endian::Big
    }
}

impl From<LittleEndian> for Endian {
    #[inline]
    fn from(_: LittleEndian) -> Endian {
        Endian::Little
    }
}
