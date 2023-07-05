use crate::integral::*;

mod byteorder;

/// Trait
pub trait Endianness {
    fn read_u8(bytes: &[u8]) -> U8;

    fn read_u16(bytes: &[u8]) -> U16;

    fn read_u32(bytes: &[u8]) -> U32;

    fn read_u64(bytes: &[u8]) -> U64;

    fn read_u128(bytes: &[u8]) -> U128;

    fn read_i8(bytes: &[u8]) -> I8;

    fn read_i16(bytes: &[u8]) -> I16;

    fn read_i32(bytes: &[u8]) -> I32;

    fn read_i64(bytes: &[u8]) -> I64;

    fn read_i128(bytes: &[u8]) -> I128;
}

/// Defines big-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `u32` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, BigEndian};
///
/// let mut buf = [0; 4];
/// BigEndian::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, BigEndian::read_u32(&buf));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BigEndian {}

impl Default for BigEndian {
    fn default() -> BigEndian {
        panic!("BigEndian default")
    }
}

/// A type alias for [`BigEndian`].
///
/// [`BigEndian`]: enum.BigEndian.html
pub type BE = BigEndian;

/// Defines little-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `u32` numbers in little endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, LittleEndian};
///
/// let mut buf = [0; 4];
/// LittleEndian::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LittleEndian {}

impl Default for LittleEndian {
    fn default() -> LittleEndian {
        panic!("LittleEndian default")
    }
}

/// A type alias for [`LittleEndian`].
///
/// [`LittleEndian`]: enum.LittleEndian.html
pub type LE = LittleEndian;

/// Defines network byte order serialization.
///
/// Network byte order is defined by [RFC 1700][1] to be big-endian, and is
/// referred to in several protocol specifications.  This type is an alias of
/// [`BigEndian`].
///
/// [1]: https://tools.ietf.org/html/rfc1700
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `i16` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, NetworkEndian, BigEndian};
///
/// let mut buf = [0; 2];
/// BigEndian::write_i16(&mut buf, -5_000);
/// assert_eq!(-5_000, NetworkEndian::read_i16(&buf));
/// ```
///
/// [`BigEndian`]: enum.BigEndian.html
pub type NetworkEndian = BigEndian;

/// Defines system native-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// On this platform, this is an alias for [`LittleEndian`].
///
/// [`LittleEndian`]: enum.LittleEndian.html
#[cfg(target_endian = "little")]
pub type NativeEndian = LittleEndian;

/// Defines system native-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// On this platform, this is an alias for [`BigEndian`].
///
/// [`BigEndian`]: enum.BigEndian.html
#[cfg(target_endian = "big")]
pub type NativeEndian = BigEndian;
