use criterion::{
    Criterion,
    criterion_group,
};
use opaque_vec::TypedProjVec;

fn bench_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 1000];

    c.bench_function("vec_swap_remove", |b| {
        b.iter_batched(
            || vec![0_i32; 1000],
            |mut vec| {
                for _ in 0..vec.len() {
                    let _ = core::hint::black_box(vec.swap_remove(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_typed_proj_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("typed_proj_vec_swap_remove", |b| {
        b.iter_batched(
            || TypedProjVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut proj_vec| {
                for _ in 0..proj_vec.len() {
                    let _ = core::hint::black_box(proj_vec.swap_remove(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_swap_remove, bench_typed_proj_vec_swap_remove, bench_vec_swap_remove);
