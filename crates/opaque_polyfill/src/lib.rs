#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

pub mod alloc_layout_extra;
pub mod range_types;
pub mod slice_ptr_get;
pub mod slice_range;
