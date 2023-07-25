//! This module contains traits that define abstractions for this crate's ABI.
//!
//! # Type Layout
//!
//! The ABI (Application Binary Interface) is based on a combination of the
//! `#[repr(C)]`, `#[repr(transparent)]` and `#[repr(aligned(int))]` layout
//! attributes.memory layout representation. This makes the ABI predictable, relative
//! simple and straightforward to define.
//!
//! # Soundness
//!
//! Types within this module are sound so long as they are compatible with the ABI
//! that this crate uses. Since these traits all can be derived, even complex types
//! such as structs and unions can be validated at compile time. This zero-cost
//! abstraction means that little to no runtime costs are incurred as a result of
//! using this crate.
//!
//! # Derive
//!
//! It is strongly recommended that you use the `derive` macros included in the
//! `abio_derive` sister crate to validate the layout of your types at compile
//! time. Relying on runtime checks is more error-prone and does not provide the same
//! safety guarantees available when deriving the traits for your types.

pub mod abi;
pub use abi::Abi;

mod bytes_of;
pub use bytes_of::BytesOf;

pub mod decode;
pub use decode::Decode;

pub mod config;

mod slice;
pub use slice::Slice;

mod zeroable;
pub use zeroable::Zeroable;

/// Types whose values can be safely transmuted between byte arrays of the same size.
///
/// # Safety
///
/// It must be safe to transmute between any byte array (with length equal to the
/// size of the type) and `Self`.
///
/// This is true for these primitive types: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`,
/// `u16`, `u32`, `u64`, `u128`, `f32`, `f64`. The raw pointer types are not pod
/// under strict provenance rules but can be through the 'int2ptr' feature.
/// Primitives such as `str` and `bool` are not pod because not every valid byte
/// pattern is a valid instance of these types. References or types with lifetimes
/// are _never_ pod.
///
/// Arrays and slices of pod types are also pod themselves.
///
/// Note that it is legal for pod types to be a [ZST](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts).
///
/// When `Pod` is implemented for a user defined type it must meet the following
/// requirements:
///
/// * Must be annotated with [`#[repr(C)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc)
///   or [`#[repr(transparent)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent).
/// * Must have every field's type implement `Pod` itself.
/// * Must not have any padding between its fields, define dummy fields to cover the
///   padding.
///
/// # Derive macro
///
/// To help with safely implementing this trait for user defined types, a [derive
/// macro](derive@Pod) is provided to implement the `Pod` trait if the requirements
/// are satisfied.
pub unsafe trait Pod: 'static {}
