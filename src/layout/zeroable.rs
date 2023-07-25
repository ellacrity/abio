//! Primitives that may legally be represented with the all-zeroes bit pattern.
//!
//! Note: This [implementation][bytemuck-zeroable] is heavily derived and/or inspired
//! by the excellent [`bytemuck`][bytemuck] crate.
//!
//! [bytemuck]: https://docs.rs/bytemuck/latest/bytemuck/
//! [bytemuck-zeroable]: https://github.com/Lokathor/bytemuck/blob/main/src/zeroable.rs

use crate::integer::*;
use crate::Chunk;

/// Trait defining Types that can exist represented by the all-zero byte-pattern.
///
/// # Safety
///
/// There is no guarantee that an all-zero byte-pattern represents a valid value
/// of some type `T`. For example, the all-zero byte-pattern is not a valid value
/// for reference types (`&T`, `&mut T`) and functions pointers. Using `zeroed`
/// on such types causes immediate [undefined behaviour][ub] because [the Rust
/// compiler assumes][invariants] that there always is a valid value in a variable it
/// considers initialized.
///
/// This trait is mainly useful for low-level FFI and memory manipulation, but it
/// should generally be avoided in regular code. It should be used with caution,
/// and the caller must ensure that the usage of this method is safe for the
/// specific type being used.
///
/// [zeroed]: MaybeUninit::zeroed
/// [ub]: ../../reference/behavior-considered-undefined.html
/// [invariants]: MaybeUninit#initialization-invariant
pub unsafe trait Zeroable: Sized + 'static {
    /// Initializes this type as a contiguous region of memory represented by the
    /// all-zero byte pattern.
    ///
    /// # Safety
    ///
    /// The caller must ensure that it does not violate any of the following
    /// invariants:
    /// * `Self` cannot be a reference type, such as `&T` and `&mut T`
    /// * `Self` may not represent a function pointer
    ///
    /// Incorrect usage of this method may lead to **undefined behaviour** and memory
    /// safety issues.
    #[inline]
    unsafe fn zero_memory() -> Self {
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
    // unsigned integer primitives
    u8, u16, u32, u64, u128, usize,
    // signed integer primitives
    i8, i16, i32, i64, i128, isize,
    // unsigned endian-aware integers
    U8, U16, U32, U64, U128, Usize,
    // signed endian-aware integers
    I8, I16, I32, I64, I128, Isize,
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

unsafe impl<T: ?Sized + 'static> Zeroable for core::marker::PhantomData<T> {}

//==============================================================================
// Blanket implementations for Tuple types with a stopgap of 3 elements
//==============================================================================
unsafe impl<A: Zeroable> Zeroable for (A,) {}
unsafe impl<A: Zeroable, B: Zeroable> Zeroable for (A, B) {}
unsafe impl<A: Zeroable, B: Zeroable, C: Zeroable> Zeroable for (A, B, C) {}

// Constant generic arrays
unsafe impl<T, const N: usize> Zeroable for [T; N] where T: Zeroable {}
unsafe impl<const N: usize> Zeroable for Chunk<N> {}
