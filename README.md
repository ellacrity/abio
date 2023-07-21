# [`abio`][crate-docs]

[`abio`][crate-docs] is a lightweight crate that provides safe abstractions for working with ABI-compatible data represented as raw bytes.

Additional information can be found within the official [crate documentation][crate-docs].

## Development Status

This project is under active development. We are actively seeking contributors, particularly those that can assist with writing and maintaining documentation.

## Purpose

Rust does not yet have a stable ABI. We cannot rely on the <u>["default" ABI][repr-default]</u> for stability, but it is possible to use existing memory representations to create an ABI that is consistent. This "ABI" this crate attempts to define uses traits to enforce contracts for types adhering to the ABI. This approach is not new and several existing projects work in a very similar manner.

## Goals

This crate is an attempt at implementing <b><u>["Safe Transmute"][safe-transmute]</u></b>.

- Expose safe, flexible and ergonomic APIs for manipulating bytes
  - Add escape hatches behind feature gates to allow `unsafe` operations
- Platform-agnostic operations for full, cross-platform support
  - This crate does not contain any actual FFI bindings
  - [`build.rs`](/build.rs) script target features, such as [`simd``][simd-feature] support and [__`atomics`__][atomics-feature]
- Runtime decoding of types to support endian-aware read/write operations
- Full support for [`no-std`][no-std] environments
  - Add support for Rust's standard library via the `std` feature
- Use zero-copy decoding and encoding operations
- Avoid unnecessary allocations
- Read-like operations use immutable, borrowed data to allow parallelization
- Define an ABI that maintains consistency with the host operating system `extern "system"` ABI
  - Note that this ABI may differ depending on your target platform

## Non-Goals

- Support for File or Socket I/O
  - Read/Write primitives will be provided in a higher-level crate
- 
  - Higher-level abstractions like this will exist in another crate
- 


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
<!-- TODO: Add correct links -->
[atomics-feature]: https://doc.rust-lang.org/
[simd-feature]: https://doc.rust-lang.org/

<!-- Unstable / RFC's -->
[safe-transmute]: https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html