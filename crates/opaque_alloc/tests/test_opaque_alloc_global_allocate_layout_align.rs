#![feature(allocator_api)]
#![feature(slice_ptr_get)]
use std::alloc::{Allocator, Global, Layout};
use imgui_vulkan_renderer_opaque_alloc::OpaqueAlloc;

fn run_test_opaque_alloc_allocate_align_with_layout(opaque_alloc: OpaqueAlloc, layout: Layout) {
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

fn run_test_opaque_alloc_allocate_align_with_size_align(size: usize, align: usize) {
    let opaque_alloc = OpaqueAlloc::new::<Global>(Global);
    let layout = Layout::from_size_align(size, align).unwrap();

    run_test_opaque_alloc_allocate_align_with_layout(opaque_alloc, layout);
}

#[test]
fn test_opaque_alloc_allocate_align_with_size_1024_align_1() {
    run_test_opaque_alloc_allocate_align_with_size_align(1024, 1);
}

#[test]
fn test_opaque_alloc_allocate_align_with_size_1024_align_2() {
    run_test_opaque_alloc_allocate_align_with_size_align(1024, 2);
}

#[test]
fn test_opaque_alloc_allocate_align_with_size_1024_align_4() {
    run_test_opaque_alloc_allocate_align_with_size_align(1024, 4);
}

#[test]
fn test_opaque_alloc_allocate_align_with_size_1024_align_8() {
    run_test_opaque_alloc_allocate_align_with_size_align(1024, 8);
}

#[test]
fn test_opaque_alloc_allocate_align_with_size_1024_align_16() {
    run_test_opaque_alloc_allocate_align_with_size_align(1024, 16);
}

#[test]
fn test_opaque_alloc_allocate_align_with_size_1024_align_32() {
    run_test_opaque_alloc_allocate_align_with_size_align(1024, 32);
}

#[test]
fn test_opaque_alloc_allocate_align_with_size_1024_align_64() {
    run_test_opaque_alloc_allocate_align_with_size_align(1024, 64);
}

#[test]
fn test_opaque_alloc_allocate_align_with_size_1024_align_128() {
    run_test_opaque_alloc_allocate_align_with_size_align(1024, 128);
}

#[test]
fn test_opaque_alloc_allocate_align_with_size_1024_align_256() {
    run_test_opaque_alloc_allocate_align_with_size_align(1024, 256);
}
