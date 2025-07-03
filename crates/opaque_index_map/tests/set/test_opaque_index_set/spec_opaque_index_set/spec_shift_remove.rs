use crate::set::common::erased::strategy_type_erased_index_set_max_len;
use opaque_index_map::OpaqueIndexSet;

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

fn prop_shift_remove_contains<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone::<T, S, A>();
    let values: Vec<T> = set.iter::<T, S, A>().cloned().collect();
    for value in values.iter() {
        prop_assert!(set.contains::<_, T, S, A>(value));

        set.shift_remove::<_, T, S, A>(value);

        prop_assert!(!set.contains::<_, T, S, A>(value));
    }

    Ok(())
}

fn prop_shift_remove_get<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone::<T, S, A>();
    let values: Vec<T> = set.iter::<T, S, A>().cloned().collect();
    for value in values.iter() {
        let expected = set.get::<_, T, S, A>(value).cloned();

        prop_assert!(expected.is_some());

        set.shift_remove::<_, T, S, A>(value);

        let result = set.get::<_, T, S, A>(value);

        prop_assert!(result.is_none());
    }

    Ok(())
}

fn prop_shift_remove_len<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone::<T, S, A>();
    let values: Vec<T> = set.iter::<T, S, A>().cloned().collect();
    for (i, value_i) in values.iter().enumerate() {
        set.shift_remove::<_, T, S, A>(value_i);

        let expected = values.len() - i - 1;
        let result = set.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_remove_preserves_order<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<T, S, A>(set: &OpaqueIndexSet, index: usize, value: &T) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut set_entries: Vec<T> = set.iter::<T, S, A>().cloned().collect();
        set_entries.remove(index);

        set_entries
    }

    fn result<T, S, A>(set: &OpaqueIndexSet, value: &T) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut new_set = set.clone::<T, S, A>();
        new_set.shift_remove::<_, T, S, A>(value);

        let ordered_entries: Vec<T> = new_set
            .iter::<T, S, A>()
            .cloned()
            .collect();

        ordered_entries
    }
    
    let base_set = entries.clone::<T, S, A>();
    let base_values: Vec<T> = base_set.iter::<T, S, A>().cloned().collect();
    for (index, value) in base_values.iter().enumerate() {
        let expected = expected::<T, S, A>(&entries, index, &value);
        let result = result::<T, S, A>(&base_set, value);

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
                fn prop_shift_remove_contains(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_shift_remove_contains::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_shift_remove_get(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_shift_remove_get::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_shift_remove_len(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_shift_remove_len::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_shift_remove_preserves_order(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_shift_remove_preserves_order::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
