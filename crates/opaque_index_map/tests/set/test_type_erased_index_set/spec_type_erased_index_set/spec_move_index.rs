use crate::set::common::erased::{
    SingleBoundedValue,
    strategy_type_erased_index_set_len,
};
use opaque_index_map::TypeErasedIndexSet;

use core::any;
use core::fmt;
use std::{hash, ops};
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn strategy_prop_move_index<T, S, A>(max_length: usize) -> impl Strategy<Value = (TypeErasedIndexSet, usize, usize)>
where
    T: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn clamped_interval(max_length: usize) -> ops::RangeInclusive<usize> {
        if max_length == 0 {
            1..=1
        } else {
            1..=max_length
        }
    }

    clamped_interval(max_length).prop_flat_map(move |length| {
        let set = strategy_type_erased_index_set_len::<T, S, A>(length);
        (set, 0..length, 0..length)
    })
}

fn prop_move_index_eq<T, S, A>(entries: TypeErasedIndexSet, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone::<T, S, A>();
    set.move_index::<T, S, A>(from, to);

    prop_assert_eq!(set.as_proj::<T, S, A>(), entries.as_proj::<T, S, A>());

    Ok(())
}

fn prop_move_index_move_index<T, S, A>(entries: TypeErasedIndexSet, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone::<T, S, A>();
    set.move_index::<T, S, A>(from, to);
    set.move_index::<T, S, A>(to, from);

    prop_assert_eq!(set.as_slice::<T, S, A>(), entries.as_slice::<T, S, A>());

    Ok(())
}

fn prop_move_index_values<T, S, A>(entries: TypeErasedIndexSet, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone::<T, S, A>();
    set.move_index::<T, S, A>(from, to);


    prop_assert_eq!(set.get_index::<T, S, A>(to), entries.get_index::<T, S, A>(from));

    Ok(())
}

fn prop_move_index_len<T, S, A>(entries: TypeErasedIndexSet, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone::<T, S, A>();
    set.move_index::<T, S, A>(from, to);

    prop_assert_eq!(set.len(), entries.len());

    Ok(())
}

fn prop_move_index_get_index<T, S, A>(entries: TypeErasedIndexSet, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone::<T, S, A>();
    set.move_index::<T, S, A>(from, to);

    let entries_iter = (0..entries.len()).filter(|i| i != &from);
    let set_iter = (0..set.len()).filter(|j| j != &to);
    for (i, j) in entries_iter.zip(set_iter) {
        let expected = entries.get_index::<T, S, A>(i);
        let result = set.get_index::<T, S, A>(j);

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
            use super::*;
            proptest! {
                #[test]
                fn prop_move_index_eq((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_move_index_eq::<$value_typ, $build_hasher_typ, $alloc_typ>(entries, from, to)?
                }

                #[test]
                fn prop_move_index_move_index((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_move_index_move_index::<$value_typ, $build_hasher_typ, $alloc_typ>(entries, from, to)?
                }

                #[test]
                fn prop_move_index_values((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_move_index_values::<$value_typ, $build_hasher_typ, $alloc_typ>(entries, from, to)?
                }

                #[test]
                fn prop_move_index_len((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_move_index_len::<$value_typ, $build_hasher_typ, $alloc_typ>(entries, from, to)?
                }

                #[test]
                fn prop_move_index_get_index((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_move_index_get_index::<$value_typ, $build_hasher_typ, $alloc_typ>(entries, from, to)?
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
    strategy_prop_move_index,
);
generate_props!(
    usize,
    usize,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_move_index,
);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_move_index,
);
