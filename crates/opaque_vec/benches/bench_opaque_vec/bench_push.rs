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

fn bench_vec_push(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_push", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for _ in 0..1024 {
                vec.push(core::hint::black_box(dummy_data));
            }

            vec
        });
    });
}

fn bench_opaque_vec_push(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("opaque_vec_push", |b| {
        b.iter(|| {
            let mut opaque_vec = OpaqueVec::new::<i32>();
            for _ in 0..1024 {
                opaque_vec.push::<i32, alloc::Global>(core::hint::black_box(dummy_data));
            }

            opaque_vec
        });
    });
}

criterion_group!(bench_push, bench_opaque_vec_push, bench_vec_push);
