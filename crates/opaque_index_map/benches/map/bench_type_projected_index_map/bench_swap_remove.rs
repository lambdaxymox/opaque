use opaque_index_map::map::TypeProjectedIndexMap;

use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexMap;

use std::hash;
use std::vec::Vec;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

fn bench_index_map_swap_remove(c: &mut Criterion) {
    let keys = 0..1000;
    let values = 1..1001;
    let map = IndexMap::<i32, i32, hash::RandomState>::from_iter(keys.zip(values));
    let keys: Vec<i32> = map.keys().cloned().collect();

    c.bench_function("index_map_swap_remove", |b| {
        b.iter_batched(
            || map.clone(),
            |mut map| {
                for key in keys.iter() {
                    let _ = core::hint::black_box(map.swap_remove(key));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_type_projected_index_map_swap_remove(c: &mut Criterion) {
    let keys = 0..1000;
    let values = 1..1001;

    c.bench_function("type_projected_index_map_swap_remove", |b| {
        b.iter_batched(
            || TypeProjectedIndexMap::<i32, i32, hash::RandomState, alloc::Global>::from_iter(keys.clone().zip(values.clone())),
            |mut proj_map| {
                let keys: Vec<i32> = proj_map.keys().cloned().collect();
                for key in keys.iter() {
                    let _ = core::hint::black_box(proj_map.swap_remove(key));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(
    bench_swap_remove,
    bench_type_projected_index_map_swap_remove,
    bench_index_map_swap_remove
);
