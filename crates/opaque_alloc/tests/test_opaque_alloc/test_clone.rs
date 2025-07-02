use opaque_alloc::OpaqueAlloc;

use core::any;
use core::ptr::NonNull;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use allocator_api2::alloc;

fn clone<A>(opaque_alloc: OpaqueAlloc) -> OpaqueAlloc
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let proj_alloc = opaque_alloc.as_proj::<A>();
    let cloned_proj_alloc = proj_alloc.clone();
    OpaqueAlloc::from_proj(cloned_proj_alloc)
}

fn run_test_opaque_alloc_clone<A>(alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let expected = OpaqueAlloc::new(alloc);
    let _ = clone::<A>(expected);
}

#[test]
fn test_opaque_alloc_clone_global() {
    let alloc = alloc::Global;

    run_test_opaque_alloc_clone(alloc);
}

#[test]
fn test_opaque_alloc_clone_wrapping_alloc() {
    #[derive(Clone)]
    struct WrappingAlloc<A> {
        alloc: A,
    }

    impl<A> WrappingAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        fn new(alloc: A) -> Self {
            Self { alloc, }
        }
    }

    unsafe impl<A> alloc::Allocator for WrappingAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            self.alloc.allocate(layout)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
            unsafe {
                self.alloc.deallocate(ptr, layout)
            }
        }
    }

    let alloc = WrappingAlloc::new(alloc::Global);

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
