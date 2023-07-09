//! This module contains core traits that ensure compatibility with a consistent ABI.
//!
//! The ABI is essentially the same as `#[repr(C)]`, making it simple and
//! straightforward to define.
//!
//! # Soundness
//!
//! Types within this module are sound so long as they are compatible with the ABI
//! that this crate uses. Since these traits all can be derived, even complex types
//! such as structs and unions can be validated at compile time. This zero-cost
//! abstraction means that little to no runtime costs are incurred as a result of
//! using this crate.
//!
//! # Automatic Implementation
//!
//! It is strongly recommended that you use the `derive` macros included in the
//! `aligned_derive` sister crate to validate the layout of your types at compile
//! time. Relying on runtime checks is more error-prone and does not provide the same
//! safety guarantees available when deriving the traits for your types.

mod aligned;
pub use aligned::{Aligned, NoPadding};

mod zeroable;
pub use zeroable::Zeroable;

mod codec;
pub(crate) use codec::BytesExt;
pub use codec::{Decodable, Decode};
