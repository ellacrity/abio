//! Module containing the [`Sealed`] trait, which prevents downstream users of
//! this crate from implementing certain items.
//!
//! # Usage
//!
//! The types that are [`Sealed`] are very likely to change over time as this library
//! grows and matures. Since this trait effectively blocks users from being able to
//! take advantage of certain features, the current stance is to be careful about
//! applying `Sealed`. This allows users more flexibility while avoiding future
//! breaking changes.
#[doc(hidden)]
pub trait Sealed {}

mod private {
    use crate::Alignment;
    use super::Sealed;

    impl<T> Sealed for T where T: Alignment {}
    
    impl<const N: usize> Sealed for crate::Chunk<N> {}

    impl Sealed for crate::context::endian::BigEndian {}
    impl Sealed for crate::context::endian::LittleEndian {}
}
