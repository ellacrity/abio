//! Primitives that may legally be represented with the all-zeroes bit pattern.
//!
//! Note: This [implementation][bytemuck-zeroable] is heavily derived and/or inspired
//! by the excellent [`bytemuck`][bytemuck] crate.
//!
//! [bytemuck]: https://docs.rs/bytemuck/latest/bytemuck/
//! [bytemuck-zeroable]: https://github.com/Lokathor/bytemuck/blob/main/src/zeroable.rs

use crate::integral::aligned::*;

/// Types that can be represented by the all-zero byte-pattern.
///
///
/// This means that, for example, the padding byte in `(u8, u16)` is not
/// necessarily zeroed.
///
/// # Safety
///
/// There is no guarantee that an all-zero byte-pattern represents a valid value
/// of some type `T`. For example, the all-zero byte-pattern is not a valid value
/// for reference types (`&T`, `&mut T`) and functions pointers. Using `zeroed`
/// on such types causes immediate [undefined behavior][ub] because [the Rust
/// compiler assumes][inv] that there always is a valid value in a variable it
/// considers initialized.
///
/// This has the same effect as [`MaybeUninit::zeroed().assume_init()`][zeroed].
/// It is useful for FFI sometimes, but should generally be avoided.
///
/// [zeroed]: MaybeUninit::zeroed
/// [ub]: ../../reference/behavior-considered-undefined.html
/// [inv]: MaybeUninit#initialization-invariant
pub unsafe trait Zeroable: Sized {
    /// Returns the value of type `T` represented by the all-zero byte-pattern.
    ///
    /// # Safety
    ///
    /// The caller must ensure the type `T` is valid when represented as a zeroed-out
    /// buffer.
    ///
    /// Types that violate this contract include:
    /// * Reference types, such as `&T` and `&mut T`
    /// * Function pointers
    #[inline]
    unsafe fn zero() -> Self {
        core::mem::zeroed::<Self>()
    }
}

macro_rules! impl_zeroable_trait {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl Zeroable for $ty {}
        )*
    };
}

impl_zeroable_trait! {
    (),
    bool,
    char,
    *const str,
    *mut str,
    core::marker::PhantomPinned,
    // unsigned integral primitives
    u8, u16, u32, u64, u128, usize,
    // signed integral primitives
    i8, i16, i32, i64, i128, isize,
    // unsigned endian-aware integrals
    U8, U16, U32, U64, U128, USize,
    // signed endian-aware integrals
    I8, I16, I32, I64, I128, ISize,
    // floating point numbers
    f32, f64,
}

macro_rules! impl_zeroable_trait_for_generic {
    // Rule for types with wrappers around some generic `T`
    ($($wrapper:tt: $ty:ty),* $(,)?) => {
        $(
            unsafe impl<$wrapper: Zeroable> Zeroable for $ty {}
        )*
    };
    // Rule for non-wrapping types
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl<T: Zeroable> Zeroable for $ty {}
        )*
    };
}

impl_zeroable_trait_for_generic! {
    *const T,
    *const [T],
    *mut T,
    *mut [T],
}
impl_zeroable_trait_for_generic! {
    T: core::num::Wrapping<T>,
    T: core::cmp::Reverse<T>,
    T: core::mem::MaybeUninit<T>,
    T: core::mem::ManuallyDrop<T>,
    T: core::cell::UnsafeCell<T>,
    T: core::cell::Cell<T>,
}

unsafe impl<T: ?Sized> Zeroable for core::marker::PhantomData<T> {}

//==============================================================================
// Blanket implementations for Tuple types
//==============================================================================
unsafe impl<A: Zeroable> Zeroable for (A,) {}
unsafe impl<A: Zeroable, B: Zeroable> Zeroable for (A, B) {}
unsafe impl<A: Zeroable, B: Zeroable, C: Zeroable> Zeroable for (A, B, C) {}
unsafe impl<A: Zeroable, B: Zeroable, C: Zeroable, D: Zeroable> Zeroable for (A, B, C, D) {}
unsafe impl<A: Zeroable, B: Zeroable, C: Zeroable, D: Zeroable, E: Zeroable> Zeroable
    for (A, B, C, D, E)
{
}
unsafe impl<A: Zeroable, B: Zeroable, C: Zeroable, D: Zeroable, E: Zeroable, F: Zeroable> Zeroable
    for (A, B, C, D, E, F)
{
}
unsafe impl<A: Zeroable, B: Zeroable, C: Zeroable, D: Zeroable, E: Zeroable, F: Zeroable, G: Zeroable>
    Zeroable for (A, B, C, D, E, F, G)
{
}
unsafe impl<
        A: Zeroable,
        B: Zeroable,
        C: Zeroable,
        D: Zeroable,
        E: Zeroable,
        F: Zeroable,
        G: Zeroable,
        H: Zeroable,
    > Zeroable for (A, B, C, D, E, F, G, H)
{
}
unsafe impl<
        A: Zeroable,
        B: Zeroable,
        C: Zeroable,
        D: Zeroable,
        E: Zeroable,
        F: Zeroable,
        G: Zeroable,
        H: Zeroable,
        I: Zeroable,
    > Zeroable for (A, B, C, D, E, F, G, H, I)
{
}

unsafe impl<T, const N: usize> Zeroable for [T; N] where T: Zeroable {}
