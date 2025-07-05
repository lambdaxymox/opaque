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

fn bench_index_map_as_slice_index(c: &mut Criterion) {
    let keys = 0..10000;
    let values = 1..10001;
    let map = IndexMap::<i32, i32, hash::RandomState>::from_iter(keys.zip(values));

    c.bench_function("index_map_as_slice_index", |b| {
        b.iter(|| {
            let slice = map.as_slice();
            for i in 0..slice.len() {
                let _ = core::hint::black_box(slice[i]);
            }
        });
    });
}

fn bench_type_erased_index_map_as_slice_index(c: &mut Criterion) {
    let keys = 0..10000;
    let values = 1..10001;
    let opaque_map = TypeErasedIndexMap::from_iter(keys.zip(values));

    c.bench_function("opaque_index_map_as_slice_index", |b| {
        b.iter(|| {
            let slice = opaque_map.as_slice::<i32, i32, hash::RandomState, alloc::Global>();
            for i in 0..slice.len() {
                let _ = core::hint::black_box(slice[i]);
            }
        });
    });
}

criterion_group!(bench_as_slice_index, bench_type_erased_index_map_as_slice_index, bench_index_map_as_slice_index);
