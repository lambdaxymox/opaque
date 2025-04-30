#![feature(allocator_api)]
use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use opaque_blob_vec::OpaqueBlobVec;
use opaque_alloc::OpaqueAlloc;

use core::ptr::NonNull;

fn new_opaque_blob_vec() -> OpaqueBlobVec {
    let alloc = OpaqueAlloc::new(std::alloc::Global);
    let layout = core::alloc::Layout::new::<i32>();
    let drop_fn = None;

    OpaqueBlobVec::new_in(alloc, layout, drop_fn)
}

fn bench_vec_replace_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_replace_insert_last", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for _ in 0..1024 {
                vec.push(criterion::black_box(dummy_data));
            }

            vec
        });
    });
}

fn bench_opaque_blob_vec_replace_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("opaque_blob_vec_replace_insert_last", |b| {
        b.iter(|| {
            let mut opaque_blob_vec = new_opaque_blob_vec();
            for _ in 0..1024 {
                let last_index = if opaque_blob_vec.len() == 0 { 0 } else { opaque_blob_vec.len() - 1 };
                let ptr = unsafe { NonNull::new_unchecked(&dummy_data as *const i32 as *mut u8) };
                opaque_blob_vec.replace_insert(last_index, criterion::black_box(ptr));
            }

            opaque_blob_vec
        });
    });
}

criterion_group!(
    opaque_blob_vec_benches,
    bench_opaque_blob_vec_replace_insert_last,
    bench_vec_replace_insert_last
);
criterion_main!(opaque_blob_vec_benches);
