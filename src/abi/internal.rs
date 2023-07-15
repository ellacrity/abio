//! WIP: Documentation needs help

use core::mem::{align_of_val, size_of_val};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::integral::aligned::*;

/// Core marker trait that all eligible types must implement to ensure their
/// compatibility with the operating system ABI. The default for every platform is
/// its "native" C ABI.
///
/// # Derive
///
/// You are strongly encouraged to use the procedural derive macro to implement this
/// trait for your type. The macro will parse your type's syntax tree and verify its
/// layout, such as the size and alignment of its fields, and the existence or lack
/// of padding bytes. Padding bytes are currently not allowed.
///
/// # Soundness
///
/// This trait declares that the implementor is 100% safe to construct and is
/// compatible with the ABI existing in its environment. allows for certain
/// superpowers, such as zero-copy serialization and deserialization. If the byte
/// order serialization is not known until runtime, the caller can use the
/// [`endian`][endian] module to read and write from arbitrary bytes using
/// endian-specific instructions.
///
/// to read and write from bytes in an
/// endian-agnostic way. eto  within the bounds of this crate, is to guarantee that
/// the type is completely safe to construct within the current environment.
///
/// You should annotate your type with `#[derive(Abi)]`, which will ensure that
/// your type is compatible with the methods contained in the [`abio`] crate.
///
/// # Safety
///
/// This trait must only be implemented for types with known alignment values. For
/// complex types, such as structs, each field must implement `Abi`. Failing to
/// uphold these invariants is **undefined behaviour**.
///
/// [ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
pub unsafe trait Abi: Sized + Copy + 'static {
    /// Returns the [ABI]-required minimum alignment of a type in bytes.
    ///
    /// Every reference to a value of the type `T` must be a multiple of this number.
    ///
    /// This is the alignment used for struct fields. It may be smaller than the
    /// preferred alignment.
    ///
    /// [ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
    const ALIGN: usize = core::mem::align_of::<Self>();

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

    const IS_ZST: bool = Self::SIZE == 0;
    /// Returns the [ABI]-required minimum alignment of the type of the value that
    /// `val` points to in bytes.
    ///
    /// Every reference to a value of the type `T` must be a multiple of this number.
    ///
    /// [ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
    #[inline]
    fn min_align(&self) -> usize {
        align_of_val(self)
    }

    #[inline]
    fn runtime_size(&self) -> usize {
        size_of_val(self)
    }
}

// const-generics are supported for all array types `[T; N] where T: Abi`.
unsafe impl<T, const N: usize> Abi for [T; N] where T: Abi {}

macro_rules! impl_abi_for_primitives {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl Abi for $ty {}
        )*
    };
}

impl_abi_for_primitives!((), bool, char);
impl_abi_for_primitives!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_abi_for_primitives!(U8, U16, U32, U64, U128, USize, I8, I16, I32, I64, I128, ISize);
impl_abi_for_primitives!(NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize);
impl_abi_for_primitives!(NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize);

unsafe impl<T: Abi> Abi for *const T {}
unsafe impl<T: Abi> Abi for *mut T {}
