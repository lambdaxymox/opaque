use crate::set::common::projected::strategy_type_projected_index_set_max_len;
use opaque_index_map::TypeProjectedIndexSet;

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

fn prop_into_iter_contains<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    for value in set.clone().into_iter() {
        prop_assert!(set.contains(&value));
    }

    Ok(())
}

fn prop_into_iter_get<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    for value in set.clone().into_iter() {
        let expected = Some(value.clone());
        let result = set.get(&value).cloned();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_get_full<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    for value in set.clone().into_iter() {
        let expected = Some(value.clone());
        let result = set.get_full(&value).map(|(_i, v)| v.clone());

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_get_index<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    for (index, value) in set.clone().into_iter().enumerate() {
        let expected = Some(value.clone());
        let result = set.get_index(index).map(|v| v.clone());

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_get_index_of<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    for (index, value) in set.clone().into_iter().enumerate() {
        let expected = Some(index);
        let result = set.get_index_of(&value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_into_iter_ordering<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    let mut iter = set.clone().into_iter();
    for i in 0..set.len() {
        let expected = set.get_index(i).cloned();
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
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_iter_contains(entries)?
                }

                #[test]
                fn prop_into_iter_get(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_iter_get(entries)?
                }

                #[test]
                fn prop_into_iter_get_full(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_iter_get_full(entries)?
                }

                #[test]
                fn prop_into_iter_get_index(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_iter_get_index(entries)?
                }

                #[test]
                fn prop_into_iter_get_index_of(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_iter_get_index_of(entries)?
                }

                #[test]
                fn prop_into_iter_ordering(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_into_iter_ordering(entries)?
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
    strategy_type_projected_index_set_max_len,
);
generate_props!(
    usize,
    usize,
    hash::RandomState,
    alloc::Global,
    32, 
    strategy_type_projected_index_set_max_len,
);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    32, 
    strategy_type_projected_index_set_max_len,
);
