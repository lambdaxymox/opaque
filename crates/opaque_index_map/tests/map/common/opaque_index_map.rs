use opaque_index_map::map::OpaqueIndexMap;

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

pub fn from_entries_in<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A) -> OpaqueIndexMap
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut map = OpaqueIndexMap::with_hasher_in::<K, V, S, A>(build_hasher, alloc);
    for (key, value) in entries.iter().cloned() {
        map.insert::<K, V, S, A>(key, value);
    }

    map
}

pub fn from_entries_full_in<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A) -> OpaqueIndexMap
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut map = OpaqueIndexMap::with_hasher_in::<K, V, S, A>(build_hasher, alloc);
    for (key, value) in entries.iter().cloned() {
        map.insert_full::<K, V, S, A>(key, value);
    }

    map
}

pub fn clone<K, V, S, A>(opaque_map: &OpaqueIndexMap) -> OpaqueIndexMap
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let proj_map = opaque_map.as_proj::<K, V, S, A>();
    let cloned_proj_map = proj_map.clone();

    OpaqueIndexMap::from_proj(cloned_proj_map)
}
