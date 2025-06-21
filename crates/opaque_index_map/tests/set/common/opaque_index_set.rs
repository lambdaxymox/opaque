use opaque_index_map::set::OpaqueIndexSet;

use core::any;
use core::ptr::NonNull;
use std::hash;
use std::alloc;

#[derive(Debug, Clone, Copy, Default)]
pub struct WrappingAlloc1<A> {
    alloc: A,
}

impl<A> WrappingAlloc1<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub const fn new(alloc: A) -> Self {
        Self { alloc, }
    }
}

unsafe impl<A> alloc::Allocator for WrappingAlloc1<A>
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

#[derive(Debug, Clone, Copy, Default)]
pub struct WrappingAlloc2<A> {
    alloc: A,
}

impl<A> WrappingAlloc2<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub const fn new(alloc: A) -> Self {
        Self { alloc, }
    }
}

unsafe impl<A> alloc::Allocator for WrappingAlloc2<A>
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

pub fn from_entries_in<T, S, A>(entries: &[T], build_hasher: S, alloc: A) -> OpaqueIndexSet
where
    T: any::Any + Clone + Eq + hash::Hash,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut set = OpaqueIndexSet::with_hasher_in::<T, S, A>(build_hasher, alloc);
    for value in entries.iter().cloned() {
        set.insert::<T, S, A>(value);
    }

    set
}

pub fn from_entries_full_in<T, S, A>(entries: &[T], build_hasher: S, alloc: A) -> OpaqueIndexSet
where
    T: any::Any + Clone + Eq + hash::Hash,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut set = OpaqueIndexSet::with_hasher_in::<T, S, A>(build_hasher, alloc);
    for value in entries.iter().cloned() {
        set.insert_full::<T, S, A>(value);
    }

    set
}

pub fn clone<T, S, A>(opaque_set: &OpaqueIndexSet) -> OpaqueIndexSet
where
    T: any::Any + Clone + Eq + hash::Hash,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let proj_set = opaque_set.as_proj::<T, S, A>();
    let cloned_proj_set = proj_set.clone();

    OpaqueIndexSet::from_proj(cloned_proj_set)
}
