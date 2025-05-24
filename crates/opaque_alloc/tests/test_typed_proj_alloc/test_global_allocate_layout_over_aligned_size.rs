use opaque_alloc::TypedProjAlloc;
use std::alloc;
use std::alloc::{
    Allocator,
    Layout,
};
use core::any;

fn run_test_typed_proj_alloc_allocate_size_with_layout_over_aligned_allocation<A>(proj_alloc: TypedProjAlloc<A>, layout: Layout)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let result = unsafe {
        let allocation_ptr = proj_alloc
            .allocate(layout.clone())
            .unwrap_or_else(|_| std::alloc::handle_alloc_error(layout));

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

fn run_test_typed_proj_alloc_allocate_size_over_aligned_allocation_with_size_align<A>(alloc: A, size: usize, align: usize)
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_alloc = TypedProjAlloc::new(alloc);
    let layout = Layout::from_size_align(size, align).expect(&format!(
        "Failed to construct layout with size `{:?}` and alignment `{:?}`",
        size, align
    ));

    run_test_typed_proj_alloc_allocate_size_with_layout_over_aligned_allocation::<A>(proj_alloc, layout);
}

#[test]
fn test_typed_proj_alloc_allocate_size_over_aligned_allocation_with_size_1024_align_2048() {
    let alloc = alloc::Global;

    run_test_typed_proj_alloc_allocate_size_over_aligned_allocation_with_size_align(alloc,1024, 2048);
}

#[test]
fn test_typed_proj_alloc_allocate_size_over_aligned_allocation_with_size_1024_align_4096() {
    let alloc = alloc::Global;

    run_test_typed_proj_alloc_allocate_size_over_aligned_allocation_with_size_align(alloc, 1024, 4096);
}

#[test]
fn test_typed_proj_alloc_allocate_size_over_aligned_allocation_with_size_1024_align_8192() {
    let alloc = alloc::Global;

    run_test_typed_proj_alloc_allocate_size_over_aligned_allocation_with_size_align(alloc, 1024, 8192);
}
