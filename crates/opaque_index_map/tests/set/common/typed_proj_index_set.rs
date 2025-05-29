use opaque_index_map::set::TypedProjIndexSet;

use core::any;
use std::hash;
use std::alloc;

pub fn from_entries_in<T, S, A>(entries: &[T], build_hasher: S, alloc: A) -> TypedProjIndexSet<T, S, A>
where
    T: any::Any + Clone + Eq + hash::Hash,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut set = TypedProjIndexSet::with_hasher_in(build_hasher, alloc);
    for value in entries.iter().cloned() {
        set.insert(value);
    }

    set
}

pub fn from_entries_full_in<T, S, A>(entries: &[T], build_hasher: S, alloc: A) -> TypedProjIndexSet<T, S, A>
where
    T: any::Any + Clone + Eq + hash::Hash,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut set = TypedProjIndexSet::with_hasher_in(build_hasher, alloc);
    for value in entries.iter().cloned() {
        set.insert_full(value);
    }

    set
}
