type MaybeNullInner<T> = Option<NonNull<T>>;

/// Pointer type that provides trait and type extensions Rust's raw pointer types.
///
/// # Usage
///
/// This type is extremely useful for FFI operations where a parameter or return value represents a pointer that may or may not be null. [`NonNull<T>`] by itself will outright reject null values, even transitively, as it may never represent a null value. By wrapping this type in an [`Option<T>`] instead, the type can easily represent both.
///
/// # Relationship to Null
///
/// Unlike [`NonNull<T>`], type type can represent `NULL` pointers due to its [`Option<T>`] wrapper. When the inner value is `None`, the pointer represents a null pointer. If the inner value is `Some(..)`, then the pointer represents a non-null pointer.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MaybeNull<T> {
    inner: MaybeNullInner<T>,
}

impl<T> MaybeNull<T> {
    pub fn new(inner: MaybeNullInner<T>) -> MaybeNull<T> {
        Self { inner }
    }
}

impl<T: Abi> MaybeNull<T> {
    #[inline]
    pub(crate) fn is_non_null(self) -> bool {
        self.inner.is_some()
    }

    #[inline]
    pub(crate) fn is_null(self) -> bool {
        self.inner.is_none()
    }
}

impl<T> Default<T> for MaybeNull {
    fn default() -> Option<T> {
        None
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ConstPointer<T: Abi> {
    inner: MaybeNull<T>,
}

impl<T: Abi> ConstPointer<T> {
    pub const fn new(inner: MaybeNull<T>) -> ConstPointer<T> {
        Self { inner }
    }

    pub fn as_maybe_null(&self) -> MaybeNull<T> {
        match self.inner.inner {
            Some(p) => todo!(),
            None => MaybeNull::<T>::null(),
        }
    }
}

impl<T: Abi> IntoInner for ConstPointer<T> {}

impl<T: Abi> Deref for ConstPointer<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        match self.inner {
            Some(ptr) => {
                // ptr is `Some` value, and non null, so not `0`
                if ptr.as_ptr().align_offset(T::MIN_ALIGN) != 0 {
                    return Option::<&T>::None;
                } else {
                    Some(unsafe { &*ptr.as_ptr() })
                }
            }
            None => return Option::<&T>::None,
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct MutPointer<T: Abi>(*mut T);
