use crate::set::common;
use crate::set::common::projected::{
    WrappingBuildHasher1,
    WrappingBuildHasher2,
    strategy_type_projected_index_set_max_len,
};
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

fn prop_append_contains_source<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone();
    let mut destination = entries2.clone();

    source.append(&mut destination);

    for value in entries1.iter() {
        prop_assert!(source.contains(value));
    }

    for value in entries2.iter() {
        prop_assert!(source.contains(value));
    }

    Ok(())
}

fn prop_append_contains_destination<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone();
    let mut destination = entries2.clone();

    source.append(&mut destination);

    for value in entries1.iter() {
        prop_assert!(!destination.contains(value));
    }

    for value in entries2.iter() {
        prop_assert!(!destination.contains(value));
    }

    Ok(())
}

fn prop_append_get_source<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone();
    let mut destination = entries2.clone();

    source.append(&mut destination);

    let expected_vec = {
        let mut entries = Vec::from_iter(entries1.iter().cloned());
        entries.extend(entries2.iter().cloned());

        entries
    };

    for value in expected_vec.iter() {
        let expected = Some(value);
        let result = source.get(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_append_get_destination<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone();
    let mut destination = entries2.clone();

    source.append(&mut destination);

    for value in entries1.iter() {
        prop_assert!(destination.get(value).is_none());
    }

    for value in entries2.iter() {
        prop_assert!(destination.get(value).is_none());
    }

    Ok(())
}

fn prop_append_get_full_source<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone();
    let mut destination = entries2.clone();

    source.append(&mut destination);

    let expected_vec = {
        let mut entries = Vec::from_iter(entries1.iter().cloned());
        entries.extend(entries2.iter().cloned());

        common::projected::last_entry_per_key_ordered(&entries)
    };

    for (index, value) in expected_vec.iter().enumerate() {
        let expected = Some((index, value));
        let result = source.get_full(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_append_get_full_destination<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone();
    let mut destination = entries2.clone();

    source.append(&mut destination);

    for value in entries1.iter() {
        prop_assert!(destination.get_full(value).is_none());
    }

    for value in entries2.iter() {
        prop_assert!(destination.get_full(value).is_none());
    }

    Ok(())
}

fn prop_append_get_index_of_source<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone();
    let mut destination = entries2.clone();

    source.append(&mut destination);

    let expected_vec = {
        let mut entries = Vec::from_iter(entries1.iter().cloned());
        entries.extend(entries2.iter().cloned());

        entries
    };

    for (index, value) in expected_vec.iter().enumerate() {
        let expected = Some(value);
        let result = source.get(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_append_get_index_of_destination<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone();
    let mut destination = entries2.clone();

    source.append(&mut destination);

    for value in entries1.iter() {
        prop_assert!(destination.get_index_of(value).is_none());
    }

    for value in entries2.iter() {
        prop_assert!(destination.get_index_of(value).is_none());
    }

    Ok(())
}

fn prop_append_len_source<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone();
    let mut destination = entries2.clone();

    source.append(&mut destination);

    prop_assert!(source.len() <= entries1.len() + entries2.len());

    Ok(())
}

fn prop_append_len_destination<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone();
    let mut destination = entries2.clone();

    source.append(&mut destination);

    prop_assert_eq!(destination.len(), 0);

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
                fn prop_append_contains_source(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ> = entries2;
                    super::prop_append_contains_source(entries1, entries2)?
                }

                #[test]
                fn prop_append_contains_destination(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ> = entries2;
                    super::prop_append_contains_destination(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_source(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ> = entries2;
                    super::prop_append_get_source(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_destination(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ> = entries2;
                    super::prop_append_get_destination(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_full_source(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ> = entries2;
                    super::prop_append_get_full_source(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_full_destination(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ> = entries2;
                    super::prop_append_get_full_destination(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_index_of_source(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ> = entries2;
                    super::prop_append_get_index_of_source(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_index_of_destination(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ> = entries2;
                    super::prop_append_get_index_of_destination(entries1, entries2)?
                }

                #[test]
                fn prop_append_len_source(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ> = entries2;
                    super::prop_append_len_source(entries1, entries2)?
                }

                #[test]
                fn prop_append_len_destination(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ> = entries2;
                    super::prop_append_len_destination(entries1, entries2)?
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
