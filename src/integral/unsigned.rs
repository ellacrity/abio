//! Aligned, endian-aware __unsigned__ integral types.

use core::mem::{align_of, size_of};

use crate::bytes::AlignedBytes;
use crate::error::LayoutError;
use crate::SizeMismatchError;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct U8(u8);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct U16(u16);

impl U16 {
    pub fn read_aligned(bytes: &[u8]) -> Result<U16, SizeMismatchError> {
        let src_len = bytes.len();
        if src_len < ::core::mem::size_of::<Self>() {
            return Err(SizeMismatchError::new::<Self>(src_len));
        }

        let parsed: [u8; ::core::mem::size_of::<Self>()] = bytes[..::core::mem::size_of::<Self>()]
            .try_into()
            .expect("infallible conversion occurred from bytes to integer");
        if parsed.is_ptr_aligned::<Self>() {
            Ok(U16::new(u16::from_le_bytes(parsed)))
        } else {
            Err(SizeMismatchError::new::<Self>(src_len))
        }
    }

    pub const fn from_le_bytes(bytes: [u8; core::mem::size_of::<Self>()]) -> U16 {
        U16(u16::from_le_bytes(bytes))
    }
}

/// Aligned, 32-bit integer with native-endian byte order.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct U32(u32);

impl U32 {
    // /// Creates a new [`U32`] from a fixed size array of bytes.
    // pub fn new(value: u32) -> U32 {
    //     U32(u32::from_le(value))
    // }

    /// Reads an aligned, endian-aware [`U32`] from a slice of bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn read(bytes: &[u8]) -> Result<U32, LayoutError> {
        if bytes.len() < ::core::mem::size_of::<Self>() {
            return Err(LayoutError::new("unsufficient memory to construct U32 from bytes: `bytes.len() < size_of::<Self>()` "));
        }

        let byte_array: [u8; ::core::mem::size_of::<Self>()] = bytes
            [..::core::mem::size_of::<Self>()]
            .try_into()
            .expect("infallible conversion occurred from bytes to integer");

        if !bytes.is_ptr_aligned::<Self>() {
            Err(LayoutError::new("attempted to read from an unaligned pointer"))
        } else {
            Ok(Self::new(u32::from_le_bytes(byte_array)))
        }
    }

    // /// Convert this [U32] into a fixed size array of bytes.
    // pub fn to_bytes(&self) -> &[u8] {
    //     &self.0.to_le_bytes()[..]
    // }

    // /// Return the memory representation of this integer as a byte array in
    // /// little-endian byte order.
    // pub fn to_le_bytes(self) -> [u8; size_of::<Self>()] {
    //     self.0.to_le_bytes()
    // }

    /// Gets the inner value from this container in little-endian byte order.
    pub fn get_le(self) -> u32 {
        self.0.to_le()
    }

    /// Gets the inner value from this container in big-endian byte order.
    pub fn get_be(self) -> u32 {
        self.0.to_be()
    }

    pub const fn from_le_bytes(bytes: [u8; core::mem::size_of::<Self>()]) -> U32 {
        U32(u32::from_le_bytes(bytes))
    }
}

impl From<u32> for U32 {
    fn from(value: u32) -> U32 {
        U32::new(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct U64(u64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct U128(u128);

macro_rules! impl_unsigned_int {
    (
        $wrapper:ident -> $inner:tt
    ) => {
        impl $wrapper {
            #[doc = concat!("Creates a new [`", stringify!($wrapper), "`] from a fixed size array of bytes.")]
            pub fn new(value: $inner) -> $wrapper {
                $wrapper(<$inner>::from_le(value))
            }

            /// Return the memory representation of this integer as a byte array in
            /// little-endian byte order.
            pub fn to_le_bytes(self) -> [u8; size_of::<Self>()] {
                self.0.to_le_bytes()
            }

            /// Gets the value from this container in native-endian byte order.
            pub fn get(self) -> $inner {
                self.0
            }
        }
    };
}

macro_rules! impl_aligned_read {
    ($ty:ty) => {
        $(
            /// Reads an aligned, endian-aware [`U32`] from a slice of bytes.
            ///
            /// # Errors
            ///
            /// This function will return an error if .
            pub fn read(bytes: &[u8]) -> Result<$ty, $crate::error::LayoutError> {
                read_aligned!(bytes, $ty -> read_$inner)
            }
        )*
    };
}

impl_unsigned_int!(U8 -> u8);
impl_unsigned_int!(U16 -> u16);
impl_unsigned_int!(U32 -> u32);
impl_unsigned_int!(U64 -> u64);
impl_unsigned_int!(U128 -> u128);
