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

mod bench_type_erased_alloc;
mod bench_type_projected_alloc;

use criterion::criterion_main;

criterion_main!(
    bench_type_erased_alloc::bench_type_erased_alloc,
    bench_type_projected_alloc::bench_type_projected_alloc,
);
