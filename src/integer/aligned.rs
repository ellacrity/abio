//! Aligned, endian-aware integer types that can be deserialized from the input
//! source.

// FIXME: Fix `ISize` and `USize` to account for platform-specific size / alignment.
// (ellacrity)

impl_aligned_integer! {
    "A signed, 8-bit", I8, i8, 1,
    "A signed, 16-bit", I16, i16, 2,
    "A signed, 32-bit", I32, i32, 4,
    "A signed, 64-bit", I64, i64, 8,
    "A signed, 128-bit", I128, i128, 16,
    "A signed, pointer-sized", Isize, isize, 8,
    "An unsigned, 8-bit", U8, u8, 1,
    "An unsigned, 16-bit", U16, u16, 2,
    "An unsigned, 32-bit", U32, u32, 4,
    "An unsigned, 64-bit", U64, u64, 8,
    "An unsigned, 128-bit", U128, u128, 16,
    "An unsigned, platform-dependent", Usize, usize, 8,
}

// macro_rules! impl_decode_aligned {
//     ($($ty:ty: $decoder:ident),* $(,)?) => {
//         $(
//             impl Decode<$ty> for $ty {
//                 fn decode<E: $crate::endian::Endian>(bytes: &[u8], endian: E) ->
// $crate::Result<Self> {                     if E::is_big_endian(&endian) {
//                         LittleEndian::$decoder(bytes)
//                     } else {
//                         BigEndian::$decoder(bytes)
//                     }
//                 }
//             }
//         )*
//     };
// }

// impl_decode_aligned! {
//     U8: read_u8,
//     U16: read_u16,
//     U32: read_u32,
//     U64: read_u64,
//     U128: read_u128,
//     I8: read_i8,
//     I16: read_i16,
//     I32: read_i32,
//     I64: read_i64,
//     I128: read_i128,
// }
