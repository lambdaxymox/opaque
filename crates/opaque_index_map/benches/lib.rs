#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![cfg_attr(
    feature = "nightly",
    feature(
        // slice_iter_mut_as_mut_slice,
        allocator_api,
    )
)]
#![no_std]
extern crate std;

mod map;
mod set;

use criterion::criterion_main;

criterion_main!(
    map::bench_type_erased_index_map::bench_as_slice_index,
    map::bench_type_erased_index_map::bench_get,
    map::bench_type_erased_index_map::bench_get_full,
    map::bench_type_erased_index_map::bench_get_index_of,
    map::bench_type_erased_index_map::bench_get_key_value,
    map::bench_type_erased_index_map::bench_insert,
    map::bench_type_erased_index_map::bench_insert_full,
    map::bench_type_erased_index_map::bench_shift_insert,
    map::bench_type_erased_index_map::bench_shift_remove,
    map::bench_type_erased_index_map::bench_swap_remove,
    map::bench_type_projected_index_map::bench_as_slice_index,
    map::bench_type_projected_index_map::bench_get,
    map::bench_type_projected_index_map::bench_get_full,
    map::bench_type_projected_index_map::bench_get_index_of,
    map::bench_type_projected_index_map::bench_get_key_value,
    map::bench_type_projected_index_map::bench_insert,
    map::bench_type_projected_index_map::bench_insert_full,
    map::bench_type_projected_index_map::bench_shift_insert,
    map::bench_type_projected_index_map::bench_shift_remove,
    map::bench_type_projected_index_map::bench_swap_remove,
    set::bench_type_projected_index_set::bench_as_slice_index,
    set::bench_type_erased_index_set::bench_as_slice_index,
    set::bench_type_erased_index_set::bench_get,
    set::bench_type_erased_index_set::bench_get_full,
    set::bench_type_erased_index_set::bench_get_index_of,
    set::bench_type_erased_index_set::bench_insert,
    set::bench_type_erased_index_set::bench_insert_full,
    set::bench_type_erased_index_set::bench_shift_insert,
    set::bench_type_erased_index_set::bench_shift_remove,
    set::bench_type_erased_index_set::bench_swap_remove,
    set::bench_type_projected_index_set::bench_get,
    set::bench_type_projected_index_set::bench_get_full,
    set::bench_type_projected_index_set::bench_get_index_of,
    set::bench_type_projected_index_set::bench_insert,
    set::bench_type_projected_index_set::bench_insert_full,
    set::bench_type_projected_index_set::bench_shift_insert,
    set::bench_type_projected_index_set::bench_shift_remove,
    set::bench_type_projected_index_set::bench_swap_remove,
);
