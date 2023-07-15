//! Module containing types and primitives for working with slices of bytes with a
//! known size.
//!
//! Although the type is represented essentially as a slice, there is a subtle
//! difference. [`BytesChunk`], the core type comprising this module, is
//! parameterized over the size, or capacity, of its underlying backing buffer. This
//! allows the compiler to make more aggressive optimizations, since the size of the
//! slice is explicit.
/*
use core::ptr::NonNull;
use core::slice;

use crate::abi::Abi;
use crate::util::{to_byte_array, FromInner};
use crate::{Bytes, Error, Result, Zeroable};

/// Abstract chunk of memory bounds which represent
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArrayChunk<'chunk, T: Abi, const CAP: usize> {
    head: usize,
    /// Inner buffer containing the chunk of memory.
    inner: &'chunk [T; CAP],
}

unsafe impl<'chunk, T: Abi + Send, const CAP: usize> Send for ArrayChunk<'chunk, T, CAP> {}
unsafe impl<'chunk, T: Abi + Sync, const CAP: usize> Sync for ArrayChunk<'chunk, T, CAP> {}

unsafe impl<'chunk, T: Abi, const CAP: usize> Abi for ArrayChunk<'chunk, T, CAP> {}
unsafe impl<'chunk, T: Abi + Zeroable, const CAP: usize> Zeroable for ArrayChunk<'chunk, T, CAP> {}

impl<'chunk, T: Abi, const CAP: usize> ArrayChunk<'chunk, T, CAP> {
    pub const fn new() -> Self {
        ArrayChunk { head: 0, inner: &[0u8; CAP] }
    }

    pub fn from_slice(bytes: &[u8], offset: usize, size: usize) -> Option<Self> {
        assert!(
            bytes.len() >= offset + size,
            "Chunk broke invariant (`bytes.len() < offset + size`). Invalid Chunk."
        );

        assert!(
            bytes.len() > CAP || offset + size > CAP,
            "Chunk SIZE is invalid. Illegal type construction."
        );

        let mut buf = [0u8; CAP];
        buf.copy_from_slice(&bytes[offset..offset + size]);
        let base_ptr = unsafe { NonNull::new_unchecked(bytes.as_ptr().add(offset).cast_mut()) };
        let end_bound = unsafe { base_ptr.as_ptr().add(size) };
        let inner = unsafe {
            let bytes = slice::from_raw_parts(base_ptr.as_ptr().cast::<u8>(), size);
            to_byte_array::<CAP>(bytes)
        };

        ArrayChunk { inner, head: todo!() }
    }

    pub const fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    /// Get the remaining capacity of this array. The capacity is calculated by
    /// subtracting the length by the total capacity.
    ///
    /// This method is useful for obtaining the remaining number of elements, or
    /// slots.
    pub(in crate::abi::codec) const fn remaining_capacity(&self) -> usize {
        CAP.saturating_sub(self.head)
    }

    pub(in crate::abi::codec) const fn has_capacity_for(&self, item: T) -> bool {
        self.remaining_capacity() >= T::SIZE
    }

    pub(in crate::abi::codec) const fn exhausted(&self, bytes: &[u8]) -> bool {
        bytes.len() < self.len()
    }

    pub const fn size(_self: &Self) -> usize {
        assert_eq!(_self.head, CAP, "Chunk produced invalid value for size: `self.len() != SIZE`");
        CAP
    }

    /// Returns the position within the array that represents its length.
    pub const fn len(&self) -> usize {
        self.head
    }

    /// Returns the total capacity of this array.
    pub const fn capacity(_: &Self) -> usize {
        CAP
    }

    pub fn push(&mut self, item: T) {
        if self.has_capacity_for(item) {
            // have enough space to push a byte onto the stack
            unsafe { self.write_element(item) };
        } else {
            panic!("Cannot push item onto the ArrayChunk stack due to overflow.");
        }
    }

    unsafe fn write_element<A: Abi>(&mut self, item: T) {
        self.as_write_ptr().write(item);
        self.head += 1;
    }

    unsafe fn read_element<A: Abi>(&mut self, item: T) -> T {
        self.head -= 1;
        self.as_read_ptr().read();
    }

    pub fn try_push(&mut self, item: T) -> Result<()> {
        if self.has_capacity_for(item) {
            self.push(item);
            Ok(())
        } else {
            Err(Error::out_of_bounds(1, self.remaining_capacity()))
        }
    }

    /// Returns the pointer to the head of the array.
    ///
    /// This method is used as a convenience method for `read` operations.
    fn as_read_ptr(&self) -> *const u8 {
        unsafe { self.inner.as_ptr().add(self.head) }
    }

    fn as_write_ptr(&mut self) -> *mut u8 {
        unsafe { self.inner.as_mut_ptr().add(self.head) }
    }

    pub(in crate::abi::codec) fn consume(&self) -> [u8; CAP] {
        FromInner::<[u8; CAP]>::from_inner(self.inner)
    }

    fn advance_by<A: Abi>(&mut self) {
        self.head += A::SIZE;
    }

    fn offset_by<A: Abi>(&mut self, offset: isize) {
        // if `offset == 0`, then self.head is not modified, and it is a noop
        if offset.is_negative() {
            self.head -= offset;
        } else if offset.is_positive() {
            self.head += offset;
        }
    }
}

impl<const N: usize> FromInner<[u8; N]> for [u8; N] {
    fn from_inner(inner: [u8; N]) -> Self {
        Self::from(inner)
    }
}

impl<'chunk, T, const CAP: usize> Default for ArrayChunk<'chunk, T, CAP> {
    fn default() -> Self {
        Self { inner: [0u8; CAP], head: 0 }
    }
}

pub const fn min_const_impl(this: usize, other: usize) -> usize {
    if !(this >= other && this != other) {
        this
    } else {
        other
    }
}

impl<'a, T, const CAP: usize> From<ArrayChunk<'a, T, CAP>> for Bytes<'a> {
    fn from(chunk: ArrayChunk<'a, T, CAP>) -> Self {
        Bytes::read_chunk(chunk.consume())
    }
}
 */
