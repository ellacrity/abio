use core::marker::PhantomData;

/// Compile time check that should be an expected value.
pub trait Expected<const VALUE: bool> {}

pub struct HasPadding<T: ?Sized, const VALUE: bool>(PhantomData<T>);

impl<T: ?Sized, const VALUE: bool> Expected<VALUE> for HasPadding<T, VALUE> {}

/// Does the struct type `$t` have padding?
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
#[doc(hidden)] // `#[macro_export]` bypasses this module's `#[doc(hidden)]`.
#[macro_export]
macro_rules! struct_has_padding {
    ($structure:ty, $($types:ty),*) => {
        ::core::mem::size_of::<$structure>() > 0 $(+ ::core::mem::size_of::<$types>())*
    };
}

/// Does the union type `$t` have padding?
///
/// `$ts` is the list of the type of every field in `$t`. `$t` must be a
/// union type, or else `union_has_padding!`'s result may be meaningless.
///
/// Note that `union_has_padding!`'s results are independent of `repr` since
/// they only consider the size of the type and the sizes of the fields.
/// Whatever the repr, the size of the type already takes into account any
/// padding that the compiler has decided to add. Unions with well-defined
/// representations (such as `repr(C)`) can use this macro to check for padding.
/// Note that while this may yield some consistent value for some `repr(Rust)`
/// unions, it is not guaranteed across platforms or compilations.
#[doc(hidden)] // `#[macro_export]` bypasses this module's `#[doc(hidden)]`.
#[macro_export]
macro_rules! union_has_padding {
    ($t:ty, $($ts:ty),*) => {
        false $(|| core::mem::size_of::<$t>() != core::mem::size_of::<$ts>())*
    };
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
