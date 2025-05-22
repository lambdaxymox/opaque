use criterion::{
    Criterion,
    criterion_group,
};
use opaque_blob_vec::TypedProjBlobVec;
use opaque_alloc::TypedProjAlloc;

use core::ptr::NonNull;
use std::alloc;

fn new_typed_proj_blob_vec() -> TypedProjBlobVec<alloc::Global> {
    let alloc = TypedProjAlloc::new(alloc::Global);
    let layout = alloc::Layout::new::<i32>();
    let drop_fn = None;

    TypedProjBlobVec::new_in(alloc, layout, drop_fn)
}

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

fn bench_typed_proj_blob_vec_push(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("typed_proj_blob_vec_push", |b| {
        b.iter(|| {
            let mut proj_blob_vec = new_typed_proj_blob_vec();
            for _ in 0..1024 {
                let ptr = unsafe { NonNull::new_unchecked(&dummy_data as *const i32 as *mut u8) };
                proj_blob_vec.push(criterion::black_box(ptr));
            }

            proj_blob_vec
        });
    });
}

criterion_group!(bench_push, bench_typed_proj_blob_vec_push, bench_vec_push);
