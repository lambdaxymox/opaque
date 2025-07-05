use opaque_alloc::TypeErasedAlloc;

use core::any;
use core::ptr::NonNull;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

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

#[test]
fn test_type_erased_alloc_has_alloc_type1() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);

    assert!(opaque_alloc.has_allocator_type::<alloc::Global>());
}

#[test]
fn test_type_erased_alloc_has_alloc_type2() {
    let opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));

    assert!(opaque_alloc.has_allocator_type::<WrappingAlloc<alloc::Global>>());
}

#[test]
fn test_type_erased_alloc_into_proj_has_alloc_type1() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    let proj_alloc = opaque_alloc.into_proj::<alloc::Global>();
    let opaque_alloc = TypeErasedAlloc::from_proj(proj_alloc);

    assert!(opaque_alloc.has_allocator_type::<alloc::Global>());
}

#[test]
fn test_type_erased_alloc_into_proj_has_alloc_type2() {
    let opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let proj_alloc = opaque_alloc.into_proj::<WrappingAlloc<alloc::Global>>();
    let opaque_alloc = TypeErasedAlloc::from_proj(proj_alloc);

    assert!(opaque_alloc.has_allocator_type::<WrappingAlloc<alloc::Global>>());
}

#[test]
fn test_type_erased_alloc_not_has_alloc_type1() {
    let opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));

    assert!(!opaque_alloc.has_allocator_type::<alloc::Global>());
}

#[test]
fn test_type_erased_alloc_not_has_alloc_type2() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);

    assert!(!opaque_alloc.has_allocator_type::<WrappingAlloc<alloc::Global>>());
}

#[test]
fn test_type_erased_alloc_into_proj_not_has_alloc_type1() {
    let opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let proj_alloc = opaque_alloc.into_proj::<WrappingAlloc<alloc::Global>>();
    let opaque_alloc = TypeErasedAlloc::from_proj(proj_alloc);

    assert!(!opaque_alloc.has_allocator_type::<alloc::Global>());
}

#[test]
fn test_type_erased_alloc_into_proj_not_has_alloc_type2() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    let proj_alloc = opaque_alloc.into_proj::<alloc::Global>();
    let opaque_alloc = TypeErasedAlloc::from_proj(proj_alloc);

    assert!(!opaque_alloc.has_allocator_type::<WrappingAlloc<alloc::Global>>());
}
