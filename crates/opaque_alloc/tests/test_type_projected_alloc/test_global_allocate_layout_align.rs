use opaque_alloc::TypeProjectedAlloc;

use alloc_crate::format;
use core::any;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_polyfill::slice_ptr_get;

#[cfg(feature = "nightly")]
fn run_test_type_projected_alloc_allocate_align_with_layout<A>(proj_alloc: TypeProjectedAlloc<A>, layout: alloc::Layout)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    use alloc::Allocator;

    let expected = 0;
    let result = unsafe {
        let allocation_ptr = proj_alloc
            .allocate(layout.clone())
            .unwrap_or_else(|_| alloc::handle_alloc_error(layout));

        let ptr = allocation_ptr.as_non_null_ptr().as_ptr();

        let ptr_align_offset = ptr.align_offset(layout.align());

        proj_alloc.deallocate(allocation_ptr.as_non_null_ptr(), layout);

        ptr_align_offset
    };

    assert_eq!(result, expected);
}

#[cfg(not(feature = "nightly"))]
fn run_test_type_projected_alloc_allocate_align_with_layout<A>(proj_alloc: TypeProjectedAlloc<A>, layout: alloc::Layout)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    use alloc::Allocator;

    let expected = 0;
    let result = unsafe {
        let allocation_ptr = proj_alloc
            .allocate(layout.clone())
            .unwrap_or_else(|_| alloc::handle_alloc_error(layout));

        let ptr = slice_ptr_get::as_non_null_ptr(allocation_ptr).as_ptr();

        let ptr_align_offset = ptr.align_offset(layout.align());

        proj_alloc.deallocate(slice_ptr_get::as_non_null_ptr(allocation_ptr), layout);

        ptr_align_offset
    };

    assert_eq!(result, expected);
}

fn run_test_type_projected_alloc_allocate_align_with_size_align<A>(alloc: A, size: usize, align: usize)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_alloc = TypeProjectedAlloc::new(alloc);
    let layout = alloc::Layout::from_size_align(size, align).expect(&format!(
        "Failed to construct layout with size `{:?}` and alignment `{:?}`",
        size, align
    ));

    run_test_type_projected_alloc_allocate_align_with_layout::<A>(proj_alloc, layout);
}

#[test]
fn test_type_projected_alloc_allocate_align_large() {
    let alloc = alloc::Global;
    let alignments = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
    let sizes = [1024, 2048, 4096, 8192];

    for size in sizes.iter().copied() {
        for align in alignments.iter().copied() {
            run_test_type_projected_alloc_allocate_align_with_size_align(alloc, size, align);
        }
    }
}

#[test]
fn test_type_projected_alloc_allocate_align_small() {
    let alloc = alloc::Global;
    let alignments = [1, 2, 4, 8];
    let sizes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512];

    for size in sizes.iter().copied() {
        for align in alignments.iter().copied() {
            run_test_type_projected_alloc_allocate_align_with_size_align(alloc, size, align);
        }
    }
}
