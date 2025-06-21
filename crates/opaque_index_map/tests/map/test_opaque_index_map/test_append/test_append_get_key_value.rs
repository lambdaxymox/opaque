use crate::map::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use opaque_index_map_testing as oimt;

fn expected<K, V>(values1: &[(K, V)], values2: &[(K, V)]) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let mut entries = Vec::from(values1);
    entries.extend_from_slice(values2);

    oimt::map::last_entry_per_key_ordered(&entries)
}

fn run_test_opaque_index_map_append_get_source<K, V, S1, A1, S2, A2>(
    values1: &[(K, V)],
    values2: &[(K, V)],
    build_hasher1: S1,
    alloc1: A1,
    build_hasher2: S2,
    alloc2: A2,
)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A1: any::Any + alloc::Allocator + Send + Sync + Clone,
    A2: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = common::opaque_index_map::from_entries_in(
        values1,
        build_hasher1,
        alloc1
    );
    let mut destination = common::opaque_index_map::from_entries_in(
        values2,
        build_hasher2,
        alloc2
    );

    source.append::<K, V, S1, A1, S2, A2>(&mut destination);

    let expected_vec = expected(values1, values2);
    for (key, value) in expected_vec.iter() {
        let expected = Some((key, value));
        let result = source.get_key_value::<_, K, V, S1, A1>(key);

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_index_map_append_get_destination<K, V, S1, A1, S2, A2>(
    values1: &[(K, V)],
    values2: &[(K, V)],
    build_hasher1: S1,
    alloc1: A1,
    build_hasher2: S2,
    alloc2: A2,
)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A1: any::Any + alloc::Allocator + Send + Sync + Clone,
    A2: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = common::opaque_index_map::from_entries_in(
        values1,
        build_hasher1,
        alloc1
    );
    let mut destination = common::opaque_index_map::from_entries_in(
        values2,
        build_hasher2,
        alloc2
    );

    source.append::<K, V, S1, A1, S2, A2>(&mut destination);

    for (key, _) in values1.iter() {
        assert!(destination.get_key_value::<_, K, V, S2, A2>(key).is_none());
    }

    for (key, _) in values2.iter() {
        assert!(destination.get_key_value::<_, K, V, S2, A2>(key).is_none());
    }
}

