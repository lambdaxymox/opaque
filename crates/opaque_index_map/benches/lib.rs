#![feature(allocator_api)]
mod bench_opaque_index_map;
mod bench_typed_proj_index_map;

use criterion::criterion_main;

criterion_main!(
    bench_opaque_index_map::bench_as_slice_index,
    bench_opaque_index_map::bench_get,
    bench_opaque_index_map::bench_get_full,
    bench_opaque_index_map::bench_get_index_of,
    bench_opaque_index_map::bench_get_key_value,
    bench_opaque_index_map::bench_insert,
    bench_opaque_index_map::bench_insert_full,
    bench_opaque_index_map::bench_shift_insert,
    bench_opaque_index_map::bench_shift_remove,
    bench_opaque_index_map::bench_swap_remove,
    bench_typed_proj_index_map::bench_as_slice_index,
    bench_typed_proj_index_map::bench_get,
    bench_typed_proj_index_map::bench_get_full,
    bench_typed_proj_index_map::bench_get_index_of,
    bench_typed_proj_index_map::bench_get_key_value,
    bench_typed_proj_index_map::bench_insert,
    bench_typed_proj_index_map::bench_insert_full,
    bench_typed_proj_index_map::bench_shift_insert,
    bench_typed_proj_index_map::bench_shift_remove,
    bench_typed_proj_index_map::bench_swap_remove,
);

