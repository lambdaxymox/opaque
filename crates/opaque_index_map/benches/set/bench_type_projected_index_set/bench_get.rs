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

fn bench_index_set_get(c: &mut Criterion) {
    let values = 0..1000;
    let set = IndexSet::<i32, hash::RandomState>::from_iter(values);

    c.bench_function("index_set_get", |b| {
        b.iter(|| {
            for value in set.iter() {
                let _ = core::hint::black_box(set.get(value));
            }
        });
    });
}

fn bench_type_projected_index_set_get(c: &mut Criterion) {
    let values = 0..1000;
    let proj_set = TypeProjectedIndexSet::<i32, hash::RandomState, alloc::Global>::from_iter(values);

    c.bench_function("type_projected_index_set_get", |b| {
        b.iter(|| {
            for value in proj_set.iter() {
                let _ = core::hint::black_box(proj_set.get(value));
            }
        });
    });
}

criterion_group!(bench_get, bench_type_projected_index_set_get, bench_index_set_get);
