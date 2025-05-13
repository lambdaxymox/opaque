use crate::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;
use opaque_index_map::TypedProjIndexMap;

use opaque_index_map_testing as oimt;

fn expected<K, V>(entries: &[(K, V)]) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let expected: Vec<(K, V)> = oimt::last_entry_per_key_ordered(entries).iter().cloned().collect();

    expected
}

fn result<K, V>(map: &TypedProjIndexMap<K, V, hash::RandomState, alloc::Global>) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let result: Vec<(K, V)> = map
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();

    result
}

fn run_test_typed_proj_index_map_insert_full_iter<K, V>(entries: &[(K, V)])
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let map = common::typed_proj_index_map::from_entries_full::<K, V>(entries);
    let expected = expected::<K, V>(&entries);
    let result = result::<K, V>(&map);

    assert_eq!(result, expected);
}

fn run_test_typed_proj_index_map_insert_full_iter_values<K, V>(entries: &[(K, V)])
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let iter = oimt::PrefixGenerator::new(entries);
    for entries in iter {
        run_test_typed_proj_index_map_insert_full_iter(entries);
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_index_map_insert_full_iter_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                run_test_typed_proj_index_map_insert_full_iter_values(&entries);
            }

            #[test]
            fn test_typed_proj_index_map_insert_full_iter_range_values() {
                let spec = $range_spec;
                let entries = oimt::range_entries::<$key_typ, $value_typ>(spec);
                run_test_typed_proj_index_map_insert_full_iter_values(&entries);
            }

            #[test]
            fn test_typed_proj_index_map_insert_full_iter_const_values() {
                let spec = $const_spec;
                let entries = oimt::constant_key_entries::<$key_typ, $value_typ>(spec);
                run_test_typed_proj_index_map_insert_full_iter_values(&entries);
            }
        }
    };
}

generate_tests!(
    u64_i64,
    key_type = u64,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    usize_i64,
    key_type = usize,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
