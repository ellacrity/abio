//! Cursor primitives for efficiently traversing buffers.

use core::ops::Index;

use crate::Bytes;

// TODO: Add essential arithmetic trait impls for `BytePos`.
macro_rules! impl_arithmetic_for_type {
    ($($ty:ty),*) => {
        $(
            impl ::core::ops::Add for $ty {
                type Output = $ty;

                fn add(self, other: $ty) -> Self::Output {
                    <$ty>::new(self.0 + other.0)
                }
            }
            impl ::core::ops::AddAssign for $ty {
                fn add_assign(&mut self, other: $ty) {
                    $ty::advance(self, other.get());
                }
            }
        )*
    };
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BytePos {
    value: usize,
}

impl BytePos {
    pub const fn new(value: usize) -> BytePos {
        BytePos { value }
    }

    pub const fn get(&self) -> usize {
        self.value
    }

    pub fn get_mut(&mut self) -> &mut usize {
        &mut self.value
    }

    pub fn advance(&mut self, count: usize) {
        self.value += count;
    }

    pub const fn saturating_add(self, count: usize) -> BytePos {
        let new_pos = self.value.saturating_add(count);
        BytePos { value: new_pos }
    }
}

impl ::core::ops::Add for BytePos {
    type Output = BytePos;

    fn add(self, other: BytePos) -> Self::Output {
        BytePos::new(self.get() + other.get())
    }
}
impl ::core::ops::AddAssign<BytePos> for BytePos {
    fn add_assign(&mut self, other: BytePos) {
        BytePos::advance(self, other.get());
    }
}
impl ::core::ops::AddAssign<usize> for BytePos {
    fn add_assign(&mut self, other: usize) {
        BytePos::advance(self, other);
    }
}
impl ::core::ops::AddAssign<BytePos> for usize {
    fn add_assign(&mut self, other: BytePos) {
        *self = *self + other.get();
    }
}
impl ::core::ops::Sub for BytePos {
    type Output = BytePos;

    fn sub(self, rhs: Self) -> Self::Output {
        BytePos::from(self.get().saturating_sub(rhs.get()))
    }
}

impl ::core::ops::Add<usize> for BytePos {
    type Output = usize;

    fn add(self, rhs: usize) -> Self::Output {
        self.value + rhs
    }
}
impl ::core::ops::Add<BytePos> for usize {
    type Output = BytePos;

    fn add(self, rhs: BytePos) -> Self::Output {
        BytePos::new(self + rhs.get())
    }
}

impl From<usize> for BytePos {
    fn from(value: usize) -> Self {
        BytePos { value }
    }
}
