#![deny(unsafe_op_in_unsafe_fn)]
#![feature(allocator_api)]
mod map;
mod set;

use criterion::criterion_main;

criterion_main!(
    map::bench_opaque_index_map::bench_as_slice_index,
    map::bench_opaque_index_map::bench_get,
    map::bench_opaque_index_map::bench_get_full,
    map::bench_opaque_index_map::bench_get_index_of,
    map::bench_opaque_index_map::bench_get_key_value,
    map::bench_opaque_index_map::bench_insert,
    map::bench_opaque_index_map::bench_insert_full,
    map::bench_opaque_index_map::bench_shift_insert,
    map::bench_opaque_index_map::bench_shift_remove,
    map::bench_opaque_index_map::bench_swap_remove,
    map::bench_typed_proj_index_map::bench_as_slice_index,
    map::bench_typed_proj_index_map::bench_get,
    map::bench_typed_proj_index_map::bench_get_full,
    map::bench_typed_proj_index_map::bench_get_index_of,
    map::bench_typed_proj_index_map::bench_get_key_value,
    map::bench_typed_proj_index_map::bench_insert,
    map::bench_typed_proj_index_map::bench_insert_full,
    map::bench_typed_proj_index_map::bench_shift_insert,
    map::bench_typed_proj_index_map::bench_shift_remove,
    map::bench_typed_proj_index_map::bench_swap_remove,
    set::bench_typed_proj_index_set::bench_as_slice_index,
    set::bench_opaque_index_set::bench_as_slice_index,
    set::bench_opaque_index_set::bench_get,
    set::bench_opaque_index_set::bench_get_full,
    set::bench_opaque_index_set::bench_get_index_of,
    set::bench_opaque_index_set::bench_insert,
    set::bench_opaque_index_set::bench_insert_full,
    set::bench_opaque_index_set::bench_shift_insert,
    set::bench_opaque_index_set::bench_shift_remove,
    set::bench_opaque_index_set::bench_swap_remove,
    set::bench_typed_proj_index_set::bench_get,
    set::bench_typed_proj_index_set::bench_get_full,
    set::bench_typed_proj_index_set::bench_get_index_of,
    set::bench_typed_proj_index_set::bench_insert,
    set::bench_typed_proj_index_set::bench_insert_full,
    set::bench_typed_proj_index_set::bench_shift_insert,
    set::bench_typed_proj_index_set::bench_shift_remove,
    set::bench_typed_proj_index_set::bench_swap_remove,
);
