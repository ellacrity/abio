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

pub mod endian;

mod zeroable;
pub use zeroable::Zeroable;
