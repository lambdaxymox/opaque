use opaque_vec::TypedProjVec;

use criterion::{
    Criterion,
    criterion_group,
};

use alloc_crate::vec::Vec;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

fn bench_vec_shift_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_shift_insert_last", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1024 {
                vec.insert(i, core::hint::black_box(dummy_data));
            }

            vec
        });
    });
}

fn bench_typed_proj_vec_shift_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("typed_proj_vec_shift_insert_last", |b| {
        b.iter(|| {
            let mut proj_vec = TypedProjVec::new();
            for i in 0..1024 {
                proj_vec.shift_insert(i, core::hint::black_box(dummy_data));
            }

            proj_vec
        });
    });
}

criterion_group!(bench_shift_insert, bench_typed_proj_vec_shift_insert_last, bench_vec_shift_insert_last);
