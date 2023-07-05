use core::{array, slice};

// FIXME: Implement derive macros to implement aligned integral types (ella)
mod unsigned;
pub use unsigned::*;

// FIXME: Implement derive macros to implement aligned integral types (ella)
mod signed;
pub use signed::*;

use crate::{AsBytes, FromBytes, Pod, Zeroable};

unsafe impl AsBytes for u8 {
    fn as_bytes(&self) -> &[u8] {
        array::from_ref(self)
    }
}
unsafe impl FromBytes for u8 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        assert_preconditions!(bytes, Self);
        if bytes.is_empty() || bytes.len() < Self::SIZE {
            None
        } else {
            let bytes: [u8; core::mem::size_of::<Self>()] = bytes[..Self::SIZE]
                .try_into()
                .expect("infallible operation occurred converting bytes to array");
            Some(Self::from_le_bytes(bytes))
        }
    }
}
unsafe impl Zeroable for u8 {}
unsafe impl Pod for u8 {}

unsafe impl AsBytes for u32 {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            let len = core::mem::size_of_val(self);
            slice::from_raw_parts(self as *const Self as *const u8, len)
        }
    }
}
unsafe impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        assert_preconditions!(bytes, Self);
        if bytes.is_empty() || bytes.len() < Self::SIZE {
            None
        } else {
            let bytes: [u8; core::mem::size_of::<Self>()] = bytes[..Self::SIZE]
                .try_into()
                .expect("infallible operation occurred converting bytes to array");
            Some(Self::from_le_bytes(bytes))
        }
    }
}
unsafe impl Zeroable for u32 {}
unsafe impl Pod for u32 {}

unsafe impl AsBytes for U16 {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            let len = core::mem::size_of_val(self);
            slice::from_raw_parts(self.get() as *const u16 as *const u8, len)
        }
    }
}
unsafe impl FromBytes for U16 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        assert_preconditions!(bytes, Self);
        if bytes.is_empty() || bytes.len() < Self::SIZE {
            None
        } else {
            let bytes: [u8; core::mem::size_of::<Self>()] = bytes[..Self::SIZE]
                .try_into()
                .expect("infallible operation occurred converting bytes to array");
            Some(Self::from_le_bytes(bytes))
        }
    }
}
unsafe impl Zeroable for U16 {}
unsafe impl Pod for U16 {}

unsafe impl AsBytes for U32 {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            let len = core::mem::size_of_val(self);
            slice::from_raw_parts(self.get() as *const u32 as *const u8, len)
        }
    }
}
unsafe impl FromBytes for U32 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        assert_preconditions!(bytes, Self);
        if bytes.is_empty() || bytes.len() < Self::SIZE {
            None
        } else {
            let bytes: [u8; core::mem::size_of::<Self>()] = bytes[..Self::SIZE]
                .try_into()
                .expect("infallible operation occurred converting bytes to array");
            Some(Self::from_le_bytes(bytes))
        }
    }
}
unsafe impl Zeroable for U32 {}
unsafe impl Pod for U32 {}
