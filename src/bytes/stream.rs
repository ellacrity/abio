use core::fmt;
use core::iter::{Copied, Enumerate, FusedIterator};
use core::ops::{Range, RangeFrom, RangeTo};
use core::slice::Iter;

use crate::{Result, Source};

pub trait Slice<'src>: Source {
    fn slice(&self, range: Range<usize>) -> Result<&'src [u8]>;

    fn slice_from(&self, from: RangeFrom<usize>) -> Result<&'src [u8]>;

    fn slice_to(&self, to: RangeTo<usize>) -> Result<&'src [u8]>;

    /// Splits the slice at the offset where `predicate` returns true.
    ///
    /// Returns `None` if the `predicate` function never returns true.
    fn slice_until<I, F>(&self, predicate: F) -> SliceUntil<&'src [u8], F>
    where
        I: IntoIterator<Item = u8>,
        F: Fn(&u8) -> bool;

    fn iter_stream(&self) -> Copied<Iter<'src, u8>>;

    fn enumerate_stream(&self) -> Enumerate<Copied<Iter<'src, u8>>>;
}

/// An iterator that accepts elements until `predicate` returns `true`.
///
/// This `struct` is created by the [`take_until`] method on [`ReadSlice`]. See its
/// documentation for more.
///
/// [`take_while`]: Iterator::take_while
/// [`Iterator`]: trait.Iterator.html
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct SliceUntil<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I, P> SliceUntil<I, P> {
    pub(in crate::bytes) fn new(iter: I, predicate: P) -> SliceUntil<I, P> {
        SliceUntil { iter, flag: false, predicate }
    }
}

impl<I: fmt::Debug, P> fmt::Debug for SliceUntil<I, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TakeUntil").field("iter", &self.iter).field("flag", &self.flag).finish()
    }
}

impl<I: Iterator, P> Iterator for SliceUntil<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.flag {
            None
        } else {
            let Some(item) = self.iter.next() else { return None };
            if !(self.predicate)(&item) {
                Some(item)
            } else {
                self.flag = true;
                None
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.flag {
            (0, Some(0))
        } else {
            let (_, upper) = self.iter.size_hint();
            (0, upper) // can't know a lower bound, due to the predicate
        }
    }
}

impl<I, P> FusedIterator for SliceUntil<I, P>
where
    I: FusedIterator,
    P: FnMut(&I::Item) -> bool,
{
}
