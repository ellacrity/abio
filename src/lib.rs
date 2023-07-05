//! [`aligned`][crate] is a low-level crate for performing endian-aware operations on
//! raw byte slices, converting to and from concrete types using zero-copy
//! serialization and deserialization routines. This crate is an implementation of
//! "safe transmute".
//!
//! # Alignment
//!
//! Unless otherwise specified, it should be assumed that alignment will be checked
//! and that aligned reads will be used to access the underlying data.

// ? Consider disallowing `incomplete_features` and `generic_const_exprs`
#![allow(incomplete_features)]
#![feature(generic_const_exprs, inline_const, strict_provenance, trait_alias)]
#![no_std]

#[macro_use]
mod macros;

mod bytes;

pub mod integral;

mod error;
pub use error::{
    BufferOverflowError, BufferUnderflowError, LayoutError, MisalignedAccessError,
    SizeMismatchError,
};

#[doc(hidden)]
mod util;

// Re-export `bytes` module.
pub use bytes::*;

/// Types that can be converted to their raw byte representation.
///
/// # Safety
///
/// Implementing this trait is generally safe, as long as the result is consistent
/// with regard to byte order / endianness. That is, types that implement this trait
/// must choose an explicit endianness to represent the type with. Mixing and
/// matching may introduce **undefined behaviour**. Depending on the implementation,
/// it may also be completely sound. It is up to the implementor to enforce the
/// contract and uphold any safety guarantees that the contract introduces.
pub unsafe trait AsBytes: Copy + Sized {
    fn as_bytes(&self) -> &[u8];
}

/// Types that can constructed from raw slices of bytes.
///
/// # Safety
///
/// The following invariants must be upheld, or else it is immediate **undefined
/// behaviour**:
///   * The byte slice cannot be empty: `bytes.len() > 0`, or `!bytes.is_empty()`
///   * Byte slices must uphold the invariant that `bytes.len() >=
///     mem::size_of::<Self>()`
///
/// # Notes
///
/// Implementors of this trait must be very careful. This trait must be implemented
/// to be inherently compatible with [`AsBytes`], or else conversions may cause
/// unpredictable results. If this happens, it is considered immediate **undefined
/// behaviour**.
pub unsafe trait FromBytes: Zeroable {
    /// Converts a slice of bytes to a concrete type.

    fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

/// Types that can be represented by the all-zero byte-pattern.
///
///
/// This means that, for example, the padding byte in `(u8, u16)` is not
/// necessarily zeroed.
///
/// # Safety
///
/// There is no guarantee that an all-zero byte-pattern represents a valid value
/// of some type `T`. For example, the all-zero byte-pattern is not a valid value
/// for reference types (`&T`, `&mut T`) and functions pointers. Using `zeroed`
/// on such types causes immediate [undefined behavior][ub] because [the Rust
/// compiler assumes][inv] that there always is a valid value in a variable it
/// considers initialized.
///
/// This has the same effect as [`MaybeUninit::zeroed().assume_init()`][zeroed].
/// It is useful for FFI sometimes, but should generally be avoided.
///
/// [zeroed]: MaybeUninit::zeroed
/// [ub]: ../../reference/behavior-considered-undefined.html
/// [inv]: MaybeUninit#initialization-invariant
pub unsafe trait Zeroable: Copy + Sized {
    /// Returns the value of type `T` represented by the all-zero byte-pattern.
    ///
    /// # Safety
    ///
    /// The caller must ensure the type `T` is valid when represented as a zeroed-out
    /// buffer.
    ///
    /// Types that violate this contract include:
    /// * Reference types, such as `&T` and `&mut T`
    /// * Function pointers
    unsafe fn zero() -> Self {
        core::mem::zeroed::<Self>()
    }
}

/// Types that represent "plain old data". These types do not contain any references
/// or raw pointers.
///
/// # Safety
///
/// To safely implement this trait for your typpe, it must satisfy the following
/// invariants:
/// * Cannot contain any references or raw pointers.
/// * Must have a fixed size known at compile time.
/// * Well-defined layout compatible with the native OS C ABI.
pub unsafe trait Pod: FromBytes + AsBytes + 'static {
    const ALIGN: usize = core::mem::align_of::<Self>();

    const SIZE: usize = core::mem::size_of::<Self>();

    // FIXME: use const assertions in derive to verify layout:
    // const _: () = assert!(core::mem::size_of::<Self>() == Self::SIZE);
    // const _: () = assert!(core::mem::align_of::<Self>() == Self::ALIGN);
    #[doc(hidden)]
    unsafe fn __verify_layout(self)
    where
        [u8; core::mem::size_of::<Self>()]: Sized,
    {
        assert!(core::mem::size_of::<Self>() == Self::SIZE);
        assert!(core::mem::align_of::<Self>() == Self::ALIGN);
    }
}
