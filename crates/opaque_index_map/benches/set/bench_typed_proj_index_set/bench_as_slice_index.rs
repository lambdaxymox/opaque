use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;
use opaque_index_map::set::TypedProjIndexSet;

use std::hash;
use std::alloc;

fn bench_index_set_as_slice_index(c: &mut Criterion) {
    let values = 0..10000;
    let set = IndexSet::<i32, hash::RandomState>::from_iter(values);

    c.bench_function("index_set_as_slice_index", |b| {
        b.iter(|| {
            let slice = set.as_slice();
            for i in 0..slice.len() {
                let _ = criterion::black_box(slice[i]);
            }
        });
    });
}

fn bench_typed_proj_index_set_as_slice_index(c: &mut Criterion) {
    let values = 0..10000;
    let proj_set = TypedProjIndexSet::<i32, hash::RandomState, alloc::Global>::from_iter(values);

    c.bench_function("typed_proj_index_set_as_slice_index", |b| {
        b.iter(|| {
            let slice = proj_set.as_slice();
            for i in 0..slice.len() {
                let _ = criterion::black_box(slice[i]);
            }
        });
    });
}

criterion_group!(bench_as_slice_index, bench_typed_proj_index_set_as_slice_index, bench_index_set_as_slice_index);
