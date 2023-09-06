//! Module containing utilities for working with spanned regions of memory.
//!
//! The central type of the module is the [`Span`] type, which is very useful for
//! performing slicing and subslicing operations on both byte slices and fixed size
//! arrays.

use core::ops::{Index, Range};

use crate::source::Chunk;
use crate::Bytes;

/// A bounded region of memory defined by a pair of indices that point to the same
/// [allocated object][allocated-object].
///
/// This is primarily a utility type, but its API will likely be expanded in the
/// future to make it more powerful. For example, it may be used to encode contextual
/// information via the type system that can later be retrieved.
///
/// # Usage
///
/// In [`abio`][crate], spans are used primarily as an "adapter" type, allowing
/// for simple, elegant and ergonomic operations on byte slices where offset values
/// are required. [`Span`]s can be used to operate on [`Source`] types to produce
/// slices (along with other exotic types, such as [DST][dst]s) as well as types with
/// fixed sizes, such as [`Chunk`]s.
///
/// [allocated-object]: https://doc.rust-lang.org/std/ptr/index.html#allocated-object
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Span {
    /// Start offset of the span.
    start: usize,
    /// End offset of the span.
    end: usize,
}

impl Span {
    /// Maximum allowed length of a contiguous region of memory in bytes.
    const MAX_SIZE: usize = isize::MAX as usize;

    /// Creates a new [`Span`] from an offset and size.
    ///
    /// Spans are a simple abstraction for performing safe operations on slices. They
    /// are very similar to the [`Range<usize>`] type, but instead hold a `
    /// representing the start of the span and `end` offset.
    #[inline(always)]
    pub const fn new(start: usize, size: usize) -> Self {
        // Ensure that a Span does not overflow LLVM's GEP
        debug_assert!(start + size <= Span::MAX_SIZE, "Illegal construction of Span type due to integer overflow. Maximum allowed size is 0x7fff_ffff_ffff_ffff");
        Self { start, end: start.saturating_add(size) }
    }

    /// Constructs a new [`Span`] instance from a [`Range<usize>`].
    #[inline]
    pub const fn from_range(range: Range<usize>) -> Self {
        debug_assert!(range.start < range.end, "Illegal Span construction from Range. Valid ranges must fulfill `start < end`. ZSTs are not supported.");
        Self { start: range.start, end: range.end }
    }

    /// Returns the length of this [`Span`].
    #[inline]
    #[doc(alias = "len")]
    pub const fn size(&self) -> usize {
        self.end
            .saturating_sub(self.start)
    }

    /// Returns `true` if the length of the span is 0.
    #[must_use]
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Returns the start of this span, also referred to as its `offset`.
    #[inline]
    #[doc(alias = "offset")]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the end the span, represented as its upper bound, or index.
    #[inline]
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Converts this [`Span`] into a [`Range<usize>`].
    #[inline]
    pub const fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Advances the span forward by `T::SIZE` bytes.
    #[inline]
    pub fn advance(&mut self, count: usize) {
        self.start += count;
        // Move end of span same length, but add 1 to avoid ZST.
        self.end = self.start.saturating_add(1);
    }
}

impl From<usize> for Span {
    fn from(size: usize) -> Span {
        Span::new(0, size)
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Span {
        Span::from_range(range)
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Range<usize> {
        Range { start: span.start, end: span.end }
    }
}

impl Index<Span> for [u8] {
    type Output = [u8];

    fn index(&self, span: Span) -> &Self::Output {
        &self[span.range()]
    }
}

impl<'data> Index<Span> for Bytes<'data> {
    type Output = [u8];

    fn index(&self, span: Span) -> &Self::Output {
        &self.as_slice()[span.range()]
    }
}
impl<const N: usize> Index<Span> for Chunk<N> {
    type Output = [u8];

    fn index(&self, span: Span) -> &Self::Output {
        self.subslice(span.range())
    }
}

impl Copy for Span {}
impl Clone for Span {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for Span {
    fn default() -> Self {
        Self { start: 0, end: 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn span_index_operations() {
        // create a stream of bytes where we have a `u32`, `u16`, then message of arbitrary
        // length.
        let bytes = *b"\xC8\x80\x40\x55\x00\x0CMESSAGE: byte span index\r\n";
        let span = Span::new(0, 4);
        assert_eq!(span.size(), 4);
        let bytes = &bytes[span];
        assert_eq!(bytes, b"\xC8\x80\x40\x55");

        let chunk = Chunk::<4>::from_slice(bytes)
            .expect("Chunk construction from bytes with size '4' failed.");
        let value = u32::from_le_bytes(chunk.into_array());
        assert_eq!(value, 1430290632);

        let bytes = &0x12345678u32.to_be_bytes()[..];
        let value = util::read_aligned_bytes::<u32>(bytes)
            .expect("Failed to read value from spanned bytes");
        assert_eq!(
            value.to_be(),
            0x12345678,
            "Span bytes (0..4) should equal 3363848277, got {value}"
        );
    }
}
