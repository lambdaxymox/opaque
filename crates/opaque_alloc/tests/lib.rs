#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![feature(slice_ptr_get)]
#![cfg_attr(
    feature = "nightly",
    feature(
        allocator_api,
    )
)]
#![no_std]
extern crate alloc as alloc_crate;

mod test_opaque_alloc;
mod test_typed_proj_alloc;
mod test_type_safety;
