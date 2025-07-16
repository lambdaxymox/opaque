#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(
    feature = "nightly",
    feature(optimize_attribute, alloc_layout_extra, slice_range, allocator_api,)
)]
#![no_std]
extern crate alloc as alloc_crate;

mod raw_vec;
mod vec_inner;
mod zst;

mod drain;
mod extract_if;
mod into_iter;
mod splice;
mod try_project_vec_error;
mod vec;

pub use crate::drain::*;
pub use crate::extract_if::*;
pub use crate::into_iter::*;
pub use crate::splice::*;
pub use crate::try_project_vec_error::*;
pub use crate::vec::*;
