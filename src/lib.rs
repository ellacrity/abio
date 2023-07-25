//! [`abio`][abio] is a low-level crate for performing endian-aware operations
//! on raw byte slices, converting to and from concrete types using zero-copy
//! serialization and deserialization routines. This crate is an attempt at
//! implementing ["safe transmute"][safe-transmute].
//!
//! This project is currently under active development and there are expected to be
//! many breaking changes while trying to stabilize the project as quickly as
//! possible.
//!
//! [abio]: https://docs.rs/abio/latest/src/abio
//! [safe-transmute]: https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html
#![no_std]
#![feature(const_trait_impl, trait_alias, strict_provenance)]

pub mod integer;
pub use integer::{Integer, NonZeroInteger};

mod layout;
pub use layout::decode::Decode;
pub use layout::{config, Abi, BytesOf, Zeroable};

mod contiguous;
pub use contiguous::{Array, Bytes, Chunk, Source, Span};

mod error;
pub use error::{Error, Result};

#[cfg(feature = "shims")]
pub mod shims;
#[cfg(not(feature = "shims"))]
mod shims;

// Enable traits to be derived if the `derived` feature is enabled
#[cfg(feature = "derive")]
pub use abio_derive::{Abi, AsBytes, Decode, Zeroable};
