//! Aligned, endian-aware integer types that can be decoded from the input
//! source.

#[macro_use]
mod macros;

use core::mem;

use crate::{util, Abi, Alignment, Endianness, Error};

gen_aligned_integer! {
    "An 8-bit",  "signed", I8, i8, 1,
    "A 16-bit",  "signed", I16, i16, 2,
    "A 32-bit",  "signed", I32, i32, 4,
    "A 64-bit",  "signed", I64, i64, 8,
    "A 128-bit", "signed", I128, i128, 16,
    "An 8-bit", "unsigned", U8, u8, 1,
    "A 16-bit", "unsigned", U16, u16, 2,
    "A 32-bit", "unsigned", U32, u32, 4,
    "A 64-bit", "unsigned", U64, u64, 8,
    "A 128-bit", "unsigned", U128, u128, 16,
}
#[cfg(target_pointer_width = "32")]
gen_aligned_integer!("A pointer-sized", "signed", Isize, isize, 4);
#[cfg(target_pointer_width = "32")]
gen_aligned_integer!("A pointer-sized", "unsigned", Usize, usize, 4);
#[cfg(target_pointer_width = "64")]
gen_aligned_integer!("A pointer-sized", "signed", Isize, isize, 8);
#[cfg(target_pointer_width = "64")]
gen_aligned_integer!("A pointer-sized", "unsigned", Usize, usize, 8);

impl_fmt_traits! {
    I8, I16, I32, I64, I128, Isize, U8, U16, U32, U64, U128, Usize
}

macro_rules! impl_decode_aligned {
    ($($ty:ty, $size:literal),* $(,)?) => {
        $(
            impl<'de> $crate::codec::Decode<'de> for $ty {
                fn decode<E: $crate::Endianness>(bytes: &'de [u8], offset: usize) -> $crate::Result<(&'de $ty, usize)> {
                    let res = match E::ENDIAN {
                        $crate::Endian::Little => $crate::util::read_le_bytes(bytes),
                        $crate::Endian::Big => $crate::util::read_be_bytes(bytes),
                    };
                    let ptr = res.as_ptr();
                    if ptr.is_aligned_with::<Self>() {
                        Ok(&*(ptr as *const Self, Self::SIZE))
                    } else {
                        Err($crate::Error::misaligned_access(ptr))
                    }
                }
            }
        )*
    };
}

impl<'de> crate::codec::Decode<'de> for u32 {
    fn decode<E: Endianness>(bytes: &[u8]) -> crate::Result<(&'de Self, usize)> {
        let res = match E::ENDIAN {
            crate::Endian::Little => crate::util::read_le_bytes(bytes),
            crate::Endian::Big => crate::util::read_be_bytes(bytes),
        };
        let ptr = res.as_ptr();
        if ptr.is_aligned_with::<Self>() {
            Ok(&*(ptr as *const u32, u32::SIZE))
        } else {
            Err(crate::Error::misaligned_access(ptr))
        }
    }
}

impl_decode_aligned! {
    U8, 1,
    U16, 2,
    U32, 4,
    U64, 8,
    U128, 16,
    I8, 1,
    I16, 2,
    I32, 4,
    I64, 8,
    I128, 16,
}

impl<'de> crate::Decode<'de> for u8 {
    fn decode<E: Endianness>(bytes: &[u8]) -> crate::Result<(&'de Self, usize)> {
        let res = unsafe {
            bytes
                .as_ptr()
                .cast::<Self>()
                .as_ref()
        };
        let res = res.ok_or_else(|| Error::decoder_failed())?;
        Ok((res, mem::size_of_val(res)))
    }
}

impl<'de> crate::Decode<'de> for u16 {
    fn decode<E: crate::Endianness>(
        bytes: &'de [u8],
        offset: usize,
    ) -> crate::Result<(&'de Self, usize)> {
        // Try to read a chunk with given `$size`, starting at `offset`
        let Ok(chunk) = crate::Chunk::<2>::read_bytes_offset::<E>(bytes, offset) else {
            return Err(crate::Error::decoder_failed());
        };
        // Check if the returned chunk lies on a byte boundary with matching requirements of
        // `T`.
        let ptr = chunk.as_ptr();
        if !ptr.is_aligned_with::<u16>() {
            Err(crate::Error::misaligned_access::<Self>(chunk.as_ptr().addr()))
        } else {
            // Encode endianness stored in the `Context`, and decode chunk appropriately.
            let target = match E::ENDIAN {
                crate::Endian::Little => u16::from_le_bytes(chunk.to_le_bytes()),
                crate::Endian::Big => u16::from_be_bytes(chunk.to_be_bytes()),
            };
            Ok((&target, 2))
        }
    }
}
