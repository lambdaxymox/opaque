use opaque_index_map::set::TypeProjectedIndexSet;

use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;

use std::hash;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

fn bench_index_set_as_slice_index(c: &mut Criterion) {
    let values = 0..10000;
    let set = IndexSet::<i32, hash::RandomState>::from_iter(values);

    c.bench_function("index_set_as_slice_index", |b| {
        b.iter(|| {
            let slice = set.as_slice();
            for i in 0..slice.len() {
                let _ = core::hint::black_box(slice[i]);
            }
        });
    });
}

fn bench_type_projected_index_set_as_slice_index(c: &mut Criterion) {
    let values = 0..10000;
    let proj_set = TypeProjectedIndexSet::<i32, hash::RandomState, alloc::Global>::from_iter(values);

    c.bench_function("type_projected_index_set_as_slice_index", |b| {
        b.iter(|| {
            let slice = proj_set.as_slice();
            for i in 0..slice.len() {
                let _ = core::hint::black_box(slice[i]);
            }
        });
    });
}

criterion_group!(
    bench_as_slice_index,
    bench_type_projected_index_set_as_slice_index,
    bench_index_set_as_slice_index
);
