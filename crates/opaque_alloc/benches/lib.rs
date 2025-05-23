#![deny(unsafe_op_in_unsafe_fn)]
#![feature(allocator_api)]
#![feature(slice_ptr_get)]
mod bench_opaque_alloc;
mod bench_typed_proj_alloc;

use criterion::criterion_main;

criterion_main!(
    bench_opaque_alloc::bench_opaque_alloc,
    bench_typed_proj_alloc::bench_typed_proj_alloc,
);
