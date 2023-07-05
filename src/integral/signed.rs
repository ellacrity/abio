//! Aligned, endian-aware __signed__ integral types.

use core::{mem, slice};

use crate::{AsBytes, FromBytes, Pod, Zeroable};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct I8(i8);

impl I8 {
    pub const fn new(value: i8) -> I8 {
        I8(value.to_le())
    }

    pub const fn from_ptr(ptr: *const u8) -> I8 {
        I8::new(unsafe { ptr.cast::<i8>().read() })
    }

    pub const fn get(&self) -> i8 {
        self.0
    }
}

unsafe impl AsBytes for I8 {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.get() as *const i8 as *const u8,
                core::mem::size_of_val(self),
            )
        }
    }
}

unsafe impl FromBytes for I8 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        assert_preconditions!(bytes, Self);
        if bytes.is_empty() || bytes.len() < Self::SIZE {
            None
        } else {
            let bytes: [u8; 1] = bytes[..Self::SIZE]
                .try_into()
                .expect("infallible operation occurred converting bytes to array");
            Some(I8::new(i8::from_le_bytes(bytes)))
        }
    }
}

unsafe impl Pod for I8 {}
unsafe impl Zeroable for I8 {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct I16(i16);

impl I16 {
    pub const fn new(value: i16) -> I16 {
        I16(value.to_le())
    }

    pub const fn from_ptr(ptr: *const u8) -> I16 {
        let mut pos = 0;

        I16::new(unsafe { ptr.cast::<i16>().read() })
    }

    pub const fn get(&self) -> i16 {
        self.0
    }
}

unsafe impl AsBytes for I16 {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            // NOTE: This function does not have a Self: Sized bound.
            // size_of_val works for unsized values too.
            let len = mem::size_of_val(self);
            slice::from_raw_parts(self as *const Self as *const u8, len)
        }
    }
}
unsafe impl FromBytes for I16 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        assert_preconditions!(bytes, Self);
        if bytes.is_empty() || bytes.len() < Self::SIZE {
            None
        } else {
            let bytes: [u8; 2] = bytes[..Self::SIZE]
                .try_into()
                .expect("infallible operation occurred converting bytes to array");
            Some(I16::new(i16::from_le_bytes(bytes)))
        }
    }
}
unsafe impl Pod for I16 {}
unsafe impl Zeroable for I16 {}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct I32(i32);
impl I32 {
    pub const fn new(value: i32) -> I32 {
        I32(value.to_le())
    }

    pub const fn from_ptr(ptr: *const u8) -> I32 {
        let mut pos = 0;

        I32::new(unsafe { ptr.cast::<i32>().read() })
    }

    pub const fn get(&self) -> i32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct I64(i64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct I128(i128);
