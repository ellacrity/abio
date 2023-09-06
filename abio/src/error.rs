//! Common errors that may occur within the [`abio`][abio] crate.
//!
//! [abio]: https://docs.rs/abio/latest/abio/
use core::fmt;
use core::ops::Range;

mod internal;

/// Core error type for representing failure states originating within the
/// [`abio`][crate] crate.
///
/// This type is actually a thin wrapper around an enum with common error variants.
/// These variants each represent a particular failure state.
pub struct Error {
    kind: internal::ErrorKind,
}

// ISSUE: https://github.com/ellacrity/abio/issues/5
impl Error {
    /// Creates a new [`Error`] instance from an inner [`ErrorKind`].
    pub(crate) const fn new(kind: internal::ErrorKind) -> Error {
        Error { kind }
    }

    /// The reading subroutine failed due to the presence of malformed data.
    pub(crate) const fn decoder_failed() -> Error {
        Error::new(internal::ErrorKind::EncodeFailed {
            message: "Deserialzation routine failed due to the presence of malformed data.",
        })
    }

    /// The serialization subroutine failed due to the presence of malformed data.
    pub(crate) const fn encoder_failed() -> Error {
        Error::new(internal::ErrorKind::SerializationFailed {
            message: "Serialization routine failed due to the presence of malformed data.",
        })
    }

    /// The read operation failed due to an unexpected reason, described by
    /// `message`.
    pub(crate) const fn read_failed(message: &'static str) -> Error {
        Error::new(internal::ErrorKind::EncodeFailure { message })
    }

    /// The slice of data contains an unexpected or missing sentinel value.
    pub(crate) const fn invalid_sentinel_slice() -> Error {
        Error::new(internal::ErrorKind::InvalidSentinelOffset)
    }

    /// This operation failed due to an attempted memory access outside the bounds of
    /// the allocated object.
    pub(crate) const fn out_of_bounds(needed: usize, available: usize) -> Error {
        Error::new(internal::ErrorKind::OutOfBounds(OutOfBoundsError::new(
            needed, available,
        )))
    }

    /// The pointer is not aligned properly to meet the layout requirements of a
    /// type.
    pub(crate) fn misaligned_access<T: crate::Abi>(ptr: *const T) -> Error {
        Error::new(internal::ErrorKind::MisalignedAccess { ptr: ptr.addr() })
    }

    /// The number of bytes in the source buffer do not match the number of bytes
    /// comprising a concrete type of some type `T`.
    pub(crate) const fn size_mismatch(expected: usize, actual: usize) -> Error {
        Error::new(internal::ErrorKind::SizeMismatch { expected, actual })
    }

    /// The system has entered an unknown or unexpected failure state.
    ///
    /// This state may or may not be recoverable from, and it should be assumed that
    /// a runtime panic is likely if the failure is not handled properly.
    pub(crate) const fn internal_failure() -> Error {
        Error::new(internal::ErrorKind::InternalFailure)
    }

    /// A type conversion operation failed due to the types having incompatible
    /// layouts.
    pub(crate) const fn incompatible_types() -> Error {
        Error::new(internal::ErrorKind::IncompatibleTypes)
    }

    /// The operation would cause a dereference on a pointer known to be invalid.
    pub(crate) const fn null_reference() -> Error {
        Error::new(internal::ErrorKind::NullReference)
    }

    /// Error with a detailed message meant for debugging purposes.
    pub(crate) const fn verbose(message: &'static str) -> Error {
        Error::new(internal::ErrorKind::Verbose { message })
    }

    /// The `range` instance represents a pair of indices that would violate the
    /// safety contract provided by the [`Span`] type.
    ///
    /// [`Span`]: https://docs
    pub(crate) const fn invalid_span(range: Range<usize>) -> Error {
        Error::new(internal::ErrorKind::InvalidSpan {
            start: range.start,
            end: range.end,
        })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.kind, f)
    }
}

