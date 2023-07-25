//! WIP: Documentation needs help

use core::mem::{align_of_val, size_of_val};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::{integer::*, BytesOf, Decode};
use crate::{Chunk, Zeroable};

/// A trait that a type must implement to be considered compatible with the
/// [`ABI`][ABI].
///
/// # Restrictions
///
/// This trait is extremely restrictive, because it carries with it many guarantees.
/// It also forbits the construction of types that are not compatible with the ABI.
/// Blanket implementations are provided for common types to ensure that you do not
/// have to write your own boilerplate code to support built-in primitives, such as
/// [`Integer`] types.
///
/// Notable restrictions include:
/// * Your type must be `Sized + Copy`
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
pub unsafe trait Abi: BytesOf + Sized + 'static {
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
    fn alignment(&self) -> usize {
        debug_assert_eq!(align_of_val(self), Self::ALIGN);
        Self::ALIGN
    }

    /// Returns the size of the pointed-to value in bytes at runtime.
    ///
    /// This is usually the same as [`size_of::<T>()`]. However, when `T` *has* no
    /// statically-known size, e.g., a slice [`[T]`][slice] or a [trait object],
    /// then `size_of_val` can be used to get the dynamically-known size.
    ///
    /// [trait object]: ../../book/ch17-02-trait-objects.html
    #[inline]
    fn size(&self) -> usize {
        debug_assert_eq!(size_of_val(self), Self::SIZE);
        Self::SIZE
    }

    /// Attempts to interpret the bits comprising this type as the target type `T`,
    /// where `T: Abi`.
    #[inline]
    fn try_cast_bytes<T: Decode>(&self) -> Result<T, crate::Error> {
        if self.size() != T::SIZE {
            Err(crate::Error::size_mismatch(T::SIZE, self.size()))
        } else if !self.is_access_aligned::<T>() {
            Err(crate::Error::misaligned_access(self.bytes_of()))
        }
    }
    /*
    /// Reinterpets the bytes comprising this type as another type `T` where `T:
    /// Abi`.
    ///
    /// # Safety
    ///
    /// Your type must fulfill the following criteria:
    /// * Cannot be a ZST
    /// * Has known layout requirements at compile time, such as size and minimum
    ///   alignment
    #[inline]
    unsafe fn to_abi_type<T: Abi>(&self) -> Option<&T> {
        assert!(!Self::IS_ZST, "cannot construct ZST from `Self` where `Self: Abi`");
        assert!(!T::IS_ZST, "cannot construct ZST from `T` where `T: Abi`");
        assert!(self.size() != 0, "cannot construct `Abi` type where `self.size() == 0`");
        if self.size() == 0 || Self::IS_ZST {
            None
        } else {
            Some(<*const _>::from(&self).cast::<T>().read())
        }
    } */
}

// #[repr(transparent)]
// pub struct AbiType<S, A> {
//     source: S,
//     marker: PhantomData<A>,
// }

// impl<S: Source, A: Abi> Deref for AbiType<S, A> {
//     type Target = A;

//     fn deref(&self) -> &Self::Target {
//         self.source.read_chunk_at(0)
//     }
// }

// const-generics are supported for all array types `[T; N] where T: Abi`.
unsafe impl<T, const N: usize> Abi for [T; N] where T: Abi + Integer + Zeroable {}

macro_rules! impl_abi_for_primitives {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl Abi for $ty {}
        )*
    };
}

impl_abi_for_primitives!((), bool, char);
impl_abi_for_primitives!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl_abi_for_primitives!(U8, U16, U32, U64, U128, Usize, I8, I16, I32, I64, I128, Isize);
impl_abi_for_primitives!(NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize);
impl_abi_for_primitives!(NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize);

// FIXME: Test that this is actually sound, because it may not be within the bounds
// of this crate. (ellacrity)
unsafe impl<T: Abi> Abi for *const T {}
unsafe impl<T: Abi> Abi for *mut T {}

unsafe impl<const N: usize> Abi for Chunk<N> {}
