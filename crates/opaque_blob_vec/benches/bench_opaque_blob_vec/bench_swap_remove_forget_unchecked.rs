use criterion::{
    Criterion,
    criterion_group,
};
use opaque_blob_vec::OpaqueBlobVec;
use opaque_alloc::TypedProjAlloc;

use core::ptr::NonNull;
use std::alloc;

fn create_opaque_blob_vec(len: usize, dummy_data: i32) -> OpaqueBlobVec {
    let alloc = TypedProjAlloc::new(alloc::Global);
    let layout = alloc::Layout::new::<i32>();
    let drop_fn = None;
    let mut opaque_blob_vec = OpaqueBlobVec::new_in(alloc, layout, drop_fn);
    for i in 0..len {
        let ptr = unsafe { NonNull::new_unchecked(&dummy_data as *const i32 as *mut u8) };
        opaque_blob_vec.push::<alloc::Global>(ptr);
    }

    opaque_blob_vec
}

fn clone(opaque_blob_vec: &OpaqueBlobVec) -> OpaqueBlobVec {
    let new_alloc = {
        let proj_old_alloc = opaque_blob_vec.allocator::<alloc::Global>();
        Clone::clone(proj_old_alloc)
    };
    let new_element_layout = opaque_blob_vec.element_layout();
    let new_capacity = opaque_blob_vec.capacity();
    let new_drop_fn = None;

    let new_opaque_blob_vec = unsafe {
        let mut _new_opaque_blob_vec = OpaqueBlobVec::with_capacity_in(new_capacity, new_alloc, new_element_layout, new_drop_fn);
        let length = opaque_blob_vec.len();
        let data_ptr = NonNull::new_unchecked(opaque_blob_vec.as_ptr() as *mut u8);
        _new_opaque_blob_vec.append::<alloc::Global>(data_ptr, length);
        _new_opaque_blob_vec
    };

    new_opaque_blob_vec
}

fn bench_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let vec = vec![dummy_data; 1000];

    c.bench_function("vec_swap_remove", |b| {
        b.iter_batched(
            || vec.clone(),
            |mut vec| {
                for _ in 0..vec.len() {
                    let _ = core::hint::black_box(vec.swap_remove(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_opaque_blob_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let opaque_blob_vec = create_opaque_blob_vec(1000, dummy_data);

    c.bench_function("opaque_blob_vec_swap_remove", |b| {
        b.iter_batched(
            || clone(&opaque_blob_vec),
            |mut opaque_blob_vec| {
                for _ in 0..opaque_blob_vec.len() {
                    let _ = core::hint::black_box(opaque_blob_vec.swap_remove_forget_unchecked::<alloc::Global>(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_swap_remove_forget_unchecked, bench_opaque_blob_vec_swap_remove, bench_vec_swap_remove);
