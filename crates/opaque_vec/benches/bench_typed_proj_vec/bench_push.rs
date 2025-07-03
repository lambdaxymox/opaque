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

fn bench_vec_push(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_push", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for _ in 0..1024 {
                vec.push(core::hint::black_box(dummy_data));
            }

            vec
        });
    });
}

fn bench_typed_proj_vec_push(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("typed_proj_vec_push", |b| {
        b.iter(|| {
            let mut proj_vec = TypedProjVec::new();
            for _ in 0..1024 {
                proj_vec.push(core::hint::black_box(dummy_data));
            }

            proj_vec
        });
    });
}

criterion_group!(bench_push, bench_typed_proj_vec_push, bench_vec_push);
