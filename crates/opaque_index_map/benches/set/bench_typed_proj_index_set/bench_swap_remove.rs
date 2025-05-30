use criterion::{
    Criterion,
    criterion_group,
};
use indexmap::IndexSet;
use opaque_index_map::set::TypedProjIndexSet;

use std::hash;
use std::alloc;

fn bench_index_set_swap_remove(c: &mut Criterion) {
    let values = 0..1000;
    let base_set = IndexSet::<i32, hash::RandomState>::from_iter(values);
    let values_vec: Vec<i32> = base_set.iter().cloned().collect();

    c.bench_function("index_set_swap_remove", |b| {
        b.iter_batched(
            || base_set.clone(),
            |mut set| {
                for value in values_vec.iter() {
                    let _ = criterion::black_box(set.swap_remove(value));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_typed_proj_index_set_swap_remove(c: &mut Criterion) {
    let values = 0..1000;

    c.bench_function("typed_proj_index_set_swap_remove", |b| {
        b.iter_batched(
            || TypedProjIndexSet::<i32, hash::RandomState, alloc::Global>::from_iter(values.clone()),
            |mut proj_set| {
                let values_vec: Vec<i32> = proj_set.iter().cloned().collect();
                for value in values_vec.iter() {
                    let _ = criterion::black_box(proj_set.swap_remove(value));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_swap_remove, bench_typed_proj_index_set_swap_remove, bench_index_set_swap_remove);
