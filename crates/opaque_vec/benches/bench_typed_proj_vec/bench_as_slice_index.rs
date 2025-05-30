use criterion::{
    Criterion,
    criterion_group,
};
use opaque_vec::TypedProjVec;

fn bench_vec_as_slice_index(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let mut vec = vec![dummy_data; 10000];

    c.bench_function("vec_as_slice_index", |b| {
        b.iter(|| {
            let slice = vec.as_slice();
            for i in 0..slice.len() {
                let _ = core::hint::black_box(slice[i]);
            }
        });
    });
}

fn bench_typed_proj_vec_as_slice_index(c: &mut Criterion) {
    let dummy_data = 0_i32;
    let proj_vec = TypedProjVec::from_iter((0..10000).map(|_| dummy_data));

    c.bench_function("typed_proj_vec_as_slice_index", |b| {
        b.iter(|| {
            let slice = proj_vec.as_slice();
            for i in 0..slice.len() {
                let _ = core::hint::black_box(slice[i]);
            }
        });
    });
}

criterion_group!(bench_as_slice_index, bench_typed_proj_vec_as_slice_index, bench_vec_as_slice_index);
