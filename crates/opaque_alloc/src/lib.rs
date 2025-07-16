#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(feature = "nightly", feature(optimize_attribute, allocator_api,))]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

mod alloc;
mod alloc_inner;
mod try_project_alloc_error;

pub use crate::alloc::*;
pub use crate::try_project_alloc_error::*;
