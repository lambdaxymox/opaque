use crate::map::common::projected::strategy_type_projected_index_map_max_len;
use opaque_index_map::TypeProjectedIndexMap;

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

fn prop_into_keys_contains_key<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    for key in map.clone().into_keys() {
        prop_assert!(map.contains_key(&key));
    }

    Ok(())
}

fn prop_into_keys_get<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    for key in map.clone().into_keys() {
        prop_assert!(map.get(&key).is_some());
    }

    Ok(())
}

fn prop_into_keys_get_full<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    for key in map.clone().into_keys() {
        prop_assert!(map.get_full(&key).is_some());
    }

    Ok(())
}

fn prop_into_keys_get_full_mut<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    let keys: Vec<K> = map.clone().into_keys().collect();
    for key in keys.iter() {
        prop_assert!(map.get_mut(key).is_some());
    }

    Ok(())
}

fn prop_into_keys_get_index<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    for (index, key) in map.clone().into_keys().enumerate() {
        let expected = Some(key.clone());
        let result = map.get_index(index).map(|(k, v)| k.clone());

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_keys_get_index_of<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    for (index, key) in map.clone().into_keys().enumerate() {
        let expected = Some(index);
        let result = map.get_index_of(&key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_keys_get_key_value<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    for key in map.clone().into_keys() {
        prop_assert!(map.get_key_value(&key).is_some());
    }

    Ok(())
}

fn prop_into_keys_get_mut<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    let keys: Vec<K> = map.clone().into_keys().collect();
    for key in keys.iter() {
        prop_assert!(map.get_mut(key).is_some());
    }

    Ok(())
}

fn prop_into_keys_ordering<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    let mut iter = map.clone().into_keys();
    for i in 0..map.len() {
        let expected = map.get_index(i).map(|(k, _v)| k).cloned();
        let result = iter.next();

        prop_assert_eq!(result, expected);
    }

    prop_assert_eq!(iter.next(), None);

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
                fn prop_into_keys_contains_key(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_keys_contains_key(entries)?
                }

                #[test]
                fn prop_into_keys_get(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_keys_get(entries)?
                }

                #[test]
                fn prop_into_keys_get_full(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_keys_get_full(entries)?
                }

                #[test]
                fn prop_into_keys_get_full_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_keys_get_full_mut(entries)?
                }

                #[test]
                fn prop_into_keys_get_index(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_keys_get_index(entries)?
                }

                #[test]
                fn prop_into_keys_get_index_of(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_keys_get_index_of(entries)?
                }

                #[test]
                fn prop_into_keys_get_key_value(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_keys_get_key_value(entries)?
                }

                #[test]
                fn prop_into_keys_get_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_keys_get_mut(entries)?
                }

                #[test]
                fn prop_into_keys_ordering(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_keys_ordering(entries)?
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
    strategy_type_projected_index_map_max_len,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
);
