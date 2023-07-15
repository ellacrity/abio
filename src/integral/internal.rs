use core::slice;

use crate::integral::aligned::{ISize, USize, I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};
use crate::{Abi, Zeroable};

macro_rules! impl_aligned_integral {
    ( $( $sign:literal, $name:ident: $inner:tt -> $size:literal ),* $(,)?) => {
        $(
            #[doc = concat!($sign, stringify!($size), "-bit integral type with explicit alignment requirements")]
            #[doc = ""]
            #[doc = "Without the explicit `align` representation hint, this type may have different"]
            #[doc = "alignment requirements on different machines. This helps to ensure that the type"]
            #[doc = "has a predictable layout in memory and that operations assuming a particular"]
            #[doc = "alignment value are sound."]
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[repr(C, align($size))]
            pub struct $name($inner);

            impl $name {
                #[doc = concat!("Creates a new [`", stringify!($name), "`] from a fixed size array of bytes.")]
                #[inline]
                pub const fn new(value: $inner) -> $name {
                    $name(<$inner>::from_le(value))
                }

                #[doc = concat!("Reads an aligned, endian-aware [`", stringify!($name), "`] from a slice of bytes.")]
                #[doc = ""]
                #[doc = "This method is endian-agnostic, using the host native endianness, or byte order, to"]
                #[doc = "perform any required conversions. In many cases, this results in a noop, since the"]
                #[doc = "bytes already exist in the appropriate byte order serialization type."]
                #[doc = ""]
                #[doc = "# Errors"]
                #[doc = ""]
                #[doc = "This function will return an error if the byte slice cannot be converted into an array"]
                #[doc = "of the appropriate size."]
                #[inline]
                pub fn read_bytes(bytes: &[u8]) -> crate::Result<$name> {
                    let array = <[u8; $size]>::try_from(&bytes[..$size])?;
                    Ok(Self::from_le_bytes(array))
                }

                #[doc = "Create a native endian integer value from its representation as a byte array in little endian."]
                #[inline]
                pub const fn from_le_bytes(bytes: [u8; $size]) -> Self {
                    Self(<$inner>::from_le_bytes(bytes))
                }

                #[doc = "Create a native endian integer value from its representation as a byte array in big endian."]
                #[inline]
                pub const fn from_be_bytes(bytes: [u8; $size]) -> Self {
                    Self(<$inner>::from_be_bytes(bytes))
                }

                #[doc = "Return the memory representation of this integer as a byte array in little-endian byte order."]
                #[inline]
                pub const fn to_le_bytes(self) -> [u8; $size] {
                    self.0.to_le_bytes()
                }
                #[doc = "Return the memory representation of this integer as a byte array in little-endian byte order."]
                #[inline]
                pub const fn to_be_bytes(self) -> [u8; $size] {
                    self.0.to_be_bytes()
                }
                #[doc = concat!("Gets the aligned, [`", stringify!($inner), "`] value from this container in native-endian byte order.")]
                #[inline(always)]
                pub const fn get(self) -> $inner {
                    self.0.to_le()
                }
                #[doc = concat!("Gets the inner [`", stringify!($inner), "`] value from this container in little-endian byte order.")]
                #[inline(always)]
                pub const fn get_le(self) -> $inner {
                    self.0.to_le()
                }
                #[doc = concat!("Gets the inner [`", stringify!($inner), "`] value from this container in big-endian byte order.")]
                #[inline(always)]
                pub const fn get_be(self) -> $inner {
                    self.0.to_be()
                }
            }

            impl From<$inner> for $name {
                fn from(value: $inner) -> $name {
                    $name(value)
                }
            }

            impl $crate::util::FromInner<$inner> for $name {
                fn from_inner(inner: $inner) -> $name {
                    $name(inner)
                }
            }
            impl $crate::util::IntoInner<$inner> for $name {
                fn into_inner(self) -> $inner {
                    self.get()
                }
            }

            impl ::core::fmt::Display for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Display::fmt(&self.get(), f)
                }
            }
            impl ::core::fmt::LowerHex for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::LowerHex::fmt(&self.get(), f)
                }
            }
            impl ::core::fmt::UpperHex for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::UpperHex::fmt(&self.get(), f)
                }
            }
            impl ::core::fmt::Binary for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Binary::fmt(&self.get(), f)
                }
            }
            impl ::core::fmt::Octal for $name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Octal::fmt(&self.get(), f)
                }
            }
        )*
    };
}

// FIXME: Add ability to get `Integral` value at runtime.

/// Marker trait for endian-aware integral types.
///
/// # Safety
///
/// This trait must only be implemented for integral types, or wrapper types
/// containing integrals. Implementing this trait for types that fail to meet these
/// requirements results in immediate **undefined behaviour**.
pub unsafe trait Integral: Abi + Zeroable {
    type Repr: Integral;

    fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    fn as_bytes(&self) -> &[u8] {
        unsafe {
            let data = self.as_ptr().cast::<u8>();
            let len = core::mem::size_of::<Self>();
            slice::from_raw_parts(data, len)
        }
    }

    fn value(&self) -> Self::Repr {
        unsafe { self.as_ptr().cast::<Self::Repr>().read() }
    }
}

macro_rules! impl_integral_for_primitives {
    ($($ty:ty: $inner:ty),* $(,)?) => {
        $(
            unsafe impl Integral for $ty {
                type Repr = $inner;
            }
        )*
    };
}

impl_integral_for_primitives! {
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    u128: u128,
    usize: usize,

    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    i128: i128,
    isize: isize,

    U8: u8,
    U16: u16,
    U32: u32,
    U64: u64,
    U128: u128,
    USize: usize,

    I8: i8,
    I16: i16,
    I32: i32,
    I64: i64,
    I128: i128,
    ISize: isize,
}
