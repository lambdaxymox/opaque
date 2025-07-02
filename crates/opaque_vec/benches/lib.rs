#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(
    feature = "nightly",
    feature(
        allocator_api,
        slice_ptr_get,
    )
)]
#![no_std]
extern crate alloc as alloc_crate;

mod bench_opaque_vec;
mod bench_typed_proj_vec;

use criterion::criterion_main;

criterion_main!(
    bench_opaque_vec::bench_as_slice_index,
    bench_opaque_vec::bench_get,
    bench_opaque_vec::bench_push,
    bench_opaque_vec::bench_replace_insert,
    bench_opaque_vec::bench_shift_insert,
    bench_opaque_vec::bench_shift_remove,
    bench_opaque_vec::bench_swap_remove,
    bench_typed_proj_vec::bench_as_slice_index,
    bench_typed_proj_vec::bench_get,
    bench_typed_proj_vec::bench_push,
    bench_typed_proj_vec::bench_replace_insert,
    bench_typed_proj_vec::bench_shift_insert,
    bench_typed_proj_vec::bench_shift_remove,
    bench_typed_proj_vec::bench_swap_remove,
);
