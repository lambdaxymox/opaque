use opaque_index_map::set::OpaqueIndexSet;

use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;

use std::hash;
use std::vec::Vec;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

fn bench_index_set_swap_remove(c: &mut Criterion) {
    let values = 0..1000;
    let map = IndexSet::<i32, hash::RandomState>::from_iter(values);
    let values_vec: Vec<i32> = map.iter().cloned().collect();

    c.bench_function("index_set_swap_remove", |b| {
        b.iter_batched(
            || map.clone(),
            |mut map| {
                for value in values_vec.iter() {
                    let _ = core::hint::black_box(map.swap_remove(value));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_opaque_index_set_swap_remove(c: &mut Criterion) {
    let values = 0..1000;

    c.bench_function("opaque_index_set_swap_remove", |b| {
        b.iter_batched(
            || OpaqueIndexSet::from_iter(values.clone()),
            |mut opaque_set| {
                let keys: Vec<i32> = opaque_set.iter::<i32, hash::RandomState, alloc::Global>().cloned().collect();
                for key in keys.iter() {
                    let _ = core::hint::black_box(opaque_set.swap_remove::<i32, i32, hash::RandomState, alloc::Global>(key));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_swap_remove, bench_opaque_index_set_swap_remove, bench_index_set_swap_remove);
