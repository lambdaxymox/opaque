#![deny(unsafe_op_in_unsafe_fn)]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub mod map;
pub mod set;
