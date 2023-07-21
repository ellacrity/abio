//! Module containing utilities for working with spanned regions of memory.
//!
//! The central type of the module is the [`Span`] type, which is very useful for
//! performing slicing and subslicing operations on both byte slices and fixed size
//! arrays.

use core::num::NonZeroUsize;
use core::ops::{Add, AddAssign, Index, IndexMut, Range};
use core::slice;

use crate::contiguous::Chunk;
use crate::{Array, AsBytes, Bytes, Source};

// FIXME: Make Span's methods `const` as soon as they are supported.

/// A region of memory defined by a pair of indices marking the start and end offsets
/// of an allocated object in memory.
///
/// # Usage
///
/// In [`abio`][crate], spans are used primarily as an "adapter" type, allowing for
/// simple, elegant and ergonomic operations on byte slices where offset values are
/// required. [`Span`]s can be used to operate on [`Source`] types to produce slices
/// (along with other exotic types, such as [DST][dst]s) as well as types with fixed
/// sizes, such as [`Chunk`]s.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Span {
    /// Start offset of the span.
    start: usize,
    /// End offset of the span.
    end: usize,
}

impl Add<usize> for Span {
    type Output = usize;

    fn add(self, other: usize) -> Self::Output {
        self.start + other
    }
}
impl AddAssign<usize> for Span {
    fn add_assign(&mut self, rhs: usize) {
        let mut start = self.start;
        let mut end = self.end;
        start += rhs;
        let end = if start > end {
            end = start;
            end
        } else {
            end += rhs;
            end
        };
        *self = Self { start, end };
    }
}

impl Add<Span> for Span {
    type Output = Span;

    fn add(self, other: Span) -> Self::Output {
        Span::new(self.start + other.start, self.end + other.end)
    }
}

impl AddAssign<Span> for Span {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self { start: self.start + rhs.start, end: self.end + rhs.end }
    }
}

impl Span {
    /// Creates a new [`Span`] from a `start and `end` offset.
    ///
    /// # Panics
    ///
    /// This contructor method will panic if `start < end`.
    #[inline(always)]
    pub const fn new(offset: usize, size: usize) -> Self {
        Span { start: offset, end: offset + size }
    }

    #[inline]
    pub const fn from_bytes(bytes: &[u8]) -> Span {
        Self { start: 0, end: bytes.len() }
    }

    #[inline]
    pub fn from_source<S: Source>(source: &S) -> Span {
        Self { start: 0, end: source.source_len() }
    }

    /// Constructs a new [`Span`] by reading data from a sized [`Chunk`], read from
    /// some input [`Source`].
    ///
    /// Returns `None` if the
    #[inline]
    pub fn from_chunk<'chunk, A: Array<'chunk, N>, const N: usize>(chunk: Chunk<N>) -> Span {
        Self { start: 0, end: chunk.as_slice().len() }
    }

    /// Returns the size of this [`Span`].
    #[inline]
    pub const fn size(&self) -> usize {
        self.end.saturating_sub(self.start)
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

    #[inline]
    pub(crate) const fn empty() -> Span {
        Span::new(usize::MIN, isize::MAX as usize)
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

impl IndexMut<Span> for [u8] {
    fn index_mut(&mut self, span: Span) -> &mut Self::Output {
        &mut self[span.start..span.end]
    }
}

impl<'a> Index<Span> for Bytes<'a> {
    type Output = [u8];

    fn index(&self, span: Span) -> &Self::Output {
        self.slice(span.start..span.end)
    }
}
