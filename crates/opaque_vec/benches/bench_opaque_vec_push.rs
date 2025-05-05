#![feature(allocator_api)]
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use opaque_vec::OpaqueVec;

use std::alloc;

fn bench_vec_push(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_push", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for _ in 0..1024 {
                vec.push(criterion::black_box(dummy_data));
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
                opaque_vec.push::<i32, alloc::Global>(criterion::black_box(dummy_data));
            }

            opaque_vec
        });
    });
}

criterion_group!(opaque_vec_benches, bench_opaque_vec_push, bench_vec_push);
criterion_main!(opaque_vec_benches);
