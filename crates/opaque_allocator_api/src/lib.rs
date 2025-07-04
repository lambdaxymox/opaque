//! This crate provides a polyfill for Rust’s unstable `allocator_api` feature, enabling support
//! for custom allocators on stable Rust and compatibility across Rust versions. This crate enables
//! library users to access the global allocator in particular, so that the type-projected and
//! type-erased data structures can be used in Rust stable.
//!
//! In particular, we need to specify the memory allocator to project a type-erased data structure
//! into a type-projected data structure, so this ensures we can use the global allocator in stable
//! by providing a polyfill implementation for it. In principle, the library user could write custom
//! allocators against the polyfill trait too, but that's not the primary use case.
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(not(feature = "nightly"))]
pub mod alloc;
