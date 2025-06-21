use crate::map::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map_testing as oimt;

fn run_test_typed_proj_index_map_swap_remove_entry_get<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = common::typed_proj_index_map::from_entries_in(entries, build_hasher, alloc);
    let keys: Vec<K> = map.keys().cloned().collect();
    for key in keys.iter() {
        let expected = map
            .get(key)
            .cloned();
        let result = map
            .swap_remove_entry(key)
            .map(|(k, v)| v);

        assert_eq!(result, expected);
    }
}

fn run_test_typed_proj_index_map_swap_remove_entry_get_values<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator = oimt::map::PrefixGenerator::new(entries);
    for entries in iterator {
        run_test_typed_proj_index_map_swap_remove_entry_get(entries, build_hasher.clone(), alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_index_map_swap_remove_entry_get_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::map::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_swap_remove_entry_get_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_typed_proj_index_map_swap_remove_entry_get_range_values() {
                let spec = $range_spec;
                let entries = oimt::map::range_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_swap_remove_entry_get_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_typed_proj_index_map_swap_remove_entry_get_constant_values() {
                let spec = $const_spec;
                let entries = oimt::map::constant_key_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_swap_remove_entry_get_values(&entries, build_hasher, alloc);
            }
        }
    };
}

generate_tests!(
    u64_i64,
    key_type = u64,
    value_type = i64,
    range_spec = oimt::map::RangeEntriesSpec::new(Box::new(0..=127), Box::new(1..=128)),
    const_spec = oimt::map::ConstantKeyEntriesSpec::new(126, Box::new(1..=128))
);
generate_tests!(
    usize_i64,
    key_type = usize,
    value_type = i64,
    range_spec = oimt::map::RangeEntriesSpec::new(Box::new(0..=127), Box::new(1..=128)),
    const_spec = oimt::map::ConstantKeyEntriesSpec::new(126, Box::new(1..=128))
);
generate_tests!(
    string_i64,
    key_type = String,
    value_type = i64,
    range_spec = oimt::map::RangeEntriesSpec::new(Box::new(oimt::map::StringRangeInclusive::new(0..=127)), Box::new(1..=128)),
    const_spec = oimt::map::ConstantKeyEntriesSpec::new(String::from("foo"), Box::new(1..=128))
);
generate_tests!(
    string_string,
    key_type = String,
    value_type = String,
    range_spec = oimt::map::RangeEntriesSpec::new(Box::new(oimt::map::StringRangeInclusive::new(0..=127)), Box::new(oimt::map::StringRangeInclusive::new(1..=128))),
    const_spec = oimt::map::ConstantKeyEntriesSpec::new(String::from("foo"), Box::new(oimt::map::StringRangeInclusive::new(1..=128)))
);
