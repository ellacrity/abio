<h1 align="center" style="font-size:2.4em;color:#3BF7B8;letter-spacing:-1px">abio</h3>

<p align="center">
  Safely transmute raw byte slices directly to concrete types
</p>

## Purpose

With `abio`, you can execute endian-aware operations on raw byte slices without compromising performance. By adhering to a conventional C-like ABI, it facilitates straightforward conversions between bytes and concrete types. Whenever feasible, `abio` employs zero-copy routines for serialization and deserialization to minimize overhead and maximize efficiency.

### Zero-cost Abstractions

Leveraging "zero-cost abstractions", you can craft custom complex types while offloading checks and validations to compile time, sidestepping runtime penalties.

## Safe Transmute

This crate offers a novel approach to implementing ["safe transmute"][safe-transmute]. It builds upon existing solutions, while making different decisions regarding how attempting to unify them into a crate that provides users with an
This crate offers a novel approach to solving the "safe transmute" in Rust. Unlike existing solutions, it addresses the challenge of safe transmutation uniquely, building upon previous attempts

This crate provides a unique solution to ["safe transmute"][safe-transmute]. The challenge of safe transmutation in Rust hasn't been definitively solved, with various previous crates offering solutions, each with its own strengths and weaknesses. If you're unfamiliar with the significance or potential applications of this library, exploring these prior solutions would be a beneficial starting point.

[Official Documentation][crate-docs]

### Sans-I/O Operations

This crate provides its own custom [`ABI`][ABI] by enforcing a set of contracts
via traits. operates directly on binary data, or bytes. There is no actual I/O
performed at any point within the codebase.

This design approach is inspired by, and largely derived from [Sans I/O][sans-io]. Decoupling the I/O itself from the reader/writer interfaces allows implementing binary format parsers, network protocols, and other higher-level I/O abstractions without introducing the overhead associated with syscalls.This crate is agnostic with regard to where
the bytes originate from.

### Predictable Type Layout

When you use the standard Rust ABI without specifying a `#[repr(..)]` attribute, the Rust compiler has the freedom to lay out your data structures in memory however it deems optimal. This can mean that the layout might change between different compiler versions, optimization levels, or even different compilation targets. As a result, the standard Rust ABI does not provide any guarantees about the memory layout of your types.

This does not necessarily mean that the layout will change or be non-deterministic, but rather that it's not something you should rely on if you need to have a stable memory layout.

If you need to read and write raw bytes from/to your data structures, and you need this to work reliably and soundly, then you should use one of the `#[repr(..)]` attributes to specify the layout of your data structures. For example, `#[repr(C)]` is commonly used for this purpose, because it specifies that your data structure should have the same memory layout as it would in C, which is a stable, well-defined layout.

### Soundness

Creating code that isn't grounded on an [ABI (Application Binary Interface)][ABI] with a predictable layout can easily introduce unsoundness. This crate is significantly younger than other, similar crates, and there may be breaking changes
introduced due to a discovery of unsoundess. This crate aims to guarantee that if your code compiles, then it is sound.

### Is This Crate Right For You?

This crate does not perform any syscalls to open, read, flush and/or close files,
sockets, or other related operating system resources. Instead, this crate
provides useful primitives to build higher-level abstractions on top of, such as
file and network-based I/O. This crate can also be very useful for developing
binary file format parsers, custom network protocols, or working with FFI in a
safer, more controlled manner.

If you are looking for a higher-level crate with that type of behaviour, then this crate may not be for you.

[abio]: https://github.com/ellacrity/abio
[crate-docs]: https://docs.rs/abio/latest/abio
[safe-transmute]: https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html
[ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
