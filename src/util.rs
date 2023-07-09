use core::marker::PhantomData;

/// Converts a dynamically-sized slice of bytes to a fixed size array of bytes.
///
/// Returns an error `bytes.len() != SIZE`.
pub const fn as_byte_array<const SIZE: usize>(bytes: &[u8]) -> crate::Result<[u8; SIZE]> {
    if bytes.len() != SIZE {
        Err(crate::Error::size_mismatch(SIZE, bytes.len()))
    } else {
        // SAFETY: The length of the input bytes matches the length of the output array.
        // Additionally,
        Ok(unsafe { bytes.as_ptr().cast::<[u8; SIZE]>().read() })
    }
}

/// Compile time check that should be an expected value.
pub trait Expected<const VALUE: bool> {}

pub struct HasPadding<T: ?Sized, const VALUE: bool>(PhantomData<T>);

impl<T: ?Sized, const VALUE: bool> Expected<VALUE> for HasPadding<T, VALUE> {}

/// Returns true iff the struct or union does not contain any padding bytes.
///
/// `$ts` is the list of the type of every field in `$t`. `$t` must be a
/// struct type, or else `struct_has_padding!`'s result may be meaningless.
///
/// Note that `struct_has_padding!`'s results are independent of `repr` since
/// they only consider the size of the type and the sizes of the fields.
/// Whatever the repr, the size of the type already takes into account any
/// padding that the compiler has decided to add. Structs with well-defined
/// representations (such as `repr(C)`) can use this macro to check for padding.
/// Note that while this may yield some consistent value for some `repr(Rust)`
/// structs, it is not guaranteed across platforms or compilations.
#[doc(hidden)]
#[macro_export]
macro_rules! is_without_padding {
    (union -> $container:ty, $($fields:ty),*) => {
        false $(|| ::core::mem::size_of::<$container>() != ::core::mem::size_of::<$fields>())*
    };
    (struct -> $container:ty, $($fields:ty),*) => {
        ::core::mem::size_of::<$container>() > 0 $(+ ::core::mem::size_of::<$fields>())*
    };
}

#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {{
        // Create an instance of the type
        let instance = unsafe { ::core::mem::zeroed::<$ty>() };

        // Get the address of the field
        let field_ptr: *const _ = &instance.$field;

        // Get the address of the instance
        let instance_ptr: *const _ = &instance;

        // Calculate the offset as the difference between the two addresses
        let offset_bytes = field_ptr as usize - instance_ptr as usize;

        offset_bytes
    }};
}

#[doc(hidden)]
pub trait FromInner<T> {
    fn from_inner(inner: T) -> Self;
}

#[doc(hidden)]
pub trait AsInner<T> {
    fn as_inner(&self) -> &T;
}

#[doc(hidden)]
pub trait AsInnerMut<T> {
    fn as_inner_mut(&mut self) -> &mut T;
}

#[doc(hidden)]
pub trait IntoInner<T> {
    fn into_inner(self) -> T;
}
