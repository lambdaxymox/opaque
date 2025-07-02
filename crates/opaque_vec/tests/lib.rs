#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(
    feature = "nightly",
    feature(
        optimize_attribute,
        alloc_layout_extra,
        slice_range,
        allocator_api,
    )
)]
#![no_std]
extern crate std;

mod common;
mod test_opaque_vec;
mod test_typed_proj_vec;
mod test_type_safety;
