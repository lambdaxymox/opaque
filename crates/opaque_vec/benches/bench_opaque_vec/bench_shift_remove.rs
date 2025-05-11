use criterion::{
    Criterion,
    criterion_group,
};
use opaque_vec::OpaqueVec;

use std::alloc;

fn bench_vec_shift_remove_last(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 1000];

    c.bench_function("vec_shift_remove_last", |b| {
        b.iter_batched(
            || vec![0_i32; 1000],
            |mut vec| {
                for _ in 0..vec.len() {
                    let last_index = vec.len() - 1;
                    let _ = criterion::black_box(vec.remove(last_index));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_opaque_vec_shift_remove_last(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut opaque_vec = OpaqueVec::from_iter((0..1000).map(|_| dummy_data));

    c.bench_function("opaque_vec_shift_remove_last", |b| {
        b.iter_batched(
            || OpaqueVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut opaque_vec| {
                for _ in 0..opaque_vec.len::<i32, alloc::Global>() {
                    let last_index = opaque_vec.len::<i32, alloc::Global>() - 1;
                    let _ = criterion::black_box(opaque_vec.shift_remove::<i32, alloc::Global>(last_index));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_shift_remove, bench_opaque_vec_shift_remove_last, bench_vec_shift_remove_last);
