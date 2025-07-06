#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(
    feature = "nightly",
    feature(
        optimize_attribute,
        slice_iter_mut_as_mut_slice,
        allocator_api,
    )
)]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

mod range_ops;
mod map_inner;
mod slice_eq;

pub mod equivalent;
pub mod get_disjoint_mut_error;
pub mod map;
pub mod set;
mod try_project_index_map_error;
mod try_project_index_set_error;

pub use crate::equivalent::Equivalent;
pub use crate::get_disjoint_mut_error::GetDisjointMutError;
pub use crate::map::{
    TypeProjectedIndexMap,
    TypeErasedIndexMap,
};
pub use crate::set::{
    TypeProjectedIndexSet,
    TypeErasedIndexSet,
};
pub use crate::try_project_index_map_error::{
    TryProjectIndexMapError,
    TryProjectIndexMapErrorKind,
};
pub use crate::try_project_index_set_error::{
    TryProjectIndexSetError,
    TryProjectIndexSetErrorKind,
};
