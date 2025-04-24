mod common;

use opaque_vec::OpaqueVec;
use opaque_index_map::OpaqueIndexMap;
use core::{fmt, hash};
use crate::common::key_value_generators as kvg;

fn from_entries<K, V>(entries: &[(K, V)]) -> OpaqueIndexMap
where
    K: Clone + Eq + hash::Hash + 'static,
    V: Clone + Eq + 'static,
{
    let mut map = OpaqueIndexMap::new::<K, V>();
    for (key, value) in entries.iter().cloned() {
        map.insert(key, value);
    }

    map
}

fn run_test_opaque_index_map_insert_get_full<K, V>(entries: &[(K, V)])
where
    K: Clone + Eq + hash::Hash + fmt::Debug + 'static,
    V: Clone + Eq + fmt::Debug + 'static,
{
    let map = from_entries(entries);
    for (index, (key, value)) in entries.iter().enumerate() {
        let expected = Some((index, key, value));
        let result = map.get_full::<K, K, V>(key);

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_index_map_insert_get_full_values<K, V>(entries: &[(K, V)])
where
    K: Clone + Eq + hash::Hash + fmt::Debug + 'static,
    V: Clone + Eq + fmt::Debug + 'static,
{
    for len in 0..entries.len() {
        let prefix_entries = &entries[0..len];
        run_test_opaque_index_map_insert_get_full(prefix_entries);
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, key_range = $key_range:expr, value_range = $value_range:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_insert_get_full_empty() {
                let keys: [$key_typ; 0] = [];
                let values: [$value_typ; 0] = [];
                let keys_vec = OpaqueVec::from(&keys);
                let values_vec = OpaqueVec::from(&values);
                let entries = kvg::key_value_pairs(keys_vec.iter::<$key_typ>().cloned(), values_vec.iter::<$value_typ>().cloned());

                run_test_opaque_index_map_insert_get_full_values(entries.as_slice::<($key_typ, $value_typ)>());
            }

            #[test]
            fn test_opaque_index_map_insert_get_full_range_values() {
                let entries = kvg::entries::<$key_typ, $value_typ>($key_range, $value_range);
                run_test_opaque_index_map_insert_get_full_values(entries.as_slice::<($key_typ, $value_typ)>());
            }
        }
    };
}

generate_tests!(u16_i8,    key_type = u16, value_type = i8,    key_range = (0..=126),  value_range = (1..=127));
generate_tests!(u16_i16,   key_type = u16, value_type = i16,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u16_i32,   key_type = u16, value_type = i32,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u16_i64,   key_type = u16, value_type = i64,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u16_i128,  key_type = u16, value_type = i128,  key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u16_isize, key_type = u16, value_type = isize, key_range = (0..=1023), value_range = (1..=1024));

generate_tests!(u32_i8,    key_type = u32, value_type = i8,    key_range = (0..=126),  value_range = (1..=127));
generate_tests!(u32_i16,   key_type = u32, value_type = i16,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u32_i32,   key_type = u32, value_type = i32,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u32_i64,   key_type = u32, value_type = i64,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u32_i128,  key_type = u32, value_type = i128,  key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u32_isize, key_type = u32, value_type = isize, key_range = (0..=1023), value_range = (1..=1024));

generate_tests!(u64_i8,    key_type = u64, value_type = i8,    key_range = (0..=126),  value_range = (1..=127));
generate_tests!(u64_i16,   key_type = u64, value_type = i16,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u64_i32,   key_type = u64, value_type = i32,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u64_i64,   key_type = u64, value_type = i64,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u64_i128,  key_type = u64, value_type = i128,  key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u64_isize, key_type = u64, value_type = isize, key_range = (0..=1023), value_range = (1..=1024));

generate_tests!(u128_i8,    key_type = u128, value_type = i8,    key_range = (0..=126),  value_range = (1..=127));
generate_tests!(u128_i16,   key_type = u128, value_type = i16,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u128_i32,   key_type = u128, value_type = i32,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u128_i64,   key_type = u128, value_type = i64,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u128_i128,  key_type = u128, value_type = i128,  key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(u128_isize, key_type = u128, value_type = isize, key_range = (0..=1023), value_range = (1..=1024));

generate_tests!(usize_i8,    key_type = usize, value_type = i8,    key_range = (0..=126),  value_range = (1..=127));
generate_tests!(usize_i16,   key_type = usize, value_type = i16,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(usize_i32,   key_type = usize, value_type = i32,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(usize_i64,   key_type = usize, value_type = i64,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(usize_i128,  key_type = usize, value_type = i128,  key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(usize_isize, key_type = usize, value_type = isize, key_range = (0..=1023), value_range = (1..=1024));

generate_tests!(i16_i8,    key_type = i16, value_type = i8,    key_range = (0..=126),  value_range = (1..=127));
generate_tests!(i16_i16,   key_type = i16, value_type = i16,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i16_i32,   key_type = i16, value_type = i32,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i16_i64,   key_type = i16, value_type = i64,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i16_i128,  key_type = i16, value_type = i128,  key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i16_isize, key_type = i16, value_type = isize, key_range = (0..=1023), value_range = (1..=1024));

generate_tests!(i32_i8,    key_type = i32, value_type = i8,    key_range = (0..=126),  value_range = (1..=127));
generate_tests!(i32_i16,   key_type = i32, value_type = i16,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i32_i32,   key_type = i32, value_type = i32,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i32_i64,   key_type = i32, value_type = i64,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i32_i128,  key_type = i32, value_type = i128,  key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i32_isize, key_type = i32, value_type = isize, key_range = (0..=1023), value_range = (1..=1024));

generate_tests!(i64_i8,    key_type = i64, value_type = i8,    key_range = (0..=126),  value_range = (1..=127));
generate_tests!(i64_i16,   key_type = i64, value_type = i16,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i64_i32,   key_type = i64, value_type = i32,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i64_i64,   key_type = i64, value_type = i64,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i64_i128,  key_type = i64, value_type = i128,  key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i64_isize, key_type = i64, value_type = isize, key_range = (0..=1023), value_range = (1..=1024));

generate_tests!(i128_i8,    key_type = i128, value_type = i8,    key_range = (0..=126),  value_range = (1..=127));
generate_tests!(i128_i16,   key_type = i128, value_type = i16,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i128_i32,   key_type = i128, value_type = i32,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i128_i64,   key_type = i128, value_type = i64,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i128_i128,  key_type = i128, value_type = i128,  key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(i128_isize, key_type = i128, value_type = isize, key_range = (0..=1023), value_range = (1..=1024));

generate_tests!(isize_i8,    key_type = isize, value_type = i8,    key_range = (0..=126),  value_range = (1..=127));
generate_tests!(isize_i16,   key_type = isize, value_type = i16,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(isize_i32,   key_type = isize, value_type = i32,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(isize_i64,   key_type = isize, value_type = i64,   key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(isize_i128,  key_type = isize, value_type = i128,  key_range = (0..=1023), value_range = (1..=1024));
generate_tests!(isize_isize, key_type = isize, value_type = isize, key_range = (0..=1023), value_range = (1..=1024));
