use opaque_vec::TypedProjVec;

use criterion::{
    Criterion,
    criterion_group,
};

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(feature = "nightly")]
use alloc_crate::vec::Vec;

#[cfg(not(feature = "nightly"))]
use allocator_api2::alloc;

#[cfg(not(feature = "nightly"))]
use allocator_api2::vec::Vec;

fn bench_vec_get(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = Vec::from_iter((0..1000).map(|_| dummy_data));

    c.bench_function("vec_get", |b| {
        b.iter(|| {
            for i in 0..vec.len() {
                let _ = core::hint::black_box(vec.get(i));
            }
        });
    });
}

fn bench_typed_proj_vec_get(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let proj_vec = TypedProjVec::from_iter((0..1000).map(|_| dummy_data));

    c.bench_function("typed_proj_vec_get", |b| {
        b.iter(|| {
            for i in 0..proj_vec.len() {
                let _ = core::hint::black_box(proj_vec.get(i));
            }
        });
    });
}

criterion_group!(bench_get, bench_typed_proj_vec_get, bench_vec_get);
