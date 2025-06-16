use crate::map::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map_testing as oimt;

fn run_test_typed_proj_index_map_insert_full_preserves_order_new_entry<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A, new_entry: &(K, V))
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = common::typed_proj_index_map::from_entries_in(entries, build_hasher, alloc);

    assert!(!map.contains_key(&new_entry.0));

    let keys_before: Vec<K> = map.keys().cloned().collect();

    map.insert_full(new_entry.0.clone(), new_entry.1.clone());

    let keys_after: Vec<K> = map.keys().cloned().collect();

    let expected = {
        let mut _vec = keys_before.clone();
        _vec.push(new_entry.0.clone());
        _vec
    };
    let result = keys_after;

    assert_eq!(result, expected);
}

fn run_test_typed_proj_index_map_insert_full_preserves_order_new_entry_values<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A, new_entry: &(K, V))
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator = oimt::map::PrefixGenerator::new(entries);
    for entries in iterator {
        run_test_typed_proj_index_map_insert_full_preserves_order_new_entry(entries, build_hasher.clone(), alloc.clone(), new_entry);
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, new_entry = $new_entry:expr, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_index_map_insert_full_preserves_order_new_entry_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::map::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                let new_entry = $new_entry;
                run_test_typed_proj_index_map_insert_full_preserves_order_new_entry_values(&entries, build_hasher, alloc, &new_entry);
            }

            #[test]
            fn test_typed_proj_index_map_insert_full_preserves_order_new_entry_range_values() {
                let spec = $range_spec;
                let entries = oimt::map::range_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                let new_entry = $new_entry;
                run_test_typed_proj_index_map_insert_full_preserves_order_new_entry_values(&entries, build_hasher, alloc, &new_entry);
            }

            #[test]
            fn test_typed_proj_index_map_insert_full_preserves_order_new_entry_constant_values() {
                let spec = $const_spec;
                let entries = oimt::map::constant_key_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                let new_entry = $new_entry;
                run_test_typed_proj_index_map_insert_full_preserves_order_new_entry_values(&entries, build_hasher, alloc, &new_entry);
            }
        }
    };
}

generate_tests!(
    u64_i64,
    key_type = u64,
    value_type = i64,
    new_entry = (u64::MAX, i64::MAX),
    range_spec = oimt::map::RangeEntriesSpec::new(Box::new(0..=127), Box::new(1..=128)),
    const_spec = oimt::map::ConstantKeyEntriesSpec::new(126, Box::new(1..=128))
);
generate_tests!(
    usize_i64,
    key_type = usize,
    value_type = i64,
    new_entry = (usize::MAX, i64::MAX),
    range_spec = oimt::map::RangeEntriesSpec::new(Box::new(0..=127), Box::new(1..=128)),
    const_spec = oimt::map::ConstantKeyEntriesSpec::new(126, Box::new(1..=128))
);
generate_tests!(
    string_i64,
    key_type = String,
    value_type = i64,
    new_entry = (isize::MAX.to_string(), i64::MAX),
    range_spec = oimt::map::RangeEntriesSpec::new(Box::new(oimt::map::StringRangeInclusive::new(0..=127)), Box::new(1..=128)),
    const_spec = oimt::map::ConstantKeyEntriesSpec::new(String::from("foo"), Box::new(1..=128))
);
generate_tests!(
    string_string,
    key_type = String,
    value_type = String,
    new_entry = (isize::MAX.to_string(), isize::MAX.to_string()),
    range_spec = oimt::map::RangeEntriesSpec::new(Box::new(oimt::map::StringRangeInclusive::new(0..=127)), Box::new(oimt::map::StringRangeInclusive::new(1..=128))),
    const_spec = oimt::map::ConstantKeyEntriesSpec::new(String::from("foo"), Box::new(oimt::map::StringRangeInclusive::new(1..=128)))
);
