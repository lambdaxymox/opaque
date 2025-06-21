use opaque_index_map::map::TypedProjIndexMap;

use core::any;
use std::hash;
use std::alloc;

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
