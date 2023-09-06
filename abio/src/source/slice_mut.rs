use core::marker::PhantomData;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BytesMut<'data> {
    ptr: *mut u8,
    end: *mut u8,
    _lifetime: PhantomData<&'data u8>,
}

impl<'data> BytesMut<'data> {
    pub fn new(bytes: &'data mut [u8]) -> BytesMut<'data> {
        Self {
            ptr: bytes.as_ptr(),
            end: unsafe { bytes.as_ptr().add(bytes.len()) },
            _lifetime: PhantomData,
        }
    }
}
