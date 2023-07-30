use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::integer::{Isize, Usize, I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};
use crate::{Abi, BytesOf};

macro_rules! impl_fmt_traits {
    ( $($name:ty),* $(,)?) => {
        $(
            impl ::core::fmt::Display for $name {

                #[allow(clippy::missing_inline_in_public_items)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #[cfg(target_endian = "big")]
                    {
                        ::core::fmt::Display::fmt(&self.get_be(), f)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        ::core::fmt::Display::fmt(&self.get_le(), f)
                    }
                }
            }
            impl ::core::fmt::LowerHex for $name {
                #[allow(clippy::missing_inline_in_public_items)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {

                    #[cfg(target_endian = "big")]
                    {
                        ::core::fmt::LowerHex::fmt(&self.get_be(), f)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        ::core::fmt::LowerHex::fmt(&self.get_le(), f)
                    }
                }
            }
            impl ::core::fmt::UpperHex for $name {
                #[allow(clippy::missing_inline_in_public_items)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #[cfg(target_endian = "big")]
                    {
                        ::core::fmt::UpperHex::fmt(&self.get_be(), f)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        ::core::fmt::UpperHex::fmt(&self.get_le(), f)
                    }
                }
            }
            impl ::core::fmt::Binary for $name {
                #[allow(clippy::missing_inline_in_public_items)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #[cfg(target_endian = "big")]
                    {
                        ::core::fmt::Display::fmt(&self.get_be(), f)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        ::core::fmt::Display::fmt(&self.get_le(), f)
                    }
                }
            }
            impl ::core::fmt::Octal for $name {
                #[allow(clippy::missing_inline_in_public_items)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    #[cfg(target_endian = "big")]
                    {
                        ::core::fmt::Octal::fmt(&self.get_be(), f)
                    }
                    #[cfg(target_endian = "little")]
                    {
                        ::core::fmt::Octal::fmt(&self.get_le(), f)
                    }
                }
            }
        )*
    }
}

macro_rules! impl_aligned_integer {
    ( $($kind:literal, $name:ident, $inner:tt, $size:literal),* $(,)?) => {
        $(
            #[doc = concat!($kind, " ", "integer type with explicit alignment requirements")]
            #[doc = ""]
            #[doc = "Without the explicit `align` representation hint, this type may have different"]
            #[doc = "alignment requirements on different machines. This helps to ensure that the type"]
            #[doc = "has a predictable layout in memory and that operations assuming a particular"]
            #[doc = "alignment value are sound."]
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[repr(C, align($size))]
            pub struct $name($inner);

            impl $name {
                #[doc = concat!("Creates a new [`", stringify!($name), "`] from a integer value.")]
                #[doc = ""]
                #[doc = "# Endianness"]
                #[doc = ""]
                #[doc = "The newly-created instance uses the target machines native endianness by"]
                #[doc = "default. To create this type with an explicitly-specified endianness, use"]
                #[doc = "one of the specialized constructor methods."]
                #[inline]
                pub const fn new(value: $inner) -> Self {
                    cfg_if::cfg_if! {
                        if #[cfg(target_endian = "big")] {
                            Self::new_be(value)
                        } else if #[cfg(target_endian = "little")] {
                            Self::new_le(value)
                        } else {
                            compile_error!("[Error]: Invalid target_endian value.\nValid values: 'big', 'little'.");
                        }
                    }
                }

                #[doc = concat!("Creates a new [`", stringify!($name), "`] from a big endian integer value.")]
                #[inline]
                pub const fn new_be(value: $inner) -> Self {
                    Self(<$inner>::from_be(value))
                }

                #[doc = concat!("Creates a new [`", stringify!($name), "`] from an integer, interpreting it")]
                #[doc = "as little endian byte order"]
                #[inline]
                pub const fn new_le(value: $inner) -> Self {
                    Self(<$inner>::from_le(value))
                }

                #[doc = concat!("Reads a new [`", stringify!($name), "`] in little endian byte order")]
                #[doc = concat!("from a chunk of bytes with size ", $size)]
                #[inline]
                pub const fn from_le_chunk(chunk: $crate::source::Chunk<$size>) -> Self {
                    Self::from_le_bytes(chunk.into_array())
                }

                #[doc = concat!("Reads a new [`", stringify!($name), "`] in big endian byte order")]
                #[doc = concat!("from a chunk of bytes with size ", $size)]
                #[inline]
                pub const fn from_be_chunk(chunk: $crate::source::Chunk<$size>) -> Self {
                    Self::from_be_bytes(chunk.into_array())
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

                #[doc = concat!("Get the [`", stringify!($inner), "`] aligned integer in the the specified byte order.")]
                #[inline(always)]
                pub const fn get(self, endian: $crate::Endian) -> $inner {
                    match endian {
                        $crate::Endian::Big => <$inner>::from_be(self.0),
                        $crate::Endian::Little => <$inner>::from_le(self.0)
                    }
                }

                #[doc = concat!("Get the [`", stringify!($inner), "`] aligned integer in native-endian byte order.")]
                #[doc = ""]
                #[doc = "This method uses the target endian value, calculated at compile time, to determine"]
                #[doc = "which byte order serialization variant to use."]
                #[inline(always)]
                pub const fn get_ne(self) -> $inner {
                    cfg_if::cfg_if! {
                        if #[cfg(target_endian = "big")] {
                            self.0.to_be()
                        } else if #[cfg(target_endian = "little")] {
                            self.0.to_le()
                        } else {
                            compile_error!("[Error]: Invalid target_endian value.\nValid values: 'big', 'little'.");
                        }
                    }
                }

                #[doc = concat!("Get the [`", stringify!($inner), "`] aligned integer in little-endian byte order.")]
                #[inline(always)]
                pub const fn get_le(self) -> $inner {
                    self.0.to_le()
                }
                #[doc = concat!("Get the [`", stringify!($inner), "`] aligned integer in big-endian byte order.")]
                #[inline(always)]
                pub const fn get_be(self) -> $inner {
                    self.0.to_be()
                }
            }

            impl ::core::ops::Deref for $name {
                type Target = $inner;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl ::core::convert::From<$inner> for $name {
                fn from(value: $inner) -> $name {
                    $name(value)
                }
            }

            impl ::core::convert::From<$name> for $inner {
                fn from(value: $name) -> $inner {
                    value.get_le()
                }
            }

            impl const $crate::shims::FromInner<$inner> for $name {
                fn from_inner(inner: $inner) -> $name {
                    cfg_if::cfg_if! {
                        if #[cfg(target_endian = "big")] {
                            Self::new_be(inner)
                        } else if #[cfg(target_endian = "little")] {
                            Self::new_be(inner)
                        } else {
                            compile_error!("[Error]: Invalid target_endian value.\nValid values: 'big', 'little'.");
                        }
                    }
                }
            }
            impl const $crate::shims::IntoInner<$inner> for $name {
                fn into_inner(self) -> $inner {
                    cfg_if::cfg_if! {
                        if #[cfg(target_endian = "big")] {
                            self.0.to_be()
                        } else if #[cfg(target_endian = "little")] {
                            self.0.to_le()
                        } else {
                            compile_error!("[Error]: Invalid target_endian value.\nValid values: 'big', 'little'.");
                        }
                    }
                }
            }
        )*
    };
}

