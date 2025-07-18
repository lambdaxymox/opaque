use opaque_index_map::map::TypeProjectedIndexMap;

use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexMap;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

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

fn bench_type_projected_index_map_insert_full(c: &mut Criterion) {
    c.bench_function("type_projected_index_map_insert_full", |b| {
        b.iter(|| {
            let keys = 0..1000;
            let values = 1..1001;
            let mut proj_map = TypeProjectedIndexMap::new();
            for (key, value) in keys.zip(values) {
                proj_map.insert_full(key, value);
            }

            proj_map
        });
    });
}

criterion_group!(
    bench_insert_full,
    bench_type_projected_index_map_insert_full,
    bench_index_map_insert_full
);
