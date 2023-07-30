A trait representing valid memory types used as an input [Source] into the abstract machine as defined by this crate's [Application Binary Interface (ABI)][abi].

# I/O Operations
The Source trait offers low-level methods as a foundation for higher-level I/O operations. The methods provided here operate on arbitrary regions of memory represented as byte slices, and they do not conduct file-based or network I/O.

 # Endianness

Note that this trait does not inherently handle endianness. If data manipulation requires specific endianness, consider using the [Endian][endian] trait. It provides methods for reading bytes in both big and little endian byte order serialization.

# ABI Compatibility

The Source trait is used to mark input sources, such as byte slices, that can be processed by the system as ABI-compatible types. Types implementing Source can be interpreted as valid, untrusted input sources. This trait assists in populating transient buffers before passing the read bytes into a method from the [Decode] trait for additional processing.

A Source represents a valid, readable data source for the system, abstracting the concept of input. This allows input data to be validated upfront, parsed when needed, and quickly traversed for indexing operations.

# Safety

Implementing this trait for a type incompatible with [abio][crate]'s ABI might lead to unpredictable results and is consequently considered undefined behaviour. The caller is responsible for ensuring their input type satisfies the contract defined by this trait.

It is highly recommended to use the derive procedural macros offered by the [abio_derive][abio_derive] crate. These macros help conduct compile-time checks to minimize runtime errors.

[endian]: crate::layout::Endian
[abi]: https://en.wikipedia.org/wiki/Application_binary_interface