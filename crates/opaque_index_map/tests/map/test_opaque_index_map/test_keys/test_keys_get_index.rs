use crate::map::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map_testing as oimt;

fn run_test_opaque_index_map_keys_get_index<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = common::opaque_index_map::from_entries_in(entries, build_hasher, alloc);
    for (index, key) in map.keys::<K, V, S, A>().enumerate() {
        let expected = Some(key.clone());
        let result = map.get_index::<K, V, S, A>(index).map(|(k, v)| k.clone());

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_index_map_keys_get_index_values<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = oimt::map::PrefixGenerator::new(entries);
    for entries in iter {
        run_test_opaque_index_map_keys_get_index(entries, build_hasher.clone(), alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_keys_get_index_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::map::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_opaque_index_map_keys_get_index_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_opaque_index_map_keys_get_index_range_values() {
                let spec = $range_spec;
                let entries = oimt::map::range_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_opaque_index_map_keys_get_index_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_opaque_index_map_keys_get_index_constant_values() {
                let spec = $const_spec;
                let entries = oimt::map::constant_key_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_opaque_index_map_keys_get_index_values(&entries, build_hasher, alloc);
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
