#![deny(unsafe_op_in_unsafe_fn)]
#![feature(allocator_api)]
#![feature(slice_range)]
#![feature(slice_iter_mut_as_mut_slice)]
#![feature(optimize_attribute)]
mod equivalent;
mod range_ops;
mod index_map_inner;
mod index_map;

pub use crate::equivalent::*;
pub use crate::index_map::*;
