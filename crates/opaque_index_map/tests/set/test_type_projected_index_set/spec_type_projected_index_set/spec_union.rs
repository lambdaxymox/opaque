use crate::set::common::projected::{
    WrappingBuildHasher1,
    WrappingBuildHasher2,
    WrappingBuildHasher3,
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

fn prop_union_with_self<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_union_in(&entries, &entries);

    prop_assert_eq!(set, entries);

    Ok(())
}

fn prop_union_with_empty<T, S1, S2, A>(entries: TypeProjectedIndexSet<T, S1, A>) -> Result<(), TestCaseError>
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
    let set = from_union_in(&entries, &empty_set);

    prop_assert_eq!(set, entries);

    Ok(())
}

fn prop_union_contains<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_union_in(&entries1, &entries2);

    for value in entries1.iter() {
        prop_assert!(set.contains(value));
    }

    for value in entries2.iter() {
        prop_assert!(set.contains(value));
    }

    Ok(())
}

fn prop_union_get<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_union_in(&entries1, &entries2);

    for value in entries1.iter() {
        prop_assert!(set.get(value).is_some());
    }

    for value in entries2.iter() {
        prop_assert!(set.get(value).is_some());
    }

    Ok(())
}

fn prop_union_get_full<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_union_in(&entries1, &entries2);

    for value in entries1.iter() {
        prop_assert!(set.get_full(value).is_some());
    }

    for value in entries2.iter() {
        prop_assert!(set.get_full(value).is_some());
    }

    Ok(())
}

fn prop_union_commutative<T, S1, S2, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let lhs = from_union_in(&entries1, &entries2);
    let rhs = from_union_in(&entries2, &entries1);

    prop_assert_eq!(lhs, rhs);

    Ok(())
}

fn prop_union_associative<T, S1, S2, S3, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
    entries3: TypeProjectedIndexSet<T, S3, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    S3: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S3::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let lhs = from_union_in(
        &from_union_in(&entries1, &entries2),
        &entries3,
    );
    let rhs = from_union_in(
        &entries1,
        &from_union_in(&entries2, &entries3),
    );

    prop_assert_eq!(lhs, rhs);

    Ok(())
}

fn prop_union_distributive_with_intersection<T, S1, S2, S3, A>(
    entries1: TypeProjectedIndexSet<T, S1, A>,
    entries2: TypeProjectedIndexSet<T, S2, A>,
    entries3: TypeProjectedIndexSet<T, S3, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    S3: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S3::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let lhs = from_union_in(
        &entries1,
        &from_intersection_in(&entries2, &entries3),
    );
    let rhs = from_intersection_in(
        &from_union_in(&entries1, &entries2),
        &from_union_in(&entries1, &entries3),
    );

    prop_assert_eq!(lhs, rhs);

    Ok(())
}

fn prop_union_is_subset<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_union_in(&entries1, &entries2);

    prop_assert!(entries1.is_subset(&set));
    prop_assert!(entries2.is_subset(&set));

    Ok(())
}

fn prop_union_is_superset<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_union_in(&entries1, &entries2);

    prop_assert!(set.is_superset(&entries1));
    prop_assert!(set.is_superset(&entries2));

    Ok(())
}

fn prop_union_len1<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_union_in(&entries1, &entries2);

    prop_assert!(entries1.len() <= set.len());
    prop_assert!(entries2.len() <= set.len());

    Ok(())
}

fn prop_union_len2<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_union_in(&entries1, &entries2);

    prop_assert!(set.len() <= entries1.len() + entries2.len());

    Ok(())
}

fn prop_union_ordering1<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_union_in(&entries1, &entries2);

    for i in 1..entries1.len() {
        let previous_index = entries1.get_index_of(&set[i - 1]);
        let current_index = entries1.get_index_of(&set[i]);

        prop_assert!(previous_index.is_some());
        prop_assert!(current_index.is_some());
        prop_assert!(previous_index < current_index);
    }

    for i in (entries1.len() + 1)..set.len() {
        let previous_index = entries2.get_index_of(&set[i - 1]);
        let current_index = entries2.get_index_of(&set[i]);

        prop_assert!(previous_index.is_some());
        prop_assert!(current_index.is_some());
        prop_assert!(previous_index < current_index);
    }

    Ok(())
}

fn prop_union_ordering2<T, S1, S2, A>(entries1: TypeProjectedIndexSet<T, S1, A>, entries2: TypeProjectedIndexSet<T, S2, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_union_in(&entries1, &entries2);

    for i in 0..entries1.len() {
        prop_assert!(entries1.contains(&set[i]));
    }

    for i in entries1.len()..set.len() {
        prop_assert!(entries2.contains(&set[i]));
        prop_assert!(!entries1.contains(&set[i]));
    }

    Ok(())
}

macro_rules! generate_props {
    (
        $module_name:ident,
        $value_typ:ty,
        $build_hasher_typ1:ty,
        $build_hasher_typ2:ty,
        $build_hasher_typ3:ty,
        $alloc_typ:ty,
        $max_length:expr,
        $set_gen:ident,
    ) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_union_with_self(entries in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries;
                    super::prop_union_with_self(entries)?
                }

                #[test]
                fn prop_union_with_empty(entries in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries;
                    super::prop_union_with_empty::<$value_typ, $build_hasher_typ1, $build_hasher_typ2, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_union_contains(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_union_contains(entries1, entries2)?
                }

                #[test]
                fn prop_union_get(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_union_get(entries1, entries2)?
                }

                #[test]
                fn prop_union_get_full(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_union_get_full(entries1, entries2)?
                }

                #[test]
                fn prop_union_commutative(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_union_commutative(entries1, entries2)?
                }

                #[test]
                fn prop_union_associative(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                    entries3 in super::$set_gen::<$value_typ, $build_hasher_typ3, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    let entries3: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ3, $alloc_typ> = entries3;
                    super::prop_union_associative(entries1, entries2, entries3)?
                }

                #[test]
                fn prop_union_distributive_with_intersection(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                    entries3 in super::$set_gen::<$value_typ, $build_hasher_typ3, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    let entries3: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ3, $alloc_typ> = entries3;
                    super::prop_union_distributive_with_intersection(entries1, entries2, entries3)?
                }

                #[test]
                fn prop_union_is_subset(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_union_is_subset(entries1, entries2)?
                }

                #[test]
                fn prop_union_is_superset(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_union_is_superset(entries1, entries2)?
                }

                #[test]
                fn prop_union_len1(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_union_len1(entries1, entries2)?
                }

                #[test]
                fn prop_union_len2(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_union_len2(entries1, entries2)?
                }

                #[test]
                fn prop_union_ordering1(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_union_ordering1(entries1, entries2)?
                }

                #[test]
                fn prop_union_ordering2(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ1, $alloc_typ> = entries1;
                    let entries2: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ2, $alloc_typ> = entries2;
                    super::prop_union_ordering2(entries1, entries2)?
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
    WrappingBuildHasher3<hash::RandomState>,
    alloc::Global,
    32,
    strategy_type_projected_index_set_max_len,
);
generate_props!(
    usize,
    usize,
    WrappingBuildHasher1<hash::RandomState>,
    WrappingBuildHasher2<hash::RandomState>,
    WrappingBuildHasher3<hash::RandomState>,
    alloc::Global,
    32,
    strategy_type_projected_index_set_max_len,
);
generate_props!(
    string,
    String,
    WrappingBuildHasher1<hash::RandomState>,
    WrappingBuildHasher2<hash::RandomState>,
    WrappingBuildHasher3<hash::RandomState>,
    alloc::Global,
    32,
    strategy_type_projected_index_set_max_len,
);