fn run_test_opaque_index_map_append_get_source_values<K, V, S1, A1, S2, A2>(
    values1: &[(K, V)],
    values2: &[(K, V)],
    build_hasher1: S1,
    alloc1: A1,
    build_hasher2: S2,
    alloc2: A2
)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A1: any::Any + alloc::Allocator + Send + Sync + Clone,
    A2: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator1 = oimt::map::PrefixGenerator::new(values1);
    for source in iterator1 {
        let iterator2 = oimt::map::PrefixGenerator::new(values2);
        for destination in iterator2 {
            run_test_opaque_index_map_append_get_source(
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

fn run_test_opaque_index_map_append_get_destination_values<K, V, S1, A1, S2, A2>(
    values1: &[(K, V)],
    values2: &[(K, V)],
    build_hasher1: S1,
    alloc1: A1,
    build_hasher2: S2,
    alloc2: A2
)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A1: any::Any + alloc::Allocator + Send + Sync + Clone,
    A2: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator1 = oimt::map::PrefixGenerator::new(values1);
    for source in iterator1 {
        let iterator2 = oimt::map::PrefixGenerator::new(values2);
        for destination in iterator2 {
            run_test_opaque_index_map_append_get_destination(
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
    (
        $module_name:ident,
        key_type = $key_typ:ty,
        value_type = $value_typ:ty,
        src_range_spec = $src_range_spec:expr,
        dst_range_spec = $dst_range_spec:expr
    ) => {
        mod $module_name {
            use super::*;

            #[test]
            fn run_test_opaque_index_map_append_get_source_empty() {
                let values1: [($key_typ, $value_typ); 0] = [];
                let values2: [($key_typ, $value_typ); 0] = [];
                let build_hasher1 = hash::RandomState::new();
                let alloc1 = common::opaque_index_map::WrappingAlloc1::new(alloc::Global);
                let build_hasher2 = hash::RandomState::new();
                let alloc2 = common::opaque_index_map::WrappingAlloc2::new(alloc::Global);
                run_test_opaque_index_map_append_get_source_values(&values1, &values2, build_hasher1, alloc1, build_hasher2, alloc2);
            }

            #[test]
            fn test_opaque_index_map_append_get_source_range_values() {
                let values1 = oimt::map::range_entries::<$key_typ, $value_typ>($src_range_spec);
                let values2 = oimt::map::range_entries::<$key_typ, $value_typ>($dst_range_spec);
                let build_hasher1 = hash::RandomState::new();
                let alloc1 = common::opaque_index_map::WrappingAlloc1::new(alloc::Global);
                let build_hasher2 = hash::RandomState::new();
                let alloc2 = common::opaque_index_map::WrappingAlloc2::new(alloc::Global);
                run_test_opaque_index_map_append_get_source_values(&values1, &values2, build_hasher1, alloc1, build_hasher2, alloc2);
            }

            #[test]
            fn run_test_opaque_index_map_append_get_destination_empty() {
                let values1: [($key_typ, $value_typ); 0] = [];
                let values2: [($key_typ, $value_typ); 0] = [];
                let build_hasher1 = hash::RandomState::new();
                let alloc1 = common::opaque_index_map::WrappingAlloc1::new(alloc::Global);
                let build_hasher2 = hash::RandomState::new();
                let alloc2 = common::opaque_index_map::WrappingAlloc2::new(alloc::Global);
                run_test_opaque_index_map_append_get_destination_values(&values1, &values2, build_hasher1, alloc1, build_hasher2, alloc2);
            }

            #[test]
            fn test_opaque_index_map_append_get_destination_range_values() {
                let values1 = oimt::map::range_entries::<$key_typ, $value_typ>($src_range_spec);
                let values2 = oimt::map::range_entries::<$key_typ, $value_typ>($dst_range_spec);
                let build_hasher1 = hash::RandomState::new();
                let alloc1 = common::opaque_index_map::WrappingAlloc1::new(alloc::Global);
                let build_hasher2 = hash::RandomState::new();
                let alloc2 = common::opaque_index_map::WrappingAlloc2::new(alloc::Global);
                run_test_opaque_index_map_append_get_destination_values(&values1, &values2, build_hasher1, alloc1, build_hasher2, alloc2);
            }
        }
    };
}

generate_tests!(
    u64_i64,
    key_type = u64,
    value_type = i64,
    src_range_spec = oimt::map::RangeEntriesSpec::new(Box::new(0..=63), Box::new(1..=64)),
    dst_range_spec = oimt::map::RangeEntriesSpec::new(Box::new(64..=128), Box::new(65..=129))
);
generate_tests!(
    usize_i64,
    key_type = usize,
    value_type = i64,
    src_range_spec = oimt::map::RangeEntriesSpec::new(Box::new(0..=63), Box::new(1..=64)),
    dst_range_spec = oimt::map::RangeEntriesSpec::new(Box::new(64..=128), Box::new(65..=129))
);
generate_tests!(
    string_i64,
    key_type = String,
    value_type = i64,
    src_range_spec = oimt::map::RangeEntriesSpec::new(Box::new(oimt::map::StringRangeInclusive::new(0..=63)), Box::new(1..=64)),
    dst_range_spec = oimt::map::RangeEntriesSpec::new(Box::new(oimt::map::StringRangeInclusive::new(64..=128)), Box::new(65..=129))
);
generate_tests!(
    string_string,
    key_type = String,
    value_type = String,
    src_range_spec = oimt::map::RangeEntriesSpec::new(Box::new(oimt::map::StringRangeInclusive::new(0..=63)), Box::new(oimt::map::StringRangeInclusive::new(1..=64))),
    dst_range_spec = oimt::map::RangeEntriesSpec::new(Box::new(oimt::map::StringRangeInclusive::new(64..=128)), Box::new(oimt::map::StringRangeInclusive::new(65..=129)))
);
