//! Sealed, private traits to prevent downstream users of the crate from
//! implementing this trait.

use core::cell::{Ref, RefMut};

use crate::bytes::Bytes;
use crate::contiguous::Chunk;

#[doc(hidden)]
pub(crate) trait Sealed {}

impl Sealed for [u8] {}
impl Sealed for &'_ [u8] {}
impl Sealed for Bytes<'_> {}
impl Sealed for Ref<'_, u8> {}
impl Sealed for RefMut<'_, u8> {}

impl Sealed for str {}
impl Sealed for &'_ str {}

impl<const N: usize> Sealed for [u8; N] {}
impl<const N: usize> Sealed for Chunk<N> {}
