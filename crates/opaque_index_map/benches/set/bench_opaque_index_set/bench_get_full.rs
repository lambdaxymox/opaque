use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;
use opaque_index_map::set::OpaqueIndexSet;

use std::hash;
use std::alloc;

fn bench_index_set_get_full(c: &mut Criterion) {
    let values = 0..1000;
    let map = IndexSet::<i32, hash::RandomState>::from_iter(values);

    c.bench_function("index_set_get_full", |b| {
        b.iter(|| {
            for key in map.iter() {
                let _ = core::hint::black_box(map.get_full(key));
            }
        });
    });
}

fn bench_opaque_index_set_get_full(c: &mut Criterion) {
    let values = 0..1000;
    let opaque_set = OpaqueIndexSet::from_iter(values);

    c.bench_function("opaque_index_set_get_full", |b| {
        b.iter(|| {
            for key in opaque_set.iter::<i32, hash::RandomState, alloc::Global>() {
                let _ = core::hint::black_box(opaque_set.get_full::<i32, i32, hash::RandomState, alloc::Global>(key));
            }
        });
    });
}

criterion_group!(bench_get_full, bench_opaque_index_set_get_full, bench_index_set_get_full);
