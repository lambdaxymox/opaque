use crate::map::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map_testing as oimt;

fn run_test_typed_proj_index_map_iter_get_mut<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = common::typed_proj_index_map::from_entries(entries, build_hasher, alloc);
    let entries: Vec<(K, V)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    for (key, value) in entries.iter() {
        let expected = Some(value.clone());
        let result = map.get_mut(key).cloned();

        assert_eq!(result, expected);
    }
}

fn run_test_typed_proj_index_map_iter_get_mut_values<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = oimt::PrefixGenerator::new(entries);
    for entries in iter {
        run_test_typed_proj_index_map_iter_get_mut(entries, build_hasher.clone(), alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_index_map_iter_get_mut_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_iter_get_mut_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_typed_proj_index_map_iter_get_mut_range_values() {
                let spec = $range_spec;
                let entries = oimt::range_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_iter_get_mut_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_typed_proj_index_map_iter_get_mut_constant_values() {
                let spec = $const_spec;
                let entries = oimt::constant_key_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_iter_get_mut_values(&entries, build_hasher, alloc);
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
