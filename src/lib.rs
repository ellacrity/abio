//! [`abio`][abio] is a low-level crate for performing endian-aware operations
//! on raw byte slices, converting to and from concrete types using zero-copy
//! serialization and deserialization routines. This crate is an implementation of
//! "safe transmute".
//!
//! # Alignment
//!
//! Unless otherwise specified, it should be assumed that alignment will be checked
//! and that aligned reads will be used to access the underlying data.
//!
//! [abio]: https://docs.rs/abio/latest/src/abio
#![no_std]
#![feature(
    try_trait_v2,
    // ISSUE (1): Remove unstable features that are due to be stabilized.
    maybe_uninit_uninit_array, 
    // ISSUE (1): Remove unstable features that are due to be stabilized.
    maybe_uninit_array_assume_init,
    // ISSUE (1): Remove unstable features that are due to be stabilized.
    slice_first_last_chunk,
    // Actively opting in to the "Strict Provenance" experiment
    strict_provenance,
)]

mod bytes;
pub use bytes::Bytes;

mod integral;
pub use integral::{Integral};
pub use integral::aligned::*;

#[cfg(feature = "prelude")]
pub mod prelude {
    pub use crate::integral::signed::{I128, I16, I32, I64, I8};
    pub use crate::integral::unsigned::{U128, U16, U32, U64, U8};
}
#[cfg(feature = "prelude")]
pub use prelude::*;

pub mod abi;
pub use abi::{Abi,  Deserialize, Source, Zeroable, Deserializer, LittleEndian, BigEndian, LE, BE};

mod types;

mod error;
pub use error::{Error, Result};

#[doc(hidden)]
mod util;

#[cfg(feature = "derive")]
pub use abio_derive::{Abi, Decode, Source, Zeroable};
