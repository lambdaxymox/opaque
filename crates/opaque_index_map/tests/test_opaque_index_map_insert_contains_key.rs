mod common;

use opaque_vec::OpaqueVec;
use opaque_index_map::OpaqueIndexMap;
use core::{fmt, hash};
use crate::common::key_value_generators as kvg;
use crate::common::key_value_generators::PrefixGenerator;

fn run_test_opaque_index_map_insert_contains_key<K, V>(entries: &[(K, V)])
where
    K: Clone + Eq + hash::Hash + 'static,
    V: Clone + Eq + 'static,
{
    let mut map = OpaqueIndexMap::new::<K, V>();

    for key in entries.iter().map(|tuple| &tuple.0) {
        assert!(!map.contains_key::<K, K, V>(key));
    }

    for (key, value) in entries.iter().cloned() {
        map.insert::<K, V>(key, value);
    }

    for key in entries.iter().map(|tuple| &tuple.0) {
        assert!(map.contains_key::<K, K, V>(key));
    }
}

fn run_test_opaque_index_map_insert_contains_key_values<K, V>(generator: PrefixGenerator<K, V>)
where
    K: Clone + Eq + hash::Hash + fmt::Debug + 'static,
    V: Clone + Eq + fmt::Debug + 'static,
{
    for entries in generator {
        run_test_opaque_index_map_insert_contains_key(entries.as_slice());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty, range_gen = $range_gen:expr, const_gen = $const_gen:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_contains_key_empty() {
                let keys: [$key_typ; 0] = [];
                let values: [$value_typ; 0] = [];
                let keys_vec = OpaqueVec::from(&keys);
                let values_vec = OpaqueVec::from(&values);
                let generator = kvg::key_value_pairs(keys_vec.iter::<$key_typ>().cloned(), values_vec.iter::<$value_typ>().cloned());
                run_test_opaque_index_map_insert_contains_key_values(generator);
            }

            #[test]
            fn test_opaque_index_map_contains_key_range_values() {
                let generator = $range_gen;
                run_test_opaque_index_map_insert_contains_key_values(generator);
            }

            #[test]
            fn test_opaque_index_map_contains_key_constant_values() {
                let generator = $const_gen;
                run_test_opaque_index_map_insert_contains_key_values(generator);
            }
        }
    };
}

generate_tests!(u16_i8,  key_type = u16, value_type = i8, range_gen = kvg::range_entries::<u16, i8>(0..=126, 1..=127), const_gen = kvg::constant_key_entries::<u16, i8>(126, 1..=127));
generate_tests!(u16_i16, key_type = u16, value_type = i16, range_gen = kvg::range_entries::<u16, i16>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u16, i16>(1023, 1..=1024));
generate_tests!(u16_i32, key_type = u16, value_type = i32, range_gen = kvg::range_entries::<u16, i32>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u16, i32>(1023, 1..=1024));
generate_tests!(u16_i64,  key_type = u16, value_type = i64, range_gen = kvg::range_entries::<u16, i64>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u16, i64>(1023, 1..=1024));
generate_tests!(u16_i128, key_type = u16, value_type = i128, range_gen = kvg::range_entries::<u16, i128>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u16, i128>(1023, 1..=1024));
generate_tests!(u16_isize, key_type = u16, value_type = isize, range_gen = kvg::range_entries::<u16, isize>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u16, isize>(1023, 1..=1024));

generate_tests!(u32_i8, key_type = u32, value_type = i8, range_gen = kvg::range_entries::<u32, i8>(0..=126, 1..=127), const_gen = kvg::constant_key_entries::<u32, i8>(126, 1..=127));
generate_tests!(u32_i16, key_type = u32, value_type = i16, range_gen = kvg::range_entries::<u32, i16>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u32, i16>(1023, 1..=1024));
generate_tests!(u32_i32, key_type = u32, value_type = i32, range_gen = kvg::range_entries::<u32, i32>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u32, i32>(1023, 1..=1024));
generate_tests!(u32_i64, key_type = u32, value_type = i64, range_gen = kvg::range_entries::<u32, i64>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u32, i64>(1023, 1..=1024));
generate_tests!(u32_i128, key_type = u32, value_type = i128, range_gen = kvg::range_entries::<u32, i128>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u32, i128>(1023, 1..=1024));
generate_tests!(u32_isize, key_type = u32, value_type = isize, range_gen = kvg::range_entries::<u32, isize>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u32, isize>(1023, 1..=1024));

