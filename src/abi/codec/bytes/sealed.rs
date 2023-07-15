//! Sealed, private traits to prevent downstream users of the crate from
//! implementing this trait.

use core::cell::Ref;

use crate::Bytes;
pub trait Sealed {}

impl Sealed for [u8] {}
impl<'a> Sealed for &'a [u8] {}
impl<'a> Sealed for Bytes<'a> {}
impl<'a> Sealed for Ref<'a, u8> {}

impl Sealed for str {}
impl<'a> Sealed for &'a str {}

impl<const N: usize> Sealed for [u8; N] {}
