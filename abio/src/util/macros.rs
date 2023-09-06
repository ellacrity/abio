use core::mem;

use crate::util;

#[doc(hidden)]
macro_rules! const_assert_size {
    ($bytes:ident, $size:literal) => {{
        {
            assert_eq!($bytes.len(), $size);
            assert_eq!(::core::mem::size_of_val($bytes), $size);
        };
    }};
}

pub(crate) const fn array_from_slice<const LEN: usize, const REVERSE: bool>(
    bytes: &[u8],
) -> [u8; LEN] {
    util::copy_to_array::<LEN, REVERSE>(bytes)
}

/// Reverses the order of the bytes, changing little endian to big endian or big
/// endian to little endian.
///
/// FIXME: This should not be part of the public API.
#[macro_export]
#[doc(hidden)]
macro_rules! reverse_byte_order {
    (@from_ne: $bytes:ident, $size:literal) => {{
        // Simply read it as-is, since it is native endian
        unsafe { ::core::mem::transmute::<_, [u8; $size]>($bytes) }
    }};
    (@from_be: $bytes:ident, $size:literal) => {
        #[cfg(target_endian = "big")]
        {
            // got same bytes and array length
            ::core::mem::transmute::<_, [u8; $size]>($bytes)
        }
        #[cfg(not(target_endian = "big"))]
        {
            $crate::util::read_be_bytes($bytes)
        }
    };
    (@from_le: $bytes: ident, $size:literal) => {
        const_assert_size!($bytes, $size);
        #[cfg(target_endian = "little")]
        {
            $crate::util::read_ne_bytes($bytes)
        }
        #[cfg(not(target_endian = "little"))]
        {}
    };
    () => {};
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const fn generate_byte_array<const IS_NE: bool>() -> [u8; 8] {
        if !IS_NE {
            *b"\x00\x01\x02\x03\x04\x05\x06\x07"
        } else {
            *b"\x07\x06\x05\x04\x03\x02\x01\x00"
        }
    }

    #[test]
    fn reversing_byte_order_ctfe() {
        let array = &generate_byte_array::<true>()[..];
        reverse_byte_order!(@from_be: array, 8);
        let expected = *b"\x07\x06\x05\x04\x03\x02\x01\x00";
    }
}
