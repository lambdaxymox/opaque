use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexMap;
use opaque_index_map::TypedProjIndexMap;

use std::alloc;
use std::hash;

fn bench_index_map_insert_full(c: &mut Criterion) {
    c.bench_function("index_map_insert_full", |b| {
        b.iter(|| {
            let keys = 0..1000;
            let values = 1..1001;
            let mut map = IndexMap::new();
            for (key, value) in keys.zip(values) {
                map.insert_full(key, value);
            }

            map
        });
    });
}

fn bench_typed_proj_index_map_insert_full(c: &mut Criterion) {
    c.bench_function("typed_proj_index_map_insert_full", |b| {
        b.iter(|| {
            let keys = 0..1000;
            let values = 1..1001;
            let mut proj_map = TypedProjIndexMap::new();
            for (key, value) in keys.zip(values) {
                proj_map.insert_full(key, value);
            }

            proj_map
        });
    });
}

criterion_group!(
    bench_insert_full,
    bench_typed_proj_index_map_insert_full,
    bench_index_map_insert_full
);
