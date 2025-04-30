#![feature(allocator_api)]
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use opaque_blob_vec::OpaqueBlobVec;
use opaque_alloc::OpaqueAlloc;

use core::ptr::NonNull;

fn create_opaque_blob_vec(len: usize, dummy_data: i32) -> OpaqueBlobVec {
    let alloc = OpaqueAlloc::new(std::alloc::Global);
    let layout = core::alloc::Layout::new::<i32>();
    let drop_fn = None;
    let mut opaque_blob_vec = OpaqueBlobVec::new_in(alloc, layout, drop_fn);
    for i in 0..len {
        let ptr = unsafe { NonNull::new_unchecked(&dummy_data as *const i32 as *mut u8) };
        opaque_blob_vec.push(ptr);
    }

    opaque_blob_vec
}

fn bench_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 1000];

    c.bench_function("vec_swap_remove", |b| {
        b.iter_batched(
            || vec![0_i32; 1000],
            |mut vec| {
                for _ in 0..vec.len() {
                    let _ = criterion::black_box(vec.swap_remove(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_opaque_blob_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut opaque_blob_vec = create_opaque_blob_vec(1000, dummy_data);

    c.bench_function("opaque_blob_vec_swap_remove", |b| {
        b.iter_batched(
            || opaque_blob_vec.clone(),
            |mut opaque_blob_vec| {
                for _ in 0..opaque_blob_vec.len() {
                    let _ = criterion::black_box(opaque_blob_vec.swap_remove_forget_unchecked(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(opaque_blob_vec_benches, bench_opaque_blob_vec_swap_remove, bench_vec_swap_remove);
criterion_main!(opaque_blob_vec_benches);
