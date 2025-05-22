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

fn clone(proj_blob_vec: &TypedProjBlobVec<alloc::Global>) -> TypedProjBlobVec<alloc::Global> {
    let new_alloc = {
        let proj_old_alloc = proj_blob_vec.allocator();
        Clone::clone(proj_old_alloc)
    };
    let new_element_layout = proj_blob_vec.element_layout();
    let new_capacity = proj_blob_vec.capacity();
    let new_drop_fn = None;

    let new_proj_blob_vec = unsafe {
        let mut _new_proj_blob_vec = TypedProjBlobVec::with_capacity_in(new_capacity, new_alloc, new_element_layout, new_drop_fn);
        let length = proj_blob_vec.len();
        let data_ptr = NonNull::new_unchecked(proj_blob_vec.as_ptr() as *mut u8);
        _new_proj_blob_vec.append(data_ptr, length);
        _new_proj_blob_vec
    };

    new_proj_blob_vec
}

fn bench_vec_shift_remove_last(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let vec = vec![dummy_data; 1000];

    c.bench_function("vec_shift_remove_last", |b| {
        b.iter_batched(
            || vec.clone(),
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

fn bench_typed_proj_blob_vec_shift_remove_last(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let proj_blob_vec = create_typed_proj_blob_vec(1000, dummy_data);

    c.bench_function("typed_proj_blob_vec_shift_remove_last", |b| {
        b.iter_batched(
            || clone(&proj_blob_vec),
            |mut proj_blob_vec| {
                for _ in 0..proj_blob_vec.len() {
                    let last_index = proj_blob_vec.len() - 1;
                    let _ = criterion::black_box(proj_blob_vec.shift_remove_forget_unchecked(last_index));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_shift_remove_forget_unchecked, bench_typed_proj_blob_vec_shift_remove_last, bench_vec_shift_remove_last);
