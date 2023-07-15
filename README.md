# [`abio`][crate-docs]

[`abio`][crate-docs] is a lightweight crate that provides safe abstractions for working with ABI-compatible data represented as raw bytes.

Additional information can be found within the official [crate documentation][crate-docs].

## Purpose

Rust does not yet have a stable ABI. We cannot rely on the ["default" ABI][repr-default] for stability, but it is possible to use existing memory representations to create an ABI that is consistent. This "ABI" this crate attempts to define uses traits to enforce contracts for types adhering to the ABI. This approach is not new and several existing projects work in a very similar manner.

## Goals

This crate is an attempt at implementing ["Safe Transmute"][safe-transmute].

- Expose safe, flexible and ergonomic APIs for manipulating arbitrary bytes/memory
  - Potential feature-gated escape hatches for edge cases
- Platform-agnostic operations for full, cross-platform support
  - Tier-1 targets are prioritized
  - Tier-2 and below support will be "best effort"
- First-class support for [`no-std`][no-std] environments
  - Add support for Rust's standard library via the `std` feature
- Use zero-copy decoding and encoding operations
- Avoid all unnecessary allocations
- Read-like operations use immutable, borrowed data to allow parallelization
- Define an ABI that maintains consistency with the host operating system `extern "system"` ABI
  - Note that this ABI may differ depending on your target platform

## Non-Goals

- Read/Write primitives
  - Higher-level abstractions like this will exist in another crate
- 

## Development Status

This project is under active development and seeking additional contributors. 

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `abio` by you, shall be licensed as MIT, without any additional terms or conditions.

Thanks for your help improving the project! We are so happy to have you! We have a contributing guide to help you get involved in the axum project.

## Licensing

This software is licensed under the [MIT license](/LICENSE).

<!-- External Links -->
[crate-docs]: https://docs.rs/abio/latest/abio/

[no-std]: https://doc.rust-lang.org/reference/names/preludes.html?highlight=no-std#the-no_std-attribute
[repr-default]: https://doc.rust-lang.org/reference/type-layout.html?highlight=layout#the-default-representation
[repr-c]: https://doc.rust-lang.org/reference/type-layout.html?highlight=layout#the-c-representation
[repr-transparent]: https://doc.rust-lang.org/reference/type-layout.html?highlight=layout#the-transparent-representation
[repr-primitives]: https://doc.rust-lang.org/reference/type-layout.html?highlight=layout#primitive-representations
