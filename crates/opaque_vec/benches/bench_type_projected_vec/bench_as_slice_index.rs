use opaque_vec::TypeProjectedVec;

use criterion::{
    Criterion,
    criterion_group,
};

use alloc_crate::vec::Vec;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

fn bench_vec_as_slice_index(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let vec = Vec::from_iter((0..10000).map(|_| dummy_data));

    c.bench_function("vec_as_slice_index", |b| {
        b.iter(|| {
            let slice = vec.as_slice();
            for i in 0..slice.len() {
                let _ = core::hint::black_box(slice[i]);
            }
        });
    });
}

fn bench_type_projected_vec_as_slice_index(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let proj_vec = TypeProjectedVec::from_iter((0..10000).map(|_| dummy_data));

    c.bench_function("type_projected_vec_as_slice_index", |b| {
        b.iter(|| {
            let slice = proj_vec.as_slice();
            for i in 0..slice.len() {
                let _ = core::hint::black_box(slice[i]);
            }
        });
    });
}

criterion_group!(
    bench_as_slice_index,
    bench_type_projected_vec_as_slice_index,
    bench_vec_as_slice_index
);
