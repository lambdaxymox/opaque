#![feature(allocator_api)]
#![feature(slice_ptr_get)]
use std::alloc::{Allocator, Global, Layout};
use opaque_alloc::OpaqueAlloc;

fn run_test_opaque_alloc_allocate_align_with_layout_over_aligned_allocation(opaque_alloc: OpaqueAlloc, layout: Layout) {
    let expected = 0;
    let result = unsafe {
        let allocation_ptr = opaque_alloc
            .allocate(layout.clone())
            .unwrap_or_else(|_| std::alloc::handle_alloc_error(layout));

        let ptr = allocation_ptr.as_non_null_ptr().as_ptr();
        let ptr_align_offset = ptr.align_offset(layout.align());

        opaque_alloc.deallocate(allocation_ptr.as_non_null_ptr(), layout);

        ptr_align_offset
    };

    assert_eq!(result, expected);
}

fn run_test_opaque_alloc_allocate_align_over_aligned_allocation_with_size_align(size: usize, align: usize) {
    let opaque_alloc = OpaqueAlloc::new::<Global>(Global);
    let layout = Layout::from_size_align(size, align)
        .expect(&format!("Failed to construct layout with size `{:?}` and alignment `{:?}`", size, align));
    
    run_test_opaque_alloc_allocate_align_with_layout_over_aligned_allocation(opaque_alloc, layout);
}

#[test]
fn test_opaque_alloc_allocate_align_over_aligned_allocation_with_size_1024_align_2048() {
    run_test_opaque_alloc_allocate_align_over_aligned_allocation_with_size_align(1024, 2048);
}

#[test]
fn test_opaque_alloc_allocate_align_over_aligned_allocation_with_size_1024_align_4096() {
    run_test_opaque_alloc_allocate_align_over_aligned_allocation_with_size_align(1024, 4096);
}

#[test]
fn test_opaque_alloc_allocate_align_over_aligned_allocation_with_size_1024_align_8192() {
    run_test_opaque_alloc_allocate_align_over_aligned_allocation_with_size_align(1024, 8192);
}
