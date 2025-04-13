use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};
use opaque_vec::OpaqueVec;

fn bench_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 1000];

    c.bench_function("vec_swap_remove", |b| {
        b.iter_batched(
            || vec![0_i32; 1000],
            |mut vec| {
                for _ in 0..vec.len() {
                    let _ = criterion::black_box(vec.swap_remove(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

fn bench_opaque_vec_swap_remove(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut opaque_vec = OpaqueVec::from_iter((0..1000).map(|_| dummy_data));

    c.bench_function("opaque_vec_swap_remove", |b| {
        b.iter_batched(
            || OpaqueVec::from_iter((0..1000).map(|_| dummy_data)),
            |mut opaque_vec| {
                for _ in 0..opaque_vec.len() {
                    let _ = criterion::black_box(opaque_vec.swap_remove::<i32>(0));
                }
            },
            criterion::BatchSize::NumIterations(1000),
        );
    });
}

criterion_group!(opaque_vec_benches, bench_opaque_vec_swap_remove, bench_vec_swap_remove);
criterion_main!(opaque_vec_benches);
