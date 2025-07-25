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

fn bench_index_map_get_key_value(c: &mut Criterion) {
    let keys = 0..1000;
    let values = 1..1001;
    let map = IndexMap::<i32, i32, hash::RandomState>::from_iter(keys.zip(values));

    c.bench_function("index_map_get_key_value", |b| {
        b.iter(|| {
            for key in map.keys() {
                let _ = core::hint::black_box(map.get_key_value(key));
            }
        });
    });
}

fn bench_type_erased_index_map_get_key_value(c: &mut Criterion) {
    let keys = 0..1000;
    let values = 1..1001;
    let opaque_map = TypeErasedIndexMap::from_iter(keys.zip(values));

    c.bench_function("opaque_index_map_get_key_value", |b| {
        b.iter(|| {
            for key in opaque_map.keys::<i32, i32, hash::RandomState, alloc::Global>() {
                let _ = core::hint::black_box(opaque_map.get_key_value::<i32, i32, i32, hash::RandomState, alloc::Global>(key));
            }
        });
    });
}

criterion_group!(
    bench_get_key_value,
    bench_type_erased_index_map_get_key_value,
    bench_index_map_get_key_value
);
