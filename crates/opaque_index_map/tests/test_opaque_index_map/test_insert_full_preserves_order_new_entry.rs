use crate::common;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;
use opaque_index_map::OpaqueIndexMap;
use opaque_vec::OpaqueVec;

use opaque_index_map_testing as oimt;

fn run_test_opaque_index_map_insert_full_preserves_order_new_entry<K, V>(entries: &[(K, V)], new_entry: &(K, V))
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let mut map = common::opaque_index_map::from_entries(entries);

    assert!(!map.contains_key::<K, K, V, hash::RandomState, alloc::Global>(&new_entry.0));

    let keys_before: Vec<K> = map.keys::<K, V, hash::RandomState, alloc::Global>().cloned().collect();

    map.insert_full::<K, V, hash::RandomState, alloc::Global>(new_entry.0.clone(), new_entry.1.clone());

    let keys_after: Vec<K> = map.keys::<K, V, hash::RandomState, alloc::Global>().cloned().collect();

    let expected = {
        let mut _vec = keys_before.clone();
        _vec.push(new_entry.0.clone());
        _vec
    };
    let result = keys_after;

    assert_eq!(result, expected);
}

fn run_test_opaque_index_map_insert_full_preserves_order_new_entry_values<K, V>(entries: &[(K, V)], new_entry: &(K, V))
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let iter = oimt::PrefixGenerator::new(entries);
    for entries in iter {
        run_test_opaque_index_map_insert_full_preserves_order_new_entry(entries, new_entry);
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, new_entry = $new_entry:expr, range_spec = $range_spec:expr, const_spec = $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_insert_full_preserves_order_new_entry_empty() {
                let keys: Vec<$key_typ> = Vec::from(&[]);
                let values: Vec<$value_typ> = Vec::from(&[]);
                let entries = oimt::key_value_pairs(keys.iter().cloned(), values.iter().cloned());
                let new_entry = $new_entry;
                run_test_opaque_index_map_insert_full_preserves_order_new_entry_values(&entries, &new_entry);
            }

            #[test]
            fn test_opaque_index_map_insert_full_preserves_order_new_entry_range_values() {
                let spec = $range_spec;
                let entries = oimt::range_entries::<$key_typ, $value_typ>(spec);
                let new_entry = $new_entry;
                run_test_opaque_index_map_insert_full_preserves_order_new_entry_values(&entries, &new_entry);
            }

            #[test]
            fn test_opaque_index_map_insert_full_preserves_order_new_entry_constant_values() {
                let spec = $const_spec;
                let entries = oimt::constant_key_entries::<$key_typ, $value_typ>(spec);
                let new_entry = $new_entry;
                run_test_opaque_index_map_insert_full_preserves_order_new_entry_values(&entries, &new_entry);
            }
        }
    };
}

