use opaque_hash::TypeErasedBuildHasher;

use criterion;
use criterion::criterion_group;

use std::hash::{
    BuildHasher,
    Hash,
    Hasher,
    RandomState,
};

macro_rules! bench_hasher {
    ($bench_name:ident, $bench_type_erased_name:ident, $typ:ty, $value:expr) => {
        fn $bench_name(c: &mut criterion::Criterion) {
            let default_build_hasher = RandomState::new();
            let value: $typ = $value;

            c.bench_function(stringify!($bench_name), |b| {
                b.iter(|| {
                    let mut hasher = default_build_hasher.build_hasher();
                    core::hint::black_box(value).hash(&mut hasher);
                    core::hint::black_box(hasher.finish());
                });
            });
        }

        fn $bench_type_erased_name(c: &mut criterion::Criterion) {
            let default_build_hasher = RandomState::new();
            let opaque_build_hasher = TypeErasedBuildHasher::new(default_build_hasher);
            let value: $typ = $value;

            c.bench_function(stringify!($bench_type_erased_name), |b| {
                b.iter(|| {
                    let mut opaque_hasher = opaque_build_hasher.build_hasher_proj::<RandomState>();
                    core::hint::black_box(value).hash(&mut opaque_hasher);
                    core::hint::black_box(opaque_hasher.finish());
                });
            });
        }
    };
}

bench_hasher!(bench_default_hasher_i8, bench_type_erased_default_hasher_i8, i8, i8::MAX);
bench_hasher!(bench_default_hasher_i16, bench_type_erased_default_hasher_i16, i16, i16::MAX);
bench_hasher!(bench_default_hasher_i32, bench_type_erased_default_hasher_i32, i32, i32::MAX);
bench_hasher!(bench_default_hasher_i64, bench_type_erased_default_hasher_i64, i64, i64::MAX);
bench_hasher!(
    bench_default_hasher_i128,
    bench_type_erased_default_hasher_i128,
    i128,
    i128::MAX
);
bench_hasher!(
    bench_default_hasher_isize,
    bench_type_erased_default_hasher_isize,
    isize,
    isize::MAX
);

bench_hasher!(
    bench_default_hasher_str1,
    bench_type_erased_default_hasher_str1,
    &str,
    "RA4Q8lJVNwU8G8En3LO2rR5xBAPur1uSGcLiO1IK"
);


criterion_group!(
    bench_type_erased_hasher,
    bench_default_hasher_i8,
    bench_type_erased_default_hasher_i8,
    bench_default_hasher_i16,
    bench_type_erased_default_hasher_i16,
    bench_default_hasher_i32,
    bench_type_erased_default_hasher_i32,
    bench_default_hasher_i64,
    bench_type_erased_default_hasher_i64,
    bench_default_hasher_i128,
    bench_type_erased_default_hasher_i128,
    bench_default_hasher_isize,
    bench_type_erased_default_hasher_isize,
    bench_default_hasher_str1,
    bench_type_erased_default_hasher_str1,
);
