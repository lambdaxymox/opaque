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

fn prop_swap_remove_index_contains_key<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();
    let keys: Vec<K> = map.keys::<K, V, S, A>().cloned().collect();
    for key in keys.iter() {
        prop_assert!(map.contains_key::<_, K, V, S, A>(key));

        let index = map.get_index_of::<_, K, V, S, A>(key).unwrap();
        map.swap_remove_index::<K, V, S, A>(index);

        prop_assert!(!map.contains_key::<_, K, V, S, A>(key));
    }

    Ok(())
}

fn prop_swap_remove_index_get<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();
    let keys: Vec<K> = map.keys::<K, V, S, A>().cloned().collect();
    for key in keys.iter() {
        let expected = map.get::<_, K, V, S, A>(key).cloned();
        let index = map.get_index_of::<_, K, V, S, A>(key).unwrap();
        let result = map.swap_remove_index::<K, V, S, A>(index).map(|(_k, v)| v);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_swap_remove_index_get_key_value<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();
    let keys: Vec<K> = map.keys::<K, V, S, A>().cloned().collect();
    for key in keys.iter() {
        let expected = map
            .get_key_value::<_, K, V, S, A>(key)
            .map(|(k, v)| (k.clone(), v.clone()));
        let index = map.get_index_of::<_, K, V, S, A>(key).unwrap();
        let result = map.swap_remove_index::<K, V, S, A>(index);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_swap_remove_index_get_mut<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();
    let keys: Vec<K> = map.keys::<K, V, S, A>().cloned().collect();
    for key in keys.iter() {
        let expected = map.get_mut::<_, K, V, S, A>(key).cloned();
        let index = map.get_index_of::<_, K, V, S, A>(key).unwrap();
        let result = map.swap_remove_index::<K, V, S, A>(index).map(|(_k, v)| v);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_swap_remove_index_len<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();
    let keys: Vec<K> = map.keys::<K, V, S, A>().cloned().collect();
    for (i, key_i) in keys.iter().enumerate() {
        let index = map.get_index_of::<_, K, V, S, A>(key_i).unwrap();
        map.swap_remove_index::<K, V, S, A>(index);

        let expected = keys.len() - i - 1;
        let result = map.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_swap_remove_index_preserves_order<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<K, V, S, A>(map: &TypeErasedIndexMap, index: usize) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut entries = Vec::new();
        for entry in map.as_slice::<K, V, S, A>()[0..index]
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            entries.push(entry);
        }

        if index < map.len() - 1 {
            let last_entry = {
                let _entry = map.last::<K, V, S, A>().unwrap();
                (_entry.0.clone(), _entry.1.clone())
            };

            entries.push(last_entry);

            for entry in map
                .as_slice::<K, V, S, A>()[(index + 1)..(map.len() - 1)]
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
            {
                entries.push(entry);
            }
        }

        entries
    }

    fn result<K, V, S, A>(map: &TypeErasedIndexMap, key: &K) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut new_map = map.clone::<K, V, S, A>();
        let index = new_map.get_index_of::<_, K, V, S, A>(key).unwrap();
        new_map.swap_remove_index::<K, V, S, A>(index);

        let entries: Vec<(K, V)> = new_map
            .iter::<K, V, S, A>()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();

        entries
    }

    let base_map = entries.clone::<K, V, S, A>();
    let base_keys: Vec<K> = base_map.keys::<K, V, S, A>().cloned().collect();
    for (index, key) in base_keys.iter().enumerate() {
        let expected = expected::<K, V, S, A>(&entries, index);
        let result = result::<K, V, S, A>(&base_map, key);

        prop_assert_eq!(result, expected);
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
                fn prop_swap_remove_index_contains_key(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_remove_index_contains_key::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_swap_remove_index_get(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_remove_index_get::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_swap_remove_index_get_key_value(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_remove_index_get_key_value::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_swap_remove_index_get_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_remove_index_get_mut::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_swap_remove_index_len(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_remove_index_len::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_swap_remove_index_preserves_order(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_remove_index_preserves_order::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
