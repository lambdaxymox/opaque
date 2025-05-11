use criterion::{
    Criterion,
    criterion_group,
};
use opaque_vec::TypedProjVec;

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

fn bench_typed_proj_vec_get(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let proj_vec = TypedProjVec::from_iter((0..1000).map(|_| dummy_data));

    c.bench_function("typed_proj_vec_get", |b| {
        b.iter(|| {
            for i in 0..proj_vec.len() {
                let _ = criterion::black_box(proj_vec.get(i));
            }
        });
    });
}

criterion_group!(bench_get, bench_typed_proj_vec_get, bench_vec_get);
