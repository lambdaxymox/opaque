use crate::set::common::erased::strategy_type_erased_index_set_max_len;
use opaque_index_map::TypeErasedIndexSet;

use core::any;
use core::fmt;
use std::{format, hash};
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_extract_if_len<F, T, S, A>(entries: TypeErasedIndexSet, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&T) -> bool,
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut remaining = entries.clone::<T, S, A>();
    let mut extracted = TypeErasedIndexSet::with_hasher_proj_in::<T, S, A>(
        entries.hasher::<T, S, A>().clone(),
        entries.allocator::<T, S, A>().clone(),
    );
    for value in remaining.extract_if::<_, _, T, S, A>(.., |v| filter(v)) {
        extracted.shift_insert::<T, S, A>(extracted.len(), value);
    }

    prop_assert_eq!(extracted.len() + remaining.len(), entries.len());

    Ok(())
}

fn prop_extract_if_extracted<F, T, S, A>(entries: TypeErasedIndexSet, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&T) -> bool,
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut remaining = entries.clone::<T, S, A>();
    let mut extracted = TypeErasedIndexSet::with_hasher_proj_in::<T, S, A>(
        entries.hasher::<T, S, A>().clone(),
        entries.allocator::<T, S, A>().clone(),
    );
    for value in remaining.extract_if::<_, _, T, S, A>(.., |v| filter(v)) {
        extracted.shift_insert::<T, S, A>(extracted.len(), value);
    }

    for value in extracted.iter::<T, S, A>() {
        prop_assert!(filter(value));
    }

    Ok(())
}

fn prop_extract_if_remaining<F, T, S, A>(entries: TypeErasedIndexSet, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&T) -> bool,
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut remaining = entries.clone::<T, S, A>();
    let mut extracted = TypeErasedIndexSet::with_hasher_proj_in::<T, S, A>(
        entries.hasher::<T, S, A>().clone(),
        entries.allocator::<T, S, A>().clone(),
    );
    for value in remaining.extract_if::<_, _, T, S, A>(.., |v| filter(v)) {
        extracted.shift_insert::<T, S, A>(extracted.len(), value);
    }

    for value in remaining.iter::<T, S, A>() {
        prop_assert!(!filter(value));
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
        $set_gen:ident,
        $filter:expr,
    ) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_extract_if_len(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_extract_if_len::<_, $value_typ, $build_hasher_typ, $alloc_typ>(entries, $filter)?
                }

                #[test]
                fn prop_extract_if_extracted(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_extract_if_extracted::<_, $value_typ, $build_hasher_typ, $alloc_typ>(entries, $filter)?
                }

                #[test]
                fn prop_extract_if_remaining(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_extract_if_remaining::<_, $value_typ, $build_hasher_typ, $alloc_typ>(entries, $filter)?
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
    strategy_type_erased_index_set_max_len,
    |v| { v % 2 == 0 },
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
    |v| { v % 2 == 0 },
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
    |v| { v % 2 == 0 },
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
    |v| { v.len() % 2 == 0 },
);
