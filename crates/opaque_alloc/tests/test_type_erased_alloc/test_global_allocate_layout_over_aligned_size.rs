use opaque_alloc::TypeErasedAlloc;

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
fn run_test_type_erased_alloc_allocate_size_with_layout_over_aligned_allocation<A>(opaque_alloc: TypeErasedAlloc, layout: alloc::Layout)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let result = unsafe {
        let proj_alloc = opaque_alloc.as_proj::<A>();
        let allocation_ptr = proj_alloc
            .allocate(layout.clone())
            .unwrap_or_else(|_| alloc::handle_alloc_error(layout));

        let allocation_ptr_len = allocation_ptr.len();

        proj_alloc.deallocate(allocation_ptr.as_non_null_ptr(), layout);

        allocation_ptr_len
    };

    assert!(
        result >= layout.size(),
        "Allocated size `{}` smaller than requested size `{}`",
        result,
        layout.size()
    );
}

#[cfg(not(feature = "nightly"))]
fn run_test_type_erased_alloc_allocate_size_with_layout_over_aligned_allocation<A>(opaque_alloc: TypeErasedAlloc, layout: alloc::Layout)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let result = unsafe {
        let proj_alloc = opaque_alloc.as_proj::<A>();
        let allocation_ptr = proj_alloc
            .allocate(layout.clone())
            .unwrap_or_else(|_| alloc::handle_alloc_error(layout));

        let allocation_ptr_len = allocation_ptr.len();

        proj_alloc.deallocate(slice_ptr_get::as_non_null_ptr(allocation_ptr), layout);

        allocation_ptr_len
    };

    assert!(
        result >= layout.size(),
        "Allocated size `{}` smaller than requested size `{}`",
        result,
        layout.size()
    );
}

fn run_test_type_erased_alloc_allocate_size_over_aligned_allocation_with_size_align<A>(alloc: A, size: usize, align: usize)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let opaque_alloc = TypeErasedAlloc::new::<A>(alloc);
    let layout = alloc::Layout::from_size_align(size, align).expect(&format!(
        "Failed to construct layout with size `{:?}` and alignment `{:?}`",
        size, align
    ));

    run_test_type_erased_alloc_allocate_size_with_layout_over_aligned_allocation::<A>(opaque_alloc, layout);
}

#[test]
fn test_type_erased_alloc_allocate_size_over_aligned_allocation_with_size_1024_align_2048() {
    let alloc = alloc::Global;

    run_test_type_erased_alloc_allocate_size_over_aligned_allocation_with_size_align(alloc,1024, 2048);
}

#[test]
fn test_type_erased_alloc_allocate_size_over_aligned_allocation_with_size_1024_align_4096() {
    let alloc = alloc::Global;

    run_test_type_erased_alloc_allocate_size_over_aligned_allocation_with_size_align(alloc, 1024, 4096);
}

#[test]
fn test_type_erased_alloc_allocate_size_over_aligned_allocation_with_size_1024_align_8192() {
    let alloc = alloc::Global;

    run_test_type_erased_alloc_allocate_size_over_aligned_allocation_with_size_align(alloc, 1024, 8192);
}
