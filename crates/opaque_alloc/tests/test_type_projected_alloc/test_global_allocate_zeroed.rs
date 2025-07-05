use opaque_alloc::TypeProjectedAlloc;

use core::any;
use alloc_crate::format;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_polyfill::slice_ptr_get;

use alloc::Allocator;

#[cfg(feature = "nightly")]
fn run_test_type_projected_alloc_allocate_zeroed_with_layout<A>(proj_alloc: TypeProjectedAlloc<A>, layout: alloc::Layout)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    unsafe {
        let allocation_ptr = proj_alloc
            .allocate_zeroed(layout.clone())
            .unwrap_or_else(|_| alloc::handle_alloc_error(layout));

        let base_ptr = allocation_ptr.as_non_null_ptr().as_ptr();
        for i in 0..layout.size() {
            let ptr = base_ptr.add(i);

            assert_eq!(*ptr, 0);
        }

        proj_alloc.deallocate(allocation_ptr.as_non_null_ptr(), layout);
    }
}

#[cfg(not(feature = "nightly"))]
fn run_test_type_projected_alloc_allocate_zeroed_with_layout<A>(proj_alloc: TypeProjectedAlloc<A>, layout: alloc::Layout)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    unsafe {
        let allocation_ptr = proj_alloc
            .allocate_zeroed(layout.clone())
            .unwrap_or_else(|_| alloc::handle_alloc_error(layout));

        let base_ptr = slice_ptr_get::as_non_null_ptr(allocation_ptr).as_ptr();
        for i in 0..layout.size() {
            let ptr = base_ptr.add(i);

            assert_eq!(*ptr, 0);
        }

        proj_alloc.deallocate(slice_ptr_get::as_non_null_ptr(allocation_ptr), layout);
    }
}

fn run_test_type_projected_alloc_allocate_zeroed_with_size_align<A>(alloc: A, size: usize, align: usize)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let type_projected_alloc = TypeProjectedAlloc::new(alloc);
    let layout = alloc::Layout::from_size_align(size, align).expect(&format!(
        "Failed to construct layout with size `{:?}` and alignment `{:?}`",
        size, align
    ));

    run_test_type_projected_alloc_allocate_zeroed_with_layout::<A>(type_projected_alloc, layout);
}

#[test]
fn test_type_projected_alloc_allocate_zeroed_large() {
    let alloc = alloc::Global;
    let alignments = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
    let sizes = [1024, 2048, 4096, 8192];

    for size in sizes.iter().copied() {
        for align in alignments.iter().copied() {
            run_test_type_projected_alloc_allocate_zeroed_with_size_align(alloc, size, align);
        }
    }
}

#[test]
fn test_type_projected_alloc_allocate_zeroed_small() {
    let alloc = alloc::Global;
    let alignments = [1, 2, 4, 8];
    let sizes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512];

    for size in sizes.iter().copied() {
        for align in alignments.iter().copied() {
            run_test_type_projected_alloc_allocate_zeroed_with_size_align(alloc, size, align);
        }
    }
}
