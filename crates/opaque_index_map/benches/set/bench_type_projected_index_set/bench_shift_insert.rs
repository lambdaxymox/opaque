use opaque_index_map::set::TypeProjectedIndexSet;

use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

fn bench_index_set_shift_insert(c: &mut Criterion) {
    c.bench_function("index_set_shift_insert", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut set = IndexSet::new();
            for (index, value) in values.enumerate() {
                set.shift_insert(index, value);
            }

            set
        });
    });
}

fn bench_type_projected_index_set_shift_insert(c: &mut Criterion) {
    c.bench_function("type_projected_index_set_shift_insert", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut proj_set = TypeProjectedIndexSet::new();
            for (index, value) in values.enumerate() {
                proj_set.shift_insert(index, value);
            }

            proj_set
        });
    });
}

criterion_group!(
    bench_shift_insert,
    bench_type_projected_index_set_shift_insert,
    bench_index_set_shift_insert
);
