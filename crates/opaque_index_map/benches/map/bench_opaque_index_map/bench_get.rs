use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexMap;
use opaque_index_map::map::OpaqueIndexMap;

use std::hash;
use std::alloc;

fn bench_index_map_get(c: &mut Criterion) {
    let keys = 0..1000;
    let values = 1..1001;
    let map = IndexMap::<i32, i32, hash::RandomState>::from_iter(keys.zip(values));

    c.bench_function("index_map_get", |b| {
        b.iter(|| {
            for key in map.keys() {
                let _ = core::hint::black_box(map.get(key));
            }
        });
    });
}

fn bench_opaque_index_map_get(c: &mut Criterion) {
    let keys = 0..1000;
    let values = 1..1001;
    let opaque_map = OpaqueIndexMap::from_iter(keys.zip(values));

    c.bench_function("opaque_index_map_get", |b| {
        b.iter(|| {
            for key in opaque_map.keys::<i32, i32, hash::RandomState, alloc::Global>() {
                let _ = core::hint::black_box(opaque_map.get::<i32, i32, i32, hash::RandomState, alloc::Global>(key));
            }
        });
    });
}

criterion_group!(bench_get, bench_opaque_index_map_get, bench_index_map_get);
