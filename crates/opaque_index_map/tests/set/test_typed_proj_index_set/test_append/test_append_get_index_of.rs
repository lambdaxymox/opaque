use crate::set::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map_testing as oimt;

fn expected<T>(values1: &[T], values2: &[T]) -> Vec<T>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
{
    let mut entries = Vec::from(values1);
    entries.extend_from_slice(values2);

    entries
}

fn run_test_typed_proj_index_set_append_get_index_of_source<T, S1, A1, S2, A2>(
    values1: &[T],
    values2: &[T],
    build_hasher1: S1,
    alloc1: A1,
    build_hasher2: S2,
    alloc2: A2,
)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A1: any::Any + alloc::Allocator + Send + Sync + Clone,
    A2: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = common::typed_proj_index_set::from_entries_in(
        values1,
        build_hasher1,
        alloc1
    );
    let mut destination = common::typed_proj_index_set::from_entries_in(
        values2,
        build_hasher2,
        alloc2
    );

    source.append(&mut destination);

    let expected_vec = expected(values1, values2);
    for (index, value) in expected_vec.iter().enumerate() {
        let expected = Some(value);
        let result = source.get(value);

        assert_eq!(result, expected);
    }
}

fn run_test_typed_proj_index_set_append_get_index_of_destination<T, S1, A1, S2, A2>(
    values1: &[T],
    values2: &[T],
    build_hasher1: S1,
    alloc1: A1,
    build_hasher2: S2,
    alloc2: A2,
)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A1: any::Any + alloc::Allocator + Send + Sync + Clone,
    A2: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = common::typed_proj_index_set::from_entries_in(
        values1,
        build_hasher1,
        alloc1
    );
    let mut destination = common::typed_proj_index_set::from_entries_in(
        values2,
        build_hasher2,
        alloc2
    );

    source.append(&mut destination);

    for value in values1.iter() {
        assert!(destination.get_index_of(value).is_none());
    }

    for value in values2.iter() {
        assert!(destination.get_index_of(value).is_none());
    }
}

fn run_test_typed_proj_index_set_append_get_index_of_source_values<T, S1, A1, S2, A2>(
    values1: &[T],
    values2: &[T],
    build_hasher1: S1,
    alloc1: A1,
    build_hasher2: S2,
    alloc2: A2
)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A1: any::Any + alloc::Allocator + Send + Sync + Clone,
    A2: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator1 = oimt::set::PrefixGenerator::new(values1);
    for source in iterator1 {
        let iterator2 = oimt::set::PrefixGenerator::new(values2);
        for destination in iterator2 {
            run_test_typed_proj_index_set_append_get_index_of_source(
                source,
                destination,
                build_hasher1.clone(),
                alloc1.clone(),
                build_hasher2.clone(),
                alloc2.clone(),
            );
        }
    }
}

fn run_test_typed_proj_index_set_append_get_index_of_destination_values<T, S1, A1, S2, A2>(
    values1: &[T],
    values2: &[T],
    build_hasher1: S1,
    alloc1: A1,
    build_hasher2: S2,
    alloc2: A2
)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A1: any::Any + alloc::Allocator + Send + Sync + Clone,
    A2: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator1 = oimt::set::PrefixGenerator::new(values1);
    for source in iterator1 {
        let iterator2 = oimt::set::PrefixGenerator::new(values2);
        for destination in iterator2 {
            run_test_typed_proj_index_set_append_get_index_of_destination(
                source,
                destination,
                build_hasher1.clone(),
                alloc1.clone(),
                build_hasher2.clone(),
                alloc2.clone(),
            );
        }
    }
}

macro_rules! generate_tests {
    ($module_name:ident, value_type = $value_typ:ty, src_range_spec = $src_range_spec:expr, dst_range_spec = $dst_range_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn run_test_typed_proj_index_set_append_get_index_of_source_empty() {
                let values1: [$value_typ; 0] = [];
                let values2: [$value_typ; 0] = [];
                let build_hasher1 = hash::RandomState::new();
                let alloc1 = alloc::Global;
                let build_hasher2 = hash::RandomState::new();
                let alloc2 = alloc::Global;
                run_test_typed_proj_index_set_append_get_index_of_source_values(&values1, &values2, build_hasher1, alloc1, build_hasher2, alloc2);
            }

            #[test]
            fn test_typed_proj_index_set_append_get_index_of_source_range_values() {
                let values1 = opaque_index_map_testing::set::range_entries::<$value_typ>($src_range_spec);
                let values2 = opaque_index_map_testing::set::range_entries::<$value_typ>($dst_range_spec);
                let build_hasher1 = hash::RandomState::new();
                let alloc1 = alloc::Global;
                let build_hasher2 = hash::RandomState::new();
                let alloc2 = alloc::Global;
                run_test_typed_proj_index_set_append_get_index_of_source_values(&values1, &values2, build_hasher1, alloc1, build_hasher2, alloc2);
            }

            #[test]
            fn run_test_typed_proj_index_set_append_get_index_of_destination_empty() {
                let values1: [$value_typ; 0] = [];
                let values2: [$value_typ; 0] = [];
                let build_hasher1 = hash::RandomState::new();
                let alloc1 = alloc::Global;
                let build_hasher2 = hash::RandomState::new();
                let alloc2 = alloc::Global;
                run_test_typed_proj_index_set_append_get_index_of_destination_values(&values1, &values2, build_hasher1, alloc1, build_hasher2, alloc2);
            }

            #[test]
            fn test_typed_proj_index_set_append_get_index_of_destination_range_values() {
                let values1 = opaque_index_map_testing::set::range_entries::<$value_typ>($src_range_spec);
                let values2 = opaque_index_map_testing::set::range_entries::<$value_typ>($dst_range_spec);
                let build_hasher1 = hash::RandomState::new();
                let alloc1 = alloc::Global;
                let build_hasher2 = hash::RandomState::new();
                let alloc2 = alloc::Global;
                run_test_typed_proj_index_set_append_get_index_of_destination_values(&values1, &values2, build_hasher1, alloc1, build_hasher2, alloc2);
            }
        }
    };
}

generate_tests!(
    u64,
    value_type = u64,
    src_range_spec = oimt::set::RangeEntriesSpec::new(Box::new(0..=63)),
    dst_range_spec = oimt::set::RangeEntriesSpec::new(Box::new(64..=128))
);
generate_tests!(
    usize,
    value_type = usize,
    src_range_spec = oimt::set::RangeEntriesSpec::new(Box::new(0..=63)),
    dst_range_spec = oimt::set::RangeEntriesSpec::new(Box::new(64..=128))
);
generate_tests!(
    string,
    value_type = String,
    src_range_spec = oimt::set::RangeEntriesSpec::new(Box::new(oimt::set::StringRangeInclusive::new(0..=63))),
    dst_range_spec = oimt::set::RangeEntriesSpec::new(Box::new(oimt::set::StringRangeInclusive::new(64..=128)))
);
