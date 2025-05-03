#![feature(allocator_api)]
#![feature(slice_ptr_get)]
use opaque_alloc::OpaqueAlloc;
use std::alloc::{
    Allocator,
    Global,
    Layout,
};
use core::any;

fn run_test_opaque_alloc_allocate_align_with_layout<A>(opaque_alloc: OpaqueAlloc, layout: Layout)
where
    A: Allocator + any::Any,
{
    let expected = 0;
    let result = unsafe {
        let proj_alloc = opaque_alloc.as_proj::<A>();
        let allocation_ptr = proj_alloc
            .allocate(layout.clone())
            .unwrap_or_else(|_| std::alloc::handle_alloc_error(layout));

        let ptr = allocation_ptr.as_non_null_ptr().as_ptr();

        let ptr_align_offset = ptr.align_offset(layout.align());

        proj_alloc.deallocate(allocation_ptr.as_non_null_ptr(), layout);

        ptr_align_offset
    };

    assert_eq!(result, expected);
}

fn run_test_opaque_alloc_allocate_align_with_size_align<A>(alloc: A, size: usize, align: usize)
where
    A: Allocator + any::Any,
{
    let opaque_alloc = OpaqueAlloc::new::<A>(alloc);
    let layout = Layout::from_size_align(size, align).expect(&format!(
        "Failed to construct layout with size `{:?}` and alignment `{:?}`",
        size, align
    ));

    run_test_opaque_alloc_allocate_align_with_layout::<A>(opaque_alloc, layout);
}

#[test]
fn test_opaque_alloc_allocate_align_large() {
    let alloc = Global;
    let alignments = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
    let sizes = [1024, 2048, 4096, 8192];

    for size in sizes.iter().copied() {
        for align in alignments.iter().copied() {
            run_test_opaque_alloc_allocate_align_with_size_align(alloc, size, align);
        }
    }
}

#[test]
fn test_opaque_alloc_allocate_align_small() {
    let alloc = Global;
    let alignments = [1, 2, 4, 8];
    let sizes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512];

    for size in sizes.iter().copied() {
        for align in alignments.iter().copied() {
            run_test_opaque_alloc_allocate_align_with_size_align(alloc, size, align);
        }
    }
}
