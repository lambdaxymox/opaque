use crate::set::common::projected::{
    SingleBoundedValue,
    strategy_type_projected_index_set_len,
};
use opaque_index_map::TypeProjectedIndexSet;

use core::any;
use core::fmt;
use core::ops;
use std::format;
use std::hash;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn strategy_prop_swap_indices<T, S, A>(max_length: usize) -> impl Strategy<Value = (TypeProjectedIndexSet<T, S, A>, usize, usize)>
where
    T: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn clamped_interval(max_length: usize) -> ops::RangeInclusive<usize> {
        if max_length == 0 { 1..=1 } else { 1..=max_length }
    }

    clamped_interval(max_length).prop_flat_map(move |length| {
        let set = strategy_type_projected_index_set_len(length);
        (set, 0..length, 0..length)
    })
}

fn prop_swap_indices_eq<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, a: usize, b: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.swap_indices(a, b);

    prop_assert_eq!(set, entries);

    Ok(())
}

fn prop_swap_indices_swap_indices<T, S, A>(
    entries: TypeProjectedIndexSet<T, S, A>,
    a: usize,
    b: usize,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.swap_indices(a, b);
    set.swap_indices(a, b);

    prop_assert_eq!(set.as_slice(), entries.as_slice());

    Ok(())
}

fn prop_swap_indices_values<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, a: usize, b: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.swap_indices(a, b);


    prop_assert_eq!(set.get_index(a), entries.get_index(b));
    prop_assert_eq!(set.get_index(b), entries.get_index(a));

    Ok(())
}

fn prop_swap_indices_len<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, a: usize, b: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.swap_indices(a, b);

    prop_assert_eq!(set.len(), entries.len());

    Ok(())
}

fn prop_swap_indices_get_index<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, a: usize, b: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.swap_indices(a, b);

    for i in (0..set.len()).filter(|j| j != &a && j != &b) {
        let expected = entries.get_index(i);
        let result = set.get_index(i);

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
                fn prop_swap_indices_eq((entries, a, b) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_indices_eq(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_swap_indices((entries, a, b) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_indices_swap_indices(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_values((entries, a, b) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_indices_values(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_len((entries, a, b) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_indices_len(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_get_index((entries, a, b) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_indices_get_index(entries, a, b)?
                }
            }
        }
    };
}

generate_props!(u64, u64, hash::RandomState, alloc::Global, 32, strategy_prop_swap_indices,);
generate_props!(usize, usize, hash::RandomState, alloc::Global, 32, strategy_prop_swap_indices,);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_swap_indices,
);
