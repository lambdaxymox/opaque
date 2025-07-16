#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(feature = "nightly", feature(optimize_attribute,))]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

mod build_hasher;
mod build_hasher_inner;
mod hasher;
mod hasher_inner;
mod try_project_build_hasher_error;
mod try_project_hasher_error;

pub use crate::build_hasher::*;
pub use crate::hasher::*;
