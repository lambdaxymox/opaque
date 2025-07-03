use opaque_index_map::map::TypedProjIndexMap;

use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexMap;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

fn bench_index_map_shift_insert(c: &mut Criterion) {
    c.bench_function("index_map_shift_insert", |b| {
        b.iter(|| {
            let keys = 0..1000;
            let values = 1..1001;
            let mut map = IndexMap::new();
            for (index, (key, value)) in keys.zip(values).enumerate() {
                map.shift_insert(index, key, value);
            }

            map
        });
    });
}

fn bench_typed_proj_index_map_shift_insert(c: &mut Criterion) {
    c.bench_function("typed_proj_index_map_shift_insert", |b| {
        b.iter(|| {
            let keys = 0..1000;
            let values = 1..1001;
            let mut proj_map = TypedProjIndexMap::new();
            for (index, (key, value)) in keys.zip(values).enumerate() {
                proj_map.shift_insert(index, key, value);
            }

            proj_map
        });
    });
}

criterion_group!(
    bench_shift_insert,
    bench_typed_proj_index_map_shift_insert,
    bench_index_map_shift_insert
);
