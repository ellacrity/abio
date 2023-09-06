//! Traits and primitives for representing types as raw byte slices.

use core::mem::{self};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};


use crate::integer::{Isize, Usize, I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};
use crate::{Abi, Bytes, Chunk, Zeroable};

/// Trait to define types that can be represented as raw bytes.
///
/// # Endianness
/// The returned bytes are not endian-aware. Any methods belonging to this trait
/// return the bytes in the order they are currently represented in.
///
/// # Source Trait
///
/// Any type that may potentially serve as an input [`Source`] must implement this
/// trait.
///
/// # Derive
///
/// To ensure your type is compatible with [`abio`][crate], is it recommended
/// that you use the derivezeroed macro. The derive macro will verify, at compile
/// time, that your type meets the requirements of this trait and that any safety
/// invarants are upheld.
///
/// # Safety
///
/// Implementors of this trait must ensure that their type cannot be constructed from
/// a mutable reference. The contract enforced by the trait requires that it is not
/// implemented for types that are capable of obtaining a mutable reference to the
/// underlying data. Doing so is potentially **undefined behaviour** and likely
/// unsound at best.
///
/// ## Deriving
///
/// You are strongly encouraged to derive this trait for your type if possible.
///
/// // TODO: Safety docs; explain why it is unsafe to implement this trait
/// # Safety
///
/// For any type `T` that implements `AsBytes`, unsafe code can presume that it's
/// valid to treat any instance of `T` as an immutable `[u8]` slice with a length of
/// `size_of::<T>()`. Implementing `AsBytes` for a type that doesn't adhere to this
/// contract could result in undefined behavior.
///
/// To safely implement the [`AsBytes`] trait, the following invariants must be
/// upheld:
///
/// - For structs:
///   1. The type must possess a specific representation, either `repr(C)`,
///      `repr(transparent)`, or `repr(packed)`.
///   2. All fields within the struct must implement `AsBytes`.
///   3. The layout of the struct should not contain any padding:
///      - This is always satisfied by `repr(transparent)` and `repr(packed)`.
///      - For `repr(C)`, consult the layout algorithm detailed in the [Rust
///        Reference].
///
/// * Union types are unsupported. They may be supported in the future.
/// * Enums are currently unsupported. Future plans include adding support for enums.
///
/// [Rust Reference]: https://doc.rust-lang.org/reference/type-layout.html
pub unsafe trait AsBytes {
    /// Get a reference to the slice of bytes representing `self`.
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        let data = <*const _>::from(self).cast::<u8>();
        let len = mem::size_of_val(self);
        // We have a reference already, so we know `self` is a valid reference. We can create
        // a pointer to a reference. The size of the slice is determined via the
        // `mem::size_of_val` function to get the number of bytes comprising `Self`.
        unsafe { core::slice::from_raw_parts(data, len) }
    }

    /// Get a mutable reference to the slice of bytes representing `self`.
    #[inline]
    fn bytes_of_mut(&mut self) -> &mut [u8]
    where
        Self: Abi,
    {
        let data = <*mut Self>::from(self).cast::<u8>();
        let len = mem::size_of_val(self);
        // We have a mutable reference already, so we know `self` represents a valid, aligned
        // type. We first create a pointer to the first byte of the type, using a special
        // syntax that desugars to `self as *mut Self as *mut u8`.
        //
        // pairing it with
        // the size of this type instance using the `mem::size_of_val` function. to
        // get the number of bytes comprising `Self`.
        unsafe { core::slice::from_raw_parts_mut(data, len) }
    }
}

unsafe impl<'data> AsBytes for &'data [u8] {}
unsafe impl<'data> AsBytes for Bytes<'data> {}

unsafe impl<const N: usize> AsBytes for Chunk<N> {}
unsafe impl<'data, const N: usize> AsBytes for &'data Chunk<N> {}

unsafe impl<T> AsBytes for [T] where T: Abi + Zeroable {}
unsafe impl<'data, T, const N: usize> AsBytes for &'data [T; N] where T: Abi + Zeroable {}
unsafe impl<T, const N: usize> AsBytes for [T; N] where T: Abi + Zeroable {}

macro_rules! impl_bytes_of {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl AsBytes for $ty {}
        )*
    };
}

impl_bytes_of! {
    (), bool, char, f32, f64,
    i8, i16, i32, i64, i128, isize, I8, I16, I32, I64, I128, Isize,
    u8, u16, u32, u64, u128, usize, U8, U16, U32, U64, U128, Usize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
}

