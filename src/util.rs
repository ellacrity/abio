use crate::Abi;

#[macro_export]
macro_rules! array {
    ($(&)? $bytes:ident of $ty:ty) => {{
        unsafe {
            let data = $bytes.as_ptr();
            let bytes = ::core::slice::from_raw_parts(data, ::core::mem::size_of::<$ty>());
            let ptr = bytes.as_ptr().cast::<[u8; ::core::mem::size_of::<$ty>()]>();
            assert!(!ptr.is_null());
            ptr.read()
        }
    }};
    ($(&)? $bytes:ident read $size:expr) => {{
        unsafe {
            let data = $bytes.as_ptr();
            let bytes = ::core::slice::from_raw_parts(data, $size);
            let ptr = bytes.as_ptr().cast::<[u8; $size]>();
            ptr.read()
        }
    }};
    ($(&)? $bytes:ident with $size:expr) => {{
        let array_slice = unsafe {
            let data = <*const _>::from($bytes).cast::<u8>();
            assert_eq!(::core::mem::size_of_val($bytes), $size);
            ::core::slice::from_raw_parts(data, $size)
        };
        <[u8; $size]>::try_from(array_slice)
    }};
}

#[macro_export]
macro_rules! bytes_of {
    ($name:ident with $size:literal) => {{
        let data = ($name as *const Self).cast::<u8>();
        assert_eq!(::core::size_of_val($name), $size);
        ::core::slice::from_raw_parts(data, $size)
    }};
    ($name:ident) => {{
        let data = ($name as *const Self).cast::<u8>();
        let size = ::core::mem::size_of::<$name>();
        assert_eq!(::core::size_of_val($name), size);
        ::core::slice::from_raw_parts(data, size)
    }};
}

pub const unsafe fn try_cast_bytes<T: Abi, const SIZE: usize>(bytes: &[u8]) -> [u8; SIZE] {
    bytes.as_ptr().cast::<[u8; SIZE]>().read()
}

/// Converts a dynamically-sized slice of bytes to a fixed size array of bytes.
///
/// Returns an error `bytes.len() != SIZE`.
#[inline]
#[allow(dead_code)]
pub const unsafe fn to_byte_array<const SIZE: usize>(bytes: &[u8]) -> [u8; SIZE] {
    // debug_assert!(bytes.len() >= SIZE);
    // bytes.as_ptr().cast::<[u8; SIZE]>().read()
    array!(bytes with SIZE)
}

#[inline]
pub fn into_byte_array<const SIZE: usize>(bytes: &[u8]) -> Option<[u8; SIZE]> {
    match <[u8; SIZE]>::try_from(bytes) {
        Ok(array) => Some(array),
        Err(..) => None,
    }
}

#[doc(hidden)]
pub trait IntoInner<T> {
    fn into_inner(self) -> T;
}

#[doc(hidden)]
pub trait AsInner<T: ?Sized> {
    fn as_inner(&self) -> &T;
}

#[doc(hidden)]
pub trait AsInnerMut<T: ?Sized> {
    fn as_inner_mut(&mut self) -> &mut T;
}

#[doc(hidden)]
pub trait FromInner<T: ?Sized> {
    fn from_inner(inner: T) -> Self;
}

#[doc(hidden)]
pub trait FromInnerMut<T: ?Sized> {
    fn from_inner_mut(inner: &mut T) -> &mut Self;
}

#[doc(hidden)]
pub trait FromInnerRef<T: ?Sized> {
    fn from_inner_ref(inner: &T) -> &Self;
}
