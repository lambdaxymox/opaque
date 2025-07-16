use crate::map::common::projected::strategy_type_projected_index_map_max_len;
use opaque_index_map::TypeProjectedIndexMap;

use core::any;
use core::fmt;
use std::string::String;
use std::{
    format,
    hash,
};

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_extract_if_len<F, K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&K, &V) -> bool,
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut remaining = entries.clone();
    let mut extracted = TypeProjectedIndexMap::with_hasher_proj_in(entries.hasher().clone(), entries.allocator().clone());
    for (key, value) in remaining.extract_if(.., |k, v| filter(k, v)) {
        extracted.shift_insert(extracted.len(), key, value);
    }

    prop_assert_eq!(extracted.len() + remaining.len(), entries.len());

    Ok(())
}

fn prop_extract_if_extracted<F, K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&K, &V) -> bool,
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut remaining = entries.clone();
    let mut extracted = TypeProjectedIndexMap::with_hasher_proj_in(entries.hasher().clone(), entries.allocator().clone());
    for (key, value) in remaining.extract_if(.., |k, v| filter(k, v)) {
        extracted.shift_insert(extracted.len(), key, value);
    }

    for (key, value) in extracted.iter() {
        prop_assert!(filter(key, value));
    }

    Ok(())
}

fn prop_extract_if_remaining<F, K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&K, &V) -> bool,
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut remaining = entries.clone();
    let mut extracted = TypeProjectedIndexMap::with_hasher_proj_in(entries.hasher().clone(), entries.allocator().clone());
    for (key, value) in remaining.extract_if(.., |k, v| filter(k, v)) {
        extracted.shift_insert(extracted.len(), key, value);
    }

    for (key, value) in remaining.iter() {
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
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_extract_if_len(entries, $filter)?
                }

                #[test]
                fn prop_extract_if_extracted(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_extract_if_extracted(entries, $filter)?
                }

                #[test]
                fn prop_extract_if_remaining(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_extract_if_remaining(entries, $filter)?
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
    |_k, v| { v % 2 == 0 },
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
    |_k, v| { v % 2 == 0 },
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
    |_k, v| { v % 2 == 0 },
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
    |k, _v| { k.len() % 2 == 0 },
);
