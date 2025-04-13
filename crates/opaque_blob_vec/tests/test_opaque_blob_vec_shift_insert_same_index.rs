#![feature(allocator_api)]
mod common;

use crate::common::opaque_blob_vec_utils as utils;

use core::fmt;
use core::ptr::NonNull;

fn run_test_opaque_blob_vec_shift_insert_get_same_index1<T>(value: T)
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut opaque_blob_vec = utils::new_opaque_blob_vec::<T>();
    let value_ptr = NonNull::from(&value).cast::<u8>();
    opaque_blob_vec.shift_insert(0, value_ptr);

    let expected = value.clone();
    let result = unsafe {
        let ptr = opaque_blob_vec.get_unchecked(0).cast::<T>();
        ptr.read()
    };

    assert_eq!(result, expected);
}

fn run_test_opaque_blob_vec_shift_insert_get_same_index2<T>(initial_value: T, value: T)
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut opaque_blob_vec = utils::new_opaque_blob_vec::<T>();
    let initial_value_ptr = NonNull::from(&initial_value).cast::<u8>();
    opaque_blob_vec.shift_insert(0, initial_value_ptr);

    let expected_initial = initial_value.clone();
    let result_initial = unsafe {
        let ptr = opaque_blob_vec.get_unchecked(0).cast::<T>();
        ptr.read()
    };

    assert_eq!(result_initial, expected_initial);

    for _ in 0..65536 {
        let value_ptr = NonNull::from(&value).cast::<u8>();
        opaque_blob_vec.shift_insert(0, value_ptr);

        let expected = value.clone();
        let result = unsafe {
            let ptr = opaque_blob_vec.get_unchecked(0).cast::<T>();
            ptr.read()
        };

        assert_eq!(result, expected);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $initial_value:expr, $value:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_shift_insert_get_same_index1() {
                run_test_opaque_blob_vec_shift_insert_get_same_index1($value);
            }

            #[test]
            fn test_opaque_blob_vec_shift_insert_get_same_index2() {
                run_test_opaque_blob_vec_shift_insert_get_same_index2($initial_value, $value);
            }
        }
    };
}

generate_tests!(u8,    u8::MIN,    u8::MAX);
generate_tests!(u16,   u16::MIN,   u16::MAX);
generate_tests!(u32,   u32::MIN,   u32::MAX);
generate_tests!(u64,   u64::MIN,   u64::MAX);
generate_tests!(u128,  u128::MIN,  u128::MAX);
generate_tests!(usize, usize::MIN, usize::MAX);
