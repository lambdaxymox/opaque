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

fn bench_opaque_vec_shift_remove_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("opaque_vec_shift_remove_last", |b| {
        b.iter_batched(
            || OpaqueVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut opaque_vec| {
                for _ in 0..opaque_vec.len() {
                    let last_index = opaque_vec.len() - 1;
                    let _ = core::hint::black_box(opaque_vec.shift_remove::<i32, alloc::Global>(last_index));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_shift_remove, bench_opaque_vec_shift_remove_last, bench_vec_shift_remove_last);
