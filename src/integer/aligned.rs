//! Aligned, endian-aware integer types that can be deserialized from the input
//! source.

use cfg_if::cfg_if;

use crate::{BigEndian, Endianness, LittleEndian};

impl_aligned_integer! {
    "A signed, 8-bit", I8, i8, 1,
    "A signed, 16-bit", I16, i16, 2,
    "A signed, 32-bit", I32, i32, 4,
    "A signed, 64-bit", I64, i64, 8,
    "A signed, 128-bit", I128, i128, 16,
    "An unsigned, 8-bit", U8, u8, 1,
    "An unsigned, 16-bit", U16, u16, 2,
    "An unsigned, 32-bit", U32, u32, 4,
    "An unsigned, 64-bit", U64, u64, 8,
    "An unsigned, 128-bit", U128, u128, 16,
}

impl_fmt_traits! {
    I8, I16, I32, I64, I128, Isize, U8, U16, U32, U64, U128, Usize
}

// This `cfg_if` block attempts to take into account the variable size of `isize` and
// `usize` types on different platforms. Support for this feature is not 100% fleshed
// out and finished, but this should be a decent starting point.
//
// Other options include obtaining this information with a `build.rs` script.
// (ellacrity)
cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        impl_aligned_integer! {
            "A signed, pointer-sized", Isize, isize, 4,
            "An unsigned, platform-dependent", Usize, usize, 4,
        }
    } else if #[cfg(target_pointer_width = "64")] {
        impl_aligned_integer! {
            "A signed, pointer-sized", Isize, isize, 8,
            "An unsigned, platform-dependent", Usize, usize, 8,
        }
    } else {
        compile_error!("\nThis platform is not currently supported.\nSupported platforms include those with either 32-bit or 64-bit pointer widths.\n");
    }
}

macro_rules! impl_decode_aligned {
    ($($input:ty: $decoder:ident),* $(,)?) => {
        $(
            impl $crate::layout::Decode for $input {

                #[inline]
                fn decode(bytes: &[u8], codec: $crate::Codec) -> $crate::Result<Self> {
                    if bytes.len() < ::core::mem::size_of::<Self>() {
                        Err(crate::Error::out_of_bounds(::core::mem::size_of::<Self>(), bytes.len()))
                    } else if bytes.len() > codec.limit().get() as usize {
                        Err(crate::Error::decode_failed())
                    } else {
                        match codec.endian() {
                            crate::Endian::Big => <BigEndian>::$decoder(bytes),
                            crate::Endian::Little => <LittleEndian>::$decoder(bytes),
                        }
                    }
                }
            }
        )*
    };
}

impl_decode_aligned! {
    u8: read_u8,
    u16: read_u16,
    u32: read_u32,
    u64: read_u64,
    u128: read_u128,
    i8: read_i8,
    i16: read_i16,
    i32: read_i32,
    i64: read_i64,
    i128: read_i128,
}

macro_rules! read_aligned_integer {
    ($($ty:ty),* $(,)?) => {
        $(
            impl $ty {
                #[doc = concat!("Reads an aligned [`", stringify!($ty), "`] type from a slice of")]
                #[doc = "bytes using the specified endianness."]
                #[doc = ""]
                #[doc = "# Errors"]
                #[doc = ""]
                #[doc = "# Errors"]
                #[inline]
                pub const fn read_aligned(bytes: &[u8], endian: $crate::Endian) -> $crate::Result<$ty> {
                    match $crate::Chunk::from_bytes(bytes) {
                        Ok(chunk) => match endian {
                            $crate::Endian::Big => Ok(Self::from_be_chunk(chunk)),
                            $crate::Endian::Little => Ok(Self::from_le_chunk(chunk)),
                        }
                        Err(err) => Err(err),
                    }
                }
            }
        )*
    };
}

read_aligned_integer! {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
}
