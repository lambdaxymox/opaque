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

fn bench_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = Vec::from_iter((0..1000).map(|_| dummy_data));

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

fn bench_typed_proj_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("typed_proj_vec_swap_remove", |b| {
        b.iter_batched(
            || TypedProjVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut proj_vec| {
                for _ in 0..proj_vec.len() {
                    let _ = core::hint::black_box(proj_vec.swap_remove(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_swap_remove, bench_typed_proj_vec_swap_remove, bench_vec_swap_remove);
