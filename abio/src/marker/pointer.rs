use crate::{sealed, Abi};

/// The `Alignment` trait provides methods for checking and obtaining alignment
/// information of raw pointers.
///
/// # Safety
///
/// This trait is `unsafe` to implement. It relies on the correct implementation
/// of the provided methods to ensure safe behavior when dealing with raw pointers
/// and memory alignment.
///
/// # Note
///
/// This trait is sealed and cannot be implemented outside of its containing module.
/// This is to prevent unsafe external implementations.
pub unsafe trait Alignment: sealed::Sealed {
    /// Returns the alignment offset of the pointer when aligned to the type `T`.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type to which we want to check the alignment against. It must
    ///   implement the `Abi` trait.
    fn align_to<T: Abi>(self) -> usize;

    /// Returns `true` if the the pointer represented by `self` meets the alignment
    /// requirements of `T`.
    fn is_aligned_with<T: Abi>(self) -> bool;
}

unsafe impl<A: Abi> Alignment for *const A {
    #[inline(always)]
    fn align_to<T: Abi>(self) -> usize {
        assert_eq!(A::MIN_ALIGN, T::MIN_ALIGN);
        self.addr() & (T::MIN_ALIGN - 1)
    }

    #[inline(always)]
    fn is_aligned_with<T: Abi>(self) -> bool {
        assert_eq!(A::MIN_ALIGN, T::MIN_ALIGN);
        self.align_to::<T>() == 0
    }
}

unsafe impl<T: Abi> Alignment for *mut T {
    #[inline(always)]
    fn align_to<A: Abi>(self) -> usize {
        self.addr() & (A::MIN_ALIGN - 1)
    }

    #[inline(always)]
    fn is_aligned_with<A: Abi>(self) -> bool {
        self.align_to::<A>() == 0
    }
}
