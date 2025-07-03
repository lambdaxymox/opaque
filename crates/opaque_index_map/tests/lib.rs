#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(
    feature = "nightly",
    feature(
        // slice_iter_mut_as_mut_slice,
        allocator_api,
    )
)]
#![no_std]
extern crate std;

mod map;
mod set;
