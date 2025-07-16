use crate::set::common::projected::{
    WrappingBuildHasher1,
    WrappingBuildHasher2,
    strategy_type_projected_index_set_max_len,
};
use opaque_index_map::TypeProjectedIndexSet;
use opaque_hash::TypeProjectedBuildHasher;

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

fn from_difference_in<T, S1, S2, A>(
    entries1: &TypeProjectedIndexSet<T, S1, A>,
    entries2: &TypeProjectedIndexSet<T, S2, A>,
) -> TypeProjectedIndexSet<T, S1, A>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = TypeProjectedIndexSet::with_hasher_proj_in(
        entries1.hasher().clone(),
        entries1.allocator().clone(),
    );

    for value in entries1.difference(&entries2).cloned() {
        set.insert(value);
    }

    set
}

fn from_union_in<T, S1, S2, A>(
    entries1: &TypeProjectedIndexSet<T, S1, A>,
    entries2: &TypeProjectedIndexSet<T, S2, A>,
) -> TypeProjectedIndexSet<T, S1, A>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = TypeProjectedIndexSet::with_hasher_proj_in(
        entries1.hasher().clone(),
        entries1.allocator().clone(),
    );

    for value in entries1.union(&entries2).cloned() {
        set.insert(value);
    }

    set
}

fn from_intersection_in<T, S1, S2, A>(
    entries1: &TypeProjectedIndexSet<T, S1, A>,
    entries2: &TypeProjectedIndexSet<T, S2, A>,
) -> TypeProjectedIndexSet<T, S1, A>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = TypeProjectedIndexSet::with_hasher_proj_in(
        entries1.hasher().clone(),
        entries1.allocator().clone(),
    );

    for value in entries1.intersection(&entries2).cloned() {
        set.insert(value);
    }

    set
}

fn prop_difference_with_self<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries, &entries);

    prop_assert!(set.is_empty());

    Ok(())
}

fn prop_difference_with_empty1<T, S1, S2, A>(entries: TypeProjectedIndexSet<T, S1, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let empty_set = TypeProjectedIndexSet::with_hasher_proj_in(
        TypeProjectedBuildHasher::new(S2::default()),
        entries.allocator().clone(),
    );
    let set = from_difference_in(&entries, &empty_set);

    prop_assert_eq!(set, entries);

    Ok(())
}

fn prop_difference_with_empty2<T, S1, S2, A>(entries: TypeProjectedIndexSet<T, S1, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let empty_set = TypeProjectedIndexSet::with_hasher_proj_in(
        TypeProjectedBuildHasher::new(S2::default()),
        entries.allocator().clone(),
    );
    let set = from_difference_in(&empty_set, &entries);

    prop_assert!(set.is_empty());

    Ok(())
}

fn prop_difference_union_intersection<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let lhs = from_difference_in(
        &from_union_in(&entries1, &entries2),
        &from_intersection_in(&entries1, &entries2),
    );
    let rhs = from_union_in(
        &from_difference_in(&entries1, &entries2),
        &from_difference_in(&entries2, &entries1),
    );

    prop_assert_eq!(lhs, rhs);

    Ok(())
}

fn prop_difference_contains<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries1, &entries2);

    for value in set.iter() {
        prop_assert!(entries1.contains(value));
    }

    for value in set.iter() {
        prop_assert!(!entries2.contains(value));
    }

    Ok(())
}

fn prop_difference_get<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries1, &entries2);
    for value in set.iter() {
        let expected = Some(value.clone());
        let result = set.get(value).cloned();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_difference_get_full<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries1, &entries2);
    for value in set.iter() {
        let expected = Some(value.clone());
        let result = set.get_full(value).map(|(_i, v)| v.clone());

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_difference_get_index<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries1, &entries2);
    for (index, value) in set.iter().enumerate() {
        let expected = Some(value.clone());
        let result = set.get_index(index).map(|v| v.clone());

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_difference_get_index_of<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries1, &entries2);
    for (index, value) in set.iter().enumerate() {
        let expected = Some(index);
        let result = set.get_index_of(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_difference_is_subset<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries1, &entries2);

    prop_assert!(set.is_subset(&entries1));

    Ok(())
}

fn prop_difference_is_superset<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries1, &entries2);

    prop_assert!(entries1.is_superset(&set));

    Ok(())
}

fn prop_difference_is_disjoint<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries1, &entries2);

    prop_assert!(set.is_disjoint(&entries2));

    Ok(())
}

fn prop_difference_len<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries1, &entries2);

    prop_assert!(set.len() <= entries1.len());

    Ok(())
}

fn prop_difference_ordering<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_difference_in(&entries1, &entries2);
    for i in 1..set.len() {
        let previous_index = entries1.get_index_of(&set[i - 1]);
        let current_index = entries1.get_index_of(&set[i]);

        prop_assert!(previous_index.is_some());
        prop_assert!(current_index.is_some());
        prop_assert!(previous_index < current_index);
    }

    Ok(())
}

macro_rules! generate_props {
    (
        $module_name:ident,
        $value_typ:ty,
        $build_hasher_typ1:ty,
        $build_hasher_typ2:ty,
        $alloc_typ:ty,
        $max_length:expr,
        $set_gen:ident,
    ) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_difference_with_self(entries in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries;
                    super::prop_difference_with_self(entries)?
                }

                #[test]
                fn prop_difference_with_empty1(entries in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries;
                    super::prop_difference_with_empty1::<$value_typ, $build_hasher_typ1, $build_hasher_typ2, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_difference_with_empty2(entries in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries;
                    super::prop_difference_with_empty2::<$value_typ, $build_hasher_typ1, $build_hasher_typ2, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_difference_union_intersection(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_union_intersection(entries1, entries2)?
                }

                #[test]
                fn prop_difference_contains(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_contains(entries1, entries2)?
                }

                #[test]
                fn prop_difference_get(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_get(entries1, entries2)?
                }

                #[test]
                fn prop_difference_get_full(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_get_full(entries1, entries2)?
                }

                #[test]
                fn prop_difference_get_index(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_get_index(entries1, entries2)?
                }

                #[test]
                fn prop_difference_get_index_of(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_get_index_of(entries1, entries2)?
                }

                #[test]
                fn prop_difference_is_subset(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_is_subset(entries1, entries2)?
                }

                #[test]
                fn prop_difference_is_superset(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_is_superset(entries1, entries2)?
                }

                #[test]
                fn prop_difference_is_disjoint(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_is_disjoint(entries1, entries2)?
                }

                #[test]
                fn prop_difference_len(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_len(entries1, entries2)?
                }

                #[test]
                fn prop_difference_ordering(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_difference_ordering(entries1, entries2)?
                }
            }
        }
    };
}

generate_props!(
    u64,
    u64,
    WrappingBuildHasher1<hash::RandomState>,
    WrappingBuildHasher2<hash::RandomState>,
    alloc::Global,
    32,
    strategy_type_projected_index_set_max_len,
);
generate_props!(
    usize,
    usize,
    WrappingBuildHasher1<hash::RandomState>,
    WrappingBuildHasher2<hash::RandomState>,
    alloc::Global,
    32,
    strategy_type_projected_index_set_max_len,
);
generate_props!(
    string,
    String,
    WrappingBuildHasher1<hash::RandomState>,
    WrappingBuildHasher2<hash::RandomState>,
    alloc::Global,
    32,
    strategy_type_projected_index_set_max_len,
);
