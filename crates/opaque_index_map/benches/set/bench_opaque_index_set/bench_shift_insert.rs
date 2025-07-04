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

fn bench_index_set_shift_insert(c: &mut Criterion) {
    c.bench_function("index_set_shift_insert", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut map = IndexSet::new();
            for (index, value) in values.enumerate() {
                map.shift_insert(index, value);
            }

            map
        });
    });
}

fn bench_opaque_index_set_shift_insert(c: &mut Criterion) {
    c.bench_function("opaque_index_set_shift_insert", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut opaque_set = TypeErasedIndexSet::new::<i32>();
            for (index, value) in values.enumerate() {
                opaque_set.shift_insert::<i32, hash::RandomState, alloc::Global>(index, value);
            }

            opaque_set
        });
    });
}

criterion_group!(
    bench_shift_insert,
    bench_opaque_index_set_shift_insert,
    bench_index_set_shift_insert
);
