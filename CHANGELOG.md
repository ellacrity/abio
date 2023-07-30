# Changelog

- [Added]: for new features.
- [Changed]: for changes in existing functionality.
- [Deprecated]: for soon-to-be removed features.
- [Removed]: for now removed features.
- [Fixed]: for any bug fixes.
- [Security]: in case of vulnerabilities.


This document outlines the changes made to the codebase, including pending changes that have not yet been applied.

## [Unreleased]

## [0.3.1]

### Added

- 

### Changed

- Relaxed the trait bound on `BytesOf`
- Added `const fn` whenever possible to push as much validation to compile time as possible.
- `Decode` trait made more useful:
  -  Can handle endianness
  -  Can be configured to aset a limit on the maximum allowed buffered bytes
     -  This is techniquely a fix, since otherwise this could easily result in a DOS attack where the attacker could send an arbitrarily large input, causing the program to crash.

## [0.3.0] - 2023-07-20

### Added

- `Chunk` type for for working with byte arrays with a fixed size
  - The `Chunk` type is generic over its length, wrapping the `[u8; N]` type
- Introduced the `Span` type for indexing, slicing and general 
- Add declarative macros to auto-generate trait implementations
  - `aligned` integer types are defined via `macro_rules`
- Add `BytesOf` type for types that 

### Changed

- `BytesOf` trait is now called `BytesOf`


## [Unreleased]

[unreleased]: https://github.com/ellacrity/abio/compare/v1.1.1...HEAD
[1.1.1]: https://github.com/ellacrity/abio/compare/v1.1.0...v1.1.1
[1.1.0]: https://github.com/ellacrity/abio/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/ellacrity/abio/compare/v0.3.0...v1.0.0
[0.3.0]: https://github.com/ellacrity/abio/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ellacrity/abio/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ellacrity/abio/compare/v0.0.8...v0.1.0
[0.0.8]: https://github.com/ellacrity/abio/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/ellacrity/abio/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/ellacrity/abio/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/ellacrity/abio/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/ellacrity/abio/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/ellacrity/abio/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/ellacrity/abio/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/ellacrity/abio/releases/tag/v0.0.1

What is a changelog?

A changelog is a file which contains a curated, chronologically ordered list of notable changes for each version of a project.
Why keep 
