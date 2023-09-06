/// Generates the implementations for the `aligned` integer primitives.
#[doc(hidden)]
#[macro_export]
macro_rules! gen_aligned_integer {
    ( $($prefix:literal, $sign:literal, $Type:ident, $inner:ty, $size:literal),* $(,)?) => {
        $(
            #[doc = concat!($prefix, " ", $sign, " integer type with explicit alignment requirements.")]
            #[doc = ""]
            #[doc = concat!("This type is a wrapper for the built-in primitive [`", stringify!($inner), "`] type.")]
            #[doc = ""]
            #[doc = "# Memory Layout"]
            #[doc = ""]
            #[doc = concat!("The [`", stringify!($Type), "`] type enforces a memory alignment of, ", stringify!($size), " bytes")]
            #[doc = ""]
            #[doc = "# Padding"]
            #[doc = ""]
            #[doc = "This type may introduce padding bytes to align the type correctly in memory, depending on the surrounding"]
            #[doc = "layout and specific alignment requirements. The `U8`/`u8` primitives do not impose any alignment requirements"]
            #[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[repr(transparent)]
            pub struct $Type($inner);
            // // pub struct $Type<E: $crate::Context>($inner, ::core::marker::PhantomData<C>);

            impl $Type {
                #[doc = concat!("Create a new endian-aware [`", stringify!($Type), "`] instance from a value.")]
                #[doc = ""]
                #[doc = "# Context"]
                #[doc = ""]
                #[doc = "The [`Context`] trait is used as a generic type parameter, allowing the interpretation of"]
                #[doc = "integer to be determined by the end user. This constructor requires an explicit byte order"]
                #[doc = "serialization type to ensure a consistent and predictable API."]
                #[inline]
                pub const fn new<E: $crate::Endianness>(value: $inner) -> $Type {
                    match E::ENDIAN {
                        $crate::Endian::Little => $Type::from_le(value.to_le()),
                        $crate::Endian::Big => $Type::from_be(value.to_be()),
                    }
                }

                #[doc = concat!("Encodes an aligned, endian-aware [`", stringify!($Type), "`] type from a slice of")]
                #[doc = "bytes, starting at `offset`."]
                #[doc = ""]
                #[doc = "# Context"]
                #[doc = ""]
                #[doc = "The [`Context`] trait is used as a generic type parameter, allowing the interpretation of"]
                #[doc = "integer to be determined by the end user. This constructor requires an explicit byte order"]
                #[doc = "serialization type to ensure a consistent and predictable API."]
                #[doc = ""]
                #[doc = "# Errors"]
                #[doc = ""]
                #[doc = "This method returns an error if `bytes.len() < offset + size_of::<Self>()`"]
                #[inline]
                pub const fn read_aligned<E: $crate::Endianness>(bytes: &[u8], offset: usize) -> $crate::Result<$Type> {
                    let Ok(chunk) = $crate::Chunk::from_bytes_at::<E>(bytes, offset) else {
                        return Err($crate::Error::read_failed(
                            "Chunk could not be created from bytes with given offset",
                        ));
                    };

                    let endian = E::ENDIAN;
                    if endian.is_little_endian() {
                        // handle data normally
                        Self::_read_le_chunk::<$Type>(chunk)
                    } else {
                        // reverse order of bytes, with MSB being the LSB.
                        Self::_parse_be_chunk::<$Type>(chunk)
                    }

                    match E::ENDIAN {
                        $crate::Endian::Big => Ok(Self::from_be_chunk(chunk)),
                        $crate::Endian::Little => Ok(Self::from_le_chunk(chunk)),
                    }
                }


                #[doc = concat!("Create a native-endian [`", stringify!($Type), "`] from a big endian integer.")]
                #[doc = ""]
                #[doc = "# Potential Optimization"]
                #[doc = ""]
                #[doc = "For machines where the native endianness is 'big', this is a no-op. On little endian targets, the byte order must be swapped."]
                #[inline]
                pub const fn from_be(value: $inner) -> Self {
                    #[cfg(target_endian = "big")]
                    {
                        Self(value)
                    }
                    #[cfg(not(target_endian = "big"))]
                    {
                        // swap byte order, then read the bytes into a buffer
                        Self(<$inner>::from_be(value))
                    }
                }

                #[doc = concat!("Create a native-endian [`", stringify!($Type), "`] from a little endian integer.")]
                #[doc = "as little endian byte order"]
                #[doc = "# Potential Optimization"]
                #[doc = ""]
                #[doc = "For machines where the native endianness is 'big', this is a no-op. On little endian targets, the byte order must be swapped."]
                #[inline]
                pub const fn from_le(value: $inner) -> Self {
                    Self(<$inner>::from_le(value))
                }

                #[doc = concat!("Create a native-endian [`", stringify!($Type), "`], .")]
                #[inline]
                pub const fn from_ne(value: $inner) -> Self {
                    #[cfg(target_endian = "big")]
                    {
                        Self(<$inner>::from_be(value))
                    }
                    #[cfg(target_endian = "little")]
                    {
                        Self(<$inner>::from_le(value))
                    }
                }

                #[doc = concat!("Encode a new [`", stringify!($Type), "`] in little endian byte order")]
                #[doc = concat!("from a [`Chunk`] containing ", stringify!($size), " bytes.")]
                #[inline]
                pub const fn from_chunk<E: $crate::Endianness>(chunk: $crate::Chunk<$size>) -> Self {
                    match E::ENDIAN {
                        $crate::Endian::Big => Self::from_be_chunk(chunk.to_be()),
                        $crate::Endian::Little => Self::from_le_chunk(chunk.to_le())

                    }
                }

                #[doc = concat!("Encode a new [`", stringify!($Type), "`] in little endian byte order")]
                #[doc = concat!("from a [`Chunk`] containing ", stringify!($size), " bytes.")]
                #[inline]
                pub const fn from_le_chunk(chunk: $crate::Chunk<$size>) -> Self {
                    Self::from_le_bytes(chunk.to_le_bytes())
                }

                #[doc = concat!("Encode a new [`", stringify!($Type), "`] in big endian byte order")]
                #[doc = concat!("from a [`Chunk`] containing ", stringify!($size), " bytes.")]
                #[inline]
                pub const fn from_be_chunk(chunk: $crate::Chunk<$size>) -> Self {
                    Self::from_be_bytes(chunk.to_be_bytes())
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
                pub const fn get<E: $crate::Endianness>(self) -> $inner {
                    match E::ENDIAN {
                        $crate::Endian::Big => <$inner>::from_be(self.0.to_be()),
                        $crate::Endian::Little => <$inner>::from_le(self.0.to_le())
                    }
                }

                #[doc = concat!("Get the [`", stringify!($inner), "`] aligned integer in native-endian byte order.")]
                #[doc = ""]
                #[doc = "This method uses the target endian value, calculated at compile time, to determine"]
                #[doc = "which byte order serialization variant to use."]
                #[inline(always)]
                pub const fn get_ne(self) -> $inner {
                    #[cfg(target_endian = "big")]
                    {
                        self.0.to_be()
                    }
                    #[cfg(target_endian = "little")]
                    {
                        self.0.to_le()
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

            impl Copy for $Type {}
            impl Clone for $Type {
                fn clone(&self) -> Self {
                    *self
                }
            }
            impl Default for $Type {
                fn default() -> Self {
                    Self(0)
                }
            }

            impl ::core::ops::Deref for $Type {
                type Target = $inner;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl ::core::convert::From<$inner> for $Type {
                fn from(value: $inner) -> $Type {
                    $Type(value)
                }
            }

            impl ::core::convert::From<$Type> for $inner {
                fn from(value: $Type) -> $inner {
                    value.get_ne()
                }
            }

            impl const $crate::util::FromInner<$inner> for $Type {
                fn from_inner(inner: $inner) -> $Type {
                    Self::from_ne(inner)
                }
            }
            impl const $crate::util::IntoInner<$inner> for $Type {
                fn into_inner(self) -> $inner {
                    self.get_ne()
                }
            }

            impl PartialEq<$inner> for $Type {
                fn eq(&self, other: &$inner) -> bool {
                    #[cfg(target_endian = "big")]
                    {
                        self.get::<$crate::NativeEndian>().eq(&other.to_be())
                    }
                    #[cfg(target_endian = "little")]
                    {
                        self.get::<$crate::NativeEndian>().eq(&other.to_le())
                    }
                }
            }

            impl PartialEq<$Type> for $inner {
                fn eq(&self, other: &$Type) -> bool {
                    #[cfg(target_endian = "big")]
                    {
                        self.to_be().eq(&other.get::<$crate::BE>())
                    }
                    #[cfg(target_endian = "little")]
                    {
                        self.to_le().eq(&other.get::<$crate::LE>())
                    }
                }
            }
        )*
    };
}

macro_rules! impl_fmt_traits {
    ( $($Type:ty),* $(,)?) => {
        $(
            impl ::core::fmt::Display for $Type {
                #[allow(clippy::missing_inline_in_public_items)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Display::fmt(&self.get::<$crate::NativeEndian>(), f)
                }
            }
            impl ::core::fmt::LowerHex for $Type {
                #[allow(clippy::missing_inline_in_public_items)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::LowerHex::fmt(&self.get::<$crate::NativeEndian>(), f)
                }
            }
            impl ::core::fmt::UpperHex for $Type {
                #[allow(clippy::missing_inline_in_public_items)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::UpperHex::fmt(&self.get::<$crate::NativeEndian>(), f)
                }
            }
            impl ::core::fmt::Binary for $Type {
                #[allow(clippy::missing_inline_in_public_items)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Binary::fmt(&self.get::<$crate::NativeEndian>(), f)
                }
            }
            impl ::core::fmt::Octal for $Type {
                #[allow(clippy::missing_inline_in_public_items)]
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Octal::fmt(&self.get::<$crate::NativeEndian>(), f)
                }
            }
        )*
    }
}
