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

fn bench_vec_get_unchecked(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 1000];

    c.bench_function("vec_get_unchecked", |b| {
        b.iter(|| {
            for i in 0..vec.len() {
                let _ = criterion::black_box(unsafe { vec.get_unchecked(i) });
            }
        });
    });
}

fn bench_vec_get(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 1000];

    c.bench_function("vec_get", |b| {
        b.iter(|| {
            for i in 0..vec.len() {
                let _ = criterion::black_box(vec.get(i));
            }
        });
    });
}

fn bench_opaque_blob_vec_get_unchecked(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let opaque_blob_vec = create_opaque_blob_vec(1000, dummy_data);

    c.bench_function("opaque_blob_vec_get_unchecked", |b| {
        b.iter(|| {
            for i in 0..opaque_blob_vec.len() {
                let _ = criterion::black_box(opaque_blob_vec.get_unchecked(i));
            }
        });
    });
}

criterion_group!(opaque_blob_vec_benches, bench_opaque_blob_vec_get_unchecked, bench_vec_get_unchecked, bench_vec_get);
criterion_main!(opaque_blob_vec_benches);
