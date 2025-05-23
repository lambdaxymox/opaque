use crate::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;
use opaque_index_map::OpaqueIndexMap;

use opaque_index_map_testing as oimt;

fn run_test_opaque_index_map_swap_remove_full_get_full<K, V>(entries: &[(K, V)])
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let mut map = common::opaque_index_map::from_entries(&entries);
    let keys: Vec<K> = map.keys::<K, V, hash::RandomState, alloc::Global>().cloned().collect();
    for key in keys.iter() {
        let expected = map
            .get_full::<K, K, V, hash::RandomState, alloc::Global>(key)
            .map(|(i, k, v)| (i, k.clone(), v.clone()));
        let result = map.swap_remove_full::<K, K, V, hash::RandomState, alloc::Global>(key);

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_index_map_swap_remove_full_get_full_values<K, V>(entries: &[(K, V)])
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let iter = oimt::PrefixGenerator::new(entries);
    for entries in iter {
        run_test_opaque_index_map_swap_remove_full_get_full(entries);
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_swap_remove_full_get_full_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                run_test_opaque_index_map_swap_remove_full_get_full_values(&entries);
            }

            #[test]
            fn test_opaque_index_map_swap_remove_full_get_full_range_values() {
                let spec = $range_spec;
                let entries = oimt::range_entries::<$key_typ, $value_typ>(spec);
                run_test_opaque_index_map_swap_remove_full_get_full_values(&entries);
            }

            #[test]
            fn test_opaque_index_map_swap_remove_full_get_full_constant_values() {
                let spec = $const_spec;
                let entries = oimt::constant_key_entries::<$key_typ, $value_typ>(spec);
                run_test_opaque_index_map_swap_remove_full_get_full_values(&entries);
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
