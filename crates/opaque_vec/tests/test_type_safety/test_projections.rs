use opaque_vec::TypeErasedVec;

use core::any;
use core::ptr::NonNull;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[derive(Clone, Default)]
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
fn test_type_erased_vec_into_proj_correct_type1() {
    let opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.into_proj::<i32, alloc::Global>();
}

#[test]
fn test_type_erased_vec_into_proj_correct_type2() {
    let opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.into_proj::<i32, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_vec_into_proj_correct_type3() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    for _ in 0..65536 {
        let proj_vec = opaque_vec.into_proj::<i32, alloc::Global>();
        opaque_vec = TypeErasedVec::from_proj(proj_vec);
    }
}

#[test]
fn test_type_erased_vec_into_proj_correct_type4() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    for _ in 0..65536 {
        let proj_vec = opaque_vec.into_proj::<i32, WrappingAlloc<alloc::Global>>();
        opaque_vec = TypeErasedVec::from_proj(proj_vec);
    }
}

#[test]
#[should_panic]
fn test_type_erased_vec_into_proj_panics_wrong_type1() {
    let opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.into_proj::<i32, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_into_proj_panics_wrong_type2() {
    let opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.into_proj::<i32, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_into_proj_panics_wrong_type3() {
    let opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.into_proj::<u32, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_into_proj_panics_wrong_type4() {
    let opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.into_proj::<u32, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_into_proj_panics_wrong_type5() {
    let opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.into_proj::<u32, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_into_proj_panics_wrong_type6() {
    let opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.into_proj::<u32, alloc::Global>();
}

#[test]
fn test_type_erased_vec_as_proj_correct_type1() {
    let opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.as_proj::<i32, alloc::Global>();
}

#[test]
fn test_type_erased_vec_as_proj_correct_type2() {
    let opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.as_proj::<i32, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_vec_as_proj_correct_type3() {
    let opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    for _ in 0..65536 {
        let _ = opaque_vec.as_proj::<i32, alloc::Global>();
    }
}

#[test]
fn test_type_erased_vec_as_proj_correct_type4() {
    let opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    for _ in 0..65536 {
        let _ = opaque_vec.as_proj::<i32, WrappingAlloc<alloc::Global>>();
    }
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_panics_wrong_type1() {
    let opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.as_proj::<i32, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_panics_wrong_type2() {
    let opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.as_proj::<i32, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_panics_wrong_type3() {
    let opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.as_proj::<u32, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_panics_wrong_type4() {
    let opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.as_proj::<u32, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_panics_wrong_type5() {
    let opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.as_proj::<u32, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_panics_wrong_type6() {
    let opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.as_proj::<u32, alloc::Global>();
}

#[test]
fn test_type_erased_vec_as_proj_mut_correct_type1() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.as_proj_mut::<i32, alloc::Global>();
}

#[test]
fn test_type_erased_vec_as_proj_mut_correct_type2() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.as_proj_mut::<i32, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_vec_as_proj_mut_correct_type3() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    for _ in 0..65536 {
        let _ = opaque_vec.as_proj_mut::<i32, alloc::Global>();
    }
}

#[test]
fn test_type_erased_vec_as_proj_mut_correct_type4() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    for _ in 0..65536 {
        let _ = opaque_vec.as_proj_mut::<i32, WrappingAlloc<alloc::Global>>();
    }
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_mut_panics_wrong_type1() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.as_proj_mut::<i32, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_mut_panics_wrong_type2() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.as_proj_mut::<i32, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_mut_panics_wrong_type3() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.as_proj_mut::<u32, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_mut_panics_wrong_type4() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.as_proj_mut::<u32, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_mut_panics_wrong_type5() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, alloc::Global>(alloc::Global);
    let _ = opaque_vec.as_proj_mut::<u32, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_vec_as_proj_mut_panics_wrong_type6() {
    let mut opaque_vec = TypeErasedVec::new_in::<i32, WrappingAlloc<alloc::Global>>(WrappingAlloc::new(alloc::Global));
    let _ = opaque_vec.as_proj_mut::<u32, alloc::Global>();
}
