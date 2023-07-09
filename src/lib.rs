//! [`aligned`][aligned] is a low-level crate for performing endian-aware operations
//! on raw byte slices, converting to and from concrete types using zero-copy
//! serialization and deserialization routines. This crate is an implementation of
//! "safe transmute".
//!
//! # Alignment
//!
//! Unless otherwise specified, it should be assumed that alignment will be checked
//! and that aligned reads will be used to access the underlying data.
//!
//! [aligned]: https://docs.rs/aligned-rs/latest/src/aligned-rs
#![allow(incomplete_features)]
#![feature(generic_const_exprs, inline_const, strict_provenance, trait_alias)]
#![no_std]

#[macro_use]
mod macros;

mod bytes;
pub use bytes::Bytes;

pub mod integral;

mod abi;
pub use abi::{Aligned, Decodable, Decode, NoPadding, Zeroable};

mod error;
pub use error::Error;
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[doc(hidden)]
mod util;

mod aligned {
    //! Re-export derive macros:
    //!  `Aligned`, `ReadBytes`, `Decode`.
    //!
    //! This allows the derive macros to work within this crate, tricking the
    //! compiler into resolving items from the correct crate (the derive crate).
    pub(crate) use crate::*;
}
