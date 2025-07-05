use opaque_vec::TypeErasedVec;

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

fn bench_type_erased_vec_shift_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("opaque_vec_shift_insert_last", |b| {
        b.iter(|| {
            let mut opaque_vec = TypeErasedVec::new::<i32>();
            for i in 0..1024 {
                opaque_vec.shift_insert::<i32, alloc::Global>(i, core::hint::black_box(dummy_data));
            }

            opaque_vec
        });
    });
}

criterion_group!(bench_shift_insert, bench_type_erased_vec_shift_insert_last, bench_vec_shift_insert_last);
