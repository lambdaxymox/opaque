#![feature(allocator_api)]
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use opaque_vec::OpaqueVec;

use std::alloc;

fn bench_vec_as_slice_index(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 10000];

    c.bench_function("vec_as_slice_index", |b| {
        b.iter(|| {
            let slice = vec.as_slice();
            for i in 0..slice.len() {
                let _ = criterion::black_box(slice[i]);
            }
        });
    });
}

fn bench_opaque_vec_as_slice_index(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let opaque_vec = OpaqueVec::from_iter((0..10000).map(|_| dummy_data));

    c.bench_function("opaque_vec_as_slice_index", |b| {
        b.iter(|| {
            let slice = opaque_vec.as_slice::<i32, alloc::Global>();
            for i in 0..slice.len() {
                let _ = criterion::black_box(slice[i]);
            }
        });
    });
}

criterion_group!(opaque_vec_benches, bench_opaque_vec_as_slice_index, bench_vec_as_slice_index);
criterion_main!(opaque_vec_benches);
