use criterion::{
    Criterion,
    criterion_group,
};
use opaque_blob_vec::TypedProjBlobVec;
use opaque_alloc::TypedProjAlloc;

use core::ptr::NonNull;
use std::alloc;

fn create_typed_proj_blob_vec(len: usize, dummy_data: i32) -> TypedProjBlobVec<alloc::Global> {
    let alloc = TypedProjAlloc::new(alloc::Global);
    let layout = alloc::Layout::new::<i32>();
    let drop_fn = None;
    let mut proj_blob_vec = TypedProjBlobVec::new_in(alloc, layout, drop_fn);
    for i in 0..len {
        let ptr = unsafe { NonNull::new_unchecked(&dummy_data as *const i32 as *mut u8) };
        proj_blob_vec.push(ptr);
    }

    proj_blob_vec
}

fn bench_vec_get_unchecked(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let vec = vec![dummy_data; 1000];

    c.bench_function("vec_get_unchecked", |b| {
        b.iter(|| {
            for i in 0..vec.len() {
                let _ = core::hint::black_box(unsafe { vec.get_unchecked(i) });
            }
        });
    });
}

fn bench_vec_get(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let vec = vec![dummy_data; 1000];

    c.bench_function("vec_get", |b| {
        b.iter(|| {
            for i in 0..vec.len() {
                let _ = core::hint::black_box(vec.get(i));
            }
        });
    });
}

fn bench_typed_proj_blob_vec_get_unchecked(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let proj_blob_vec = create_typed_proj_blob_vec(1000, dummy_data);

    c.bench_function("typed_proj_blob_vec_get_unchecked", |b| {
        b.iter(|| {
            for i in 0..proj_blob_vec.len() {
                let _ = core::hint::black_box(proj_blob_vec.get_unchecked(i));
            }
        });
    });
}

criterion_group!(bench_get_unchecked, bench_typed_proj_blob_vec_get_unchecked, bench_vec_get_unchecked, bench_vec_get);
