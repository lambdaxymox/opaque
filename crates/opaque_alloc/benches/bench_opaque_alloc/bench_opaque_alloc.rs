use opaque_alloc::OpaqueAlloc;

use criterion;
use criterion::criterion_group;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use allocator_api2::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_polyfill::slice_ptr_get;

#[cfg(feature = "nightly")]
macro_rules! bench_alloc {
    ($bench_name:ident, $bench_opaque_name:ident, size => $size:expr, align => $align:expr) => {
        fn $bench_name(c: &mut criterion::Criterion) {
            use alloc::Allocator;
            let alloc = alloc::Global;
            let layout = alloc::Layout::from_size_align($size, $align).unwrap();

            c.bench_function(stringify!($bench_name), |b| {
                b.iter(|| unsafe {
                    let allocation_ptr = core::hint::black_box(alloc.allocate(layout.clone())).unwrap();
                    alloc.deallocate(allocation_ptr.as_non_null_ptr(), layout);
                });
            });
        }

        fn $bench_opaque_name(c: &mut criterion::Criterion) {
            use alloc::Allocator;
            let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
            let layout = alloc::Layout::from_size_align($size, $align).unwrap();

            c.bench_function(stringify!($bench_opaque_name), |b| {
                b.iter(|| unsafe {
                    let proj_alloc = opaque_alloc.as_proj::<alloc::Global>();
                    let allocation_ptr = core::hint::black_box(proj_alloc.allocate(layout.clone()).unwrap());
                    proj_alloc.deallocate(allocation_ptr.as_non_null_ptr(), layout);
                });
            });
        }
    };
}

#[cfg(not(feature = "nightly"))]
macro_rules! bench_alloc {
    ($bench_name:ident, $bench_opaque_name:ident, size => $size:expr, align => $align:expr) => {
        fn $bench_name(c: &mut criterion::Criterion) {
            use alloc::Allocator;
            let alloc = alloc::Global;
            let layout = alloc::Layout::from_size_align($size, $align).unwrap();

            c.bench_function(stringify!($bench_name), |b| {
                b.iter(|| unsafe {
                    let allocation_ptr = core::hint::black_box(alloc.allocate(layout.clone())).unwrap();
                    alloc.deallocate(slice_ptr_get::as_non_null_ptr(allocation_ptr), layout);
                });
            });
        }

        fn $bench_opaque_name(c: &mut criterion::Criterion) {
            use alloc::Allocator;
            let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
            let layout = alloc::Layout::from_size_align($size, $align).unwrap();

            c.bench_function(stringify!($bench_opaque_name), |b| {
                b.iter(|| unsafe {
                    let proj_alloc = opaque_alloc.as_proj::<alloc::Global>();
                    let allocation_ptr = core::hint::black_box(proj_alloc.allocate(layout.clone()).unwrap());
                    proj_alloc.deallocate(slice_ptr_get::as_non_null_ptr(allocation_ptr), layout);
                });
            });
        }
    };
}

bench_alloc!(bench_alloc_size_1_align_1,     bench_opaque_alloc_size_1_align_1,     size => 1,   align => 1);
bench_alloc!(bench_alloc_size_2_align_2,     bench_opaque_alloc_size_2_align_2,     size => 2,   align => 2);
bench_alloc!(bench_alloc_size_4_align_4,     bench_opaque_alloc_size_4_align_4,     size => 4,   align => 4);
bench_alloc!(bench_alloc_size_8_align_8,     bench_opaque_alloc_size_8_align_8,     size => 8,   align => 8);
bench_alloc!(bench_alloc_size_16_align_16,   bench_opaque_alloc_size_16_align_16,   size => 16,  align => 16);
bench_alloc!(bench_alloc_size_32_align_32,   bench_opaque_alloc_size_32_align_32,   size => 32,  align => 32);
bench_alloc!(bench_alloc_size_64_align_64,   bench_opaque_alloc_size_64_align_64,   size => 64,  align => 64);
bench_alloc!(bench_alloc_size_128_align_128, bench_opaque_alloc_size_128_align_128, size => 128, align => 128);
bench_alloc!(bench_alloc_size_256_align_256, bench_opaque_alloc_size_256_align_256, size => 256, align => 256);

criterion_group!(
    bench_opaque_alloc,
    bench_alloc_size_1_align_1,
    bench_opaque_alloc_size_1_align_1,
    bench_alloc_size_2_align_2,
    bench_opaque_alloc_size_2_align_2,
    bench_alloc_size_4_align_4,
    bench_opaque_alloc_size_4_align_4,
    bench_alloc_size_8_align_8,
    bench_opaque_alloc_size_8_align_8,
    bench_alloc_size_16_align_16,
    bench_opaque_alloc_size_16_align_16,
    bench_alloc_size_32_align_32,
    bench_opaque_alloc_size_32_align_32,
    bench_alloc_size_64_align_64,
    bench_opaque_alloc_size_64_align_64,
    bench_alloc_size_128_align_128,
    bench_opaque_alloc_size_128_align_128,
    bench_alloc_size_256_align_256,
    bench_opaque_alloc_size_256_align_256,
);