generate_tests!(u64_i8, key_type = u64, value_type = i8, range_gen = kvg::range_entries::<u64, i8>(0..=126, 1..=127), const_gen = kvg::constant_key_entries::<u64, i8>(126, 1..=127));
generate_tests!(u64_i16, key_type = u64, value_type = i16, range_gen = kvg::range_entries::<u64, i16>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u64, i16>(1023, 1..=1024));
generate_tests!(u64_i32, key_type = u64, value_type = i32, range_gen = kvg::range_entries::<u64, i32>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u64, i32>(1023, 1..=1024));
generate_tests!(u64_i64, key_type = u64, value_type = i64, range_gen = kvg::range_entries::<u64, i64>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u64, i64>(1023, 1..=1024));
generate_tests!(u64_i128, key_type = u64, value_type = i128, range_gen = kvg::range_entries::<u64, i128>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u64, i128>(1023, 1..=1024));
generate_tests!(u64_isize, key_type = u64, value_type = isize, range_gen = kvg::range_entries::<u64, isize>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u64, isize>(1023, 1..=1024));

generate_tests!(u128_i8, key_type = u128, value_type = i8, range_gen = kvg::range_entries::<u128, i8>(0..=126, 1..=127), const_gen = kvg::constant_key_entries::<u128, i8>(126, 1..=127));
generate_tests!(u128_i16, key_type = u128, value_type = i16, range_gen = kvg::range_entries::<u128, i16>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u128, i16>(1023, 1..=1024));
generate_tests!(u128_i32, key_type = u128, value_type = i32, range_gen = kvg::range_entries::<u128, i32>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u128, i32>(1023, 1..=1024));
generate_tests!(u128_i64, key_type = u128, value_type = i64, range_gen = kvg::range_entries::<u128, i64>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u128, i64>(1023, 1..=1024));
generate_tests!(u128_i128, key_type = u128, value_type = i128, range_gen = kvg::range_entries::<u128, i128>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u128, i128>(1023, 1..=1024));
generate_tests!(u128_isize, key_type = u128, value_type = isize, range_gen = kvg::range_entries::<u128, isize>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<u128, isize>(1023, 1..=1024));

generate_tests!(usize_i8, key_type = usize, value_type = i8, range_gen = kvg::range_entries::<usize, i8>(0..=126, 1..=127), const_gen = kvg::constant_key_entries::<usize, i8>(126, 1..=127));
generate_tests!(usize_i16, key_type = usize, value_type = i16, range_gen = kvg::range_entries::<usize, i16>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<usize, i16>(1023, 1..=1024));
generate_tests!(usize_i32, key_type = usize, value_type = i32, range_gen = kvg::range_entries::<usize, i32>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<usize, i32>(1023, 1..=1024));
generate_tests!(usize_i64, key_type = usize, value_type = i64, range_gen = kvg::range_entries::<usize, i64>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<usize, i64>(1023, 1..=1024));
generate_tests!(usize_i128, key_type = usize, value_type = i128, range_gen = kvg::range_entries::<usize, i128>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<usize, i128>(1023, 1..=1024));
generate_tests!(usize_isize, key_type = usize, value_type = isize, range_gen = kvg::range_entries::<usize, isize>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<usize, isize>(1023, 1..=1024));

generate_tests!(i16_i8, key_type = i16, value_type = i8, range_gen = kvg::range_entries::<i16, i8>(0..=126, 1..=127), const_gen = kvg::constant_key_entries::<i16, i8>(126, 1..=127));
generate_tests!(i16_i16, key_type = i16, value_type = i16, range_gen = kvg::range_entries::<i16, i16>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i16, i16>(1023, 1..=1024));
generate_tests!(i16_i32, key_type = i16, value_type = i32, range_gen = kvg::range_entries::<i16, i32>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i16, i32>(1023, 1..=1024));
generate_tests!(i16_i64, key_type = i16, value_type = i64, range_gen = kvg::range_entries::<i16, i64>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i16, i64>(1023, 1..=1024));
generate_tests!(i16_i128, key_type = i16, value_type = i128, range_gen = kvg::range_entries::<i16, i128>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i16, i128>(1023, 1..=1024));
generate_tests!(i16_isize, key_type = i16, value_type = isize, range_gen = kvg::range_entries::<i16, isize>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i16, isize>(1023, 1..=1024));

