//! WIP: Documentation needs help

use core::mem::size_of_val;
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use core::ptr::NonNull;

use crate::integer::{Isize, Usize, I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};
use crate::{Chunk, Zeroable};

/// A trait that a type must implement to be considered compatible with the
/// [`ABI`][ABI] used by this crate.
///
/// # Restrictions
///
/// This trait is extremely restrictive, because it carries with it many guarantees.
/// It also forbits the construction of types that are not compatible with the ABI.
/// Blanket implementations are provided for common types to ensure that you do not
/// have to write your own boilerplate code to support built-in primitives.
///
/// Notable restrictions include:
/// * Your type must be `Sized`
/// * Your type may not contain immutable or mutable references types
/// * Types with a `'static` lifetime are supported and considered legal
///
/// # Derive
///
/// You are strongly encouraged to use the procedural derive macro to implement this
/// trait for your type. The macro inspects your code at compile time, ensuring that
/// each field is `Abi`. This crate attempts to use zero-cost abstractions whenever
/// possible, so that checks can be moved to compile time rather than runtime to
/// reduce the error-prone process of manually implementing `unsafe` traits.
///
/// Deriving the [`Abi`] trait for your type ensures that your type:
/// * is fully compatible with the ABI defined by this crate
/// * is fundamentally sound at the type level, and it may be intepreted directly
///   from raw bytes
/// * has known size and alignment requirements, including its individual fields;
///   each field must implement ABI
/// * contains no padding bytes. Padding bytes are currently **not allowed**.
///
/// # Safety
///
/// This trait must only be implemented for types with known alignment values. For
/// complex types, such as structs, each field must implement `Abi`. Failing to
/// uphold these invariants is **undefined behaviour**.
///
/// [ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
pub unsafe trait Abi: Sized + 'static {
    /// Returns the size of a type in bytes.
    ///
    /// More specifically, this is the offset in bytes between successive elements
    /// in an array with that item type including alignment padding. Thus, for any
    /// type `T` and length `N`, `[T; N]` has a size of `N * size_of::<T>()`.
    ///
    /// In general, the size of a type is not stable across compilations, but
    /// specific types such as primitives are.
    ///
    /// Please refer to the documentation for the [`core::mem::size_of`] function for
    /// additional details.
    const SIZE: usize = core::mem::size_of::<Self>();

    /// Returns `true` if this type is a ZST, with a size of zero bytes.
    const IS_ZST: bool = Self::SIZE == 0;

    /// Returns the [ABI]-required minimum alignment of a type in bytes.
    ///
    /// Every reference to a value of the type `T` must be a multiple of this number.
    ///
    /// This is the alignment used for struct fields. It may be smaller than the
    /// preferred alignment.
    ///
    /// [ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
    const MIN_ALIGN: usize = core::mem::align_of::<Self>();

    /// Bitmask applied to the address of a pointer to verify that this type's
    /// alignment requirements are met.
    const ALIGN_MASK: usize = Self::MIN_ALIGN.saturating_sub(1);

    const MIN_VALUE: usize;

    const MAX_VALUE: usize;

    const MAYBE_ZERO: Option<NonNull<usize>>;

    /// Returns the [ABI]-required minimum alignment of the type of the value that
    /// `val` points to in bytes.
    ///
    /// Every reference to a value of the type `T` must be a multiple of this number.
    ///
    /// [ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
    #[inline]
    fn alignment(&self) -> usize {
        debug_assert_eq!(core::mem::align_of_val(self), Self::MIN_ALIGN);
        core::mem::align_of_val(self)
    }

    /// Returns the size of the pointed-to value in bytes at runtime.
    ///
    /// This is usually the same as `size_of::<T>()`. However, when `T` *has* no
    /// statically-known size, e.g., a slice [`[T]`][slice] or a [trait object],
    /// then `size_of_val` can be used to get the dynamically-known size.
    ///
    /// [slice]: core::slice
    /// [trait object]: ../../book/ch17-02-trait-objects.html
    #[inline]
    fn size(&self) -> usize {
        debug_assert_eq!(size_of_val(self), Self::SIZE);
        core::mem::size_of_val(self)
    }
}

macro_rules! impl_abi_for_primitives {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl $crate::Abi for $ty {}
        )*
    };
}

impl_abi_for_primitives!((), bool, char);
impl_abi_for_primitives!(f32, f64);
impl_abi_for_primitives!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_abi_for_primitives!(U8, U16, U32, U64, U128, Usize, I8, I16, I32, I64, I128, Isize);
impl_abi_for_primitives! {
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU128,
    NonZeroUsize,
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroI128,
    NonZeroIsize
}

// const-generics support for array types where `T` implements `Abi`.
unsafe impl<T, const N: usize> Abi for [T; N] where T: Abi + Zeroable {}
unsafe impl<const N: usize> Abi for Chunk<N> {}
