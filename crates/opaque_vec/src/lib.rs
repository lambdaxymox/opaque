#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![feature(allocator_api)]
#![cfg_attr(
    feature = "nightly",
    feature(
        optimize_attribute,
        alloc_layout_extra,
        slice_range,
    )
)]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

mod raw_vec;
mod vec_inner;
mod zst;

mod into_iter;
mod drain;
mod splice;
mod extract_if;
mod vec;

pub use crate::into_iter::*;
pub use crate::drain::*;
pub use crate::splice::*;
pub use crate::extract_if::*;
pub use crate::vec::*;
