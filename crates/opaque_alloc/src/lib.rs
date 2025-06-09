#![deny(unsafe_op_in_unsafe_fn)]
#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(optimize_attribute)]
#![no_std]
mod alloc_inner;
mod alloc;

extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub use crate::alloc::*;
