use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexMap;
use opaque_index_map::OpaqueIndexMap;

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

fn bench_opaque_index_map_insert_full(c: &mut Criterion) {
    c.bench_function("opaque_index_map_insert_full", |b| {
        b.iter(|| {
            let keys = 0..1000;
            let values = 1..1001;
            let mut opaque_map = OpaqueIndexMap::new::<i32, i32>();
            for (key, value) in keys.zip(values) {
                opaque_map.insert_full::<i32, i32, hash::RandomState, alloc::Global>(key, value);
            }

            opaque_map
        });
    });
}

criterion_group!(
    bench_insert_full,
    bench_opaque_index_map_insert_full,
    bench_index_map_insert_full
);
