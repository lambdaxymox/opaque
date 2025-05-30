use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;
use opaque_index_map::set::OpaqueIndexSet;

use std::alloc;
use std::hash;

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

fn bench_opaque_index_set_insert_full(c: &mut Criterion) {
    c.bench_function("opaque_index_set_insert_full", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut opaque_set = OpaqueIndexSet::new::<i32>();
            for value in values {
                opaque_set.insert_full::<i32, hash::RandomState, alloc::Global>(value);
            }

            opaque_set
        });
    });
}

criterion_group!(
    bench_insert_full,
    bench_opaque_index_set_insert_full,
    bench_index_set_insert_full
);
