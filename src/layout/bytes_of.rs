//! Traits and primitives for representing types as raw byte slices.

use core::mem::{self};
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use core::slice;

use crate::integer::{Isize, Usize, I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};
use crate::{Abi, Bytes, Chunk};

/// Trait to define types that can be represented as a borrowed slice of bytes, or
/// `&[u8]`.
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
/// Implementors of this trait must ensure that their type fulfills the contract
/// defined by the trait. You are strongly encouraged to derive this trait for your
/// type if possible.
///
/// // TODO: Safety docs; explain why it is unsafe to implement this trait
pub unsafe trait BytesOf {
    /// Gets the raw bytes comprising the region of memory this type occupies.
    fn bytes_of(&self) -> &[u8] {
        // We have a reference already, so we know `self` is a valid reference. We can create
        // a pointer to a reference. The size of the slice is determined via the
        // `mem::size_of_val` function to get the number of bytes comprising `Self`.
        unsafe {
            let data = <*const Self>::from(self).cast::<u8>();
            slice::from_raw_parts(data, mem::size_of_val(self))
        }
    }
}

unsafe impl<const N: usize> BytesOf for Chunk<N> {}
unsafe impl BytesOf for Bytes<'_> {}

unsafe impl<T> BytesOf for [T] where T: Abi {}
unsafe impl<'a, T> BytesOf for &'a [T] where T: Abi {}

unsafe impl<T, const N: usize> BytesOf for [T; N] where T: Abi {}

macro_rules! impl_bytes_of {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl BytesOf for $ty {}
        )*
    };
}

impl_bytes_of! {
    i8, i16, i32, i64, i128, isize, I8, I16, I32, I64, I128, Isize,
    u8, u16, u32, u64, u128, usize, U8, U16, U32, U64, U128, Usize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
}

/// Trait to define types that can be represented as a mutable borrowed slice of
/// bytes, or `&mut [u8]`.
///
/// # Safety
///
/// Implementors of this trait must ensure that their type fulfills the contract
/// defined by the trait. You are strongly encouraged to derive this trait for your
/// type if possible.
///
/// // TODO: Safety docs; explain why it is unsafe to implement this trait
pub unsafe trait BytesOfMut {
    /// .
    fn bytes_of_mut(&mut self) -> &mut [u8] {
        // We have a reference already, so we know `self` is a valid reference. We can create
        // a pointer to a reference. The size of the slice is determined via the
        // `mem::size_of_val` function to get the number of bytes comprising `Self`.
        unsafe {
            let bptr = <*mut Self>::from(self).cast::<u8>();
            slice::from_raw_parts_mut(bptr, mem::size_of_val(self))
        }
    }
}

unsafe impl<T: ?Sized> BytesOfMut for *mut T {}
unsafe impl<'a, T: Abi> BytesOfMut for &'a mut [T] {}
