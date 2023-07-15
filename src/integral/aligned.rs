//! Aligned, endian-aware integral types that can be deserialized from the input
//! source.
impl_aligned_integral! {
    "A signed", I8: i8 -> 1,
    "A signed", I16: i16 -> 2,
    "A signed", I32: i32 -> 4,
    "A signed", I64: i64 -> 8,
    "A signed", I128: i128 -> 16,
    "A signed", ISize: isize -> 8,
    "An unsigned", U8: u8 -> 1,
    "An unsigned", U16: u16 -> 2,
    "An unsigned", U32: u32 -> 4,
    "An unsigned", U64: u64 -> 8,
    "An unsigned", U128: u128 -> 16,
    "An unsigned", USize: usize -> 8
}
