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

fn bench_vec_shift_remove_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_shift_remove_last", |b| {
        b.iter_batched(
            || Vec::from_iter((0..1000).map(|_| dummy_data)),
            |mut vec| {
                for _ in 0..vec.len() {
                    let last_index = vec.len() - 1;
                    let _ = core::hint::black_box(vec.remove(last_index));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_typed_proj_vec_shift_remove_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("typed_proj_vec_shift_remove_last", |b| {
        b.iter_batched(
            || TypedProjVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut proj_vec| {
                for _ in 0..proj_vec.len() {
                    let last_index = proj_vec.len() - 1;
                    let _ = core::hint::black_box(proj_vec.shift_remove(last_index));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_shift_remove, bench_typed_proj_vec_shift_remove_last, bench_vec_shift_remove_last);
