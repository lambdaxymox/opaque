use crate::map::common;
use crate::map::common::erased::{
    WrappingBuildHasher1,
    WrappingBuildHasher2,
    strategy_type_erased_index_map_max_len,
};
use opaque_index_map::TypeErasedIndexMap;

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

fn prop_append_contains_key_source<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    for key in entries1.iter::<K, V, S1, A>().map(|(k, _v)| k) {
        prop_assert!(source.contains_key::<_, K, V, S1, A>(key));
    }

    for key in entries2.iter::<K, V, S2, A>().map(|(k, _v)| k) {
        prop_assert!(source.contains_key::<_, K, V, S1, A>(key));
    }

    Ok(())
}

fn prop_append_contains_key_destination<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    for key in entries1.iter::<K, V, S1, A>().map(|(k, _v)| k) {
        prop_assert!(!destination.contains_key::<_, K, V, S2, A>(key));
    }

    for key in entries2.iter::<K, V, S2, A>().map(|(k, _v)| k) {
        prop_assert!(!destination.contains_key::<_, K, V, S2, A>(key));
    }

    Ok(())
}

fn prop_append_get_source<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    let expected_vec = {
        let mut entries = Vec::from_iter(entries1.iter::<K, V, S1, A>().map(|(k, v)| (k.clone(), v.clone())));
        entries.extend(entries2.iter::<K, V, S2, A>().map(|(k, v)| (k.clone(), v.clone())));

        common::erased::last_entry_per_key_ordered(&entries)
    };

    for (key, value) in expected_vec.iter() {
        let expected = Some(value);
        let result = source.get::<_, K, V, S1, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_append_get_destination<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    for (key, _) in entries1.iter::<K, V, S1, A>() {
        prop_assert!(destination.get::<_, K, V, S2, A>(key).is_none());
    }

    for (key, _) in entries2.iter::<K, V, S2, A>() {
        prop_assert!(destination.get::<_, K, V, S2, A>(key).is_none());
    }

    Ok(())
}

fn prop_append_get_full_source<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    let expected_vec = {
        let mut entries = Vec::from_iter(entries1.iter::<K, V, S1, A>().map(|(k, v)| (k.clone(), v.clone())));
        entries.extend(entries2.iter::<K, V, S2, A>().map(|(k, v)| (k.clone(), v.clone())));

        common::erased::last_entry_per_key_ordered(&entries)
    };

    for (index, (key, value)) in expected_vec.iter().enumerate() {
        let expected = Some((index, key, value));
        let result = source.get_full::<_, K, V, S1, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_append_get_full_destination<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    for (key, _) in entries1.iter::<K, V, S1, A>() {
        prop_assert!(destination.get_full::<_, K, V, S2, A>(key).is_none());
    }

    for (key, _) in entries2.iter::<K, V, S2, A>() {
        prop_assert!(destination.get_full::<_, K, V, S2, A>(key).is_none());
    }

    Ok(())
}

fn prop_append_get_index_of_source<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    let expected_vec = {
        let mut entries = Vec::from_iter(entries1.iter::<K, V, S1, A>().map(|(k, v)| (k.clone(), v.clone())));
        entries.extend(entries2.iter::<K, V, S2, A>().map(|(k, v)| (k.clone(), v.clone())));

        common::erased::last_entry_per_key_ordered(&entries)
    };

    for (index, (key, _)) in expected_vec.iter().enumerate() {
        let expected = Some(index);
        let result = source.get_index_of::<_, K, V, S1, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_append_get_index_of_destination<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    for key in source.keys::<K, V, S1, A>() {
        prop_assert!(destination.get_index_of::<_, K, V, S2, A>(key).is_none());
    }

    Ok(())
}

fn prop_append_get_key_value_source<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    let expected_vec = {
        let mut entries = Vec::from_iter(entries1.iter::<K, V, S1, A>().map(|(k, v)| (k.clone(), v.clone())));
        entries.extend(entries2.iter::<K, V, S2, A>().map(|(k, v)| (k.clone(), v.clone())));

        common::erased::last_entry_per_key_ordered(&entries)
    };

    for (key, value) in expected_vec.iter() {
        let expected = Some((key, value));
        let result = source.get_key_value::<_, K, V, S1, A>(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_append_get_key_value_destination<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    for (key, _) in entries1.iter::<K, V, S1, A>() {
        prop_assert!(destination.get_key_value::<_, K, V, S2, A>(key).is_none());
    }

    for (key, _) in entries2.iter::<K, V, S2, A>() {
        prop_assert!(destination.get_key_value::<_, K, V, S2, A>(key).is_none());
    }

    Ok(())
}

fn prop_append_len_source<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    prop_assert!(source.len() <= entries1.len() + entries2.len());

    Ok(())
}

fn prop_append_len_destination<K, V, S1, S2, A>(
    entries1: TypeErasedIndexMap,
    entries2: TypeErasedIndexMap,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = entries1.clone::<K, V, S1, A>();
    let mut destination = entries2.clone::<K, V, S2, A>();

    source.append::<K, V, S1, S2, A>(&mut destination);

    prop_assert_eq!(destination.len(), 0);

    Ok(())
}

macro_rules! generate_props {
    (
        $module_name:ident,
        $key_typ:ty,
        $value_typ:ty,
        $build_hasher_typ:ty,
        $alloc_typ:ty,
        $max_src_length:expr,
        $max_dst_length:expr,
        $map_gen:ident,
    ) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_append_contains_key_source(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_contains_key_source::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_contains_key_destination(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_contains_key_destination::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_source(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_get_source::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_destination(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_get_destination::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_full_source(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_get_full_source::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_full_destination(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_get_full_destination::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_index_of_source(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_get_index_of_source::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_index_of_destination(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_get_index_of_destination::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_key_values_source(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_get_key_value_source::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_get_key_value_destination(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_get_key_value_destination::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_len_source(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_len_source::<
                        $key_typ,
                        $value_typ,
                        super::WrappingBuildHasher1<$build_hasher_typ>,
                        super::WrappingBuildHasher2<$build_hasher_typ>,
                        $alloc_typ
                    >(entries1, entries2)?
                }

                #[test]
                fn prop_append_len_destination(
                    entries1 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher1<$build_hasher_typ>, $alloc_typ>($max_src_length),
                    entries2 in super::$map_gen::<$key_typ, $value_typ, super::WrappingBuildHasher2<$build_hasher_typ>, $alloc_typ>($max_dst_length),
                ) {
                    let entries1: super::TypeErasedIndexMap = entries1;
                    let entries2: super::TypeErasedIndexMap = entries2;
                    super::prop_append_len_destination::<
                        $key_typ,
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
    u64_i64,
    u64,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    16,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    16,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    16,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    16,
    strategy_type_erased_index_map_max_len,
);
