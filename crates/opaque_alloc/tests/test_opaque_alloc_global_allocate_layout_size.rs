#![feature(allocator_api)]
#![feature(slice_ptr_get)]
use opaque_alloc::OpaqueAlloc;
use std::alloc::{
    Allocator,
    Global,
    Layout,
};

fn run_test_opaque_alloc_allocate_size_with_layout(opaque_alloc: OpaqueAlloc, layout: Layout) {
    let expected = layout.size();
    let result = unsafe {
        let allocation_ptr = opaque_alloc
            .allocate(layout.clone())
            .unwrap_or_else(|_| std::alloc::handle_alloc_error(layout));

        let allocation_size = allocation_ptr.len();

        opaque_alloc.deallocate(allocation_ptr.as_non_null_ptr(), layout);

        allocation_size
    };

    assert_eq!(result, expected);
}

fn run_test_opaque_alloc_allocate_size_with_size_align(size: usize, align: usize) {
    let opaque_alloc = OpaqueAlloc::new::<Global>(Global);
    let layout = Layout::from_size_align(size, align).expect(&format!(
        "Failed to construct layout with size `{:?}` and alignment `{:?}`",
        size, align
    ));

    run_test_opaque_alloc_allocate_size_with_layout(opaque_alloc, layout);
}

#[test]
fn test_opaque_alloc_allocate_align_large() {
    let alignments = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
    let sizes = [1024, 2048, 4096, 8192];

    for size in sizes.iter().copied() {
        for align in alignments.iter().copied() {
            run_test_opaque_alloc_allocate_size_with_size_align(size, align);
        }
    }
}

#[test]
fn test_opaque_alloc_allocate_align_small() {
    let alignments = [1, 2, 4, 8];
    let sizes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512];

    for size in sizes.iter().copied() {
        for align in alignments.iter().copied() {
            run_test_opaque_alloc_allocate_size_with_size_align(size, align);
        }
    }
}
