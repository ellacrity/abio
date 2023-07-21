//! Module containing a newtype wrapper for byte slices.
//!
//! This module provides an easier way to extend the API for `&[u8]` types, since the
//! [`Bytes`] type is local to the crate.

// TODO: Consider how best to represent borrowed and mutable borrowed bytes
// (ellacrity).
mod borrowed;
pub use borrowed::Bytes;
// mod mutable;
// pub use mutable::BytesMut;
