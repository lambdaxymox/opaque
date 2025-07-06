use opaque_index_map::TypeErasedIndexMap;

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
fn test_type_erased_index_map_into_proj_correct_type1() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.into_proj::<usize, i32, hash::RandomState, alloc::Global>();
}

#[test]
fn test_type_erased_index_map_into_proj_correct_type2() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.into_proj::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_map_into_proj_correct_type3() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    for _ in 0..65536 {
        let proj_map = opaque_map.into_proj::<usize, i32, hash::RandomState, alloc::Global>();
        opaque_map = TypeErasedIndexMap::from_proj(proj_map);
    }
}

#[test]
fn test_type_erased_index_map_into_proj_correct_type4() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let proj_map = opaque_map.into_proj::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
        opaque_map = TypeErasedIndexMap::from_proj(proj_map);
    }
}

#[test]
fn test_type_erased_index_map_into_proj_correct_type5() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.into_proj::<usize, i32, hash::RandomState, alloc::Global>();
}

#[test]
fn test_type_erased_index_map_into_proj_correct_type6() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.into_proj::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_map_into_proj_correct_type7() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        alloc::Global,
    );
    for _ in 0..65536 {
        let proj_map = opaque_map.into_proj::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
        opaque_map = TypeErasedIndexMap::from_proj(proj_map);
    }
}

#[test]
fn test_type_erased_index_map_into_proj_correct_type8() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let proj_map = opaque_map.into_proj::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>();
        opaque_map = TypeErasedIndexMap::from_proj(proj_map);
    }
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type1() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.into_proj::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type2() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.into_proj::<usize, i32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type3() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.into_proj::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type4() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.into_proj::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type5() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.into_proj::<usize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type6() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.into_proj::<usize, u32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type7() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.into_proj::<usize, u32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type8() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.into_proj::<usize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type9() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.into_proj::<isize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type10() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.into_proj::<isize, i32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type11() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.into_proj::<isize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type12() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.into_proj::<isize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type13() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.into_proj::<isize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type14() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.into_proj::<isize, u32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type15() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.into_proj::<isize, u32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_into_proj_panics_wrong_type16() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.into_proj::<isize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_map_as_proj_correct_type1() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj::<usize, i32, hash::RandomState, alloc::Global>();
}

#[test]
fn test_type_erased_index_map_as_proj_correct_type2() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_map_as_proj_correct_type3() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    for _ in 0..65536 {
        let _ = opaque_map.as_proj::<usize, i32, hash::RandomState, alloc::Global>();
    }
}

#[test]
fn test_type_erased_index_map_as_proj_correct_type4() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let _ = opaque_map.as_proj::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
    }
}

#[test]
fn test_type_erased_index_map_as_proj_correct_type5() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32,  WrappingBuildHasher<hash::RandomState>, alloc::Global>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        alloc::Global,
    );
    let _ = opaque_map.as_proj::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
fn test_type_erased_index_map_as_proj_correct_type6() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32,  WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_map_as_proj_correct_type7() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        alloc::Global,
    );
    for _ in 0..65536 {
        let _ = opaque_map.as_proj::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
    }
}

#[test]
fn test_type_erased_index_map_as_proj_correct_type8() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let _ = opaque_map.as_proj::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>();
    }
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type1() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type2() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj::<usize, i32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type3() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type4() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type5() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj::<usize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type6() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj::<usize, u32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type7() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj::<usize, u32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type8() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj::<usize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type9() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj::<isize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type10() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj::<isize, i32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type11() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj::<isize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type12() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj::<isize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type13() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj::<isize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type14() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj::<isize, u32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type15() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj::<isize, u32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_panics_wrong_type16() {
    let opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj::<isize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_map_as_proj_mut_correct_type1() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj_mut::<usize, i32, hash::RandomState, alloc::Global>();
}

#[test]
fn test_type_erased_index_map_as_proj_mut_correct_type2() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj_mut::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_map_as_proj_mut_correct_type3() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    for _ in 0..65536 {
        let _ = opaque_map.as_proj_mut::<usize, i32, hash::RandomState, alloc::Global>();
    }
}

#[test]
fn test_type_erased_index_map_as_proj_mut_correct_type4() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let _ = opaque_map.as_proj_mut::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
    }
}

#[test]
fn test_type_erased_index_map_as_proj_mut_correct_type5() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32,  WrappingBuildHasher<hash::RandomState>, alloc::Global>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        alloc::Global,
    );
    let _ = opaque_map.as_proj_mut::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
fn test_type_erased_index_map_as_proj_mut_correct_type6() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32,  WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj_mut::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>();
}

#[test]
fn test_type_erased_index_map_as_proj_mut_correct_type7() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        alloc::Global,
    );
    for _ in 0..65536 {
        let _ = opaque_map.as_proj_mut::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
    }
}

#[test]
fn test_type_erased_index_map_as_proj_mut_correct_type8() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    for _ in 0..65536 {
        let _ = opaque_map.as_proj_mut::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>();
    }
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type1() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj_mut::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type2() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj_mut::<usize, i32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type3() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj_mut::<usize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type4() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj_mut::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type5() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj_mut::<usize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type6() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj_mut::<usize, u32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type7() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj_mut::<usize, u32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type8() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj_mut::<usize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type9() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj_mut::<isize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type10() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj_mut::<isize, i32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type11() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj_mut::<isize, i32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type12() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj_mut::<isize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type13() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj_mut::<isize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type14() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, WrappingAlloc<alloc::Global>>(
        hash::RandomState::default(),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj_mut::<isize, u32, hash::RandomState, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type15() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, hash::RandomState, alloc::Global>(
        hash::RandomState::default(),
        alloc::Global,
    );
    let _ = opaque_map.as_proj_mut::<isize, u32, WrappingBuildHasher<hash::RandomState>, alloc::Global>();
}

#[test]
#[should_panic]
fn test_type_erased_index_map_as_proj_mut_panics_wrong_type16() {
    let mut opaque_map = TypeErasedIndexMap::with_hasher_in::<usize, i32, WrappingBuildHasher<hash::RandomState>, WrappingAlloc<alloc::Global>>(
        WrappingBuildHasher::new(hash::RandomState::default()),
        WrappingAlloc::new(alloc::Global),
    );
    let _ = opaque_map.as_proj_mut::<isize, u32, hash::RandomState, WrappingAlloc<alloc::Global>>();
}
