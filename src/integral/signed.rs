//! Aligned, endian-aware __signed__ integral types.

use crate::abi::BytesExt;
use crate::Decodable;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct I8(i8);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct I16(i16);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct I32(i32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct I64(i64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct I128(i128);

impl Decodable for I8 {}
impl Decodable for I16 {}
impl Decodable for I32 {}
impl Decodable for I64 {}
impl Decodable for I128 {}

macro_rules! impl_signed_integral {
    (
        $ty:ident -> $int:tt
    ) => {
        impl $ty {
            #[doc = concat!("Creates a new [`", stringify!($ty), "`] from a fixed size array of bytes.")]
            #[inline]
            pub const fn new(value: $int) -> $ty {
                $ty(<$int>::from_le(value))
            }

            #[doc = concat!("Reads an aligned, endian-aware [`", stringify!($ty), "`] from a slice of bytes.")]
            #[doc = ""]
            #[doc = "This method is endian-agnostic, using the host native endianness, or byte order, to"]
            #[doc = "perform any required conversions. In many cases, this results in a noop, since the"]
            #[doc = "bytes already exist in the appropriate byte order serialization type."]
            #[doc = ""]
            #[doc = "# Errors"]
            #[doc = ""]
            #[doc = "This function will return an error if ."]
            pub fn from_bytes(bytes: &[u8]) -> crate::Result<$ty> {
                if bytes.len() < ::core::mem::size_of::<Self>() {
                    return Err($crate::Error::size_mismatch(::core::mem::size_of::<Self>(), bytes.len()));
                }

                let bytes = $crate::util::as_byte_array(bytes)?;
                if !bytes.is_aligned_with::<Self>() {
                    Err(crate::Error::misaligned_access(&bytes))
                } else {
                    Ok(Self::new($int::from_le_bytes(bytes)))
                }
            }

            #[inline]
            pub const fn from_le_bytes(bytes: [u8; ::core::mem::size_of::<Self>()]) -> Self {
                Self(<$int>::from_le_bytes(bytes))
            }

            #[inline]
            pub const fn from_be_bytes(bytes: [u8; ::core::mem::size_of::<Self>()]) -> Self {
                Self(<$int>::from_be_bytes(bytes))
            }

            #[doc = "Return the memory representation of this integer as a byte array in little-endian byte order."]
            #[inline]
            pub const fn to_le_bytes(self) -> [u8; ::core::mem::size_of::<Self>()] {
                self.0.to_le_bytes()
            }
            #[doc = concat!("Gets the inner [`", stringify!($int), "`] value from this container in native-endian byte order.")]
            #[inline(always)]
            pub const fn get(self) -> $int {
                self.0
            }
            #[doc = concat!("Gets the inner [`", stringify!($int), "`] value from this container in little-endian byte order.")]
            #[inline(always)]
            pub const fn get_le(self) -> $int {
                self.0.to_le()
            }
            #[doc = concat!("Gets the inner [`", stringify!($int), "`] value from this container in big-endian byte order.")]
            #[inline(always)]
            pub const fn get_be(self) -> $int {
                self.0.to_be()
            }
        }

        impl From<$int> for $ty {
            fn from(int: $int) -> $ty {
                $ty(int)
            }
        }

        impl ::core::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.get_le(), f)
            }
        }
        impl ::core::fmt::LowerHex for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(&self.get_le(), f)
            }
        }
        impl ::core::fmt::UpperHex for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperHex::fmt(&self.get_le(), f)
            }
        }
        impl ::core::fmt::Binary for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Binary::fmt(&self.get_le(), f)
            }
        }
        impl ::core::fmt::Octal for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Octal::fmt(&self.get_le(), f)
            }
        }
    };
}

impl_signed_integral!(I8 -> i8);
impl_signed_integral!(I16 -> i16);
impl_signed_integral!(I32 -> i32);
impl_signed_integral!(I64 -> i64);
impl_signed_integral!(I128 -> i128);
