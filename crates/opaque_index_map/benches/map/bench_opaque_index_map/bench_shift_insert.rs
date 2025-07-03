use opaque_index_map::map::OpaqueIndexMap;

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

fn bench_opaque_index_map_shift_insert(c: &mut Criterion) {
    c.bench_function("opaque_index_map_shift_insert", |b| {
        b.iter(|| {
            let keys = 0..1000;
            let values = 1..1001;
            let mut opaque_map = OpaqueIndexMap::new::<i32, i32>();
            for (index, (key, value)) in keys.zip(values).enumerate() {
                opaque_map.shift_insert::<i32, i32, hash::RandomState, alloc::Global>(index, key, value);
            }

            opaque_map
        });
    });
}

criterion_group!(
    bench_shift_insert,
    bench_opaque_index_map_shift_insert,
    bench_index_map_shift_insert
);
