use criterion::{
    Criterion,
    criterion_group,
};
use opaque_vec::TypedProjVec;

fn bench_vec_shift_remove_last(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 1000];

    c.bench_function("vec_shift_remove_last", |b| {
        b.iter_batched(
            || vec![0_i32; 1000],
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

fn bench_typed_proj_vec_shift_remove_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("typed_proj_vec_shift_remove_last", |b| {
        b.iter_batched(
            || TypedProjVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut proj_vec| {
                for _ in 0..proj_vec.len() {
                    let last_index = proj_vec.len() - 1;
                    let _ = criterion::black_box(proj_vec.shift_remove(last_index));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(bench_shift_remove, bench_typed_proj_vec_shift_remove_last, bench_vec_shift_remove_last);
