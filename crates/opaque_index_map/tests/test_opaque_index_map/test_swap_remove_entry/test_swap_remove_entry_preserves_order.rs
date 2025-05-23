use crate::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;
use opaque_index_map::OpaqueIndexMap;

use opaque_index_map_testing as oimt;

fn expected<K, V>(entries: &[(K, V)], index: usize, key: &K) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let mut map_entries = oimt::last_entry_per_key_ordered(entries);

    assert_eq!(map_entries[index].0, key.clone());

    map_entries.swap_remove(index);

    map_entries
}

fn result<K, V>(map: &OpaqueIndexMap, key: &K) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let mut new_map = common::opaque_index_map::clone::<K, V, hash::RandomState, alloc::Global>(map);
    new_map.swap_remove_entry::<K, K, V, hash::RandomState, alloc::Global>(key);

    let ordered_entries: Vec<(K, V)> = new_map
        .iter::<K, V, hash::RandomState, alloc::Global>()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();

    ordered_entries
}

fn run_test_opaque_index_map_swap_remove_entry_preserves_order<K, V>(entries: &[(K, V)])
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let base_map = common::opaque_index_map::from_entries(entries);
    let base_keys: Vec<K> = base_map.keys::<K, V, hash::RandomState, alloc::Global>().cloned().collect();
    for (index, key) in base_keys.iter().enumerate() {
        let expected = expected(entries, index, &key);
        let result = result::<K, V>(&base_map, key);

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_index_map_swap_remove_entry_preserves_order_values<K, V>(entries: &[(K, V)])
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let iter = oimt::PrefixGenerator::new(entries);
    for entries in iter {
        run_test_opaque_index_map_swap_remove_entry_preserves_order(entries);
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_swap_remove_entry_preserves_order_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                run_test_opaque_index_map_swap_remove_entry_preserves_order_values(&entries);
            }

            #[test]
            fn test_opaque_index_map_swap_remove_entry_preserves_order_range_values() {
                let spec = $range_spec;
                let entries = oimt::range_entries::<$key_typ, $value_typ>(spec);
                run_test_opaque_index_map_swap_remove_entry_preserves_order_values(&entries);
            }

            #[test]
            fn test_opaque_index_map_swap_remove_entry_preserves_order_constant_values() {
                let spec = $const_spec;
                let entries = oimt::constant_key_entries::<$key_typ, $value_typ>(spec);
                run_test_opaque_index_map_swap_remove_entry_preserves_order_values(&entries);
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
