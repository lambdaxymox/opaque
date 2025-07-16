use crate::map::common::erased::strategy_type_erased_index_map_max_len;
use opaque_index_map::TypeErasedIndexMap;

use core::any;
use core::fmt;
use std::{format, hash};
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_extract_if_len<F, K, V, S, A>(entries: TypeErasedIndexMap, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&K, &V) -> bool,
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut remaining = entries.clone::<K, V, S, A>();
    let mut extracted = TypeErasedIndexMap::with_hasher_proj_in::<K, V, S, A>(
        entries.hasher::<K, V, S, A>().clone(),
        entries.allocator::<K, V, S, A>().clone(),
    );
    for (key, value) in remaining.extract_if::<_, _, K, V, S, A>(.., |k, v| filter(k, v)) {
        extracted.shift_insert::<K, V, S, A>(extracted.len(), key, value);
    }

    prop_assert_eq!(extracted.len() + remaining.len(), entries.len());

    Ok(())
}

fn prop_extract_if_extracted<F, K, V, S, A>(entries: TypeErasedIndexMap, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&K, &V) -> bool,
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut remaining = entries.clone::<K, V, S, A>();
    let mut extracted = TypeErasedIndexMap::with_hasher_proj_in::<K, V, S, A>(
        entries.hasher::<K, V, S, A>().clone(),
        entries.allocator::<K, V, S, A>().clone(),
    );
    for (key, value) in remaining.extract_if::<_, _, K, V, S, A>(.., |k, v| filter(k, v)) {
        extracted.shift_insert::<K, V, S, A>(extracted.len(), key, value);
    }

    for (key, value) in extracted.iter::<K, V, S, A>() {
        prop_assert!(filter(key, value));
    }

    Ok(())
}

fn prop_extract_if_remaining<F, K, V, S, A>(entries: TypeErasedIndexMap, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&K, &V) -> bool,
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut remaining = entries.clone::<K, V, S, A>();
    let mut extracted = TypeErasedIndexMap::with_hasher_proj_in::<K, V, S, A>(
        entries.hasher::<K, V, S, A>().clone(),
        entries.allocator::<K, V, S, A>().clone(),
    );
    for (key, value) in remaining.extract_if::<_, _, K, V, S, A>(.., |k, v| filter(k, v)) {
        extracted.shift_insert::<K, V, S, A>(extracted.len(), key, value);
    }

    for (key, value) in remaining.iter::<K, V, S, A>() {
        prop_assert!(!filter(key, value));
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
        $filter:expr,
    ) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_extract_if_len(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_extract_if_len::<_, $key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries, $filter)?
                }

                #[test]
                fn prop_extract_if_extracted(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_extract_if_extracted::<_, $key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries, $filter)?
                }

                #[test]
                fn prop_extract_if_remaining(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_extract_if_remaining::<_, $key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries, $filter)?
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
    |_k, v| { v % 2 == 0 },
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
    |_k, v| { v % 2 == 0 },
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
    |_k, v| { v % 2 == 0 },
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
    |k, _v| { k.len() % 2 == 0 },
);
