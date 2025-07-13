use crate::set::common::projected::strategy_type_projected_index_set_max_len;
use opaque_index_map::TypeProjectedIndexSet;

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

fn prop_swap_remove_index_contains<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    let values: Vec<T> = set.iter().cloned().collect();
    for value in values.iter() {
        prop_assert!(set.contains(value));

        let index = set.get_index_of(value).unwrap();
        set.swap_remove_index(index);

        prop_assert!(!set.contains(value));
    }

    Ok(())
}

fn prop_swap_remove_index_get<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    let values: Vec<T> = set.iter().cloned().collect();
    for value in values.iter() {
        let expected = set.get(value).cloned();

        prop_assert!(expected.is_some());

        let index = set.get_index_of(value).unwrap();
        set.swap_remove_index(index);

        let result = set.get(value);

        prop_assert!(result.is_none());
    }

    Ok(())
}

fn prop_swap_remove_index_len<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    let values: Vec<T> = set.iter().cloned().collect();
    for (i, value_i) in values.iter().enumerate() {
        let index = set.get_index_of(value_i).unwrap();
        set.swap_remove_index(index);

        let expected = values.len() - i - 1;
        let result = set.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_swap_remove_index_preserves_order<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<T, S, A>(entries: &TypeProjectedIndexSet<T, S, A>, index: usize, value: &T) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut set_entries: Vec<T> = entries.iter().cloned().collect();

        set_entries.swap_remove(index);

        set_entries
    }

    fn result<T, S, A>(set: &TypeProjectedIndexSet<T, S, A>, value: &T) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut new_set = set.clone();
        let index = set.get_index_of(value).unwrap();
        new_set.swap_remove_index(index);

        let ordered_entries: Vec<T> = new_set
            .iter()
            .cloned()
            .collect();

        ordered_entries
    }
    
    let base_set = entries.clone();
    let base_values: Vec<T> = base_set.iter().cloned().collect();
    for (index, value) in base_values.iter().enumerate() {
        let expected = expected(&entries, index, &value);
        let result = result(&base_set, value);

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
                fn prop_swap_remove_index_contains(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_remove_index_contains(entries)?
                }

                #[test]
                fn prop_swap_remove_index_get(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_remove_index_get(entries)?
                }

                #[test]
                fn prop_swap_remove_index_len(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_remove_index_len(entries)?
                }

                #[test]
                fn prop_swap_remove_index_preserves_order(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_remove_index_preserves_order(entries)?
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
