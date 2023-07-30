#![doc = include_str!("../docs/abio.md")]
#![no_std]
#![deny(missing_docs, clippy::missing_safety_doc, clippy::missing_const_for_fn)]
// ISSUE #4: Remove unstable features when they are stabilized.
#![feature(const_trait_impl, strict_provenance)]

pub mod integer;
use integer::Integer;

mod layout;
pub use layout::decode;
pub use layout::endian;
pub use layout::endian::{BigEndian, Endian, Endianness, LittleEndian, NativeEndian, BE, LE};
pub use layout::{Abi, BytesOf, Decode, Zeroable};

mod config;
pub use config::{Codec, CodecBuilder, Limit};

mod source;
pub use source::{Array, Buf, ByteArray, Bytes, Chunk, Source, Span};

mod error;
pub use error::{Error, Result};

// internal use only
mod shims;

// Enable traits to be derived if the `derived` feature is enabled
#[cfg(feature = "derive")]
pub use abio_derive::{Abi, BytesOf, Decode, Zeroable};

#[doc(hidden)]
mod sealed;
