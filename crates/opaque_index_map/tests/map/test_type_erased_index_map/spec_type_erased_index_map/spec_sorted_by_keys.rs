use crate::map::common::erased::strategy_type_erased_index_map_max_len;
use opaque_index_map::TypeErasedIndexMap;

use core::any;
use core::cmp;
use core::fmt;
use std::format;
use std::hash;
use std::string::String;
use std::vec::Vec;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn from_sorted_by_in<F, K, V, S, A>(entries: &TypeErasedIndexMap, cmp: F) -> TypeErasedIndexMap
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
    F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
{
    let mut map = TypeErasedIndexMap::with_capacity_and_hasher_proj_in::<K, V, S, A>(
        entries.len(),
        entries.hasher::<K, V, S, A>().clone(),
        entries.allocator::<K, V, S, A>().clone(),
    );

    for (key, value) in entries.clone::<K, V, S, A>().sorted_by::<F, K, V, S, A>(cmp) {
        map.insert::<K, V, S, A>(key, value);
    }

    map
}

fn prop_sorted_by_contains_key<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map: TypeErasedIndexMap = from_sorted_by_in::<_, K, V, S, A>(&entries, |k1, _v1, k2, _v2| k1.cmp(k2));

    for key in map.keys::<K, V, S, A>() {
        prop_assert!(sorted_map.contains_key::<_, K, V, S, A>(key));
    }

    for key in sorted_map.keys::<K, V, S, A>() {
        prop_assert!(map.contains_key::<_, K, V, S, A>(key));
    }

    Ok(())
}

fn prop_sorted_by_get1<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map: TypeErasedIndexMap = from_sorted_by_in::<_, K, V, S, A>(&entries, |k1, _v1, k2, _v2| k1.cmp(k2));

    for key in map.keys::<K, V, S, A>() {
        let expected = map.get::<_, K, V, S, A>(key);
        let result = sorted_map.get::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sorted_by_get2<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map: TypeErasedIndexMap = from_sorted_by_in::<_, K, V, S, A>(&entries, |k1, _v1, k2, _v2| k1.cmp(k2));

    for key in sorted_map.keys::<K, V, S, A>() {
        let expected = sorted_map.get::<_, K, V, S, A>(key);
        let result = map.get::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sorted_by_get_key_value1<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map: TypeErasedIndexMap = from_sorted_by_in::<_, K, V, S, A>(&entries, |k1, _v1, k2, _v2| k1.cmp(k2));

    for key in map.keys::<K, V, S, A>() {
        let expected = map.get_key_value::<_, K, V, S, A>(key);
        let result = sorted_map.get_key_value::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sorted_by_get_key_value2<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map: TypeErasedIndexMap = from_sorted_by_in::<_, K, V, S, A>(&entries, |k1, _v1, k2, _v2| k1.cmp(k2));

    for key in sorted_map.keys::<K, V, S, A>() {
        let expected = sorted_map.get_key_value::<_, K, V, S, A>(key);
        let result = map.get_key_value::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sorted_by_len<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map: TypeErasedIndexMap = from_sorted_by_in::<_, K, V, S, A>(&entries, |k1, _v1, k2, _v2| k1.cmp(k2));

    prop_assert_eq!(sorted_map.len(), map.len());

    Ok(())
}

fn prop_sorted_by_ordering<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let sorted_map: TypeErasedIndexMap = from_sorted_by_in::<_, K, V, S, A>(&entries, |k1, _v1, k2, _v2| k1.cmp(k2));

    let keys: Vec<K> = sorted_map.keys::<K, V, S, A>().cloned().collect();
    for i in 1..keys.len() {
        prop_assert!(keys[i - 1] <= keys[i]);
    }

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
                fn prop_sorted_by_contains_key(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sorted_by_contains_key::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sorted_by_get1(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sorted_by_get1::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sorted_by_get2(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sorted_by_get2::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sorted_by_get_key_value1(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sorted_by_get_key_value1::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sorted_by_get_key_value2(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sorted_by_get_key_value2::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sorted_by_len(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sorted_by_len::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sorted_by_ordering(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sorted_by_ordering::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
