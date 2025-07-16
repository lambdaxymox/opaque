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
        Self { alloc }
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
        unsafe { self.alloc.deallocate(ptr, layout) }
    }
}

#[test]
fn test_type_erased_alloc_into_proj_correct_type1() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.as_proj::<alloc::Global>();
}

#[test]
fn test_type_erased_alloc_into_proj_correct_type2() {
    let opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_alloc.into_proj::<WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_alloc_into_proj_correct_type3() {
    let mut opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    for _ in 0..65536 {
        let proj_alloc = opaque_alloc.into_proj::<alloc::Global>();
        opaque_alloc = TypeErasedAlloc::from_proj(proj_alloc);
    }
}

#[test]
fn test_type_erased_alloc_into_proj_correct_type4() {
    let mut opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    for _ in 0..65536 {
        let proj_alloc = opaque_alloc.into_proj::<WrappingAlloc<alloc::Global>>();
        opaque_alloc = TypeErasedAlloc::from_proj(proj_alloc);
    }
}

#[test]
#[should_panic]
fn test_type_erased_alloc_into_proj_panics_wrong_type1() {
    let opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_alloc.into_proj::<alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_alloc_into_proj_panics_wrong_type2() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.into_proj::<WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_alloc_as_proj_correct_type1() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.as_proj::<alloc::Global>();
}

#[test]
fn test_type_erased_alloc_as_proj_correct_type2() {
    let opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_alloc.as_proj::<WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_alloc_as_proj_correct_type3() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    for _ in 0..65536 {
        let _ = opaque_alloc.as_proj::<alloc::Global>();
    }
}

#[test]
fn test_type_erased_alloc_as_proj_correct_type4() {
    let opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    for _ in 0..65536 {
        let _ = opaque_alloc.as_proj::<WrappingAlloc<alloc::Global>>();
    }
}

#[test]
#[should_panic]
fn test_type_erased_alloc_as_proj_panics_wrong_type1() {
    let opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_alloc.as_proj::<alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_alloc_as_proj_panics_wrong_type2() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.as_proj::<WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_alloc_as_proj_mut_correct_type1() {
    let mut opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.as_proj_mut::<alloc::Global>();
}

#[test]
fn test_type_erased_alloc_as_proj_mut_correct_type2() {
    let mut opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_alloc.as_proj_mut::<WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_alloc_as_proj_mut_correct_type3() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    for _ in 0..65536 {
        let _ = opaque_alloc.as_proj::<alloc::Global>();
    }
}

#[test]
fn test_type_erased_alloc_as_proj_mut_correct_type4() {
    let opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    for _ in 0..65536 {
        let _ = opaque_alloc.as_proj::<WrappingAlloc<alloc::Global>>();
    }
}

#[test]
#[should_panic]
fn test_type_erased_alloc_as_proj_mut_panics_wrong_type1() {
    let mut opaque_alloc = TypeErasedAlloc::new::<WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_alloc.as_proj_mut::<alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_alloc_as_proj_mut_panics_wrong_type2() {
    let mut opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.as_proj_mut::<WrappingAlloc<alloc::Global>>();
}
