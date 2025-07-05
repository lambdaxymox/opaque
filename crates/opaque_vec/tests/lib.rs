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
extern crate alloc as alloc_crate;

extern crate std;

mod common;
mod test_type_erased_vec;
mod test_type_projected_vec;
mod test_type_safety;
