#![feature(allocator_api)]
mod bench_opaque_blob_vec;
mod bench_typed_proj_blob_vec;

use criterion::criterion_main;

criterion_main!(
    bench_opaque_blob_vec::bench_get_unchecked,
    bench_opaque_blob_vec::bench_push,
    bench_opaque_blob_vec::bench_replace_insert,
    bench_opaque_blob_vec::bench_shift_insert,
    bench_opaque_blob_vec::bench_shift_remove_forget_unchecked,
    bench_opaque_blob_vec::bench_swap_remove_forget_unchecked,
    bench_typed_proj_blob_vec::bench_get_unchecked,
    bench_typed_proj_blob_vec::bench_push,
    bench_typed_proj_blob_vec::bench_replace_insert,
    bench_typed_proj_blob_vec::bench_shift_insert,
    bench_typed_proj_blob_vec::bench_shift_remove_forget_unchecked,
    bench_typed_proj_blob_vec::bench_swap_remove_forget_unchecked,
);
