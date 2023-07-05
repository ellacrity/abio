use core::ops::Range;
use core::slice;

mod aligned_bytes;
pub use aligned_bytes::AlignedBytes;

mod cursor;
pub use cursor::BytePos;

mod endian;
pub use endian::{BigEndian, Endianness, LittleEndian, NativeEndian, NetworkEndian, BE, LE};

mod reader;
pub use reader::{BytesExt, Parser};

use crate::util::{FromInner, IntoInner};
use crate::Zeroable;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bytes<'a>(&'a [u8]);

impl<'a> Bytes<'a> {
    /// Creates a new [Bytes] instance from a slice of bytes.
    pub const fn new(bytes: &'a [u8]) -> Bytes<'a> {
        Bytes::_new(bytes, 0)
    }

    /// Creates a new [Bytes] instance from a given slice of bytes and offset.
    ///
    /// # Panics
    ///
    /// Panics if `bytes.len() < offset`.
    pub const fn new_with_offset(bytes: &'a [u8], offset: usize) -> Bytes<'a> {
        Bytes::_new(bytes, offset)
    }

    // Private constructor responsible for enforcing any invariants.
    // FIXME: Introduce support for ZST's
    // TODO: Consider creating a constructor that returns `Option<Bytes<'a>>`
    #[inline]
    const fn _new(bytes: &'a [u8], offset: usize) -> Bytes<'a> {
        debug_assert!(!bytes.is_empty(), "ZST's are currently unsupported for this type.");
        assert!(
            bytes.len() >= offset,
            "Cannot construct Bytes instance with `offset > bytes.len()`"
        );
        // "Cannot construct Bytes instance with `bytes.len() < offset`."
        Bytes(unsafe {
            slice::from_raw_parts(bytes.as_ptr().add(offset).cast::<u8>(), bytes.len())
        })
    }

    /// Returns the inner byte slice of this [Bytes] instance.
    #[inline]
    pub const fn chunk(&self) -> &'a [u8] {
        self.0
    }

    pub const fn as_ptr_range(&self) -> Range<*const u8> {
        self.0.as_ptr_range()
    }

    pub fn iter(&self) -> slice::Iter<'a, u8> {
        self.0.iter()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

unsafe impl<'a> Zeroable for Bytes<'a> {
    unsafe fn zero() -> Self {
        core::mem::zeroed::<Self>()
    }
}

impl<'a> AsRef<[u8]> for Bytes<'a> {
    fn as_ref(&self) -> &[u8] {
        self.chunk()
    }
}

// impl<'a> Deref for Bytes<'a> {
//     type Target = [u8];

//     fn deref(&self) -> &Self::Target {
//         self.as_bytes()
//     }
// }

impl<'a> From<&'a [u8]> for Bytes<'a> {
    fn from(value: &'a [u8]) -> Bytes<'a> {
        Bytes::new(value)
    }
}

impl<'a> FromInner<&'a [u8]> for Bytes<'a> {
    fn from_inner(inner: &'a [u8]) -> Bytes<'a> {
        Bytes::from(inner)
    }
}
impl<'a> IntoInner<&'a [u8]> for Bytes<'a> {
    fn into_inner(self) -> &'a [u8] {
        self.0
    }
}
