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

fn bench_vec_pop(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_shift_remove_last", |b| {
        b.iter_batched(
            || Vec::from_iter((0..1000).map(|_| dummy_data)),
            |mut vec| {
                for _ in 0..vec.len() {
                    let _ = core::hint::black_box(vec.pop());
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_type_projected_vec_pop(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("type_projected_vec_shift_remove_last", |b| {
        b.iter_batched(
            || TypeProjectedVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut proj_vec| {
                for _ in 0..proj_vec.len() {
                    let _ = core::hint::black_box(proj_vec.pop());
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_pop, bench_type_projected_vec_pop, bench_vec_pop);
