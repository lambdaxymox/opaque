use opaque_alloc::TypedProjAlloc;
use std::alloc;
use core::any;
use std::ptr::NonNull;



fn run_test_opaque_alloc_clone<A>(alloc: A)
where
    A: any::Any + alloc::Allocator + Clone,
{
    let expected = TypedProjAlloc::new(alloc);
    let _ = expected.clone();
}

#[test]
fn test_opaque_alloc_clone_global() {
    let alloc = alloc::Global;

    run_test_opaque_alloc_clone(alloc);
}

#[test]
fn test_opaque_alloc_clone_system() {
    let alloc = alloc::System;

    run_test_opaque_alloc_clone(alloc);
}

#[test]
fn test_opaque_alloc_clone_dummy_allocator() {
    #[derive(Clone)]
    struct DummyAlloc {}

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            alloc::Global.allocate(layout)
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
            unsafe {
                alloc::Global.deallocate(ptr, layout)
            }
        }
    }

    let alloc = DummyAlloc {};

    run_test_opaque_alloc_clone(alloc);
}
