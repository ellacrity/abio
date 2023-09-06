use super::OutOfBoundsError;

/// Error variant, or kind, used to more precisely represent the failure
/// condition.
#[derive(Debug, Default, PartialEq)]
pub(crate) enum ErrorKind {
    /// Error caused by a reader subroutine failure.
    ///
    /// This error variant is not very specific, so you have the ability to add
    /// contextual information via the `message` field.
    EncodeFailed {
        /// Message explaining why the reader routine failed.
        message: &'static str,
    },
    /// Error caused by a serialization subroutine failure.
    ///
    /// This error variant is not very specific, so you have the ability to add
    /// contextual information via the `message` field.
    SerializationFailed {
        /// Message explaining why the serialization failed.
        message: &'static str,
    },
    /// Error caused by a failed conversion attempt due to the types having
    /// incompatible layouts.
    IncompatibleTypes,
    /// Error caused by attempting to construct an invalid range. An invalid
    /// range is any range where the `start > end`.
    ///
    /// Internally, this error should not occur since bounds checks are
    /// performed. If you see this error, you can be very certain that
    /// the error is your fault. Be sure to check that your range types
    /// uphold the invariant above and that there are no typos on your
    /// code.
    InvalidSpan {
        /// Start of the memory range.
        start: usize,
        /// End index of the memory range.
        end: usize,
    },
    /// Error caused by an invalid operation attempting to access memory without
    /// first aligning the pointer accessing the underlying data.
    MisalignedAccess { ptr: *const () },
    /// Error caused by an invalid pointer that dereferences to null.
    NullReference,
    /// Error originating from an operation that caused an attempted memory
    /// access outside the bounds of a slice or array.
    OutOfBounds(OutOfBoundsError),
    /// Error occurring when the the sizes of two types, or regions of memory, do
    /// not have the same exact size.
    ///
    /// For source and destination types represent different sizes.
    SizeMismatch {
        /// Minimum number of bytes needed to construct an instance of the target
        /// type.
        expected: usize,
        /// Actual number of available bytes in the buffer.
        actual: usize,
    },
    /// Error caused when an operation produces a region of memory that is not
    /// properly terminated by its sentinel value.
    ///
    /// # Sentinel Values
    ///
    /// Within the context of this crate and the error module, a "sentinel value" may
    /// be a length, such as is common with Rust slice types, or it may be a byte
    /// value, such as a C style NUL terminating byte.
    ///
    /// Note that, in most cases, this should be treated as a recoverable error. This
    /// error variant exists to prevent **undefined behaviour** and instead allow the
    /// user to take action when a critical failure such as this occurs.
    InvalidSentinelOffset,
    /// Error with an unknown or unexpected origin.
    ///
    /// This error is typically a sign that something very, very wrong has
    /// happened. If there are no bugs in your code, then you should report this to
    /// the maintainers and file and issue.
    #[default]
    InternalFailure,
    /// Error with a detailed message meant for debugging purposes.
    Verbose {
        /// Detailed error message explaining the failure.
        message: &'static str,
    },
    /// Abstract error type for a generic read error.
    EncodeFailure {
        /// Message offering detailed information about the error.
        message: &'static str,
    },
    /// Abstract error type for a generic write error.
    WriteFailure {
        /// Message offering detailed information about the error.
        message: &'static str,
    },
}

impl ErrorKind {
    /// Returns `true` if the error kind is [`EncodeFailed`].
    ///
    /// [`EncodeFailed`]: ErrorKind::EncodeFailed
    #[must_use]
    pub(crate) const fn is_decode_failed(&self) -> bool {
        matches!(self, Self::EncodeFailed { .. })
    }

    /// Returns `true` if the error kind is [`SerializationFailed`].
    ///
    /// [`SerializationFailed`]: ErrorKind::SerializationFailed
    #[must_use]
    pub(crate) const fn is_serialization_failed(&self) -> bool {
        matches!(self, Self::SerializationFailed { .. })
    }

    /// Returns `true` if the error kind is [`IncompatibleTypes`].
    ///
    /// [`IncompatibleTypes`]: ErrorKind::IncompatibleTypes
    #[must_use]
    pub(crate) const fn is_incompatible_types(&self) -> bool {
        matches!(self, Self::IncompatibleTypes)
    }

    /// Returns `true` if the error kind is [`InvalidSpan`].
    ///
    /// [`InvalidSpan`]: ErrorKind::InvalidSpan
    #[must_use]
    pub(crate) const fn is_invalid_span(&self) -> bool {
        matches!(self, Self::InvalidSpan { .. })
    }

    /// Returns `true` if the error kind is [`MisalignedAccess`].
    ///
    /// [`MisalignedAccess`]: ErrorKind::MisalignedAccess
    #[must_use]
    pub(crate) const fn is_misaligned_access(&self) -> bool {
        matches!(self, Self::MisalignedAccess { .. })
    }

    /// Returns `true` if the error kind is [`NullReference`].
    ///
    /// [`NullReference`]: ErrorKind::NullReference
    #[must_use]
    pub(crate) const fn is_null_reference(&self) -> bool {
        matches!(self, Self::NullReference)
    }

    /// Returns `true` if the error kind is [`OutOfBounds`].
    ///
    /// [`OutOfBounds`]: ErrorKind::OutOfBounds
    #[must_use]
    pub(crate) const fn is_out_of_bounds(&self) -> bool {
        matches!(self, Self::OutOfBounds { .. })
    }

    /// Returns `true` if the error kind is [`SizeMismatch`].
    ///
    /// [`SizeMismatch`]: ErrorKind::SizeMismatch
    #[must_use]
    pub(crate) const fn is_size_mismatch(&self) -> bool {
        matches!(self, Self::SizeMismatch { .. })
    }

    /// Returns `true` if the error kind is [`InvalidSentinelOffset`].
    ///
    /// [`InvalidSentinelOffset`]: ErrorKind::InvalidSentinelOffset
    #[must_use]
    pub(crate) const fn is_invalid_sentinel_offset(&self) -> bool {
        matches!(self, Self::InvalidSentinelOffset)
    }

    /// Returns `true` if the error kind is [`InternalFailure`].
    ///
    /// [`InternalFailure`]: ErrorKind::InternalFailure
    #[must_use]
    pub(crate) const fn is_internal_failure(&self) -> bool {
        matches!(self, Self::InternalFailure)
    }

    /// Returns `true` if the error kind is [`Verbose`].
    ///
    /// [`Verbose`]: ErrorKind::Verbose
    #[must_use]
    pub(crate) const fn is_verbose(&self) -> bool {
        matches!(self, Self::Verbose { .. })
    }

    /// Returns `true` if the error kind is [`EncodeFailure`].
    ///
    /// [`EncodeFailure`]: ErrorKind::EncodeFailure
    #[must_use]
    pub(crate) const fn is_read_failure(&self) -> bool {
        matches!(self, Self::EncodeFailure { .. })
    }

    /// Returns `true` if the error kind is [`WriteFailure`].
    ///
    /// [`WriteFailure`]: ErrorKind::WriteFailure
    #[must_use]
    pub(crate) const fn is_write_failure(&self) -> bool {
        matches!(self, Self::WriteFailure { .. })
    }
}

impl From<OutOfBoundsError> for ErrorKind {
    fn from(e: OutOfBoundsError) -> Self {
        ErrorKind::OutOfBounds(e)
    }
}
