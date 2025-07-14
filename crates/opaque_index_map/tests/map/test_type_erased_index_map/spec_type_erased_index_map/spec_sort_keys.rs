use crate::map::common::erased::strategy_type_erased_index_map_max_len;
use opaque_index_map::TypeErasedIndexMap;

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

fn prop_sort_keys_contains_key<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map = {
        let mut _sorted_map = entries.clone::<K, V, S, A>();
        _sorted_map.sort_keys::<K, V, S, A>();
        _sorted_map
    };

    for key in map.keys::<K, V, S, A>() {
        prop_assert!(sorted_map.contains_key::<_, K, V, S, A>(key));
    }

    for key in sorted_map.keys::<K, V, S, A>() {
        prop_assert!(map.contains_key::<_, K, V, S, A>(key));
    }

    Ok(())
}

fn prop_sort_keys_get1<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map = {
        let mut _sorted_map = entries.clone::<K, V, S, A>();
        _sorted_map.sort_keys::<K, V, S, A>();
        _sorted_map
    };

    for key in map.keys::<K, V, S, A>() {
        let expected = map.get::<_, K, V, S, A>(key);
        let result = sorted_map.get::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sort_keys_get2<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map = {
        let mut _sorted_map = entries.clone::<K, V, S, A>();
        _sorted_map.sort_keys::<K, V, S, A>();
        _sorted_map
    };

    for key in sorted_map.keys::<K, V, S, A>() {
        let expected = sorted_map.get::<_, K, V, S, A>(key);
        let result = map.get::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sort_keys_get_key_value1<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map = {
        let mut _sorted_map = entries.clone::<K, V, S, A>();
        _sorted_map.sort_keys::<K, V, S, A>();
        _sorted_map
    };

    for key in map.keys::<K, V, S, A>() {
        let expected = map.get_key_value::<_, K, V, S, A>(key);
        let result = sorted_map.get_key_value::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sort_keys_get_key_value2<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map = {
        let mut _sorted_map = entries.clone::<K, V, S, A>();
        _sorted_map.sort_keys::<K, V, S, A>();
        _sorted_map
    };

    for key in sorted_map.keys::<K, V, S, A>() {
        let expected = sorted_map.get_key_value::<_, K, V, S, A>(key);
        let result = map.get_key_value::<_, K, V, S, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sort_keys_len<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let sorted_map = {
        let mut _sorted_map = entries.clone::<K, V, S, A>();
        _sorted_map.sort_keys::<K, V, S, A>();
        _sorted_map
    };

    prop_assert_eq!(sorted_map.len(), map.len());

    Ok(())
}

fn prop_sort_keys_ordering<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let sorted_map = {
        let mut _sorted_map = entries.clone::<K, V, S, A>();
        _sorted_map.sort_keys::<K, V, S, A>();
        _sorted_map
    };

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
                fn prop_sort_keys_contains_key(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sort_keys_contains_key::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sort_keys_get1(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sort_keys_get1::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sort_keys_get2(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sort_keys_get2::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sort_keys_get_key_value1(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sort_keys_get_key_value1::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sort_keys_get_key_value2(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sort_keys_get_key_value2::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sort_keys_len(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sort_keys_len::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sort_keys_ordering(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_sort_keys_ordering::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
