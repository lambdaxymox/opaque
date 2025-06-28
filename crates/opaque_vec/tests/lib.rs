#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![feature(allocator_api)]
extern crate core;

mod common;
mod test_opaque_vec;
mod test_typed_proj_vec;
mod test_type_safety;
