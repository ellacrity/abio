//! Create a highly-configurable codec instance to determine how your data is decoded
//! and encoded.

use crate::Endian;
use crate::{Error, Result};

/// Configurable type that is used to decode and encode data.
///
/// The codec uses an [`Endian`] type to determine the byte order and a [`Limit`] to
/// define the maximum size of the type to be decoded.
///
/// # Limit
///
/// Setting a limit helps reduce the risk of overflowing the stack. If no limit is
/// set, the stack could easily overflow, causing a runtime panic or, in the worst
/// case, possible undefined behaviour.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Codec {
    endian: Endian,
    limit: Limit,
}

impl Codec {
    /// Constructs a new [`CodecBuilder`] with default settings.
    ///
    /// The default settings are:
    /// * `endian`: None
    /// * `limit`: None
    ///
    /// These settings must be updated using the builder's methods before a valid
    /// `Codec` can be constructed.
    #[doc(alias = "new")]
    #[inline]
    pub const fn builder() -> CodecBuilder {
        CodecBuilder { endian: None, limit: None }
    }

    /// Returns the byte order, or endianness, associated with this codec.
    #[inline]
    pub const fn endian(&self) -> Endian {
        self.endian
    }

    /// Returns the maximum number of bytes this codec can handle.
    #[inline]
    pub const fn limit(&self) -> Limit {
        self.limit
    }

    /// Returns `true` if this codec instance is configured to use little endian byte
    /// order.
    #[inline]
    pub const fn is_big_endian(&self) -> bool {
        self.endian.is_big_endian()
    }

    /// Returns `true` if this codec instance is configured to use big endian byte
    /// order.
    #[inline]
    pub const fn is_little_endian(&self) -> bool {
        self.endian.is_little_endian()
    }
}

impl Default for Codec {
    #[inline]
    fn default() -> Self {
        Codec::builder()
            .with_little_endian()
            .with_limit(Limit::new(0x1000))
            .try_build()
            .expect("CodecBuilder failed to produce properly initialized Codec instance.")
    }
}

/// Maximum number of bytes a configured [`Codec`] instance can handle at a time.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(transparent)]
pub struct Limit(u32);

impl Limit {
    /// Default limit value. The chosen value is equivalent to the size of a page of
    /// virtual memory on most modern desktop computers and servers.
    const DEFAULT_MAX_LIMIT: u32 = 0x1000;

    /// Creates a new [`Limit`] with a given value.
    #[inline]
    pub const fn new(value: u32) -> Limit {
        Limit(value)
    }

    /// Gets the inner value for this limit.
    #[inline]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl core::ops::Deref for Limit {
    type Target = u32;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Limit {
    #[inline(always)]
    fn default() -> Limit {
        Limit(Self::DEFAULT_MAX_LIMIT)
    }
}

/// Builder that allows constructing validated instances of the [`Codec`] type.
pub struct CodecBuilder {
    /// Byte order serialization kind.
    endian: Option<Endian>,
    /// Maximum allowed buffer length.
    limit: Option<Limit>,
}

impl CodecBuilder {
    /// Creates a new [`CodecBuilder`] with all fields set to default values.
    ///
    /// This type is used to create a valid [`Codec`], which can be used to perform
    /// decoding and encoding operations.
    #[inline]
    pub const fn new() -> CodecBuilder {
        CodecBuilder { endian: None, limit: None }
    }

    /// Use big endian byte order serialization for the codec.
    #[inline]
    pub const fn with_big_endian(self) -> CodecBuilder {
        CodecBuilder { endian: Some(Endian::Little), ..self }
    }

    /// Use little endian byte order serialization for the codec.
    #[inline]
    pub const fn with_little_endian(self) -> CodecBuilder {
        CodecBuilder { endian: Some(Endian::Little), ..self }
    }

    /// Use little endian byte order serialization for the codec.
    #[inline]
    pub const fn with_const_limit<const LIMIT: u32>(self) -> CodecBuilder {
        CodecBuilder { limit: Some(Limit::new(LIMIT)), ..self }
    }

    /// Set the endianness used for the codec.
    #[inline]
    pub const fn with_endian(mut self, endian: Endian) -> CodecBuilder {
        self.endian = Some(endian);
        self
    }

    /// Set the limit used for the codec.
    #[inline]
    pub const fn with_limit(mut self, limit: Limit) -> CodecBuilder {
        self.limit = Some(limit);
        self
    }

    /// Returns a configured [`Codec`] from this [`CodecBuilder`].
    ///
    /// This operation is fallible. For the infallible version, please see
    /// [`CodecBuilder::build`][build].
    ///
    /// # Errors
    ///
    /// Returns an error if any of the expected fields are let unset.
    ///
    /// [build]: CodecBuilder::build
    #[inline]
    pub const fn try_build(self) -> Result<Codec> {
        let Some(endian) = self.endian else {
            return Err(Error::invalid_codec("endianness must be set"));
        };
        let Some(limit) = self.limit else {
            return Err(Error::invalid_codec("byte limit must be set"));
        };
        Ok(Codec { endian, limit })
    }

    /// Returns a configured [`Codec`] from this [`CodecBuilder`], using the values
    /// within the `fallback` codec if any values are left unset.
    #[inline]
    pub const fn build(self, fallback: Codec) -> Codec {
        let endian = match self.endian {
            Some(endian) => endian,
            None => fallback.endian(),
        };
        let limit = match self.limit {
            Some(limit) => limit,
            None => fallback.limit(),
        };
        Codec { endian, limit }
    }
}
