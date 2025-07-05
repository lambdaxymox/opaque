use opaque_vec::TypeProjectedVec;

use criterion::{
    Criterion,
    criterion_group,
};

use alloc_crate::vec::Vec;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

fn bench_vec_get(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let vec = Vec::from_iter((0..1000).map(|_| dummy_data));

    c.bench_function("vec_get", |b| {
        b.iter(|| {
            for i in 0..vec.len() {
                let _ = core::hint::black_box(vec.get(i));
            }
        });
    });
}

fn bench_type_projected_vec_get(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let proj_vec = TypeProjectedVec::from_iter((0..1000).map(|_| dummy_data));

    c.bench_function("type_projected_vec_get", |b| {
        b.iter(|| {
            for i in 0..proj_vec.len() {
                let _ = core::hint::black_box(proj_vec.get(i));
            }
        });
    });
}

criterion_group!(bench_get, bench_type_projected_vec_get, bench_vec_get);
