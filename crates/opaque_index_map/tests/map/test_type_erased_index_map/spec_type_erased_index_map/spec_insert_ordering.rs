use crate::map::common::erased::{
    SingleBoundedValue,
    strategy_type_erased_index_map_max_len_nonempty,
};
use opaque_index_map::TypeErasedIndexMap;

use core::any;
use core::fmt;
use std::format;
use std::hash;
use std::string::String;
use std::vec::Vec;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn strategy_prop_insert_preserves_order_new_entry<K, V, S, A>(
    max_length: usize,
) -> impl Strategy<Value = (TypeErasedIndexMap, (K, V))>
where
    K: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    V: any::Any + Clone + Eq + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    strategy_type_erased_index_map_max_len_nonempty::<K, V, S, A>(max_length + 1).prop_map(move |mut map| {
        let new_entry = map.pop::<K, V, S, A>().unwrap();
        (map, new_entry)
    })
}

fn prop_insert_preserves_order_new_entry<K, V, S, A>(
    (entries, new_entry): (TypeErasedIndexMap, (K, V)),
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();

    prop_assert!(!map.contains_key::<_, K, V, S, A>(&new_entry.0));

    let keys_before: Vec<K> = map.keys::<K, V, S, A>().cloned().collect();

    map.insert::<K, V, S, A>(new_entry.0.clone(), new_entry.1.clone());

    let keys_after: Vec<K> = map.keys::<K, V, S, A>().cloned().collect();

    let expected = {
        let mut _vec = keys_before.clone();
        _vec.push(new_entry.0.clone());
        _vec
    };
    let result = keys_after;

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    (
        $module_name:ident,
        $key_typ:ty,
        $value_typ:ty,
        $build_hasher_typ:ty,
        $alloc_typ:ty,
        $max_length:expr,
        $map_gen:ident,
    ) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_insert_preserves_order_new_entry(
                    (entries, new_entry) in
                    super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)
                ) {
                    let entries: super::TypeErasedIndexMap = entries;
                    let new_entry: ($key_typ, $value_typ) = new_entry;
                    super::prop_insert_preserves_order_new_entry::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>((entries, new_entry))?
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
    strategy_prop_insert_preserves_order_new_entry,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_insert_preserves_order_new_entry,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_insert_preserves_order_new_entry,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_insert_preserves_order_new_entry,
);
