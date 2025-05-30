use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexMap;
use opaque_index_map::map::TypedProjIndexMap;

use std::hash;
use std::alloc;

fn bench_index_map_get_full(c: &mut Criterion) {
    let keys = 0..1000;
    let values = 1..1001;
    let map = IndexMap::<i32, i32, hash::RandomState>::from_iter(keys.zip(values));

    c.bench_function("index_map_get_full", |b| {
        b.iter(|| {
            for key in map.keys() {
                let _ = core::hint::black_box(map.get_full(key));
            }
        });
    });
}

fn bench_typed_proj_index_map_get_full(c: &mut Criterion) {
    let keys = 0..1000;
    let values = 1..1001;
    let proj_map = TypedProjIndexMap::<i32, i32, hash::RandomState, alloc::Global>::from_iter(keys.zip(values));

    c.bench_function("typed_proj_index_map_get_full", |b| {
        b.iter(|| {
            for key in proj_map.keys() {
                let _ = core::hint::black_box(proj_map.get_full(key));
            }
        });
    });
}

criterion_group!(bench_get_full, bench_typed_proj_index_map_get_full, bench_index_map_get_full);
