use opaque_index_map::set::OpaqueIndexSet;

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

fn bench_index_set_insert(c: &mut Criterion) {
    c.bench_function("index_set_insert", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut map = IndexSet::new();
            for value in values {
                map.insert(value);
            }

            map
        });
    });
}

fn bench_opaque_index_set_insert(c: &mut Criterion) {
    c.bench_function("opaque_index_set_insert", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut opaque_set = OpaqueIndexSet::new::<i32>();
            for value in values {
                opaque_set.insert::<i32, hash::RandomState, alloc::Global>(value);
            }

            opaque_set
        });
    });
}

criterion_group!(
    bench_insert,
    bench_opaque_index_set_insert,
    bench_index_set_insert
);
