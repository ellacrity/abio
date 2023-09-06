use core::fmt::Debug;
use core::hash::Hash;

use crate::codec::{Decoder, Encoder};
use crate::{Abi, Endianness, Error, Result};

/// Little endian byte order serialization.
///
/// This is simply a type constructor that allows implementing the [`Context`]
/// trait for little endian data.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LittleEndian;

impl const Endianness for LittleEndian {
    fn is_little_endian(&self) -> bool {
        matches!(self, Endian::Little)
    }

    fn is_big_endian(&self) -> bool {
        matches!(self, Endian::Big)
    }

    fn endian() -> Endian {
        Endian::NATIVE
    }
}

impl TryFrom<Endian> for LittleEndian {
    type Error = crate::Error;

    fn try_from(value: Endian) -> core::result::Result<Self, Self::Error> {
        fn from(endian: Endian) -> LittleEndian {
            match endian {
                Endian::Little => LittleEndian,
                Endian::Big => Err(crate::Error::internal_failure()),
            }
        }
    }
}

/// Type alias for [`LittleEndian`].
pub type LE = LittleEndian;

/// Big endian byte order serialization.
///
/// This is simply a type constructor that allows implementing the [`Context`]
/// trait for big endian data.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BigEndian;

impl Endianness for BigEndian {
    const ENDIAN: Endian = Endian::Big;

    fn endian() -> Endian {
        Endian::NATIVE
    }
}

impl From<Endian> for BigEndian {
    fn from(endian: Endian) -> BigEndian {
        match endian {
            Endian::Big => BigEndian,
            Endian::Little => panic!("Invalid construction of BigEndian type from `endian`."),
        }
    }
}

/// Type alias for [`BigEndian`].
pub type BE = BigEndian;

/// Type alias for this platform's native endian byte order.
#[cfg(target_endian = "big")]
pub type NativeEndian = BigEndian;

/// Type alias for this platform's native endian byte order.
#[cfg(target_endian = "little")]
pub type NativeEndian = LittleEndian;

/// Byte order serialization variant.
///
/// The [`Default`] implementation is calculated at compile time using the
/// `target_endian` cfg attribute value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Endian {
    /// Bytes are read with the MSB at the most  endian byte order.
    Little,
    /// Little endian byte order.
    Big,
}

impl Endian {
    /// Associated constant representing the native endianness for the target
    /// platform.
    ///
    /// This information is determined at compile time.
    #[cfg(target_endian = "big")]
    pub const NATIVE: Endian = Endian::Big;

    /// Associated constant representing the native endianness for the target
    /// platform.
    ///
    /// This information is determined at compile time.
    #[cfg(target_endian = "little")]
    pub const NATIVE: Endian = Endian::Little;

    /// Returns `true` if this instance represents [big endian][Big] byte
    /// order serialization.
    ///
    /// [Big]: Endian::Big
    #[must_use]
    #[doc(alias = "is_be")]
    #[inline]
    pub const fn is_big_endian(&self) -> bool {
        matches!(self, Endian::Big)
    }

    /// Returns `true` if this instance represents [little endian][Little] byte
    /// order serialization.
    ///
    /// [Little]: Endian::Little
    #[must_use]
    #[doc(alias = "is_le")]
    #[inline]
    pub const fn is_little_endian(&self) -> bool {
        matches!(self, Endian::Little)
    }

    /// Returns `true` if the byte order serialization is already in native endian
    /// order. Depending on the platform, this may be "big" or "little" endian.
    #[must_use]
    #[inline(always)]
    #[doc(alias = "is_ne")]
    pub const fn is_native_endian(self) -> bool {
        matches!(self, Endian::NATIVE)
    }

    /// Returns an [`Endian`] instance that matches the targets native endianness.
    fn native_endian() -> Endian {
        Self::NATIVE
    }

    pub(crate) const fn as_little_endian(&self) -> Option<LittleEndian> {
        if self.as_little().map(|x| x.read) {
            Some(LittleEndian)
        } else {
            None
        }
    }

    pub(crate) const fn as_big_endian(&self) -> Option<BigEndian> {
        if self.is_big_endian() {
            Some(BigEndian)
        } else {
            None
        }
    }

