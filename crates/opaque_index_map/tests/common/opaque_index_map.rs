use opaque_index_map::map::OpaqueIndexMap;

use core::any;
use std::hash;
use std::alloc;

pub fn from_entries_in<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A) -> OpaqueIndexMap
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut map = OpaqueIndexMap::with_hasher_in::<K, V, S, A>(build_hasher, alloc);
    for (key, value) in entries.iter().cloned() {
        map.insert::<K, V, hash::RandomState, alloc::Global>(key, value);
    }

    map
}

pub fn from_entries_full_in<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A) -> OpaqueIndexMap
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut map = OpaqueIndexMap::with_hasher_in::<K, V, S, A>(build_hasher, alloc);
    for (key, value) in entries.iter().cloned() {
        map.insert_full::<K, V, hash::RandomState, alloc::Global>(key, value);
    }

    map
}

pub fn clone<K, V, S, A>(opaque_map: &OpaqueIndexMap) -> OpaqueIndexMap
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let proj_map = opaque_map.as_proj::<K, V, S, A>();
    let cloned_proj_map = proj_map.clone();

    OpaqueIndexMap::from_proj(cloned_proj_map)
}
