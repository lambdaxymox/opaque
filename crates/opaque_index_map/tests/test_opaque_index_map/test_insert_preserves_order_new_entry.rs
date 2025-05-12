use crate::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;
use opaque_index_map::OpaqueIndexMap;
use opaque_vec::OpaqueVec;

use opaque_index_map_testing as oimt;

fn run_test_opaque_index_map_insert_preserves_order_new_entry<K, V>(entries: &[(K, V)], new_entry: &(K, V))
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let mut map = common::opaque_index_map::from_entries(entries);
    
    assert!(!map.contains_key::<K, K, V, hash::RandomState, alloc::Global>(&new_entry.0));

    let keys_before: Vec<K> = map.keys::<K, V, hash::RandomState, alloc::Global>().cloned().collect();

    map.insert::<K, V, hash::RandomState, alloc::Global>(new_entry.0.clone(), new_entry.1.clone());

    let keys_after: Vec<K> = map.keys::<K, V, hash::RandomState, alloc::Global>().cloned().collect();

    let expected = {
        let mut _vec = keys_before.clone();
        _vec.push(new_entry.0.clone());
        _vec
    };
    let result = keys_after;

    assert_eq!(result, expected);
}

fn run_test_opaque_index_map_insert_preserves_order_new_entry_values<K, V>(entries: &[(K, V)], new_entry: &(K, V))
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let iter = oimt::PrefixGenerator::new(entries);
    for entries in iter {
        run_test_opaque_index_map_insert_preserves_order_new_entry(entries, new_entry);
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, new_entry = $new_entry:expr, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_insert_preserves_order_new_entry_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                let new_entry = $new_entry;
                run_test_opaque_index_map_insert_preserves_order_new_entry_values(&entries, &new_entry);
            }

            #[test]
            fn test_opaque_index_map_insert_preserves_order_new_entry_range_values() {
                let spec = $range_spec;
                let entries = oimt::range_entries::<$key_typ, $value_typ>(spec);
                let new_entry = $new_entry;
                run_test_opaque_index_map_insert_preserves_order_new_entry_values(&entries, &new_entry);
            }

            #[test]
            fn test_opaque_index_map_insert_preserves_order_new_entry_constant_values() {
                let spec = $const_spec;
                let entries = oimt::constant_key_entries::<$key_typ, $value_typ>(spec);
                let new_entry = $new_entry;
                run_test_opaque_index_map_insert_preserves_order_new_entry_values(&entries, &new_entry);
            }
        }
    };
}

generate_tests!(
    u64_i64,
    key_type = u64,
    value_type = i64,
    new_entry = (u64::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    usize_i64,
    key_type = usize,
    value_type = i64,
    new_entry = (usize::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
