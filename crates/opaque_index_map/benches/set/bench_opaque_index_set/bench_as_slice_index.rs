use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;
use opaque_index_map::set::OpaqueIndexSet;

use std::hash;
use std::alloc;

fn bench_index_set_as_slice_index(c: &mut Criterion) {
    let values = 0..10000;
    let map = IndexSet::<i32, hash::RandomState>::from_iter(values);

    c.bench_function("index_set_as_slice_index", |b| {
        b.iter(|| {
            let slice = map.as_slice();
            for i in 0..slice.len() {
                let _ = criterion::black_box(slice[i]);
            }
        });
    });
}

fn bench_opaque_index_set_as_slice_index(c: &mut Criterion) {
    let values = 0..10000;
    let opaque_set = OpaqueIndexSet::from_iter(values);

    c.bench_function("opaque_index_set_as_slice_index", |b| {
        b.iter(|| {
            let slice = opaque_set.as_slice::<i32, hash::RandomState, alloc::Global>();
            for i in 0..slice.len() {
                let _ = criterion::black_box(slice[i]);
            }
        });
    });
}

criterion_group!(bench_as_slice_index, bench_opaque_index_set_as_slice_index, bench_index_set_as_slice_index);
