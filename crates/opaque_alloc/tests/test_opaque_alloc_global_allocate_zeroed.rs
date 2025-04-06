#![feature(allocator_api)]
#![feature(slice_ptr_get)]
use std::alloc::{Allocator, Layout, Global};
use imgui_vulkan_renderer_opaque_alloc::OpaqueAlloc;

fn run_test_opaque_alloc_allocate_zeroed_with_layout(opaque_alloc: OpaqueAlloc, layout: Layout) {
    unsafe {
        let allocation_ptr = opaque_alloc
            .allocate_zeroed(layout.clone())
            .unwrap_or_else(|_| std::alloc::handle_alloc_error(layout));

        let base_ptr = allocation_ptr.as_non_null_ptr().as_ptr();
        for i in 0..layout.size() {
            let ptr = base_ptr.add(i);

            assert_eq!(*ptr, 0);
        }

        opaque_alloc.deallocate(allocation_ptr.as_non_null_ptr(), layout);
    }
}

fn run_test_opaque_alloc_allocate_zeroed_with_size_align(size: usize, align: usize) {
    let opaque_alloc = OpaqueAlloc::new::<Global>(Global);
    let layout = Layout::from_size_align(size, align).unwrap();

    run_test_opaque_alloc_allocate_zeroed_with_layout(opaque_alloc, layout);
}

#[test]
fn test_opaque_alloc_allocate_zeroed_with_size_1024_align_1() {
    run_test_opaque_alloc_allocate_zeroed_with_size_align(1024, 1);
}

#[test]
fn test_opaque_alloc_allocate_zeroed_with_size_1024_align_2() {
    run_test_opaque_alloc_allocate_zeroed_with_size_align(1024, 2);
}

#[test]
fn test_opaque_alloc_allocate_zeroed_with_size_1024_align_4() {
    run_test_opaque_alloc_allocate_zeroed_with_size_align(1024, 4);
}

#[test]
fn test_opaque_alloc_allocate_zeroed_with_size_1024_align_8() {
    run_test_opaque_alloc_allocate_zeroed_with_size_align(1024, 8);
}

#[test]
fn test_opaque_alloc_allocate_zeroed_with_size_1024_align_16() {
    run_test_opaque_alloc_allocate_zeroed_with_size_align(1024, 16);
}

#[test]
fn test_opaque_alloc_allocate_zeroed_with_size_1024_align_32() {
    run_test_opaque_alloc_allocate_zeroed_with_size_align(1024, 1);
}

#[test]
fn test_opaque_alloc_allocate_zeroed_with_size_1024_align_64() {
    run_test_opaque_alloc_allocate_zeroed_with_size_align(1024, 64);
}

#[test]
fn test_opaque_alloc_allocate_zeroed_with_size_1024_align_128() {
    run_test_opaque_alloc_allocate_zeroed_with_size_align(1024, 128);
}

#[test]
fn test_opaque_alloc_allocate_zeroed_with_size_1024_align_256() {
    run_test_opaque_alloc_allocate_zeroed_with_size_align(1024, 256);
}
