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
#![feature(strict_provenance)]

pub mod bytes;
pub use bytes::Bytes;

pub mod integer;
pub use integer::Integer;

mod layout;
pub use layout::dec::Decode;
pub use layout::{endian, Abi, AsBytes, Zeroable};

mod contiguous;
pub use contiguous::{Array, Chunk, Source, Span};

mod error;
pub use error::{Error, Result};

#[doc(hidden)]
mod util;

// Enable traits to be derived if the `derived` feature is enabled
#[cfg(feature = "derive")]
pub use abio_derive::{Abi, AsBytes, Decode, Zeroable};
