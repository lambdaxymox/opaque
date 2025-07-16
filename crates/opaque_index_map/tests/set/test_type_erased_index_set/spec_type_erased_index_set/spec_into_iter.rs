use crate::set::common::erased::strategy_type_erased_index_set_max_len;
use opaque_index_map::TypeErasedIndexSet;

use core::any;
use core::fmt;
use std::hash;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_into_iter_contains<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    for value in set.clone::<T, S, A>().into_iter::<T, S, A>() {
        prop_assert!(set.contains::<_, T, S, A>(&value));
    }

    Ok(())
}

fn prop_into_iter_get<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    for value in set.clone::<T, S, A>().into_iter::<T, S, A>() {
        let expected = Some(value.clone());
        let result = set.get::<_, T, S, A>(&value).cloned();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_get_full<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    for value in set.clone::<T, S, A>().into_iter::<T, S, A>() {
        let expected = Some(value.clone());
        let result = set.get_full::<_, T, S, A>(&value).map(|(_i, v)| v.clone());

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_get_index<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    for (index, value) in set.clone::<T, S, A>().into_iter::<T, S, A>().enumerate() {
        let expected = Some(value.clone());
        let result = set.get_index::<T, S, A>(index).map(|v| v.clone());

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_get_index_of<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    for (index, value) in set.clone::<T, S, A>().into_iter::<T, S, A>().enumerate() {
        let expected = Some(index);
        let result = set.get_index_of::<_, T, S, A>(&value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_ordering<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    let mut iter = set.clone::<T, S, A>().into_iter::<T, S, A>();
    for i in 0..set.len() {
        let expected = set.get_index::<T, S, A>(i).cloned();
        let result = iter.next();

        prop_assert_eq!(result, expected);
    }

    assert_eq!(iter.next(), None);

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
            use super::*;
            proptest! {
                #[test]
                fn prop_into_iter_contains(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_into_iter_contains::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_get(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_into_iter_get::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_get_full(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_into_iter_get_full::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_get_index(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_into_iter_get_index::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_get_index_of(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_into_iter_get_index_of::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_into_iter_ordering(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_into_iter_ordering::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
