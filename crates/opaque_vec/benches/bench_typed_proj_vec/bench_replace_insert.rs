use criterion::{
    Criterion,
    criterion_group,
};
use opaque_vec::TypedProjVec;

fn bench_vec_replace_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("vec_replace_insert_last", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for _ in 0..1024 {
                vec.push(criterion::black_box(dummy_data));
            }

            vec
        });
    });
}

fn bench_typed_proj_vec_replace_insert_last(c: &mut Criterion) {
    let dummy_data = 0_i32;

    c.bench_function("typed_proj_vec_replace_insert_last", |b| {
        b.iter(|| {
            let mut proj_vec = TypedProjVec::new();
            for _ in 0..1024 {
                let last_index = if proj_vec.len() == 0 { 0 } else { proj_vec.len() - 1 };
                proj_vec.replace_insert(last_index, criterion::black_box(dummy_data));
            }

            proj_vec
        });
    });
}

criterion_group!(
    bench_replace_insert,
    bench_typed_proj_vec_replace_insert_last,
    bench_vec_replace_insert_last
);
