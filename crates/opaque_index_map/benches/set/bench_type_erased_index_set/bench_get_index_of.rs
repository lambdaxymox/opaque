use opaque_index_map::set::TypeErasedIndexSet;

use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;

use std::alloc;
use std::hash;

fn bench_index_set_get_index_of(c: &mut Criterion) {
    let values = 0..1000;
    let map = IndexSet::<i32, hash::RandomState>::from_iter(values);

    c.bench_function("index_set_get_index_of", |b| {
        b.iter(|| {
            for key in map.iter() {
                let _ = core::hint::black_box(map.get_index_of(key));
            }
        });
    });
}

fn bench_type_erased_index_set_get_index_of(c: &mut Criterion) {
    let values = 0..1000;
    let opaque_set = TypeErasedIndexSet::from_iter(values);

    c.bench_function("opaque_index_set_get_index_of", |b| {
        b.iter(|| {
            for key in opaque_set.iter::<i32, hash::RandomState, alloc::Global>() {
                let _ = core::hint::black_box(opaque_set.get_index_of::<i32, i32, hash::RandomState, alloc::Global>(key));
            }
        });
    });
}

criterion_group!(
    bench_get_index_of,
    bench_type_erased_index_set_get_index_of,
    bench_index_set_get_index_of
);
