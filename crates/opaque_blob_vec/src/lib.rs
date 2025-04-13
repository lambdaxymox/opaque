#![feature(const_eval_select)]
#![feature(allocator_api)]
#![feature(structural_match)]
#![feature(alloc_layout_extra)]
#![feature(optimize_attribute)]
#![feature(slice_range)]
mod opaque_blob_vec;
mod opaque_vec_memory;
mod range_types;
mod unique;

pub use opaque_blob_vec::OpaqueBlobVec;