generate_tests!(i32_i8, key_type = i32, value_type = i8, range_gen = kvg::range_entries::<i32, i8>(0..=126, 1..=127), const_gen = kvg::constant_key_entries::<i32, i8>(126, 1..=127));
generate_tests!(i32_i16, key_type = i32, value_type = i16, range_gen = kvg::range_entries::<i32, i16>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i32, i16>(1023, 1..=1024));
generate_tests!(i32_i32, key_type = i32, value_type = i32, range_gen = kvg::range_entries::<i32, i32>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i32, i32>(1023, 1..=1024));
generate_tests!(i32_i64, key_type = i32, value_type = i64, range_gen = kvg::range_entries::<i32, i64>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i32, i64>(1023, 1..=1024));
generate_tests!(i32_i128, key_type = i32, value_type = i128, range_gen = kvg::range_entries::<i32, i128>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i32, i128>(1023, 1..=1024));
generate_tests!(i32_isize, key_type = i32, value_type = isize, range_gen = kvg::range_entries::<i32, isize>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i32, isize>(1023, 1..=1024));

generate_tests!(i64_i8, key_type = i64, value_type = i8, range_gen = kvg::range_entries::<i64, i8>(0..=126, 1..=127), const_gen = kvg::constant_key_entries::<i64, i8>(126, 1..=127));
generate_tests!(i64_i16, key_type = i64, value_type = i16, range_gen = kvg::range_entries::<i64, i16>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i64, i16>(1023, 1..=1024));
generate_tests!(i64_i32, key_type = i64, value_type = i32, range_gen = kvg::range_entries::<i64, i32>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i64, i32>(1023, 1..=1024));
generate_tests!(i64_i64, key_type = i64, value_type = i64, range_gen = kvg::range_entries::<i64, i64>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i64, i64>(1023, 1..=1024));
generate_tests!(i64_i128, key_type = i64, value_type = i128, range_gen = kvg::range_entries::<i64, i128>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i64, i128>(1023, 1..=1024));
generate_tests!(i64_isize, key_type = i64, value_type = isize, range_gen = kvg::range_entries::<i64, isize>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i64, isize>(1023, 1..=1024));

generate_tests!(i128_i8, key_type = i128, value_type = i8, range_gen = kvg::range_entries::<i128, i8>(0..=126, 1..=127), const_gen = kvg::constant_key_entries::<i128, i8>(126, 1..=127));
generate_tests!(i128_i16, key_type = i128, value_type = i16, range_gen = kvg::range_entries::<i128, i16>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i128, i16>(1023, 1..=1024));
generate_tests!(i128_i32, key_type = i128, value_type = i32, range_gen = kvg::range_entries::<i128, i32>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i128, i32>(1023, 1..=1024));
generate_tests!(i128_i64, key_type = i128, value_type = i64, range_gen = kvg::range_entries::<i128, i64>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i128, i64>(1023, 1..=1024));
generate_tests!(i128_i128, key_type = i128, value_type = i128, range_gen = kvg::range_entries::<i128, i128>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i128, i128>(1023, 1..=1024));
generate_tests!(i128_isize, key_type = i128, value_type = isize, range_gen = kvg::range_entries::<i128, isize>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<i128, isize>(1023, 1..=1024));

generate_tests!(isize_i8, key_type = isize, value_type = i8, range_gen = kvg::range_entries::<isize, i8>(0..=126, 1..=127), const_gen = kvg::constant_key_entries::<isize, i8>(126, 1..=127));
generate_tests!(isize_i16, key_type = isize, value_type = i16, range_gen = kvg::range_entries::<isize, i16>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<isize, i16>(1023, 1..=1024));
generate_tests!(isize_i32, key_type = isize, value_type = i32, range_gen = kvg::range_entries::<isize, i32>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<isize, i32>(1023, 1..=1024));
generate_tests!(isize_i64, key_type = isize, value_type = i64, range_gen = kvg::range_entries::<isize, i64>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<isize, i64>(1023, 1..=1024));
generate_tests!(isize_i128, key_type = isize, value_type = i128, range_gen = kvg::range_entries::<isize, i128>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<isize, i128>(1023, 1..=1024));
generate_tests!(isize_isize, key_type = isize, value_type = isize, range_gen = kvg::range_entries::<isize, isize>(0..=1023, 1..=1024), const_gen = kvg::constant_key_entries::<isize, isize>(1023, 1..=1024));
