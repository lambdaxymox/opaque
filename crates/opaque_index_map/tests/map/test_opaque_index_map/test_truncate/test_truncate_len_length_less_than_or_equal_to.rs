use crate::map::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map::map::OpaqueIndexMap;

use opaque_index_map_testing as oimt;

fn expected<K, V>(entries: &[(K, V)], len: usize) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let vec: Vec<(K, V)> = oimt::map::last_entry_per_key_ordered(entries)
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .take(len)
        .collect();

    vec
}

fn result<K, V, S, A>(map: &OpaqueIndexMap, len: usize) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut cloned_map = common::opaque_index_map::clone::<K, V, S, A>(map);
    cloned_map.truncate::<K, V, S, A>(len);

    let vec: Vec<(K, V)> = cloned_map
        .iter::<K, V, S, A>()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    vec
}

fn run_test_opaque_index_map_truncate_len_length_less_than_or_equal_to<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    for len in 0..entries.len() {
        let map = common::opaque_index_map::from_entries_in(entries, build_hasher.clone(), alloc.clone());
        let expected_entries = expected::<K, V>(entries, len);
        let result_entries = result::<K, V, S, A>(&map, len);
        let expected = expected_entries.len();
        let result = result_entries.len();

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_index_map_truncate_len_length_less_than_or_equal_to_values<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = oimt::map::PrefixGenerator::new(entries);
    for entries in iter {
        run_test_opaque_index_map_truncate_len_length_less_than_or_equal_to(entries, build_hasher.clone(), alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_truncate_len_length_less_than_or_equal_to_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::map::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_opaque_index_map_truncate_len_length_less_than_or_equal_to_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_opaque_index_map_as_truncate_range_values() {
                let spec = $range_spec;
                let entries = oimt::map::range_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_opaque_index_map_truncate_len_length_less_than_or_equal_to_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_opaque_index_map_truncate_len_length_less_than_or_equal_to_const_values() {
                let spec = $const_spec;
                let entries = oimt::map::constant_key_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_opaque_index_map_truncate_len_length_less_than_or_equal_to_values(&entries, build_hasher, alloc);
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
