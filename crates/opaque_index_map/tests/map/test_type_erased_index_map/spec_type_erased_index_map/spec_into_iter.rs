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

fn prop_into_iter_contains_key<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    for (key, _) in map.clone::<K, V, S, A>().into_iter::<K, V, S, A>() {
        prop_assert!(map.contains_key::<_, K, V, S, A>(&key));
    }

    Ok(())
}

fn prop_into_iter_get<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    for (key, value) in map.clone::<K, V, S, A>().into_iter::<K, V, S, A>() {
        let expected = Some(value.clone());
        let result = map.get::<_, K, V, S, A>(&key).cloned();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_get_full<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    for (key, value) in map.clone::<K, V, S, A>().into_iter::<K, V, S, A>() {
        let expected = Some((key.clone(), value.clone()));
        let result = map.get_full::<_, K, V, S, A>(&key).map(|(i, k, v)| (k.clone(), v.clone()));

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_get_index<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    for (index, (key, value)) in map.clone::<K, V, S, A>().into_iter::<K, V, S, A>().enumerate() {
        let expected = Some((key.clone(), value.clone()));
        let result = map.get_index::<K, V, S, A>(index).map(|(k, v)| (k.clone(), v.clone()));

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_get_index_of<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    for (index, (key, _)) in map.clone::<K, V, S, A>().into_iter::<K, V, S, A>().enumerate() {
        let expected = Some(index);
        let result = map.get_index_of::<_, K, V, S, A>(&key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_get_key_value<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    for (key, value) in map.clone::<K, V, S, A>().into_iter::<K, V, S, A>() {
        let expected = Some((key.clone(), value.clone()));
        let result = map.get_key_value::<_, K, V, S, A>(&key).map(|(k, v)| (k.clone(), v.clone()));

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_ordering<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let mut iter = map.clone::<K, V, S, A>().into_iter::<K, V, S, A>();
    for i in 0..map.len() {
        let expected = map.get_index::<K, V, S, A>(i).map(|(k, v)| (k.clone(), v.clone()));
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
                fn prop_into_iter_contains_key(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_into_iter_contains_key::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_get(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_into_iter_get::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_get_full(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_into_iter_get_full::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_get_index(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_into_iter_get_index::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_get_index_of(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_into_iter_get_index_of::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_get_key_value(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_into_iter_get_key_value::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_ordering(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_into_iter_ordering::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
