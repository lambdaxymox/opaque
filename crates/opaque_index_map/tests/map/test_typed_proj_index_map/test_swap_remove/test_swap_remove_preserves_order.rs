use crate::map::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;
use opaque_index_map::map::TypedProjIndexMap;

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

fn result<K, V, S, A>(map: &TypedProjIndexMap<K, V, S, A>, key: &K) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut new_map = map.clone();
    new_map.swap_remove(key);

    let ordered_entries: Vec<(K, V)> = new_map
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();

    ordered_entries
}

fn run_test_typed_proj_index_map_swap_remove_preserves_order<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let base_map = common::typed_proj_index_map::from_entries(entries, build_hasher, alloc);
    let base_keys: Vec<K> = base_map.keys().cloned().collect();
    for (index, key) in base_keys.iter().enumerate() {
        let expected = expected(entries, index, &key);
        let result = result::<K, V, S, A>(&base_map, key);

        assert_eq!(result, expected);
    }
}

fn run_test_typed_proj_index_map_swap_remove_preserves_order_values<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = oimt::PrefixGenerator::new(entries);
    for entries in iter {
        run_test_typed_proj_index_map_swap_remove_preserves_order(entries, build_hasher.clone(), alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_index_map_swap_remove_preserves_order_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_swap_remove_preserves_order_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_typed_proj_index_map_swap_remove_preserves_order_range_values() {
                let spec = $range_spec;
                let entries = oimt::range_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_swap_remove_preserves_order_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_typed_proj_index_map_swap_remove_preserves_order_constant_values() {
                let spec = $const_spec;
                let entries = oimt::constant_key_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_swap_remove_preserves_order_values(&entries, build_hasher, alloc);
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
