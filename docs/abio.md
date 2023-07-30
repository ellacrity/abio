# abio

[`abio`][abio] is a low-level crate for performing endian-aware operations
on raw byte slices, converting to and from concrete types using zero-copy
serialization and deserialization routines. 

You can find more information about this crate's API within the official [crate documentation][crate-docs].

## Design / Purpose

This crate is an attempt at implementing ["safe transmute"][safe-transmute].

### Sans-I/O Operations

This crate provides its own custom [`ABI`][ABI] by enforcing a set of contracts
via traits. operates directly on binary data, or bytes. There is no actual I/O
performed at any point within the codebase. This design approach is heavily
inspired by is that it allows decoupling of the reading and writing logic from
the I/O itself. To be more specific, this crate is agnostic with regard to where
the bytes originate from.

### Predictable Type Layout

When you use the standard Rust ABI without specifying a `#[repr(..)]` attribute, the Rust compiler has the freedom to lay out your data structures in memory however it deems optimal. This can mean that the layout might change between different compiler versions, optimization levels, or even different compilation targets. As a result, the standard Rust ABI does not provide any guarantees about the memory layout of your types.

This does not necessarily mean that the layout will change or be non-deterministic, but rather that it's not something you should rely on if you need to have a stable memory layout.

If you need to read and write raw bytes from/to your data structures, and you need this to work reliably and soundly, then you should use one of the `#[repr(..)]` attributes to specify the layout of your data structures. For example, `#[repr(C)]` is commonly used for this purpose, because it specifies that your data structure should have the same memory layout as it would in C, which is a stable, well-defined layout.

### Soundness

Creating code that isn't grounded on an [ABI (Application Binary Interface)][ABI] with a predictable layout can easily introduce unsoundness. This crate, however, prioritizes safety over performance when faced with a trade-off, as high performance is rendered insignificant in the absence of sound and safe code. Therefore, it aims to strike an optimal balance between these two factors, which is arguably the most pragmatic approach.


[abio]: https://github.com/ellacrity/abio
[crate-docs]: https://docs.rs/abio/latest/src/abio
[safe-transmute]: https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html