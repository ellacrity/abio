use crate::{Abi, Endianness, Result};

/// Trait to define types that can write values
pub trait Encode<T: Abi> {
    /// Encode a value into a mutable slice of bytes, returning the number of
    /// bytes written.
    ///
    /// This crate uses the [`ptr::write`][`core::ptr::write`] method whenever
    /// possible because [`abio`][crate] favors soundness and correctness over
    /// performance. This helps us avoid **undefined behaviour**, and
    /// strikes a healthy balance between safety and efficiency.
    ///
    /// # Performance
    ///
    /// This operation provides a balance between performance and safety. It favors
    /// safety whenever it can only have one of the two. In most cases, it can be
    /// safe without sacrificing on speed.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to a size mismatch, misaligned
    /// memory access, or if the result represents a type with an invalid byte
    /// pattern.
    fn encode<E: Endianness>(buf: &mut [u8], value: T) -> Result<()>;
}

/// Trait to define types that can encode values into buffers of bytes.
pub trait Encoder: Endianness {
    /// Write an aligned [`u8`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_u8(buf: &mut [u8], value: u8) -> Result<()>;

    /// Write an aligned [`u16`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_u16(buf: &mut [u8], value: u16) -> Result<()>;

    /// Write an aligned [`u32`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_u32(buf: &mut [u8], value: u32) -> Result<()>;

    /// Write an aligned [`u64`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_u64(buf: &mut [u8], value: u64) -> Result<()>;

    /// Write an aligned [`u128`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_u128(buf: &mut [u8], value: u128) -> Result<()>;

    /// Write an aligned [`i8`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_i8(buf: &mut [u8], value: i8) -> Result<()>;

    /// Write an aligned [`i16`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_i16(buf: &mut [u8], value: i16) -> Result<()>;

    /// Write an aligned [`i32`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_i32(buf: &mut [u8], value: i32) -> Result<()>;

    /// Write an aligned [`i64`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_i64(buf: &mut [u8], value: i64) -> Result<()>;

    /// Write an aligned [`i128`] value into a mutable byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the write operation fails. The main source of error is
    /// when `buf` does not contain enough bytes to construct the type represented by
    /// `value`.
    fn write_i128(buf: &mut [u8], value: i128) -> Result<()>;
}
