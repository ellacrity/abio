# Traits

The various traits within the `layout` module help define a consistent and deterministic ABI for your
types to adhere to. Ensuring your types are compatible with the [`Abi`] trait is absolutely paramount.

## Abi

The [`Abi`][abi-trait] trait is the fundamental build block that this crate is built upon.

This trait is used to define what is, and reflexively, what is not ABI-compatible. Any type that implements [`abi`][abi-trait] can be decoded directly from a raw byte slice.

With that said, these restrictions might seem a bit intense. In order to provide users access to a safe API for intepreting bytes (a **very unsafe** operation), types must adhere to a strict set of rules that define its layout in memory. These same restrictions are what makes it possible to safely decode a slice of bytes into a concrete type.
