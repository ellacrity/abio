//! Common errors that may occur within the `aligned` crate.

use core::alloc::Layout;
use core::fmt;

/// Generic error associated with a mismatch in alignment.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayoutError {
    message: &'static str,
}

impl LayoutError {
    pub fn new(message: &'static str) -> Self {
        LayoutError { message }
    }
}

impl From<MisalignedAccessError> for LayoutError {
    fn from(value: MisalignedAccessError) -> Self {
        #[cfg(feature = "debugging")]
        log::error!("{value}");
        LayoutError { message: value.as_str() }
    }
}

/// Buffer overflow error.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BufferOverflowError;

impl From<SizeMismatchError> for BufferOverflowError {
    fn from(value: SizeMismatchError) -> Self {
        #[cfg(feature = "debugging")]
        log::error!("{value}");
        BufferOverflowError
    }
}

/// Error caused by an attempt to read data from a buffer that should contain bytes,
/// but due to some underyling OS issue, is empty or contains insufficient allocated
/// bytes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BufferUnderflowError;

/// An attempt was made to access memory without first aligning the pointer accessing
/// the data.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MisalignedAccessError;

impl MisalignedAccessError {
    fn as_str(&self) -> &'static str {
        "Error caused by an attempt to access memory with a misaligned pointer"
    }
}

/// Error occurring when the source and destination types occupy, or represent,
/// differently-sized memory regions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SizeMismatchError {
    needed: usize,
    actual: usize,
}

impl SizeMismatchError {
    pub const fn new<T>(actual: usize) -> SizeMismatchError {
        SizeMismatchError { needed: core::mem::size_of::<T>(), actual }
    }

    pub const fn difference(&self) -> usize {
        self.needed.abs_diff(self.actual)
    }
}

impl From<(usize, usize)> for SizeMismatchError {
    fn from(from: (usize, usize)) -> SizeMismatchError {
        let (needed, actual) = from;
        SizeMismatchError { needed, actual }
    }
}

impl fmt::Display for SizeMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SizeMismatchError: cannot transmute safely between two independently-sized types."
        )
    }
}
