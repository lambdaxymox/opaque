use crate::set::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map_testing as oimt;

fn run_test_opaque_index_set_shift_remove_full_get<T, S, A>(entries: &[T], build_hasher: S, alloc: A)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = common::opaque_index_set::from_entries_in(entries, build_hasher, alloc);
    let values: Vec<T> = set.iter::<T, S, A>().cloned().collect();
    for value in values.iter() {
        let expected = set
            .get::<T, T, S, A>(value)
            .cloned();
        let result = set
            .shift_remove_full::<T, T, S, A>(value)
            .map(|(i, v)| v);

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_index_set_shift_remove_full_get_values<T, S, A>(entries: &[T], build_hasher: S, alloc: A)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator = oimt::set::PrefixGenerator::new(entries);
    for entries in iterator {
        run_test_opaque_index_set_shift_remove_full_get(entries, build_hasher.clone(), alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, value_type = $value_typ:ty, range_spec = $range_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_set_shift_remove_full_get_empty() {
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::set::values(values.iter().cloned());
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_opaque_index_set_shift_remove_full_get_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_opaque_index_set_shift_remove_full_get_range_values() {
                let spec = $range_spec;
                let entries = oimt::set::range_entries::<$value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_opaque_index_set_shift_remove_full_get_values(&entries, build_hasher, alloc);
            }
        }
    };
}

generate_tests!(
    u64,
    value_type = u64,
    range_spec = oimt::set::RangeEntriesSpec::new(Box::new(0..=127))
);
generate_tests!(
    usize,
    value_type = usize,
    range_spec = oimt::set::RangeEntriesSpec::new(Box::new(0..=127))
);
generate_tests!(
    string,
    value_type = String,
    range_spec = oimt::set::RangeEntriesSpec::new(Box::new(oimt::set::StringRangeInclusive::new(0..=127)))
);
