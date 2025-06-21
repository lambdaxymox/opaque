use crate::set::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map::set::TypedProjIndexSet;

use opaque_index_map_testing as oimt;

fn expected<T>(entries: &[T], index: usize, value: &T) -> Vec<T>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
{
    let mut set_entries: Vec<T> = entries.iter().cloned().collect();

    assert_eq!(set_entries[index], value.clone());

    set_entries.remove(index);

    set_entries
}

fn result<T, S, A>(set: &TypedProjIndexSet<T, S, A>, value: &T) -> Vec<T>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut new_set = set.clone();
    new_set.shift_remove(value);

    let ordered_entries: Vec<T> = new_set
        .iter()
        .cloned()
        .collect();

    ordered_entries
}

fn run_test_typed_proj_index_set_shift_remove_preserves_order<T, S, A>(entries: &[T], build_hasher: S, alloc: A)
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let base_set = common::typed_proj_index_set::from_entries_in(entries, build_hasher, alloc);
    let base_values: Vec<T> = base_set.iter().cloned().collect();
    for (index, value) in base_values.iter().enumerate() {
        let expected = expected(entries, index, &value);
        let result = result::<T, S, A>(&base_set, value);

        assert_eq!(result, expected);
    }
}

fn run_test_typed_proj_index_set_shift_remove_preserves_order_values<T, S, A>(entries: &[T], build_hasher: S, alloc: A)
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator = oimt::set::PrefixGenerator::new(entries);
    for entries in iterator {
        run_test_typed_proj_index_set_shift_remove_preserves_order(entries, build_hasher.clone(), alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, value_type = $value_typ:ty, range_spec = $range_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_index_set_shift_remove_preserves_order_empty() {
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::set::values(values.iter().cloned());
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_set_shift_remove_preserves_order_values(&entries, build_hasher, alloc);
            }

            #[test]
            fn test_typed_proj_index_set_shift_remove_preserves_order_range_values() {
                let spec = $range_spec;
                let entries = oimt::set::range_entries::<$value_typ>(spec);
                let build_hasher = hash::RandomState::new();
                let alloc = alloc::Global;
                run_test_typed_proj_index_set_shift_remove_preserves_order_values(&entries, build_hasher, alloc);
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
