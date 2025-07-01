use crate::set::common;
use crate::set::common::erased::{
    WrappingBuildHasher1,
    WrappingBuildHasher2,
    strategy_type_erased_index_set_max_len,
};
use opaque_index_map::OpaqueIndexSet;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn prop_append_contains_source<T, S1, S2, A>(
    entries1: OpaqueIndexSet,
    entries2: OpaqueIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<T, S1, A>();
    let mut destination = entries2.clone::<T, S2, A>();

    source.append::<T, S1, S2, A>(&mut destination);

    for value in entries1.iter::<T, S1, A>() {
        prop_assert!(source.contains::<_, T, S1, A>(value));
    }

    for value in entries2.iter::<T, S2, A>() {
        prop_assert!(source.contains::<_, T, S1, A>(value));
    }

    Ok(())
}

fn prop_append_contains_destination<T, S1, S2, A>(
    entries1: OpaqueIndexSet,
    entries2: OpaqueIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<T, S1, A>();
    let mut destination = entries2.clone::<T, S2, A>();

    source.append::<T, S1, S2, A>(&mut destination);

    for value in entries1.iter::<T, S1, A>() {
        prop_assert!(!destination.contains::<_, T, S2, A>(value));
    }

    for value in entries2.iter::<T, S2, A>() {
        prop_assert!(!destination.contains::<_, T, S2, A>(value));
    }

    Ok(())
}

fn prop_append_get_source<T, S1, S2, A>(
    entries1: OpaqueIndexSet,
    entries2: OpaqueIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<T, S1, A>();
    let mut destination = entries2.clone::<T, S2, A>();

    source.append::<T, S1, S2, A>(&mut destination);

    let expected_vec = {
        let mut entries = Vec::from_iter(entries1.iter::<T, S1, A>().cloned());
        entries.extend(entries2.iter::<T, S2, A>().cloned());

        entries
    };

    for value in expected_vec.iter() {
        let expected = Some(value);
        let result = source.get::<_, T, S1, A>(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_append_get_destination<T, S1, S2, A>(
    entries1: OpaqueIndexSet,
    entries2: OpaqueIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<T, S1, A>();
    let mut destination = entries2.clone::<T, S2, A>();

    source.append::<T, S1, S2, A>(&mut destination);

    for value in entries1.iter::<T, S1, A>() {
        prop_assert!(destination.get::<_, T, S2, A>(value).is_none());
    }

    for value in entries2.iter::<T, S2, A>() {
        prop_assert!(destination.get::<_, T, S2, A>(value).is_none());
    }

    Ok(())
}

fn prop_append_get_full_source<T, S1, S2, A>(
    entries1: OpaqueIndexSet,
    entries2: OpaqueIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<T, S1, A>();
    let mut destination = entries2.clone::<T, S2, A>();

    source.append::<T, S1, S2, A>(&mut destination);

    let expected_vec = {
        let mut entries = Vec::from_iter(entries1.iter::<T, S1, A>().cloned());
        entries.extend(entries2.iter::<T, S2, A>().cloned());

        common::projected::last_entry_per_key_ordered(&entries)
    };

    for (index, value) in expected_vec.iter().enumerate() {
        let expected = Some((index, value));
        let result = source.get_full::<_, T, S1, A>(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_append_get_full_destination<T, S1, S2, A>(
    entries1: OpaqueIndexSet,
    entries2: OpaqueIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<T, S1, A>();
    let mut destination = entries2.clone::<T, S2, A>();

    source.append::<T, S1, S2, A>(&mut destination);

    for value in entries1.iter::<T, S1, A>() {
        prop_assert!(destination.get_full::<_, T, S2, A>(value).is_none());
    }

    for value in entries2.iter::<T, S2, A>() {
        prop_assert!(destination.get_full::<_, T, S2, A>(value).is_none());
    }

    Ok(())
}

fn prop_append_get_index_of_source<T, S1, S2, A>(
    entries1: OpaqueIndexSet,
    entries2: OpaqueIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<T, S1, A>();
    let mut destination = entries2.clone::<T, S2, A>();

    source.append::<T, S1, S2, A>(&mut destination);

    let expected_vec = {
        let mut entries = Vec::from_iter(entries1.iter::<T, S1, A>().cloned());
        entries.extend(entries2.iter::<T, S2, A>().cloned());

        entries
    };

    for (index, value) in expected_vec.iter().enumerate() {
        let expected = Some(value);
        let result = source.get::<_, T, S1, A>(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_append_get_index_of_destination<T, S1, S2, A>(
    entries1: OpaqueIndexSet,
    entries2: OpaqueIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<T, S1, A>();
    let mut destination = entries2.clone::<T, S2, A>();

    source.append::<T, S1, S2, A>(&mut destination);

    for value in entries1.iter::<T, S1, A>() {
        prop_assert!(destination.get_index_of::<_, T, S2, A>(value).is_none());
    }

    for value in entries2.iter::<T, S2, A>() {
        prop_assert!(destination.get_index_of::<_, T, S2, A>(value).is_none());
    }

    Ok(())
}

fn prop_append_len_source<T, S1, S2, A>(
    entries1: OpaqueIndexSet,
    entries2: OpaqueIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<T, S1, A>();
    let mut destination = entries2.clone::<T, S2, A>();

    source.append::<T, S1, S2, A>(&mut destination);

    prop_assert!(source.len() <= entries1.len() + entries2.len());

    Ok(())
}

fn prop_append_len_destination<T, S1, S2, A>(
    entries1: OpaqueIndexSet,
    entries2: OpaqueIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<T, S1, A>();
    let mut destination = entries2.clone::<T, S2, A>();

    source.append::<T, S1, S2, A>(&mut destination);

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
            use proptest::prelude::*;
            use std::hash;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_append_contains_source(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::OpaqueIndexSet = entries1;
                    let entries2: super::OpaqueIndexSet = entries2;
                    super::prop_append_contains_source::<
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_contains_destination(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::OpaqueIndexSet = entries1;
                    let entries2: super::OpaqueIndexSet = entries2;
                    super::prop_append_contains_destination::<
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_source(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::OpaqueIndexSet = entries1;
                    let entries2: super::OpaqueIndexSet = entries2;
                    super::prop_append_get_source::<
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_destination(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::OpaqueIndexSet = entries1;
                    let entries2: super::OpaqueIndexSet = entries2;
                    super::prop_append_get_destination::<
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_full_source(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::OpaqueIndexSet = entries1;
                    let entries2: super::OpaqueIndexSet = entries2;
                    super::prop_append_get_full_source::<
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_full_destination(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::OpaqueIndexSet = entries1;
                    let entries2: super::OpaqueIndexSet = entries2;
                    super::prop_append_get_full_destination::<
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_index_of_source(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::OpaqueIndexSet = entries1;
                    let entries2: super::OpaqueIndexSet = entries2;
                    super::prop_append_get_index_of_source::<
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_index_of_destination(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::OpaqueIndexSet = entries1;
                    let entries2: super::OpaqueIndexSet = entries2;
                    super::prop_append_get_index_of_destination::<
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_len_source(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::OpaqueIndexSet = entries1;
                    let entries2: super::OpaqueIndexSet = entries2;
                    super::prop_append_len_source::<
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_len_destination(
                    entries1 in super::$set_gen::<$value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_length),
                ) {
                    let entries1: super::OpaqueIndexSet = entries1;
                    let entries2: super::OpaqueIndexSet = entries2;
                    super::prop_append_len_destination::<
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
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
