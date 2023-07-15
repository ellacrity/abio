//! Common errors that may occur within the [`abio`][abio] crate.
//!
//! [abio]: https://docs.rs/abio/latest/abio/

use core::array::TryFromSliceError;
use core::fmt;

/// Core error type for the [`abio`][abio] crate.
///
/// This type is essentially a wrapper around an enum, which contains the most common
/// failure states.
#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Creates a new [`Error`] instance from an inner [`ErrorKind`].
    pub(crate) const fn new(repr: ErrorKind) -> Error {
        Error { kind: repr }
    }

    /// The operation caused an attempted memory access that is out of bounds.
    ///
    /// The `requested` number of bytes exceeds the `actual` number of available
    /// bytes. This error frequently appears within operations that require bounds
    /// checking.
    pub const fn out_of_bounds(requested: usize, actual: usize) -> Error {
        Error::new(ErrorKind::out_of_bounds(requested, actual))
    }

    /// The operation performed on these bytes failed due to the pointer being
    /// misaligned.
    pub fn misaligned_access(bytes: &[u8]) -> Error {
        Error::new(ErrorKind::misaligned_access(bytes))
    }

    /// The types have sizes that differ in a way that violates a particular
    /// invariant.
    ///
    /// Most operations do not require that `required == actual`. However, all "safe
    /// transmute" operations assume that `required >= actual`.
    pub const fn size_mismatch(required: usize, actual: usize) -> Error {
        Error::new(ErrorKind::size_mismatch(required, actual))
    }

    /// Returns a detailed error with a message.
    ///
    /// This function is particularly useful during debugging, or whenever you wish
    /// to emit an explicit error message.
    pub const fn verbose(message: &'static str) -> Error {
        Error::new(ErrorKind::verbose(message))
    }

    /// The program entered an unknown or unexpected failure state that cannot be
    /// recovered from.
    pub const fn internal_failure() -> Error {
        Error::new(ErrorKind::internal_failure())
    }

    /// An operation was performed that resulted in an error due to the types being
    /// incompatible. The most likely cause of this error is that the two types have
    /// incompatible layouts, such as different size or alignment requirements.
    pub const fn incompatible_types() -> Error {
        Error::new(ErrorKind::incompatible_types())
    }

    pub fn null_reference<T: ?Sized>(ptr: *const T) -> Error {
        Error::new(ErrorKind::null_reference(ptr))
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl From<TryFromSliceError> for Error {
    fn from(_: TryFromSliceError) -> Error {
        Error::incompatible_types()
    }
}

impl From<&'static str> for Error {
    fn from(message: &'static str) -> Error {
        Error::verbose(message)
    }
}

/// Error variant, or kind, used to more precisely represent the failure condition.
#[derive(Debug, Default, PartialEq)]
pub(crate) enum ErrorKind {
    /// Error originating from an operation that caused the buffer to overflow.
    OutOfBounds {
        /// Number of bytes needed for the operation.
        required: usize,
        /// Upper bound of the buffer.
        actual: usize,
    },
    /// Error caused by an invalid operation attempting to access memory without
    /// first aligning the pointer accessing the underlying data.
    MisalignedAccess {
        /// Address where the memory access occurred.
        address: usize,
    },
    /// Error caused by a failed conversion attempt due to the types having
    /// incompatible layouts.
    IncompatibleTypes,
    /// Error caused by an invalid pointer that dereferences to null.
    NullReference { address: usize },
    /// Error occurring when the source and destination types occupy, or represent,
    /// memory regions with different sizes.
    SizeMismatch {
        /// Required number of bytes needed to decode the target type.
        required: usize,
        /// Actual number of available bytes in the buffer.
        actual: usize,
    },
    /// Error with a detailed message meant for debugging purposes.
    Verbose {
        /// Detailed error message explaining the failure.
        message: &'static str,
    },
    /// Error with an unknown or unexpected origin.
    #[default]
    InternalFailure,
}

impl ErrorKind {
    const fn out_of_bounds(required: usize, actual: usize) -> Self {
        ErrorKind::OutOfBounds { required, actual }
    }

    fn misaligned_access(bytes: &[u8]) -> Self {
        ErrorKind::MisalignedAccess { address: bytes.as_ptr().addr() }
    }

    const fn size_mismatch(required: usize, actual: usize) -> Self {
        ErrorKind::SizeMismatch { required, actual }
    }

    const fn verbose(message: &'static str) -> Self {
        ErrorKind::Verbose { message }
    }

    const fn internal_failure() -> Self {
        ErrorKind::InternalFailure
    }

    const fn incompatible_types() -> ErrorKind {
        ErrorKind::IncompatibleTypes
    }

    fn null_reference<T: ?Sized>(ptr: *const T) -> ErrorKind {
        ErrorKind::NullReference { address: ptr.addr() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::OutOfBounds { required, actual } => {
                write!(f, "Out of bounds error. Required {required} bytes, but buffer size can only hold {actual} bytes.")
            }
            ErrorKind::IncompatibleTypes => {
                write!(f, "Failed to convert one type to another due to incompatible layouts.")
            }
            ErrorKind::InternalFailure => write!(
                f,
                "Entered unrecoverable failure state with an unknown or unexpected origin."
            ),
            ErrorKind::MisalignedAccess { address } => {
                write!(f, "Misaligned memory access at address: {address} ({address:p}).")
            }
            ErrorKind::NullReference { address } => {
                write!(
                    f,
                    "Invalid pointer dereferenced to null at address: {address} ({address:p})",
                )
            }
            ErrorKind::SizeMismatch { required, actual } => {
                write!(f, "Size mismatch error. Required {required} bytes, got {actual}.")
            }
            ErrorKind::Verbose { message } => write!(f, "[ERROR]: {message}"),
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
