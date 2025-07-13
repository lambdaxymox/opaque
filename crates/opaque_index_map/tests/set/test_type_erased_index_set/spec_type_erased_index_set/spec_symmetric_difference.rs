use crate::set::common::erased::{
    WrappingBuildHasher1,
    WrappingBuildHasher2,
    WrappingBuildHasher3,
    strategy_type_erased_index_set_max_len,
};
use opaque_index_map::TypeErasedIndexSet;
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

fn from_symmetric_difference_in<T, S1, S2, A>(
    entries1: &TypeErasedIndexSet,
    entries2: &TypeErasedIndexSet,
) -> TypeErasedIndexSet
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = TypeErasedIndexSet::with_hasher_proj_in::<T, S1, A>(
        entries1.hasher::<T, S1, A>().clone(),
        entries1.allocator::<T, S1, A>().clone(),
    );

    for value in entries1.symmetric_difference::<S2, T, S1, A>(&entries2).cloned() {
        set.insert::<T, S1, A>(value);
    }

    set
}

fn from_union_in<T, S1, S2, A>(
    entries1: &TypeErasedIndexSet,
    entries2: &TypeErasedIndexSet,
) -> TypeErasedIndexSet
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = TypeErasedIndexSet::with_hasher_proj_in::<T, S1, A>(
        entries1.hasher::<T, S1, A>().clone(),
        entries1.allocator::<T, S1, A>().clone(),
    );

    for value in entries1.union::<S2, T, S1, A>(&entries2).cloned() {
        set.insert::<T, S1, A>(value);
    }

    set
}

fn from_intersection_in<T, S1, S2, A>(
    entries1: &TypeErasedIndexSet,
    entries2: &TypeErasedIndexSet,
) -> TypeErasedIndexSet
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = TypeErasedIndexSet::with_hasher_proj_in::<T, S1, A>(
        entries1.hasher::<T, S1, A>().clone(),
        entries1.allocator::<T, S1, A>().clone(),
    );

    for value in entries1.intersection::<S2, T, S1, A>(&entries2).cloned() {
        set.insert::<T, S1, A>(value);
    }

    set
}

fn prop_symmetric_difference_self_inverse<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_symmetric_difference_in::<T, S, S, A>(&entries, &entries);

    prop_assert!(set.is_empty());

    Ok(())
}

fn prop_symmetric_difference_identity<T, S1, S2, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let empty_set = TypeErasedIndexSet::with_hasher_proj_in::<T, S2, A>(
        TypeProjectedBuildHasher::new(S2::default()),
        entries.allocator::<T, S1, A>().clone(),
    );
    let set = from_symmetric_difference_in::<T, S1, S2, A>(&entries, &empty_set);

    prop_assert_eq!(set.as_proj::<T, S1, A>(), entries.as_proj::<T, S1, A>());

    Ok(())
}

fn prop_symmetric_difference_commutative<T, S1, S2, A>(
    entries1: TypeErasedIndexSet,
    entries2: TypeErasedIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let lhs = from_symmetric_difference_in::<T, S1, S2, A>(&entries1, &entries2);
    let rhs = from_symmetric_difference_in::<T, S2, S1, A>(&entries2, &entries1);

    prop_assert_eq!(lhs.as_proj::<T, S1, A>(), rhs.as_proj::<T, S2, A>());

    Ok(())
}

fn prop_symmetric_difference_associative<T, S1, S2, S3, A>(
    entries1: TypeErasedIndexSet,
    entries2: TypeErasedIndexSet,
    entries3: TypeErasedIndexSet,
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
    let lhs = from_symmetric_difference_in::<T, S1, S3, A>(
        &from_symmetric_difference_in::<T, S1, S2, A>(&entries1, &entries2),
        &entries3,
    );
    let rhs = from_symmetric_difference_in::<T, S1, S2, A>(
        &entries1,
        &from_symmetric_difference_in::<T, S2, S3, A>(&entries2, &entries3),
    );

    prop_assert_eq!(lhs.as_proj::<T, S1, A>(), rhs.as_proj::<T, S1, A>());

    Ok(())
}

fn prop_symmetric_difference_union_is_subset<T, S1, S2, A>(
    entries1: TypeErasedIndexSet,
    entries2: TypeErasedIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_symmetric_difference_in::<T, S1, S2, A>(&entries1, &entries2);
    let union_entries1_entries2 = from_union_in::<T, S1, S2, A>(&entries1, &entries2);

    prop_assert!(set.is_subset::<T, S1, A, S1, A>(&union_entries1_entries2));

    Ok(())
}

fn prop_symmetric_difference_intersection_is_disjoint<T, S1, S2, A>(
    entries1: TypeErasedIndexSet,
    entries2: TypeErasedIndexSet,
) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone + Default,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_symmetric_difference_in::<T, S1, S2, A>(&entries1, &entries2);
    let intersect_entries1_entries2 = from_intersection_in::<T, S1, S2, A>(&entries1, &entries2);

    prop_assert!(set.is_disjoint::<T, S1, A, S1, A>(&intersect_entries1_entries2));

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
                fn prop_symmetric_difference_self_inverse(entries in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_symmetric_difference_self_inverse::<$value_typ, $build_hasher_typ1, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_symmetric_difference_identity(entries in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_symmetric_difference_identity::<$value_typ, $build_hasher_typ1, $build_hasher_typ2, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_symmetric_difference_commutative(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeErasedIndexSet = entries1;
                    let entries2: super::TypeErasedIndexSet = entries2;
                    super::prop_symmetric_difference_commutative::<$value_typ, $build_hasher_typ1, $build_hasher_typ2, $alloc_typ>(entries1, entries2)?
                }

                #[test]
                fn prop_symmetric_difference_associative(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                    entries3 in super::$set_gen::<$value_typ, $build_hasher_typ3, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeErasedIndexSet = entries1;
                    let entries2: super::TypeErasedIndexSet = entries2;
                    let entries3: super::TypeErasedIndexSet = entries3;
                    super::prop_symmetric_difference_associative::<$value_typ, $build_hasher_typ1, $build_hasher_typ2, $build_hasher_typ3, $alloc_typ>(entries1, entries2, entries3)?
                }

                #[test]
                fn prop_symmetric_difference_union_is_subset(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeErasedIndexSet = entries1;
                    let entries2: super::TypeErasedIndexSet = entries2;
                    super::prop_symmetric_difference_union_is_subset::<$value_typ, $build_hasher_typ1, $build_hasher_typ2, $alloc_typ>(entries1, entries2)?
                }

                #[test]
                fn prop_symmetric_difference_intersection_is_disjoint(
                    entries1 in super::$set_gen::<$value_typ, $build_hasher_typ1, $alloc_typ>($max_length),
                    entries2 in super::$set_gen::<$value_typ, $build_hasher_typ2, $alloc_typ>($max_length),
                ) {
                    let entries1: super::TypeErasedIndexSet = entries1;
                    let entries2: super::TypeErasedIndexSet = entries2;
                    super::prop_symmetric_difference_intersection_is_disjoint::<$value_typ, $build_hasher_typ1, $build_hasher_typ2, $alloc_typ>(entries1, entries2)?
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
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    usize,
    usize,
    WrappingBuildHasher1<hash::RandomState>,
    WrappingBuildHasher2<hash::RandomState>,
    WrappingBuildHasher3<hash::RandomState>,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    string,
    String,
    WrappingBuildHasher1<hash::RandomState>,
    WrappingBuildHasher2<hash::RandomState>,
    WrappingBuildHasher3<hash::RandomState>,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
