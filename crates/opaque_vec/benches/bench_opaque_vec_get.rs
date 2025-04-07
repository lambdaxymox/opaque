use criterion::{criterion_group, criterion_main, Criterion};
use opaque_vec::OpaqueVec;

fn bench_vec_get(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 1000];

    c.bench_function("vec_get", |b| {
        b.iter(|| {
            for i in 0..vec.len() {
                let _ = criterion::black_box(vec.get(i));
            }
        });
    });
}

fn bench_opaque_vec_get(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let opaque_vec = OpaqueVec::from_iter((0..1000).map(|_| dummy_data));

    c.bench_function("opaque_vec_get", |b| {
        b.iter(|| {
            for i in 0..opaque_vec.len() {
                let _ = criterion::black_box(opaque_vec.get::<i32>(i));
            }
        });
    });
}

criterion_group!(opaque_vec_benches, bench_opaque_vec_get, bench_vec_get);
criterion_main!(opaque_vec_benches);
