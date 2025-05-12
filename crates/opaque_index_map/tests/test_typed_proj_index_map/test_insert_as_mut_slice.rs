use crate::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;
use opaque_index_map::TypedProjIndexMap;

use opaque_index_map_testing as oimt;

fn expected<K, V>(entries: &[(K, V)]) -> Vec<V>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let expected: Vec<V> = oimt::last_entry_per_key_ordered(entries)
        .iter()
        .map(|(key, value)| value)
        .cloned()
        .collect();

    expected
}

fn result<K, V>(map: &mut TypedProjIndexMap<K, V, hash::RandomState, alloc::Global>) -> Vec<V>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let result: Vec<V> = map.as_mut_slice().values().cloned().collect();

    result
}

fn run_test_opaque_index_map_insert_as_mut_slice<K, V>(entries: &mut [(K, V)])
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let mut map = common::typed_proj_index_map::from_entries::<K, V>(entries);
    let expected = expected::<K, V>(&entries);
    let result = result::<K, V>(&mut map);

    assert_eq!(result, expected);
}

fn run_test_opaque_index_map_insert_as_mut_slice_values<K, V>(entries: &[(K, V)])
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let iter = oimt::PrefixGenerator::new(entries);
    for entries in iter {
        let mut cloned_entries = Vec::from(entries);
        run_test_opaque_index_map_insert_as_mut_slice(cloned_entries.as_mut());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_insert_as_mut_slice_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                run_test_opaque_index_map_insert_as_mut_slice_values(&entries);
            }

            #[test]
            fn test_opaque_index_map_insert_as_mut_slice_range_values() {
                let spec = $range_spec;
                let entries = oimt::range_entries::<$key_typ, $value_typ>(spec);
                run_test_opaque_index_map_insert_as_mut_slice_values(&entries);
            }

            #[test]
            fn test_opaque_index_map_insert_as_mut_slice_constant_values() {
                let spec = $const_spec;
                let entries = oimt::constant_key_entries::<$key_typ, $value_typ>(spec);
                run_test_opaque_index_map_insert_as_mut_slice_values(&entries);
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
