use opaque_vec::TypeErasedVec;

use criterion::{
    Criterion,
    criterion_group,
};

use alloc_crate::vec::Vec;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

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

fn bench_type_erased_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("opaque_vec_swap_remove", |b| {
        b.iter_batched(
            || TypeErasedVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut opaque_vec| {
                for _ in 0..opaque_vec.len() {
                    let _ = core::hint::black_box(opaque_vec.swap_remove::<i32, alloc::Global>(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_swap_remove, bench_type_erased_vec_swap_remove, bench_vec_swap_remove);
