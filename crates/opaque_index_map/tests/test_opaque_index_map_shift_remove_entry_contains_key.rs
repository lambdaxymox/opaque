#![feature(allocator_api)]
mod common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;
use opaque_index_map::OpaqueIndexMap;

use opaque_index_map_testing as oimt;

fn run_test_opaque_index_map_shift_remove_entry_contains_key<K, V>(entries: &[(K, V)])
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let mut map = common::from_entries(&entries);
    let keys: Vec<K> = map.keys::<K, V, hash::RandomState, alloc::Global>().cloned().collect();
    for key in keys.iter() {
        assert!(map.contains_key::<K, K, V, hash::RandomState, alloc::Global>(key));

        map.shift_remove_entry::<K, K, V, hash::RandomState, alloc::Global>(&key);

        assert!(!map.contains_key::<K, K, V, hash::RandomState, alloc::Global>(&key));
    }
}

fn run_test_opaque_index_map_shift_remove_entry_contains_key_values<K, V>(entries: &[(K, V)])
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let iter = oimt::PrefixGenerator::new(entries);
    for entries in iter {
        run_test_opaque_index_map_shift_remove_entry_contains_key(entries);
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_shift_remove_entry_contains_key_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                run_test_opaque_index_map_shift_remove_entry_contains_key_values(&entries);
            }

            #[test]
            fn test_opaque_index_map_shift_remove_entry_contains_key_range_values() {
                let spec = $range_spec;
                let entries = oimt::range_entries::<$key_typ, $value_typ>(spec);
                run_test_opaque_index_map_shift_remove_entry_contains_key_values(&entries);
            }

            #[test]
            fn test_opaque_index_map_shift_remove_entry_contains_key_constant_values() {
                let spec = $const_spec;
                let entries = oimt::constant_key_entries::<$key_typ, $value_typ>(spec);
                run_test_opaque_index_map_shift_remove_entry_contains_key_values(&entries);
            }
        }
    };
}

generate_tests!(
    u16_i8,
    key_type = u16,
    value_type = i8,
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    u16_i16,
    key_type = u16,
    value_type = i16,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u16_i32,
    key_type = u16,
    value_type = i32,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u16_i64,
    key_type = u16,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u16_i128,
    key_type = u16,
    value_type = i128,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u16_isize,
    key_type = u16,
    value_type = isize,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    u32_i8,
    key_type = u32,
    value_type = i8,
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    u32_i16,
    key_type = u32,
    value_type = i16,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u32_i32,
    key_type = u32,
    value_type = i32,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u32_i64,
    key_type = u32,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u32_i128,
    key_type = u32,
    value_type = i128,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u32_isize,
    key_type = u32,
    value_type = isize,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    u64_i8,
    key_type = u64,
    value_type = i8,
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    u64_i16,
    key_type = u64,
    value_type = i16,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u64_i32,
    key_type = u64,
    value_type = i32,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u64_i64,
    key_type = u64,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u64_i128,
    key_type = u64,
    value_type = i128,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u64_isize,
    key_type = u64,
    value_type = isize,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    u128_i8,
    key_type = u128,
    value_type = i8,
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    u128_i16,
    key_type = u128,
    value_type = i16,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u128_i32,
    key_type = u128,
    value_type = i32,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u128_i64,
    key_type = u128,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u128_i128,
    key_type = u128,
    value_type = i128,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u128_isize,
    key_type = u128,
    value_type = isize,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    usize_i8,
    key_type = usize,
    value_type = i8,
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    usize_i16,
    key_type = usize,
    value_type = i16,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    usize_i32,
    key_type = usize,
    value_type = i32,
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
generate_tests!(
    usize_i128,
    key_type = usize,
    value_type = i128,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    usize_isize,
    key_type = usize,
    value_type = isize,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    i16_i8,
    key_type = i16,
    value_type = i8,
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    i16_i16,
    key_type = i16,
    value_type = i16,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i16_i32,
    key_type = i16,
    value_type = i32,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i16_i64,
    key_type = i16,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i16_i128,
    key_type = i16,
    value_type = i128,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i16_isize,
    key_type = i16,
    value_type = isize,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    i32_i8,
    key_type = i32,
    value_type = i8,
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    i32_i16,
    key_type = i32,
    value_type = i16,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i32_i32,
    key_type = i32,
    value_type = i32,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i32_i64,
    key_type = i32,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i32_i128,
    key_type = i32,
    value_type = i128,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i32_isize,
    key_type = i32,
    value_type = isize,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    i64_i8,
    key_type = i64,
    value_type = i8,
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    i64_i16,
    key_type = i64,
    value_type = i16,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i64_i32,
    key_type = i64,
    value_type = i32,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i64_i64,
    key_type = i64,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i64_i128,
    key_type = i64,
    value_type = i128,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i64_isize,
    key_type = i64,
    value_type = isize,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    i128_i8,
    key_type = i128,
    value_type = i8,
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    i128_i16,
    key_type = i128,
    value_type = i16,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i128_i32,
    key_type = i128,
    value_type = i32,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i128_i64,
    key_type = i128,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i128_i128,
    key_type = i128,
    value_type = i128,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i128_isize,
    key_type = i128,
    value_type = isize,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    isize_i8,
    key_type = isize,
    value_type = i8,
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    isize_i16,
    key_type = isize,
    value_type = i16,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    isize_i32,
    key_type = isize,
    value_type = i32,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    isize_i64,
    key_type = isize,
    value_type = i64,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    isize_i128,
    key_type = isize,
    value_type = i128,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    isize_isize,
    key_type = isize,
    value_type = isize,
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
