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
    #[inline]
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl From<core::array::TryFromSliceError> for Error {
    #[inline]
    fn from(_: core::array::TryFromSliceError) -> Error {
        Error::incompatible_types()
    }
}

impl From<core::str::Utf8Error> for Error {
    #[inline]
    fn from(_: core::str::Utf8Error) -> Error {
        Error::incompatible_types()
    }
}

impl From<core::convert::Infallible> for Error {
    #[inline]
    fn from(_: core::convert::Infallible) -> Error {
        Error::internal_failure()
    }
}

impl From<&'static str> for Error {
    #[inline]
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
    /// Error caused by a misconfigured or invalid codec. This typically happens if
    /// the codec has not been fully configured or if it contains invalid parameters
    /// that violate an invariant enforced by the codec.
    InvalidCodec {
        /// Reason the codec initialization failed.
        reason: &'static str,
    },
    /// Error caused by an invalid operation attempting to access memory without
    /// first aligning the pointer accessing the underlying data.
    MisalignedAccess,
    /// Error caused by an invalid pointer that dereferences to null.
    NullReference,
    /// Error originating from an operation that caused an attempted memory access
    /// outside the bounds of a slice or array.
    OutOfBounds {
        /// Number of bytes needed for the operation.
        needed: usize,
        /// Actual number of available bytes.
        available: usize,
    },
    /// Error occurring when the the sizes of two types, or regions of memory, do not
    /// have the same exact size.
    ///
    /// For source and destination types represent different sizes.
    SizeMismatch {
        /// Minimum number of bytes needed to construct an instance of the target
        /// type.
        needed: usize,
        /// Actual number of available bytes in the buffer.
        available: usize,
    },
    /// Error with an unknown or unexpected origin.
    ///
    /// This error is typically a sign that something very, very wrong has happened
    /// within [`abio`][crate], and not something that you have done wrong.
    ///
    /// If you happen to encounter this error, please [open an issue][issue]
    /// describing what happened.
    ///
    /// [issue]:
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
        Error::new(ErrorKind::DecodeFailure {
            message:
                "Decoder failed; cannot write malformed bytes due to size or alignment requirements",
        })
    }

    pub(crate) const fn encode_failed() -> Error {
        Error::new(ErrorKind::EncodeFailure {
            message:
                "Encoder failed; cannot write malformed bytes due to size or alignment requirements",
        })
    }

    pub(crate) const fn out_of_bounds(needed: usize, available: usize) -> Error {
        Error::new(ErrorKind::OutOfBounds { needed, available })
    }

    pub(crate) const fn misaligned_access() -> Error {
        Error::new(ErrorKind::MisalignedAccess)
    }

    pub(crate) const fn size_mismatch(needed: usize, available: usize) -> Error {
        Error::new(ErrorKind::SizeMismatch { needed, available })
    }

    pub(crate) const fn internal_failure() -> Error {
        Error::new(ErrorKind::InternalFailure)
    }

    pub(crate) const fn incompatible_types() -> Error {
        Error::new(ErrorKind::IncompatibleTypes)
    }

    pub(crate) const fn null_reference() -> Error {
        Error::new(ErrorKind::NullReference)
    }

    pub(crate) const fn verbose(message: &'static str) -> Error {
        Error::new(ErrorKind::Verbose { message })
    }

    pub(crate) const fn invalid_codec(reason: &'static str) -> Error {
        Error::new(ErrorKind::InvalidCodec { reason })
    }
}

impl fmt::Display for Error {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::OutOfBounds { needed: required, available: actual } => {
                write!(f, "Out of bounds error. Required {required} bytes, but buffer size can only hold {actual} bytes")
            }
            ErrorKind::IncompatibleTypes => {
                write!(f, "Failed to convert one type to another due to incompatible layouts")
            }
            ErrorKind::InternalFailure => write!(
                f,
                "Entered unrecoverable failure state with an unknown or unexpected origin"
            ),
            ErrorKind::MisalignedAccess => {
                write!(f, "Misaligned memory access caused by misaligned pointer")
            }
            ErrorKind::NullReference => {
                write!(f, "Invalid pointer dereferenced to null",)
            }
            ErrorKind::SizeMismatch { needed: expected, available: actual } => {
                write!(f, "Size mismatch error (Required {expected} bytes, got {actual}")
            }
            ErrorKind::Verbose { message } => write!(f, "[ERROR]: {message}"),
            ErrorKind::DecodeFailure { message } => write!(f, "Decode failed: {message}"),
            ErrorKind::EncodeFailure { message } => write!(f, "Encode failed: {message}"),
            ErrorKind::InvalidCodec { reason } => {
                write!(f, "Invalid codec configuration: {reason}")
            }
        }
    }
}

/// Type alias for conveniently constructing `Result` types using this crate's
/// [`Error`] type.
pub type Result<T, E = Error> = core::result::Result<T, E>;
