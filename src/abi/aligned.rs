use core::ptr;

use crate::integral::{I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};

/// Core marker trait that all deserializable and serializable types must implement.
///
/// # Implementors
///
/// You should annotate your type with `#[derive(Aligned)]`, which will ensure that
/// your type is compatible with the methods contained in the [`aligned`] crate.
///
/// # Safety
///
/// This trait must only be implemented for types with known alignment values. For
/// complex types, such as structs, each field must implement `Aligned`. Failing to
/// uphold these invariants is **undefined behaviour**.
pub unsafe trait Aligned: Copy + Sized + 'static {
    /// Returns the [ABI]-required minimum alignment of a type in bytes.
    ///
    /// Every reference to a value of the type `T` must be a multiple of this number.
    ///
    /// This is the alignment used for struct fields. It may be smaller than the
    /// preferred alignment.
    ///
    /// [ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
    const MIN_ALIGN: usize = core::mem::align_of::<Self>();

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

    #[inline]
    fn min_align(&self) -> usize {
        core::mem::align_of_val(self)
    }

    /// Returns true iff this type is properly aligned with respect to where it
    /// exists in memory.
    #[inline]
    fn validate_alignment(&self) -> bool {
        let ptr = <*const Self>::from(self).cast::<u8>();
        let addr2 = ptr::addr_of!(self).cast::<u8>();
        let addr1 = ptr.addr() & (Self::MIN_ALIGN - 1);
        let addr2 = addr2.addr() & (Self::MIN_ALIGN - 1);
        assert_eq!(addr1, addr2, "Offset from ptr should be 0, but is {}", addr1.abs_diff(addr2));
        ptr.addr() & (Self::MIN_ALIGN - 1) == 0
    }
}

/// Marker trait for struct and union types that do not contain any padding.
///
/// # Safety
///
/// Implementors of this trait must ensure their type meets the following criteria:
/// * The type itself as well as its fields must all implement `Aligned`
/// * There can be no padding bytes between any of the type's fields, nor at the
///   beginning or end
pub trait NoPadding: Aligned {
    #[doc(hidden)]
    fn verify_padding(&self) -> bool {
        let size = Self::SIZE;
        let alignment = Self::MIN_ALIGN;
        let expected_padding = alignment - (size % alignment);
        expected_padding == alignment || expected_padding == 0
    }
}

impl<T> NoPadding for T where T: Aligned {}

unsafe impl<T: Aligned, const N: usize> Aligned for [T; N] {}

macro_rules! impl_aligned_for_primitives {
    ($($ty:ty),* $(,)?) => {
        $(
            unsafe impl Aligned for $ty {}
        )*
    };
}

impl_aligned_for_primitives!(u8, u16, u32, u64, u128, U8, U16, U32, U64, U128);
impl_aligned_for_primitives!(i8, i16, i32, i64, i128, I8, I16, I32, I64, I128);
impl_aligned_for_primitives!(usize, isize);
impl_aligned_for_primitives!((), bool, char);
