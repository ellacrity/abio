//! Module containing the [`Sealed`] trait, which prevents downstream users of
//! this crate from implementing certain items.

/// Trait that prevents downstream crates from implementing
#[doc(hidden)]
pub(crate) trait Sealed {}

impl Sealed for crate::endian::BigEndian {}
impl Sealed for crate::endian::LittleEndian {}

impl Sealed for core::cell::Ref<'_, u8> {}
impl Sealed for core::cell::RefMut<'_, u8> {}

impl Sealed for str {}
impl Sealed for &'_ str {}

impl Sealed for [u8] {}
impl Sealed for &'_ [u8] {}

impl Sealed for crate::source::Bytes<'_> {}

impl<const N: usize> Sealed for [u8; N] {}
impl<const N: usize> Sealed for crate::source::Chunk<N> {}
