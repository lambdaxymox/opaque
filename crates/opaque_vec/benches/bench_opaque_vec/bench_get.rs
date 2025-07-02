use opaque_vec::OpaqueVec;

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

fn bench_opaque_vec_get(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let opaque_vec = OpaqueVec::from_iter((0..1000).map(|_| dummy_data));

    c.bench_function("opaque_vec_get", |b| {
        b.iter(|| {
            for i in 0..opaque_vec.len() {
                let _ = core::hint::black_box(opaque_vec.get::<_, i32, alloc::Global>(i));
            }
        });
    });
}

criterion_group!(bench_get, bench_opaque_vec_get, bench_vec_get);