impl Eq for Error {}
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl From<internal::ErrorKind> for Error {
    #[inline]
    fn from(kind: internal::ErrorKind) -> Error {
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

impl From<core::ffi::FromBytesWithNulError> for Error {
    fn from(_: core::ffi::FromBytesWithNulError) -> Self {
        Error::verbose("Found NUL byte in unexpected position (valid CStr/CString types require only a single NUL byte at the end of the type)")
    }
}

impl From<core::ffi::FromBytesUntilNulError> for Error {
    fn from(_: core::ffi::FromBytesUntilNulError) -> Self {
        Error::verbose(
            "Failed to find expected sentinel NUL byte to terminate the length of this input",
        )
    }
}

impl From<&'static str> for Error {
    #[inline]
    fn from(message: &'static str) -> Error {
        Error::verbose(message)
    }
}

impl Error {}

impl fmt::Display for Error {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            internal::ErrorKind::OutOfBounds(e) => fmt::Display::fmt(e, f),
            internal::ErrorKind::IncompatibleTypes => {
                write!(
                    f,
                    "Failed to convert one type to another due to incompatible layouts"
                )
            }
            internal::ErrorKind::InternalFailure => write!(
                f,
                "Entered unrecoverable failure state with an unknown or unexpected origin"
            ),
            internal::ErrorKind::MisalignedAccess { ptr } => {
                write!(f, "Misaligned memory access caused by misaligned pointer; alignment displacement: {ptr:#010?}")
            }
            internal::ErrorKind::NullReference => {
                write!(f, "Invalid pointer dereferenced to null",)
            }
            internal::ErrorKind::SizeMismatch { expected, actual } => {
                write!(
                    f,
                    "Size mismatch error (Required {expected} bytes, got {actual}"
                )
            }
            internal::ErrorKind::EncodeFailed { message } => {
                write!(f, "Encode failed: {message}")
            }
            internal::ErrorKind::SerializationFailed { message } => {
                write!(f, "Encode failed: {message}")
            }
            internal::ErrorKind::InvalidSpan { start, end } => {
                write!(f, "Span cannot be constructed; invariant violation (expected `start <= end`, got: {start} | {end})")
            }
            internal::ErrorKind::InvalidSentinelOffset => {
                write!(f, "An unexpected sentinel value was found at the wrong offset, or an expected (trailing) sentinel value is missing")
            }
            internal::ErrorKind::EncodeFailure { message } => {
                write!(f, "Encode operation failed; {message}")
            }
            internal::ErrorKind::WriteFailure { message } => {
                write!(f, "Write operation failed; {message}")
            }
            internal::ErrorKind::Verbose { message } => write!(f, "{message}"),
        }
    }
}

/// Error originating from a failed attempt to inspect the target system.
///
/// This is a rare error and should only happen in two circumstances:
///   * The target triplet is simply not supported
///   * An internal parsing failure occurred. If you suspect this is the case, please
///     open an issue and provide a minimally-reproducible example.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CompilerTargetError {
    message: &'static str,
}

impl CompilerTargetError {
    /// Returns a `CompilerTargetError` with a details message for debugging
    /// purposes.
    pub const fn with_message(message: &'static str) -> Self {
        Self { message }
    }
}

/// Error returned when an attempt is made to access memory that is out of bounds of
/// any valid, allocated object.
///
/// # Platform-Specific Behaviour
///
/// On Windows, this is equivalent, for the most part, to a
/// `STATUS_ACCESS_VIOLATION`. This means that the `OutOfBoundsError` may actually be
/// occurring because you are attempting to access guarded memory with
/// read-protections enabled.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OutOfBoundsError {
    /// Number of bytes needed for the operation.
    needed: usize,
    /// Actual number of available bytes.
    available: usize,
}

impl OutOfBoundsError {
    pub const fn new(needed: usize, available: usize) -> Self {
        Self { needed, available }
    }
}

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OutOfBoundsError: needed at least {} bytes, got {}",
            &self.needed, &self.available
        )
    }
}

/// Type alias for conveniently constructing `Result` types using this crate's
/// [`Error`] type.
pub type Result<T, E = Error> = core::result::Result<T, E>;
