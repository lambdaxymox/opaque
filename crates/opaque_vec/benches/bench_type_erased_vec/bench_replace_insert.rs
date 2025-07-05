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

fn bench_vec_replace_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_replace_insert_last", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for _ in 0..1024 {
                vec.push(core::hint::black_box(dummy_data));
            }

            vec
        });
    });
}

fn bench_type_erased_vec_replace_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("opaque_vec_replace_insert_last", |b| {
        b.iter(|| {
            let mut opaque_vec = TypeErasedVec::new::<i32>();
            for _ in 0..1024 {
                let last_index = if opaque_vec.len() == 0 { 0 } else { opaque_vec.len() - 1 };
                opaque_vec.replace_insert::<i32, alloc::Global>(last_index, core::hint::black_box(dummy_data));
            }

            opaque_vec
        });
    });
}

criterion_group!(
    bench_replace_insert,
    bench_type_erased_vec_replace_insert_last,
    bench_vec_replace_insert_last
);
