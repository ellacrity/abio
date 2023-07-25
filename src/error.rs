//! Common errors that may occur within the [`abio`][abio] crate.
//!
//! [abio]: https://docs.rs/abio/latest/abio/
#![allow(dead_code)]

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
    pub(crate) const fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl From<core::array::TryFromSliceError> for Error {
    fn from(_: core::array::TryFromSliceError) -> Error {
        Error::incompatible_types()
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(_: core::array::TryFromSliceError) -> Error {
        Error::incompatible_types()
    }
}

impl From<core::convert::Infallible> for Error {
    fn from(_: core::convert::Infallible) -> Error {
        Error::internal_failure()
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
    /// Error caused by a decoding routine failure.
    DecodeFailure {
        /// Message explaining why the decoding routine failed.
        message: &'static str,
    },
    /// Error caused by an encoding routine failure.
    EncodeFailure {
        /// Message explaining why the encoding routine failed.
        message: &'static str,
    },
    /// Error caused by a failed conversion attempt due to the types having
    /// incompatible layouts.
    IncompatibleTypes,
    /// Error caused by an invalid operation attempting to access memory without
    /// first aligning the pointer accessing the underlying data.
    MisalignedAccess {
        /// Address where the memory access occurred.
        address: usize,
    },
    /// Error caused by an invalid pointer that dereferences to null.
    NullReference { address: usize },
    /// Error originating from an operation that caused an attempted memory access
    /// outside the bounds of a slice or array.
    OutOfBounds {
        /// Number of bytes needed for the operation.
        minimum: usize,
        /// Actual number of available bytes.
        available: usize,
    },
    /// Error occurring when the the sizes of two types, or regions of memory, do not
    /// have the same exact size.
    ///
    /// For source and destination types represent different sizes.
    SizeMismatch {
        /// Expected, or required, number of bytes needed to fit an instance of the
        /// target type.
        expected: usize,
        /// Actual number of available bytes in the buffer.
        actual: usize,
    },
    /// Error with an unknown or unexpected origin.
    #[default]
    InternalFailure,
    /// Error with a detailed message meant for debugging purposes.
    Verbose {
        /// Detailed error message explaining the failure.
        message: &'static str,
    },
}

impl Error {
    pub(crate) const fn decode_failed() -> Error {
        Error::new(ErrorKind::DecodeFailure { message: "Decoder failed; cannot write malformed bytes due to size or alignment requirements." })
    }

    pub(crate) const fn encode_failed() -> Error {
        Error::new(ErrorKind::EncodeFailure { message: "Encoder failed; cannot write malformed bytes due to size or alignment requirements." })
    }

    pub(crate) const fn out_of_bounds(minimum: usize, available: usize) -> Error {
        Error::new(ErrorKind::OutOfBounds { minimum, available })
    }

    pub(crate) fn misaligned_access(bytes: &[u8]) -> Error {
        Error::new(ErrorKind::MisalignedAccess { address: bytes.as_ptr().addr() })
    }

    pub(crate) const fn size_mismatch(expected: usize, actual: usize) -> Error {
        Error::new(ErrorKind::SizeMismatch { expected, actual })
    }

    pub(crate) const fn internal_failure() -> Error {
        Error::new(ErrorKind::InternalFailure)
    }

    pub(crate) const fn incompatible_types() -> Error {
        Error::new(ErrorKind::IncompatibleTypes)
    }

    pub(crate) fn null_reference<T: ?Sized>(ptr: *const T) -> Error {
        Error::new(ErrorKind::NullReference { address: ptr.addr() })
    }

    pub(crate) const fn verbose(message: &'static str) -> Error {
        Error::new(ErrorKind::Verbose { message })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::OutOfBounds { minimum: required, available: actual } => {
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
            ErrorKind::SizeMismatch { expected: required, actual } => {
                write!(f, "Size mismatch error. Required {required} bytes, got {actual}.")
            }
            ErrorKind::Verbose { message } => write!(f, "[ERROR]: {message}"),
            ErrorKind::DecodeFailure { message } => write!(f, "Decode failed: {message}"),
            ErrorKind::EncodeFailure { message } => write!(f, "Encode failed: {message}"),
        }
    }
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
