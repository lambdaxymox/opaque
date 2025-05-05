#![feature(allocator_api)]
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use opaque_vec::OpaqueVec;

use std::alloc;

fn bench_vec_pop(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 1000];

    c.bench_function("vec_shift_remove_last", |b| {
        b.iter_batched(
            || vec![0_i32; 1000],
            |mut vec| {
                for _ in 0..vec.len() {
                    let _ = criterion::black_box(vec.pop());
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_opaque_vec_pop(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut opaque_vec = OpaqueVec::from_iter((0..1000).map(|_| dummy_data));

    c.bench_function("opaque_vec_shift_remove_last", |b| {
        b.iter_batched(
            || OpaqueVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut opaque_vec| {
                for _ in 0..opaque_vec.len() {
                    let _ = criterion::black_box(opaque_vec.pop::<i32, alloc::Global>());
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(opaque_vec_benches, bench_opaque_vec_pop, bench_vec_pop);
criterion_main!(opaque_vec_benches);