/// A trait defining integer types with explicit or implicit byte order
/// serialization (endianness) that can be converted to and from slices of bytes.
///
/// # Safety
///
/// This trait may only be implemented for integer types, such as Rust's built-in
/// integer primitives. Newtype wrappers for these primitives are provided by the
/// crate. You are strongly encouraged to use them when performing operations on
/// bytes where endianness matters.
///
/// Implementing this trait for non-integer types is immediate **undefined
/// behaviour**. You are strongly encouraged to use the types provided by this crate
/// or, whenever possible, deriving the unsafe traits for your types. The derive
/// macros provided by [`abio_derive`][abio-derive] validate the layout of
/// your custom type(s) at compile time, ensuring that they will work as intended at
/// runtime.
#[const_trait]
pub unsafe trait Integer: Abi + BytesOf {
    /// Inner representation of the [`Integer`] type.
    type Repr: Integer;

    /// Get this [`Integer`] type as a raw pointer.
    #[inline(always)]
    fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    /// Returns `true` if this [`Integer`] type is equal to zero.
    fn is_zero(&self) -> bool;

    /// Gets the value of this instance at runtime.
    #[inline(always)]
    fn value(&self) -> Self::Repr {
        unsafe { self.as_ptr().cast::<Self::Repr>().read() }
    }
}

macro_rules! impl_integer_for_primitives {
    ($($ty:ty: $inner:ty),* $(,)?) => {
        $(
            unsafe impl const Integer for $ty {
                type Repr = $inner;

                #[inline(always)]
                fn is_zero(&self) -> bool {
                    self.value() == 0
                }
            }
        )*
    };
}

impl_integer_for_primitives! {
    u8:     u8,
    u16:    u16,
    u32:    u32,
    u64:    u64,
    u128:   u128,
    usize:  usize,

    i8:     i8,
    i16:    i16,
    i32:    i32,
    i64:    i64,
    i128:   i128,
    isize:  isize,

    U8:     u8,
    U16:    u16,
    U32:    u32,
    U64:    u64,
    U128:   u128,
    Usize:  usize,

    I8:     i8,
    I16:    i16,
    I32:    i32,
    I64:    i64,
    I128:   i128,
    Isize:  isize,
}

macro_rules! impl_integer_for_non_zero_primitives {
    ($($ty:ty: $inner:ty),* $(,)?) => {
        $(
            unsafe impl Integer for $ty {
                type Repr = $inner;

                #[inline(always)]
                fn is_zero(&self) -> bool {
                    false
                }
            }
        )*
    };
}

impl_integer_for_non_zero_primitives! {
    NonZeroU8:      u8,
    NonZeroU16:     u16,
    NonZeroU32:     u32,
    NonZeroU64:     u64,
    NonZeroU128:    u128,
    NonZeroUsize:   usize,
    NonZeroI8:      i8,
    NonZeroI16:     i16,
    NonZeroI32:     i32,
    NonZeroI64:     i64,
    NonZeroI128:    i128,
    NonZeroIsize:   isize,
}
