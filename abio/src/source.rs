//! Module for working with contiguous regions of memory.
//!
//! These regions may be slices, such as the [`Bytes`] type or they may represent
//! fixed size data, such as the [`Chunk`] type.
//!
//! # Zero-Cost Abstractions
//!
//! Like all of the core types in [`abio`][crate], the [`Bytes`] type leverages
//! Rust's zero-cost abstractions by extending the built-in `&[u8]` type. It provides
//! additional methods for parsing and validating inputs, converting slices to and
//! from arrays, and working with [`Span`] types.

mod array;
pub use array::Array;

// ISSUE: Add support for mutable slice type and expose via `BytesMut`. Please see the open issue at: https://github.com/ellacrity/abio/issues/6
mod slice;
pub use slice::Bytes;
mod slice_mut;
pub use slice_mut::BytesMut;

mod chunk;
pub use chunk::Chunk;

mod span;
pub use span::Span;
