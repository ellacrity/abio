//! Wrapper types and extensions for Rust's built-in integer primitives.
//!
//! This module is split into the [`signed`][signed] and [`unsigned`][unsigned]
//! submodules. These may either be merged or feature-gated in the future. Merging
//! them would make the crate itself a bit more convenient to use. Feature-gating the
//! integers, on the other hand, could be used to disable unnecessary codegen.
//!
//! [signed]: crate::integer::signed
//! [unsigned]: crate::integer::unsigned
mod aligned;
pub use aligned::{Isize, Usize, I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};
