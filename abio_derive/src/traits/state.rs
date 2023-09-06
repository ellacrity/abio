//! Traits that contain state, such as values and type instances.
//!
//! # Marker Traits
//!
//! Marker trait implementations contain "pure" functions, associated constant
//! values, and methods (with or without the [`self`] receiver). They generally do
//! not have side effects, and their implementations can typically be `const`.
//!
//! # Stateful Traits
//!
//! Stateful trait implementations include functions, associated types, and methods
//! (with or without the [`self`] receiver) called with other parameters representing
//! external data and often causing side effects.
