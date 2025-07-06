use opaque_index_map::TypeErasedIndexSet;

use core::any;
use core::ptr::NonNull;
use std::hash;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[derive(Clone, Default, Debug)]
pub struct WrappingBuildHasher<S> {
    build_hasher: S,
}

impl<S> WrappingBuildHasher<S> {
    #[inline]
    pub const fn new(build_hasher: S) -> Self {
        Self { build_hasher }
    }
}

impl<S> hash::BuildHasher for WrappingBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
{
    type Hasher = S::Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.build_hasher.build_hasher()
    }
}

#[derive(Clone, Default)]
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
fn test_type_erased_index_set_into_proj_correct_type1() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.into_proj::<i32, hash::RandomState, alloc::Global>();
}

#[test]
fn test_type_erased_index_set_into_proj_correct_type2() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.into_proj::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_set_into_proj_correct_type3() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    for _ in 0..65536 {
        let proj_set = opaque_set.into_proj::<i32, hash::RandomState, alloc::Global>();
        opaque_set = TypeErasedIndexSet::from_proj(proj_set);
    }
}

#[test]
fn test_type_erased_index_set_into_proj_correct_type4() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let proj_set = opaque_set.into_proj::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
        opaque_set = TypeErasedIndexSet::from_proj(proj_set);
    }
}

#[test]
fn test_type_erased_index_set_into_proj_correct_type5() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.into_proj::<i32, hash::RandomState, alloc::Global>();
}

#[test]
fn test_type_erased_index_set_into_proj_correct_type6() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.into_proj::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_set_into_proj_correct_type7() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        alloc::Global,
    );
    for _ in 0..65536 {
        let proj_set = opaque_set.into_proj::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
        opaque_set = TypeErasedIndexSet::from_proj(proj_set);
    }
}

#[test]
fn test_type_erased_index_set_into_proj_correct_type8() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let proj_set = opaque_set.into_proj::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>();
        opaque_set = TypeErasedIndexSet::from_proj(proj_set);
    }
}

#[test]
#[should_panic]
fn test_type_erased_index_set_into_proj_panics_wrong_type1() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.into_proj::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_into_proj_panics_wrong_type2() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.into_proj::<i32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_into_proj_panics_wrong_type3() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.into_proj::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_into_proj_panics_wrong_type4() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.into_proj::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_into_proj_panics_wrong_type5() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.into_proj::<u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_into_proj_panics_wrong_type6() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.into_proj::<u32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_into_proj_panics_wrong_type7() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.into_proj::<u32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_into_proj_panics_wrong_type8() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.into_proj::<u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_set_as_proj_correct_type1() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.as_proj::<i32, hash::RandomState, alloc::Global>();
}

#[test]
fn test_type_erased_index_set_as_proj_correct_type2() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_set_as_proj_correct_type3() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    for _ in 0..65536 {
        let _ = opaque_set.as_proj::<i32, hash::RandomState, alloc::Global>();
    }
}

#[test]
fn test_type_erased_index_set_as_proj_correct_type4() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let _ = opaque_set.as_proj::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
    }
}

#[test]
fn test_type_erased_index_set_as_proj_correct_type5() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32,  WrappingBuildHasher<hash::RandomState>, alloc::Global>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        alloc::Global,
    );
    let _ = opaque_set.as_proj::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
fn test_type_erased_index_set_as_proj_correct_type6() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32,  WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_set_as_proj_correct_type7() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        alloc::Global,
    );
    for _ in 0..65536 {
        let _ = opaque_set.as_proj::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
    }
}

#[test]
fn test_type_erased_index_set_as_proj_correct_type8() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let _ = opaque_set.as_proj::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>();
    }
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_panics_wrong_type1() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.as_proj::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_panics_wrong_type2() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj::<i32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_panics_wrong_type3() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.as_proj::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_panics_wrong_type4() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_panics_wrong_type5() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.as_proj::<u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_panics_wrong_type6() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj::<u32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_panics_wrong_type7() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.as_proj::<u32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_panics_wrong_type8() {
    let opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj::<u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_set_as_proj_mut_correct_type1() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.as_proj_mut::<i32, hash::RandomState, alloc::Global>();
}

#[test]
fn test_type_erased_index_set_as_proj_mut_correct_type2() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj_mut::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_set_as_proj_mut_correct_type3() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    for _ in 0..65536 {
        let _ = opaque_set.as_proj_mut::<i32, hash::RandomState, alloc::Global>();
    }
}

#[test]
fn test_type_erased_index_set_as_proj_mut_correct_type4() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let _ = opaque_set.as_proj_mut::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
    }
}

#[test]
fn test_type_erased_index_set_as_proj_mut_correct_type5() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32,  WrappingBuildHasher<hash::RandomState>, alloc::Global>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        alloc::Global,
    );
    let _ = opaque_set.as_proj_mut::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
fn test_type_erased_index_set_as_proj_mut_correct_type6() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32,  WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj_mut::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_set_as_proj_mut_correct_type7() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        alloc::Global,
    );
    for _ in 0..65536 {
        let _ = opaque_set.as_proj_mut::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
    }
}

#[test]
fn test_type_erased_index_set_as_proj_mut_correct_type8() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let _ = opaque_set.as_proj_mut::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>();
    }
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_mut_panics_wrong_type1() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.as_proj_mut::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_mut_panics_wrong_type2() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj_mut::<i32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_mut_panics_wrong_type3() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.as_proj_mut::<i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_mut_panics_wrong_type4() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj_mut::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_mut_panics_wrong_type5() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.as_proj_mut::<u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_mut_panics_wrong_type6() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj_mut::<u32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_mut_panics_wrong_type7() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_set.as_proj_mut::<u32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_set_as_proj_mut_panics_wrong_type8() {
    let mut opaque_set = TypeErasedIndexSet::with_hasher_in::<i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_set.as_proj_mut::<u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}
