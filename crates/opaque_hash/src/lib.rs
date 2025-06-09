#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![feature(optimize_attribute)]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

mod hasher_inner;
mod build_hasher_inner;
mod hasher;
mod build_hasher;

pub use crate::hasher::*;
pub use crate::build_hasher::*;
