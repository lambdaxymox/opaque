#![feature(allocator_api)]
#![feature(slice_ptr_get)]
use std::alloc::{Allocator, Global, Layout};
use opaque_alloc::OpaqueAlloc;

fn run_test_opaque_alloc_allocate_size_with_layout_over_aligned_allocation(opaque_alloc: OpaqueAlloc, layout: Layout) {
    let result = unsafe {
        let allocation_ptr = opaque_alloc
            .allocate(layout.clone())
            .unwrap_or_else(|_| std::alloc::handle_alloc_error(layout));

        let allocation_ptr_len = allocation_ptr.len();

        opaque_alloc.deallocate(allocation_ptr.as_non_null_ptr(), layout);

        allocation_ptr_len
    };

    assert!(
        result >= layout.size(),
        "Allocated size `{}` smaller than requested size `{}`",
        result,
        layout.size()
    );
}

fn run_test_opaque_alloc_allocate_size_over_aligned_allocation_with_size_align(size: usize, align: usize) {
    let opaque_alloc = OpaqueAlloc::new::<Global>(Global);
    let layout = Layout::from_size_align(size, align).unwrap();

    run_test_opaque_alloc_allocate_size_with_layout_over_aligned_allocation(opaque_alloc, layout);
}

#[test]
fn test_opaque_alloc_allocate_size_over_aligned_allocation_with_size_1024_align_1() {
    run_test_opaque_alloc_allocate_size_over_aligned_allocation_with_size_align(1024, 4096);
}

#[test]
fn test_opaque_alloc_allocate_size_over_aligned_allocation_with_size_1024_align_8192() {
    run_test_opaque_alloc_allocate_size_over_aligned_allocation_with_size_align(1024, 8192);
}
