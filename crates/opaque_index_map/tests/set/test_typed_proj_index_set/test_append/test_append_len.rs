use crate::set::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map_testing as oimt;

fn run_test_typed_proj_index_set_append_len_source<T, S1, S2, A>(
    values1: &[T],
    values2: &[T],
    build_hasher1: S1,
    build_hasher2: S2,
    alloc: A,
)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = common::typed_proj_index_set::from_entries_in(
        values1,
        build_hasher1,
        alloc.clone(),
    );
    let mut destination = common::typed_proj_index_set::from_entries_in(
        values2,
        build_hasher2,
        alloc.clone(),
    );

    source.append(&mut destination);

    assert!(source.len() <= values1.len() + values2.len());
}

fn run_test_typed_proj_index_set_append_len_destination<T, S1, S2, A>(
    values1: &[T],
    values2: &[T],
    build_hasher1: S1,
    build_hasher2: S2,
    alloc: A,
)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = common::typed_proj_index_set::from_entries_in(
        values1,
        build_hasher1,
        alloc.clone(),
    );
    let mut destination = common::typed_proj_index_set::from_entries_in(
        values2,
        build_hasher2,
        alloc.clone(),
    );

    source.append(&mut destination);

    assert_eq!(destination.len(), 0);
}

fn run_test_typed_proj_index_set_append_len_source_values<T, S1, S2, A>(
    values1: &[T],
    values2: &[T],
    build_hasher1: S1,
    build_hasher2: S2,
    alloc: A,
)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator1 = oimt::set::PrefixGenerator::new(values1);
    for source in iterator1 {
        let iterator2 = oimt::set::PrefixGenerator::new(values2);
        for destination in iterator2 {
            run_test_typed_proj_index_set_append_len_source(
                source,
                destination,
                build_hasher1.clone(),
                build_hasher2.clone(),
                alloc.clone(),
            );
        }
    }
}

fn run_test_typed_proj_index_set_append_len_destination_values<T, S1, S2, A>(
    values1: &[T],
    values2: &[T],
    build_hasher1: S1,
    build_hasher2: S2,
    alloc: A,
)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator1 = oimt::set::PrefixGenerator::new(values1);
    for source in iterator1 {
        let iterator2 = oimt::set::PrefixGenerator::new(values2);
        for destination in iterator2 {
            run_test_typed_proj_index_set_append_len_destination(
                source,
                destination,
                build_hasher1.clone(),
                build_hasher2.clone(),
                alloc.clone(),
            );
        }
    }
}

macro_rules! generate_tests {
    ($module_name:ident, value_type = $value_typ:ty, src_range_spec = $src_range_spec:expr, dst_range_spec = $dst_range_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn run_test_typed_proj_index_set_append_len_source_empty() {
                let values1: [$value_typ; 0] = [];
                let values2: [$value_typ; 0] = [];
                let build_hasher1 = common::typed_proj_index_set::WrappingBuildHasher1::new(hash::RandomState::new());
                let build_hasher2 = common::typed_proj_index_set::WrappingBuildHasher2::new(hash::RandomState::new());
                let alloc = alloc::Global;
                run_test_typed_proj_index_set_append_len_source_values(&values1, &values2, build_hasher1, build_hasher2, alloc);
            }

            #[test]
            fn test_typed_proj_index_set_append_len_source_range_values() {
                let values1 = oimt::set::range_entries::<$value_typ>($src_range_spec);
                let values2 = oimt::set::range_entries::<$value_typ>($dst_range_spec);
                let build_hasher1 = common::typed_proj_index_set::WrappingBuildHasher1::new(hash::RandomState::new());
                let build_hasher2 = common::typed_proj_index_set::WrappingBuildHasher2::new(hash::RandomState::new());
                let alloc = alloc::Global;
                run_test_typed_proj_index_set_append_len_source_values(&values1, &values2, build_hasher1, build_hasher2, alloc);
            }

            #[test]
            fn run_test_typed_proj_index_set_append_len_destination_empty() {
                let values1: [$value_typ; 0] = [];
                let values2: [$value_typ; 0] = [];
                let build_hasher1 = common::typed_proj_index_set::WrappingBuildHasher1::new(hash::RandomState::new());
                let build_hasher2 = common::typed_proj_index_set::WrappingBuildHasher2::new(hash::RandomState::new());
                let alloc = alloc::Global;
                run_test_typed_proj_index_set_append_len_destination_values(&values1, &values2, build_hasher1, build_hasher2, alloc);
            }

            #[test]
            fn test_typed_proj_index_set_append_len_destination_range_values() {
                let values1 = oimt::set::range_entries::<$value_typ>($src_range_spec);
                let values2 = oimt::set::range_entries::<$value_typ>($dst_range_spec);
                let build_hasher1 = common::typed_proj_index_set::WrappingBuildHasher1::new(hash::RandomState::new());
                let build_hasher2 = common::typed_proj_index_set::WrappingBuildHasher2::new(hash::RandomState::new());
                let alloc = alloc::Global;
                run_test_typed_proj_index_set_append_len_destination_values(&values1, &values2, build_hasher1, build_hasher2, alloc);
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
    src_range_spec = oimt::set::RangeEntriesSpec::new(Box::new(oimt::map::StringRangeInclusive::new(0..=63))),
    dst_range_spec = oimt::set::RangeEntriesSpec::new(Box::new(oimt::map::StringRangeInclusive::new(64..=128)))
);
