//! Common errors that may occur within the `aligned` crate.

use core::fmt;

/// Primary error type for this crate.
#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub const fn new(repr: ErrorKind) -> Error {
        Error { kind: repr }
    }

    pub const fn buffer_overflow(required: usize, actual: usize) -> Error {
        Error::new(ErrorKind::buffer_overflow(required, actual))
    }

    pub fn misaligned_access(bytes: &[u8]) -> Error {
        Error::new(ErrorKind::misaligned_access(bytes))
    }

    pub const fn size_mismatch(required: usize, actual: usize) -> Error {
        Error::new(ErrorKind::size_mismatch(required, actual))
    }

    pub const fn verbose(message: &'static str) -> Error {
        Error::new(ErrorKind::verbose(message))
    }

    pub const fn internal_failure() -> Error {
        Error::new(ErrorKind::internal_failure())
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

impl From<&'static str> for Error {
    fn from(message: &'static str) -> Error {
        Error::verbose(message)
    }
}

/// Errors that may occur from operations
#[derive(Debug, Default, PartialEq)]
pub enum ErrorKind {
    /// An operation resulted in a buffer overflow.
    BufferOverflow {
        /// Number of bytes exceeded with respect to the buffer size.
        amount: usize,
    },
    /// Error caused by an invalid operation attempting to access memory without
    /// first aligning the pointer accessing the underlying data.
    MisalignedAccess {
        /// Address where the memory access occurred.
        address: usize,
    },
    /// Error occurring when the source and destination types occupy, or represent,
    /// memoy regions with different sizes.
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
    pub(crate) const fn buffer_overflow(required: usize, actual: usize) -> Self {
        ErrorKind::BufferOverflow { amount: required.saturating_sub(actual) }
    }

    pub(crate) fn misaligned_access(bytes: &[u8]) -> Self {
        ErrorKind::MisalignedAccess { address: bytes.as_ptr().addr() }
    }

    pub(crate) const fn size_mismatch(required: usize, actual: usize) -> Self {
        ErrorKind::SizeMismatch { required, actual }
    }

    pub(crate) const fn verbose(message: &'static str) -> Self {
        ErrorKind::Verbose { message }
    }

    pub(crate) const fn internal_failure() -> Self {
        ErrorKind::InternalFailure
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::BufferOverflow { amount } => {
                write!(f, "Operation resulted in a buffer overflow by {amount} bytes.")
            }
            ErrorKind::MisalignedAccess { address } => {
                write!(f, "Misaligned memory access at address: {address} ({address:x}).")
            }
            ErrorKind::SizeMismatch { required, actual } => {
                write!(f, "Size mismatch error. Required {required} bytes, got {actual}.")
            }
            ErrorKind::Verbose { message } => write!(f, "[ERROR]: {message}"),
            ErrorKind::InternalFailure => write!(
                f,
                "Entered unrecoverable failure state with an unknown or unexpected origin."
            ),
        }
    }
}
