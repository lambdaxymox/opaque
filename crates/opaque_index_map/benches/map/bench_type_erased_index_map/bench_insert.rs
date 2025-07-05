use opaque_index_map::map::TypeErasedIndexMap;

use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexMap;

use std::hash;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

fn bench_index_map_insert(c: &mut Criterion) {
    c.bench_function("index_map_insert", |b| {
        b.iter(|| {
            let keys = 0..1000;
            let values = 1..1001;
            let mut map = IndexMap::new();
            for (key, value) in keys.zip(values) {
                map.insert(key, value);
            }

            map
        });
    });
}

fn bench_opaque_index_map_insert(c: &mut Criterion) {
    c.bench_function("opaque_index_map_insert", |b| {
        b.iter(|| {
            let keys = 0..1000;
            let values = 1..1001;
            let mut opaque_map = TypeErasedIndexMap::new::<i32, i32>();
            for (key, value) in keys.zip(values) {
                opaque_map.insert::<i32, i32, hash::RandomState, alloc::Global>(key, value);
            }

            opaque_map
        });
    });
}

criterion_group!(
    bench_insert,
    bench_opaque_index_map_insert,
    bench_index_map_insert
);
