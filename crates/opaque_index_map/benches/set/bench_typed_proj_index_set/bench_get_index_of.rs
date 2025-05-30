use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;
use opaque_index_map::set::TypedProjIndexSet;

use std::hash;
use std::alloc;

fn bench_index_map_get_index_of(c: &mut Criterion) {
    let values = 0..1000;
    let set = IndexSet::<i32, hash::RandomState>::from_iter(values);

    c.bench_function("index_map_get_index_of", |b| {
        b.iter(|| {
            for value in set.iter() {
                let _ = criterion::black_box(set.get_index_of(value));
            }
        });
    });
}

fn bench_typed_proj_index_map_get_index_of(c: &mut Criterion) {
    let values = 0..1000;
    let proj_set = TypedProjIndexSet::<i32, hash::RandomState, alloc::Global>::from_iter(values);

    c.bench_function("typed_proj_index_map_get_index_of", |b| {
        b.iter(|| {
            for value in proj_set.iter() {
                let _ = criterion::black_box(proj_set.get_index_of(value));
            }
        });
    });
}

criterion_group!(bench_get_index_of, bench_typed_proj_index_map_get_index_of, bench_index_map_get_index_of);
