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

mod bench_type_erased_vec;
mod bench_type_projected_vec;

use criterion::criterion_main;

criterion_main!(
    bench_type_erased_vec::bench_as_slice_index,
    bench_type_erased_vec::bench_get,
    bench_type_erased_vec::bench_pop,
    bench_type_erased_vec::bench_push,
    bench_type_erased_vec::bench_replace_insert,
    bench_type_erased_vec::bench_shift_insert,
    bench_type_erased_vec::bench_shift_remove,
    bench_type_erased_vec::bench_swap_remove,
    bench_type_projected_vec::bench_as_slice_index,
    bench_type_projected_vec::bench_get,
    bench_type_projected_vec::bench_pop,
    bench_type_projected_vec::bench_push,
    bench_type_projected_vec::bench_replace_insert,
    bench_type_projected_vec::bench_shift_insert,
    bench_type_projected_vec::bench_shift_remove,
    bench_type_projected_vec::bench_swap_remove,
);
