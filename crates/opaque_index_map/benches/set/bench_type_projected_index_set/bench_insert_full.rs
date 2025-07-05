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

fn bench_index_set_insert_full(c: &mut Criterion) {
    c.bench_function("index_set_insert_full", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut set = IndexSet::new();
            for value in values {
                set.insert_full(value);
            }

            set
        });
    });
}

fn bench_type_projected_index_set_insert_full(c: &mut Criterion) {
    c.bench_function("type_projected_index_set_insert_full", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut proj_set = TypeProjectedIndexSet::new();
            for value in values {
                proj_set.insert_full(value);
            }

            proj_set
        });
    });
}

criterion_group!(
    bench_insert_full,
    bench_type_projected_index_set_insert_full,
    bench_index_set_insert_full
);
