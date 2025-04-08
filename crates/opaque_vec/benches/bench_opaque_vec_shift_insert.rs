use criterion::{criterion_group, criterion_main, Criterion};
use opaque_vec::OpaqueVec;

fn bench_vec_shift_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_shift_insert_last", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1024 {
                vec.insert(i, criterion::black_box(dummy_data));
            }

            vec
        });
    });
}

fn bench_opaque_vec_shift_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("opaque_vec_shift_insert_last", |b| {
        b.iter(|| {
            let mut opaque_vec = OpaqueVec::new::<i32>();
            for i in 0..1024 {
                opaque_vec.replace_insert::<i32>(i, criterion::black_box(dummy_data));
            }

            opaque_vec
        });
    });
}

criterion_group!(opaque_vec_benches, bench_opaque_vec_shift_insert_last, bench_vec_shift_insert_last);
criterion_main!(opaque_vec_benches);
