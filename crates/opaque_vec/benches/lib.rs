#![feature(allocator_api)]
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
