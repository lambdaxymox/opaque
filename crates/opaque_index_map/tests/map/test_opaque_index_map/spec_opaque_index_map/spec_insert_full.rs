use crate::map::common::erased::strategy_type_erased_index_map_max_len;
use opaque_index_map::OpaqueIndexMap;

use core::any;
use core::fmt;
use std::hash;
use std::vec::Vec;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn from_entries_insert_full_in<K, V, S, A>(entries: &OpaqueIndexMap) -> OpaqueIndexMap
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map: OpaqueIndexMap = OpaqueIndexMap::with_hasher_proj_in::<K, V, S, A>(
        entries.hasher::<K, V, S, A>().clone(),
        entries.allocator::<K, V, S, A>().clone(),
    );

    for (key, value) in entries
        .as_slice::<K, V, S, A>()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
    {
        map.insert_full::<K, V, S, A>(key, value);
    }

    map
}

fn prop_insert_full_as_mut_slice<K, V, S, A>(mut entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    let expected = entries.as_mut_slice::<K, V, S, A>();
    let result = map.as_mut_slice::<K, V, S, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_insert_full_as_slice<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    let expected = entries.as_slice::<K, V, S, A>();
    let result = map.as_slice::<K, V, S, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_insert_full_contains_key<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = OpaqueIndexMap::with_hasher_proj_in::<K, V, S, A>(
        entries.hasher::<K, V, S, A>().clone(),
        entries.allocator::<K, V, S, A>().clone()
    );

    for key in entries.iter::<K, V, S, A>().map(|tuple| tuple.0) {
        prop_assert!(!map.contains_key::<_, K, V, S, A>(key));
    }

    for (key, value) in entries.iter::<K, V, S, A>().map(|(k, v)| (k.clone(), v.clone())) {
        map.insert::<K, V, S, A>(key, value);
    }

    for key in entries.iter::<K, V, S, A>().map(|tuple| tuple.0) {
        prop_assert!(map.contains_key::<_, K, V, S, A>(key));
    }

    Ok(())
}

fn prop_insert_full_get<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    for (key, value) in entries.iter::<K, V, S, A>() {
        let expected = Some(value);
        let result = map.get::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_full<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    for (index, (key, value)) in entries.iter::<K, V, S, A>().enumerate() {
        let expected = Some((index, key, value));
        let result = map.get_full::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_mut<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    for (key, value) in entries.iter::<K, V, S, A>() {
        let mut cloned_value = value.clone();
        let expected = Some(&mut cloned_value);
        let result = map.get_mut::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_full_mut<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    for (index, (key, value)) in entries.iter::<K, V, S, A>().enumerate() {
        let mut cloned_value = value.clone();
        let expected = Some((index, key, &mut cloned_value));
        let result = map.get_full_mut::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_index_of<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    for (index, key) in entries.keys::<K, V, S, A>().enumerate() {
        let expected = Some(index);
        let result = map.get_index_of::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_key_value<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    for (key, value) in entries.iter::<K, V, S, A>() {
        let expected = Some((key, value));
        let result = map.get_key_value::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_iter<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    for (result, expected) in map.iter::<K, V, S, A>().zip(entries.iter::<K, V, S, A>()) {
        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_iter_mut<K, V, S, A>(mut entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    for (result, expected) in map.iter_mut::<K, V, S, A>().zip(entries.iter_mut::<K, V, S, A>()) {
        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_len<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    let expected = entries.len();
    let result = map.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_insert_full_values<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    let expected: Vec<V> = entries
        .values::<K, V, S, A>()
        .cloned()
        .collect();
    let result: Vec<V> = map
        .values::<K, V, S, A>()
        .cloned()
        .collect();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_insert_full_values_mut<K, V, S, A>(mut entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = from_entries_insert_full_in::<K, V, S, A>(&entries);
    let expected: Vec<V> = entries
        .values_mut::<K, V, S, A>()
        .map(|value| value.clone())
        .collect();
    let result: Vec<V> = map
        .values_mut::<K, V, S, A>()
        .map(|value| value.clone())
        .collect();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    (
        $module_name:ident,
        $key_typ:ty,
        $value_typ:ty,
        $build_hasher_typ:ty,
        $alloc_typ:ty,
        $max_length:expr,
        $map_gen:ident,
    ) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_insert_full_as_mut_slice(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_as_mut_slice::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_as_slice(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_as_slice::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_contains_key(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_contains_key::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_get(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_get::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_get_full(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_get_full::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_get_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_get_mut::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_get_full_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_get_full_mut::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_get_index_of(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_get_index_of::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_get_key_value(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_get_key_value::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_iter(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_iter::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_iter_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_iter_mut::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_len(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_len::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }


                #[test]
                fn prop_insert_full_values(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_values::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_full_values_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_insert_full_values_mut::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }
            }
        }
    };
}

generate_props!(
    u64_i64,
    u64,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
