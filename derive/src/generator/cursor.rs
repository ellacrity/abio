use proc_macro2::extra::DelimSpan;
use proc_macro2::Delimiter;
use syn;
use syn::buffer::Cursor;

pub struct BytesCursor<'a> {
    pub(crate) inner: syn::buffer::Cursor<'a>,
}

impl<'a> BytesCursor<'a> {
    pub fn empty() -> Self {
        Self { inner: syn::buffer::Cursor::empty() }
    }

    /// Checks whether the cursor is currently pointing at the end of its valid
    /// scope.
    pub fn eof(self) -> bool {
        self.inner.eof()
    }

    pub fn group(mut self, delim: Delimiter) -> Option<(Cursor<'a>, DelimSpan, Cursor<'a>)> {
        self.inner.group(delim)
    }
}
