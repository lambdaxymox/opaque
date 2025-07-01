use crate::set::common::erased::strategy_type_erased_index_set_max_len;
use opaque_index_map::OpaqueIndexSet;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn prop_iter_contains<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    for value in set.iter::<T, S, A>() {
        prop_assert!(set.contains::<_, T, S, A>(value));
    }

    Ok(())
}

fn prop_iter_get<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    for value in set.iter::<T, S, A>() {
        let expected = Some(value.clone());
        let result = set.get::<_, T, S, A>(value).cloned();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_iter_get_full<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    for value in set.iter::<T, S, A>() {
        let expected = Some(value.clone());
        let result = set.get_full::<_, T, S, A>(value).map(|(i, v)| v.clone());

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_iter_get_index<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    for (index, value) in set.iter::<T, S, A>().enumerate() {
        let expected = Some(value.clone());
        let result = set.get_index::<T, S, A>(index).map(|v| v.clone());

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_iter_get_index_of<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    for (index, value) in set.iter::<T, S, A>().enumerate() {
        let expected = Some(index);
        let result = set.get_index_of::<_, T, S, A>(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

macro_rules! generate_props {
    (
        $module_name:ident,
        $value_typ:ty,
        $build_hasher_typ:ty,
        $alloc_typ:ty,
        $max_length:expr,
        $set_gen:ident,
    ) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::hash;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_iter_contains(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_iter_contains::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_iter_get(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_iter_get::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_iter_get_full(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_iter_get_full::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_iter_get_index(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_iter_get_index::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_iter_get_index_of(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_iter_get_index_of::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }
            }
        }
    };
}

generate_props!(
    u64,
    u64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    usize,
    usize,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
