#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(feature = "nightly", feature(allocator_api, slice_ptr_get,))]
#![no_std]
extern crate alloc as alloc_crate;

mod test_type_erased_alloc;
mod test_type_projected_alloc;
mod test_type_safety;
