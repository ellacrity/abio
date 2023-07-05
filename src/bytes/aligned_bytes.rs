use crate::Pod;

/// Extension methods for byte slices.
pub trait AlignedBytes: AsRef<[u8]> {
    /// Computes the offset that needs to be applied to the pointer in order to make
    /// it aligned to `T`.
    fn with_align_of<T: Pod>(&self) -> usize;

    /// Returns whether the pointer is properly aligned for `T`.
    fn is_ptr_aligned<T: Pod>(&self) -> bool;
}

impl AlignedBytes for [u8] {
    fn with_align_of<T: Pod>(&self) -> usize {
        self.as_ptr().addr() & (T::ALIGN - 1)
    }

    fn is_ptr_aligned<T: Pod>(&self) -> bool {
        self.with_align_of::<T>() == 0
    }
}

impl<'a> AlignedBytes for &'a [u8] {
    fn with_align_of<T: Pod>(&self) -> usize {
        self.as_ptr().addr() & (T::ALIGN - 1)
    }

    fn is_ptr_aligned<T: Pod>(&self) -> bool {
        self.with_align_of::<T>() == 0
    }
}
