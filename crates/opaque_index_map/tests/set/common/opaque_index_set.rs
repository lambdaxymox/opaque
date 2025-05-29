use opaque_index_map::set::OpaqueIndexSet;

use core::any;
use std::hash;
use std::alloc;

pub fn from_entries_in<T, S, A>(entries: &[T], build_hasher: S, alloc: A) -> OpaqueIndexSet
where
    T: any::Any + Clone + Eq + hash::Hash,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut set = OpaqueIndexSet::with_hasher_in::<T, S, A>(build_hasher, alloc);
    for value in entries.iter().cloned() {
        set.insert::<T, hash::RandomState, alloc::Global>(value);
    }

    set
}

pub fn from_entries_full_in<T, S, A>(entries: &[T], build_hasher: S, alloc: A) -> OpaqueIndexSet
where
    T: any::Any + Clone + Eq + hash::Hash,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut set = OpaqueIndexSet::with_hasher_in::<T, S, A>(build_hasher, alloc);
    for value in entries.iter().cloned() {
        set.insert_full::<T, hash::RandomState, alloc::Global>(value);
    }

    set
}

pub fn clone<T, S, A>(opaque_set: &OpaqueIndexSet) -> OpaqueIndexSet
where
    T: any::Any + Clone + Eq + hash::Hash,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let proj_set = opaque_set.as_proj::<T, S, A>();
    let cloned_proj_set = proj_set.clone();

    OpaqueIndexSet::from_proj(cloned_proj_set)
}
