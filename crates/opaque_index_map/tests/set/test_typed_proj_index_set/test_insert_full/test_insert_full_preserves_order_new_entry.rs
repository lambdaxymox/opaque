use crate::set::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map_testing as oimt;

fn run_test_typed_proj_index_set_insert_full_preserves_order_new_entry<T, S, A>(entries: &[T], build_hasher: S, alloc: A, new_entry: &T)
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = common::typed_proj_index_set::from_entries_in(entries, build_hasher, alloc);

    assert!(!set.contains(new_entry));

    let values_before: Vec<T> = set.iter().cloned().collect();

    set.insert_full(new_entry.clone());

    let values_after: Vec<T> = set.iter().cloned().collect();

    let expected = {
        let mut _vec = values_before.clone();
        _vec.push(new_entry.clone());
        _vec
    };
    let result = values_after;

    assert_eq!(result, expected);
}

fn run_test_typed_proj_index_set_insert_full_preserves_order_new_entry_values<T, S, A>(entries: &[T], build_hasher: S, alloc: A, new_entry: &T)
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator = oimt::set::PrefixGenerator::new(entries);
    for entries in iterator {
        run_test_typed_proj_index_set_insert_full_preserves_order_new_entry(entries, build_hasher.clone(), alloc.clone(), new_entry);
    }
}

macro_rules! generate_tests {
    ($module_name:ident, value_type = $value_typ:ty, new_entry = $new_entry:expr, range_spec = $range_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_index_set_insert_full_preserves_order_new_entry_empty() {
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::set::values(values.iter().cloned());
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                let new_entry = $new_entry;
                run_test_typed_proj_index_set_insert_full_preserves_order_new_entry_values(&entries, build_hasher, alloc, &new_entry);
            }

            #[test]
            fn test_typed_proj_index_set_insert_full_preserves_order_new_entry_range_values() {
                let spec = $range_spec;
                let entries = oimt::set::range_entries::<$value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                let new_entry = $new_entry;
                run_test_typed_proj_index_set_insert_full_preserves_order_new_entry_values(&entries, build_hasher, alloc, &new_entry);
            }
        }
    };
}

generate_tests!(
    u64,
    value_type = u64,
    new_entry = u64::MAX,
    range_spec = oimt::set::RangeEntriesSpec::new(Box::new(0..=127))
);
generate_tests!(
    usize,
    value_type = usize,
    new_entry = usize::MAX,
    range_spec = oimt::set::RangeEntriesSpec::new(Box::new(0..=127))
);
generate_tests!(
    string,
    value_type = String,
    new_entry = isize::MAX.to_string(),
    range_spec = oimt::set::RangeEntriesSpec::new(Box::new(oimt::set::StringRangeInclusive::new(0..=127)))
);
