use crate::map::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;
use opaque_index_map::map::TypedProjIndexMap;

use opaque_index_map_testing as oimt;

fn expected<K, V>(entries: &[(K, V)]) -> Vec<V>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let expected: Vec<V> = oimt::map::last_entry_per_key_ordered(entries)
        .iter()
        .map(|(key, value)| value)
        .cloned()
        .collect();

    expected
}

fn result<K, V, S, A>(map: &mut TypedProjIndexMap<K, V, S, A>) -> Vec<V>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let result: Vec<V> = map.as_mut_slice().values().cloned().collect();

    result
}

fn run_test_typed_proj_index_map_insert_as_mut_slice<K, V, S, A>(entries: &mut [(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = common::typed_proj_index_map::from_entries_in(entries, build_hasher, alloc);
    let expected = expected::<K, V>(&entries);
    let result = result::<K, V, S, A>(&mut map);

    assert_eq!(result, expected);
}

fn run_test_typed_proj_index_map_insert_as_mut_slice_values<K, V, S, A>(entries: &[(K, V)], build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = oimt::map::PrefixGenerator::new(entries);
    for entries in iter {
        let mut cloned_entries = Vec::from(entries);
        run_test_typed_proj_index_map_insert_as_mut_slice(cloned_entries.as_mut(), build_hasher.clone(), alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_index_map_insert_as_mut_slice_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::map::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_insert_as_mut_slice_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_typed_proj_index_map_insert_as_mut_slice_range_values() {
                let spec = $range_spec;
                let entries = oimt::map::range_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_insert_as_mut_slice_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_typed_proj_index_map_insert_as_mut_slice_constant_values() {
                let spec = $const_spec;
                let entries = oimt::map::constant_key_entries::<$key_typ, $value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_map_insert_as_mut_slice_values(&entries, build_hasher, alloc);
            }
        }
    };
}

generate_tests!(
    u64_i64,
    key_type = u64,
    value_type = i64,
    range_spec = oimt::map::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::map::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    usize_i64,
    key_type = usize,
    value_type = i64,
    range_spec = oimt::map::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::map::ConstantKeyEntriesSpec::new(126, 1..=128)
);
