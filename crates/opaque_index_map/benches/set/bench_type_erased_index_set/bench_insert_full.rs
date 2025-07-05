use opaque_index_map::set::TypeErasedIndexSet;

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

fn bench_index_set_insert_full(c: &mut Criterion) {
    c.bench_function("index_set_insert_full", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut map = IndexSet::new();
            for value in values {
                map.insert_full(value);
            }

            map
        });
    });
}

fn bench_type_erased_index_set_insert_full(c: &mut Criterion) {
    c.bench_function("opaque_index_set_insert_full", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut opaque_set = TypeErasedIndexSet::new::<i32>();
            for value in values {
                opaque_set.insert_full::<i32, hash::RandomState, alloc::Global>(value);
            }

            opaque_set
        });
    });
}

criterion_group!(
    bench_insert_full,
    bench_type_erased_index_set_insert_full,
    bench_index_set_insert_full
);
