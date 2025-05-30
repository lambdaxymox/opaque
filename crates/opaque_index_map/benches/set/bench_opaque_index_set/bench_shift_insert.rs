use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;
use opaque_index_map::set::OpaqueIndexSet;

use std::alloc;
use std::hash;

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
            let mut opaque_set = OpaqueIndexSet::new::<i32>();
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
