//! Module containing utilities for working with spanned regions of memory.
//!
//! The central type of the module is the [`Span`] type, which is very useful for
//! performing slicing and subslicing operations on both byte slices and fixed size
//! arrays.

use core::num::NonZeroUsize;
use core::ops::{Index, Range};

use crate::source::Chunk;
use crate::{Array, Error, Result, Slice, Source};

/// A region of memory defined by a pair of indices marking the start and end offsets
/// of an allocated object in memory.
///
/// # Usage
///
/// In [`abio`][crate], spans are used primarily as an "adapter" type, allowing
/// for simple, elegant and ergonomic operations on byte slices where offset values
/// are required. [`Span`]s can be used to operate on [`Source`] types to produce
/// slices (along with other exotic types, such as [DST][dst]s) as well as types with
/// fixed sizes, such as [`Chunk`]s.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Span {
    /// Start offset of the span.
    start: usize,
    /// End offset of the span.
    end: usize,
}

impl Span {
    /// Creates a new [`Span`] from a `start and `end` offset.
    #[inline(always)]
    pub const fn new(start: usize, size: usize) -> Self {
        let end = start + size;
        debug_assert!(start <= end, "Cannot construct a valid span where `start > end`.");
        Span { start, end }
    }

    /// Creates a new [`Span`], representing the range comprising the indices of the
    /// `bytes` slice.
    #[inline]
    pub const fn from_bytes(bytes: &[u8]) -> Self {
        Self { start: 0, end: bytes.len() }
    }

    /// Extracts a type `T` where `T: Abi` from a [`Source`], returning `T` and the
    /// [`Span`] representing the type.
    #[inline]
    pub fn span_source<S: Source>(&self, source: &S) -> Self {
        Self { start: 0, end: source.source_len() }
    }

    /// Constructs a new [`Span`] by reading data from a sized [`Chunk`], read from
    /// some input [`Source`].
    #[inline]
    pub const fn from_chunk<A: Array<N>, const N: usize>(chunk: Chunk<N>) -> Self {
        Self { start: 0, end: chunk.len() }
    }

    /// Returns the length of this [`Span`].
    #[inline]
    pub const fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Returns `true` if the length of the span is 0.
    #[must_use]
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the start of this span, also referred to as its `offset`.
    #[doc(alias = "offset")]
    #[inline]
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
    pub const fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Returns a [`Slice`] from a byte slice starting at `offset`, spanning `size`
    /// bytes in length.
    ///
    /// # Errors
    ///
    /// This function will return an error if `bytes.len() < offset + size`.
    #[inline]
    pub const fn span_bytes(bytes: &[u8], offset: usize, size: usize) -> Result<Slice<'_>> {
        let span = Span::new(offset, size);
        if bytes.len() < span.len() {
            Err(Error::out_of_bounds(span.len(), bytes.len()))
        } else {
            // SAFETY: The check above verifies that `self.start` and `self.size()` are
            // within the bounds of `bytes`. The only way to obtain a `Span` instance is via one
            // of its safe constructor functions, so this is safe.
            Ok(unsafe { Slice::from_slice_at_unchecked(bytes, span.len()) })
        }
    }

    #[inline]
    const fn empty() -> Span {
        Span::new(0, 0)
    }
}

impl From<usize> for Span {
    fn from(offset: usize) -> Self {
        if offset == 0 {
            Span::empty()
        } else {
            Span::new(offset, offset)
        }
    }
}

impl From<NonZeroUsize> for Span {
    fn from(offset: NonZeroUsize) -> Self {
        if offset.get() == 0 {
            Span::empty()
        } else {
            Span::new(offset.get(), offset.get())
        }
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self { start: range.start, end: range.end }
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Range<usize> {
        Self { start: span.start, end: span.end }
    }
}

impl Index<Span> for [u8] {
    type Output = [u8];

    fn index(&self, span: Span) -> &Self::Output {
        &self[span.start..span.end]
    }
}

impl<'data> Index<Span> for Slice<'data> {
    type Output = [u8];

    fn index(&self, span: Span) -> &Self::Output {
        self.slice(span.start..span.end)
    }
}
