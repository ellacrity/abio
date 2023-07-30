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

### Project Goals

- Performance on-par with similar crates, such as [`bytemuck`][bytemuck], [`scroll`][scroll] and [`zerocopy`][zerocopy]
- Expose safe, flexible and ergonomic APIs for manipulating bytes
  - Escape hatches may be provided in the future via the `shims` module.
- Evaluate as much as possible at compile time using `const` functions and evaluations
  - Types are verified at compile time when using the `derive` traits
  - Take advantage of compiler optimizations on free constants so no runtime costs are incurred
- Fully platform-agnostic for all Tier-1 targets. Tier-2 targets will be supported if possible, along with Tier-3, in that order.
- Support [`simd`][simd-feature] and [`atomics`][atomics-feature] for all eligible platforms
- Runtime decoding and encoding of types using endian-aware subroutines
- First-class support for [`no-std`][no-std] environments
  - Store data on the stack as opposed to the heap, which may not be accessible in some environments
- Use zero-copy decoding and encoding operations
- Avoid unnecessary allocations
- Read-like operations use immutable, borrowed data to allow parallelization
- Define a consistent ABI that resembles the host operating system "system" ABI, which is typically just "C".
- **TBD**

### Project Non-Goals

- I/O support (Files, Sockets, Handles, etc.)
  - Support for working with files, sockets and other resources is outside the scope of this crate
- Addition of FFI bindings or platform-specific implementations
  - `#[cfg(..)]` conditional compilation is perfectly okay, but this library is not meant to be run on any particular operating system exclusively
- **TBD**

### Is This Crate Right For You?

This crate does not perform any syscalls to open, read, flush and/or close files,
sockets, or other related operating system resources. Instead, this crate
provides useful primitives to build higher-level abstractions on top of, such as
file and network-based I/O. This crate can also be very useful for developing
binary file format parsers, custom network protocols, or working with FFI in a 
safer, more controlled manner. 

If you are looking for a higher-level crate with that type of behaviour, then this crate may not be for you.

## Development Status

This project is under active development. 

### Seeking Contributors

We are **actively seeking contributors**, particularly those that can assist with **writing and maintaining documentation**. Properly documenting a crate takes a lot of effort and time, taking that time away from tasks such as implementing new features, writing new tests, adding benchmarks, etc.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `abio` by you, shall be licensed as [MIT](#licensing), without any additional terms or conditions.

## Licensing 

This software is licensed under the [MIT license](/LICENSE).

<!-- Official Documentation -->
[crate-docs]: https://docs.rs/abio/latest/abio/

<!-- Rust Reference Links -->
[no-std]: https://doc.rust-lang.org/reference/names/preludes.html?highlight=no-std#the-no_std-attribute
[repr-default]: https://doc.rust-lang.org/reference/type-layout.html?highlight=layout#the-default-representation
[repr-c]: https://doc.rust-lang.org/reference/type-layout.html?highlight=layout#the-c-representation
[repr-transparent]: https://doc.rust-lang.org/reference/type-layout.html?highlight=layout#the-transparent-representation
[repr-primitives]: https://doc.rust-lang.org/reference/type-layout.html?highlight=layout#primitive-representations

<!-- Target features -->
[atomics-feature]: https://doc.rust-lang.org/nomicon/atomics.html
[simd-feature]: https://github.com/rust-lang/portable-simd/tree/master

<!-- Similar crates -->
[bincode]: https://crates.io/crates/bincode
[bytemuck]: https://docs.rs/bytemuck/latest/bytemuck
[dataview]: https://docs.rs/dataview/latest/dataview
[scroll]: https://docs.rs/scroll/latest/scroll
[zerocopy]: https://docs.rs/zerocopy/latest/zerocopy

<!-- Crate -->
[abio]: https://docs.rs/abio/latest/src/abio

[ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
[safe-transmute]: https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html