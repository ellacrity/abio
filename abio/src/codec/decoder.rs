use crate::{Abi, Endianness, Result};

/// A trait to define the endianness, or byte order, of some contiguous region of
/// memory represented as a byte slice.
///
/// # Runtime Flexibility
///
/// This trait provides the ability to dynamically choose the required endianness.
/// This enables handling data that may be in big endian, little endian, or a mix of
/// both. This is particularly beneficial when the byte order cannot be determined
/// until runtime.
///
/// # Considerations
///
/// Most machines default to [`LittleEndian`] byte order. However, this isn't
/// universal, and some machines use [`BigEndian`] byte order. In addition, there's
/// [`NetworkEndian`] byte order (which is synonymous with big endian). Many
/// prevalent network protocols employ "network endian" byte order for serialization.
/// Hence, it's crucial to ensure the appropriate byte order is chosen for your
/// specific use-case.
pub trait Decoder: Endianness {
    /// Decode an aligned [`u8`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_u8(&self, bytes: &[u8]) -> Result<u8>;

    /// Decode an aligned [`u16`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_u16(bytes: &[u8]) -> Result<u16>;

    /// Decode an aligned [`u32`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_u32(bytes: &[u8]) -> Result<u32>;

    /// Decode an aligned [`u64`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_u64(bytes: &[u8]) -> Result<u64>;

    /// Decode an aligned [`u128`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_u128(bytes: &[u8]) -> Result<u128>;

    /// Decode an aligned [`i8`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_i8(bytes: &[u8]) -> Result<i8>;

    /// Decode an aligned [`i16`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_i16(bytes: &[u8]) -> Result<i16>;

    /// Decode an aligned [`i32`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_i32(bytes: &[u8]) -> Result<i32>;

    /// Decode an aligned [`i64`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the operation fails due to an unsufficient number of
    /// bytes in the buffer. The byte slice must contain at least `size_of::<T>()`
    /// bytes where `T` is the return type.
    fn read_i64(bytes: &[u8]) -> Result<i64>;

    /// Decode an aligned [`i128`] from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the read operation fails. The
    fn read_i128(bytes: &[u8]) -> Result<i128>;
}

/// The [`Encode`] trait defines how a type is decoded or decoded from a
/// slice or chunk of bytes after being validated. It provides a way to translate raw
/// byte sequences back into meaningful data in a structured manner.
///
/// # Performance
///
/// Although this trait prioritizes safety and correctness over speed, it offers two
/// variants of the provided reader routines. One of them returns a
/// reference to the underlying data while the second performs a [`core::ptr::read`]
/// operation on the pointer, performing a bitwise copy. This method is not quite as
/// performant as taking a reference to a dereferenced raw pointer, so this trait
/// provides the ability to implement both.
///
/// # Size and Alignment
///
/// Correctly handling alignment is an important part of preventing
/// undefined behavior, which is a key goal of the abio crate.
///
/// # Endian-Aware Decoding
///
/// [`Encode`] leverages the [`Endianness`] trait to provide ergonmic access to
/// endian-aware read and write primitives. These primitives operate on raw byte
/// slices only, since file and network I/O is outside the scope of [`abio`][crate].
/// It ensures the proper interpretation of data according to the specific byte order
/// serialization type.
///
/// # Implementing Encode
///
/// Blanket-like implementations are provided out of the box for all simple
/// primitives and common types, so you don't have to repeat this work. If you would
/// prefer to implement [`Encode`] yourself, you may disable the
/// __`default-codec`__ feature, which is enabled by default. ability to disable them
/// behind a feature-gate.
///
/// This trait can be implemented for any type `T` where `T` implements the [`Abi`]
/// trait, and is said to be "ABI-compatible" with respect to this crate.
pub trait Decode<'data>: Abi {
    /// Decodes a concrete type from a slice of bytes, returning a reference to
    /// `Self` and the number of bytes consumed.
    ///
    /// # Errors
    ///
    /// This method returns an error if of the following conditions are true:
    ///   * `bytes.len() < Self::SIZE`
    ///   * The pointer represented by `bytes` does not meet the alignment
    ///     requirements of `Self`.
    /// [`Abi`].
    fn decode<E: Endianness>(bytes: &'data [u8]) -> Result<(&'data Self, usize)>;
}
