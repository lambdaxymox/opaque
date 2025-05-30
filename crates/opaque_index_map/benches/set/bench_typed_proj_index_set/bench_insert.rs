use criterion::{
    Criterion,
    criterion_group,
};

use indexmap::IndexSet;
use opaque_index_map::set::TypedProjIndexSet;

fn bench_index_set_insert(c: &mut Criterion) {
    c.bench_function("index_set_insert", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut set = IndexSet::new();
            for value in values {
                set.insert(value);
            }

            set
        });
    });
}

fn bench_typed_proj_index_set_insert(c: &mut Criterion) {
    c.bench_function("typed_proj_index_set_insert", |b| {
        b.iter(|| {
            let values = 0..1000;
            let mut proj_set = TypedProjIndexSet::new();
            for value in values {
                proj_set.insert(value);
            }

            proj_set
        });
    });
}

criterion_group!(
    bench_insert,
    bench_typed_proj_index_set_insert,
    bench_index_set_insert
);
