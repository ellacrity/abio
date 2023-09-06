#![doc = include_str!("../docs/ABOUT.md")]
#![no_std]
#![deny(missing_docs, clippy::missing_safety_doc, clippy::missing_const_for_fn)]
#![feature(
    const_trait_impl,
    const_maybe_uninit_uninit_array,
    maybe_uninit_uninit_array,
    maybe_uninit_array_assume_init,
    strict_provenance,
    trait_alias
)]

pub mod integer;

pub mod codec;
pub use codec::{decoder, encoder, Decode, Decoder, Encode, Encoder};

mod context;
pub use context::{BigEndian, Endian, Endianness, LittleEndian, NativeEndian, BE, LE};

mod marker;
pub use marker::{Abi, Alignment, AsBytes, Zeroable};

mod source;
pub use source::{Array, Bytes, BytesMut, Chunk, Span};

// FIXME: Remove `allow` attribute to get rid of dead code
#[allow(dead_code)]
mod error;
// Enable traits to be derived if the `derived` feature is enabled
#[cfg(feature = "derive")]
pub use abio_derive::{Abi, AsBytes, Decode, Zeroable};
pub use error::{Error, Result};

#[doc(hidden)]
mod sealed;
// internal utilites local to this crate
#[doc(hidden)]
mod util;
