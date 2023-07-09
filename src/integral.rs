// FIXME: Implement derive macros to implement aligned integral types (ella)
mod unsigned;
pub use unsigned::{U128, U16, U32, U64, U8};

// FIXME: Implement derive macros to implement aligned integral types (ella)
mod signed;
pub use signed::{I128, I16, I32, I64, I8};

use crate::{Aligned, Decodable};

impl Decodable for u8 {}
impl Decodable for u16 {}
impl Decodable for u32 {}
impl Decodable for u64 {}
impl Decodable for u128 {}

/// Marker trait for endian-aware integral types.
///
/// # Safety
///
/// This trait must only be implemented for integral types, or wrapper types
/// containing integrals. Implementing this trait for types that fail to meet these
/// requirements results in immediate **undefined behaviour**.
pub unsafe trait Integral: Aligned {}

macro_rules! impl_integral_for_aligned_primitives {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl Integral for $ty {}
        )*
    };
}

impl_integral_for_aligned_primitives! {
    u8, u16, u32, u64, u128, usize,
    U8, U16, U32, U64, U128
}
impl_integral_for_aligned_primitives! {
    i8, i16, i32, i64, i128, isize,
    I8, I16, I32, I64, I128,
}

// pub fn is_aligned_with<A: Aligned>(bytes: &[u8]) -> bool {
//     bytes.as_ptr().addr() & (A::MIN_ALIGN - 1) == 0
// }
