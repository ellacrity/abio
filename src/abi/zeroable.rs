use core::mem::ManuallyDrop;

use crate::integral::{I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};

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
    // unsigned integral primitives
    u8, u16, u32, u64, u128, usize,
    // signed integral primitives
    i8, i16, i32, i64, i128, isize,
    // floating point numbers
    f32, f64,
    // unsigned endian-aware integrals
    U8, U16, U32, U64, U128,
    // signed endian-aware integrals
    I8, I16, I32, I64, I128,

}

/*
Implementation targets borrowed from `bytemuck`:
https://github.com/Lokathor/bytemuck/blob/main/src/zeroable.rs
*/
unsafe impl<T: Zeroable> Zeroable for core::num::Wrapping<T> {}
unsafe impl<T: Zeroable> Zeroable for core::cmp::Reverse<T> {}

unsafe impl<T> Zeroable for *mut T {}
unsafe impl<T> Zeroable for *const T {}
unsafe impl<T> Zeroable for *mut [T] {}
unsafe impl<T> Zeroable for *const [T] {}
unsafe impl Zeroable for *mut str {}
unsafe impl Zeroable for *const str {}

unsafe impl<T: ?Sized> Zeroable for core::marker::PhantomData<T> {}
unsafe impl Zeroable for core::marker::PhantomPinned {}
unsafe impl<T: Zeroable> Zeroable for ManuallyDrop<T> {}
unsafe impl<T: Zeroable> Zeroable for core::cell::UnsafeCell<T> {}
unsafe impl<T: Zeroable> Zeroable for core::cell::Cell<T> {}

unsafe impl<T> Zeroable for core::mem::MaybeUninit<T> {}

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

unsafe impl<T, const N: usize> Zeroable for [T; N] where T: Zeroable {}
