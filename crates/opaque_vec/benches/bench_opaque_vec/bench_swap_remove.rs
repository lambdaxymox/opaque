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

fn bench_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_swap_remove", |b| {
        b.iter_batched(
            || Vec::from_iter((0..1000).map(|_| dummy_data)),
            |mut vec| {
                for _ in 0..vec.len() {
                    let _ = core::hint::black_box(vec.swap_remove(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_opaque_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("opaque_vec_swap_remove", |b| {
        b.iter_batched(
            || OpaqueVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut opaque_vec| {
                for _ in 0..opaque_vec.len() {
                    let _ = core::hint::black_box(opaque_vec.swap_remove::<i32, alloc::Global>(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_swap_remove, bench_opaque_vec_swap_remove, bench_vec_swap_remove);
