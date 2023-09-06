# Derive Macros

Writing the procedural macros to automatically derive `unsafe` traits is extremely challenging.
Coupling this with the fact that some of the main traits are also NOT `unsafe` makes it even more so.

## Problem

```rust
pub struct RawTypeLayout<T: ?Sized, const SIZE: usize> {
    inner: [u8; SIZE],
    marker: core::marker::PhantomData<T>,
}

pub trait Decode: Abi {
    fn decode(bytes: &[u8], offset: usize) -> Result<(Self, usize)>;
}

unsafe impl<T: TraitBound> TraitName<T> for SomeType<T> {

}
```