generate_tests!(
    u16_i8,
    key_type = u16,
    value_type = i8,
    new_entry = (u16::MAX, i8::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    u16_i16,
    key_type = u16,
    value_type = i16,
    new_entry = (u16::MAX, i16::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u16_i32,
    key_type = u16,
    value_type = i32,
    new_entry = (u16::MAX, i32::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u16_i64,
    key_type = u16,
    value_type = i64,
    new_entry = (u16::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u16_i128,
    key_type = u16,
    value_type = i128,
    new_entry = (u16::MAX, i128::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u16_isize,
    key_type = u16,
    value_type = isize,
    new_entry = (u16::MAX, isize::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    u32_i8,
    key_type = u32,
    value_type = i8,
    new_entry = (u32::MAX, i8::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    u32_i16,
    key_type = u32,
    value_type = i16,
    new_entry = (u32::MAX, i16::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u32_i32,
    key_type = u32,
    value_type = i32,
    new_entry = (u32::MAX, i32::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u32_i64,
    key_type = u32,
    value_type = i64,
    new_entry = (u32::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u32_i128,
    key_type = u32,
    value_type = i128,
    new_entry = (u32::MAX, i128::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u32_isize,
    key_type = u32,
    value_type = isize,
    new_entry = (u32::MAX, isize::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    u64_i8,
    key_type = u64,
    value_type = i8,
    new_entry = (u64::MAX, i8::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    u64_i16,
    key_type = u64,
    value_type = i16,
    new_entry = (u64::MAX, i16::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u64_i32,
    key_type = u64,
    value_type = i32,
    new_entry = (u64::MAX, i32::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u64_i64,
    key_type = u64,
    value_type = i64,
    new_entry = (u64::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u64_i128,
    key_type = u64,
    value_type = i128,
    new_entry = (u64::MAX, i128::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u64_isize,
    key_type = u64,
    value_type = isize,
    new_entry = (u64::MAX, isize::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    u128_i8,
    key_type = u128,
    value_type = i8,
    new_entry = (u128::MAX, i8::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    u128_i16,
    key_type = u128,
    value_type = i16,
    new_entry = (u128::MAX, i16::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u128_i32,
    key_type = u128,
    value_type = i32,
    new_entry = (u128::MAX, i32::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u128_i64,
    key_type = u128,
    value_type = i64,
    new_entry = (u128::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u128_i128,
    key_type = u128,
    value_type = i128,
    new_entry = (u128::MAX, i128::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    u128_isize,
    key_type = u128,
    value_type = isize,
    new_entry = (u128::MAX, isize::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    usize_i8,
    key_type = usize,
    value_type = i8,
    new_entry = (usize::MAX, i8::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    usize_i16,
    key_type = usize,
    value_type = i16,
    new_entry = (usize::MAX, i16::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    usize_i32,
    key_type = usize,
    value_type = i32,
    new_entry = (usize::MAX, i32::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    usize_i64,
    key_type = usize,
    value_type = i64,
    new_entry = (usize::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    usize_i128,
    key_type = usize,
    value_type = i128,
    new_entry = (usize::MAX, i128::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    usize_isize,
    key_type = usize,
    value_type = isize,
    new_entry = (usize::MAX, isize::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    i16_i8,
    key_type = i16,
    value_type = i8,
    new_entry = (i16::MAX, i8::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    i16_i16,
    key_type = i16,
    value_type = i16,
    new_entry = (i16::MAX, i16::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i16_i32,
    key_type = i16,
    value_type = i32,
    new_entry = (i16::MAX, i32::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i16_i64,
    key_type = i16,
    value_type = i64,
    new_entry = (i16::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i16_i128,
    key_type = i16,
    value_type = i128,
    new_entry = (i16::MAX, i128::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i16_isize,
    key_type = i16,
    value_type = isize,
    new_entry = (i16::MAX, isize::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    i32_i8,
    key_type = i32,
    value_type = i8,
    new_entry = (i32::MAX, i8::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    i32_i16,
    key_type = i32,
    value_type = i16,
    new_entry = (i32::MAX, i16::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i32_i32,
    key_type = i32,
    value_type = i32,
    new_entry = (i32::MAX, i32::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i32_i64,
    key_type = i32,
    value_type = i64,
    new_entry = (i32::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i32_i128,
    key_type = i32,
    value_type = i128,
    new_entry = (i32::MAX, i128::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i32_isize,
    key_type = i32,
    value_type = isize,
    new_entry = (i32::MAX, isize::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    i64_i8,
    key_type = i64,
    value_type = i8,
    new_entry = (i64::MAX, i8::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    i64_i16,
    key_type = i64,
    value_type = i16,
    new_entry = (i64::MAX, i16::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i64_i32,
    key_type = i64,
    value_type = i32,
    new_entry = (i64::MAX, i32::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i64_i64,
    key_type = i64,
    value_type = i64,
    new_entry = (i64::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i64_i128,
    key_type = i64,
    value_type = i128,
    new_entry = (i64::MAX, i128::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i64_isize,
    key_type = i64,
    value_type = isize,
    new_entry = (i64::MAX, isize::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    i128_i8,
    key_type = i128,
    value_type = i8,
    new_entry = (i128::MAX, i8::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    i128_i16,
    key_type = i128,
    value_type = i16,
    new_entry = (i128::MAX, i16::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i128_i32,
    key_type = i128,
    value_type = i32,
    new_entry = (i128::MAX, i32::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i128_i64,
    key_type = i128,
    value_type = i64,
    new_entry = (i128::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i128_i128,
    key_type = i128,
    value_type = i128,
    new_entry = (i128::MAX, i128::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    i128_isize,
    key_type = i128,
    value_type = isize,
    new_entry = (i128::MAX, isize::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);

generate_tests!(
    isize_i8,
    key_type = isize,
    value_type = i8,
    new_entry = (isize::MAX, i8::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=126, 1..=127),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=127)
);
generate_tests!(
    isize_i16,
    key_type = isize,
    value_type = i16,
    new_entry = (isize::MAX, i16::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    isize_i32,
    key_type = isize,
    value_type = i32,
    new_entry = (isize::MAX, i32::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    isize_i64,
    key_type = isize,
    value_type = i64,
    new_entry = (isize::MAX, i64::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    isize_i128,
    key_type = isize,
    value_type = i128,
    new_entry = (isize::MAX, i128::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
generate_tests!(
    isize_isize,
    key_type = isize,
    value_type = isize,
    new_entry = (isize::MAX, isize::MAX),
    range_spec = oimt::RangeEntriesSpec::new(0..=127, 1..=128),
    const_spec = oimt::ConstantKeyEntriesSpec::new(126, 1..=128)
);