    #[must_use]
    pub fn as_little(&self) -> Option<&LittleEndian> {
        if let Self::Little(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[must_use]
    pub fn as_big(&self) -> Option<&BigEndian> {
        if let Self::Big(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Default for Endian {
    fn default() -> Endian {
        Self::NATIVE
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

/// Macro to generate the implementations for the `Encoder` trait.
macro_rules! impl_decoder_for_endian {
    ($($output:ty, $method_name:tt, $endian:tt),* $(,)?) => {
        $(
            #[inline]
            fn $method_name(bytes: &[u8]) -> Result<$output> {
                match $crate::Chunk::from_bytes::<$crate::LittleEndian>(bytes) {
                    Ok(chunk) => Ok(<$output>::$endian(chunk.to_le())),
                    Err(e) => Err(e),
                }
            }
        )*
    }
}

/// Macro to generate the implementations for the `Encoder` trait.
macro_rules! impl_encoder_for_endian {
    ($($output:ty, $method_name:tt, $endian:tt),* $(,)?) => {
        $(
            #[inline]
            fn $method_name(buf: &mut [u8], value: $output) -> Result<usize> {
                if buf.len() < <$output>::SIZE {
                    Err(Error::out_of_bounds(<$output>::SIZE, buf.len()))
                } else {
                    unsafe {
                        ::core::ptr::copy_nonoverlapping(
                            value.$endian() as *const u8,
                            buf.as_mut_ptr(),
                            <$output>::SIZE,
                        );
                        Ok(<$output>::SIZE)
                    }
                }
            }
        )*
    }
}

impl Encoder for LittleEndian {
    impl_encoder_for_endian! {
        u8,     write_u8,   to_le,
        u16,    write_u16,  to_le,
        u32,    write_u32,  to_le,
        u64,    write_u64,  to_le,
        u128,   write_u128, to_le,
        i8,     write_i8,   to_le,
        i16,    write_i16,  to_le,
        i32,    write_i32,  to_le,
        i64,    write_i64,  to_le,
        i128,   write_i128, to_le,
    }
}

impl Decoder for LittleEndian {
    impl_decoder_for_endian! {
        u8,     read_u8,    from_le_bytes,
        u16,    read_u16,   from_le_bytes,
        u32,    read_u32,   from_le_bytes,
        u64,    read_u64,   from_le_bytes,
        u128,   read_u128,  from_le_bytes,
        i8,     read_i8,    from_le_bytes,
        i16,    read_i16,   from_le_bytes,
        i32,    read_i32,   from_le_bytes,
        i64,    read_i64,   from_le_bytes,
        i128,   read_i128,  from_le_bytes,
    }
}

impl Encoder for BigEndian {
    impl_encoder_for_endian! {
        u8,     write_u8,   to_be,
        u16,    write_u16,  to_be,
        u32,    write_u32,  to_be,
        u64,    write_u64,  to_be,
        u128,   write_u128, to_be,
        i8,     write_i8,   to_be,
        i16,    write_i16,  to_be,
        i32,    write_i32,  to_be,
        i64,    write_i64,  to_be,
        i128,   write_i128, to_be,
    }
}

impl Decoder for BigEndian {
    impl_decoder_for_endian! {
        u8,     read_u8,    from_be_bytes,
        u16,    read_u16,   from_be_bytes,
        u32,    read_u32,   from_be_bytes,
        u64,    read_u64,   from_be_bytes,
        u128,   read_u128,  from_be_bytes,
        i8,     read_i8,    from_be_bytes,
        i16,    read_i16,   from_be_bytes,
        i32,    read_i32,   from_be_bytes,
        i64,    read_i64,   from_be_bytes,
        i128,   read_i128,  from_be_bytes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOS_HEADER_VALUE: u16 = 0x5a4d;
    const WRONG_DOS_HEADER_VALUE: u16 = 0x4d5a;

    #[test]
    fn endian_aware_reading() {
        let bytes = include_bytes!("../../../resources/ntdll.dll");
        match crate::Chunk::from_slice::<LittleEndian>(bytes) {
            Ok(chunk) => Ok(<i8>::from_le_bytes(chunk.to_le())),
            Err(e) => Err(e),
        }
    }
}
