use opaque_index_map::set::OpaqueIndexSet;

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
