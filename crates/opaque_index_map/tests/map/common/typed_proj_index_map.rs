use opaque_index_map::map::TypedProjIndexMap;

use core::any;
use core::ptr::NonNull;
use std::hash;
use std::alloc;

#[derive(Clone)]
pub struct WrappingBuildHasher1<S> {
    build_hasher: S,
}

impl<S> WrappingBuildHasher1<S> {
    #[inline]
    pub const fn new(build_hasher: S) -> Self {
        Self { build_hasher }
    }
}

impl<S> hash::BuildHasher for WrappingBuildHasher1<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
{
    type Hasher = S::Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.build_hasher.build_hasher()
    }
}

#[derive(Clone)]
pub struct WrappingBuildHasher2<S> {
    build_hasher: S,
}

impl<S> WrappingBuildHasher2<S> {
    #[inline]
    pub const fn new(build_hasher: S) -> Self {
        Self { build_hasher }
    }
}

impl<S> hash::BuildHasher for WrappingBuildHasher2<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
{
    type Hasher = S::Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.build_hasher.build_hasher()
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

pub fn from_entries_in<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A) -> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut map = TypedProjIndexMap::with_hasher_in(build_hasher, alloc);
    for (key, value) in entries.iter().cloned() {
        map.insert(key, value);
    }

    map
}

pub fn from_entries_full_in<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A) -> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut map = TypedProjIndexMap::with_hasher_in(build_hasher, alloc);
    for (key, value) in entries.iter().cloned() {
        map.insert_full(key, value);
    }

    map
}
