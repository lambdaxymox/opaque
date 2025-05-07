#![feature(allocator_api)]
mod common;

use core::any;
use core::fmt;
use core::ptr::NonNull;
use std::alloc;

fn run_test_opaque_blob_vec_replace_insert_get_same_index1<T, A>(value: T, alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut opaque_blob_vec = common::new_opaque_blob_vec_in::<T, A>(alloc);
    let value_ptr = NonNull::from(&value).cast::<u8>();
    opaque_blob_vec.replace_insert(0, value_ptr);

    let expected = value.clone();
    let result = unsafe {
        let ptr = opaque_blob_vec.get_unchecked(0).cast::<T>();
        ptr.read()
    };

    assert_eq!(result, expected);
}

fn run_test_opaque_blob_vec_replace_insert_get_same_index2<T, A>(initial_value: T, value: T, alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut opaque_blob_vec = common::new_opaque_blob_vec_in::<T, A>(alloc);
    let initial_value_ptr = NonNull::from(&initial_value).cast::<u8>();
    opaque_blob_vec.replace_insert(0, initial_value_ptr);

    let expected_initial = initial_value.clone();
    let result_initial = unsafe {
        let ptr = opaque_blob_vec.get_unchecked(0).cast::<T>();
        ptr.read()
    };

    assert_eq!(result_initial, expected_initial);

    for _ in 0..65536 {
        let value_ptr = NonNull::from(&value).cast::<u8>();
        opaque_blob_vec.replace_insert(0, value_ptr);

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
            fn test_opaque_blob_vec_replace_insert_get_same_index1() {
                let value: $typ = $value;
                let alloc = alloc::Global;
                run_test_opaque_blob_vec_replace_insert_get_same_index1(value, alloc);
            }

            #[test]
            fn test_opaque_blob_vec_replace_insert_get_same_index2() {
                let initial_value = $initial_value;
                let value: $typ = $value;
                let alloc = alloc::Global;
                run_test_opaque_blob_vec_replace_insert_get_same_index2(initial_value, value, alloc);
            }
        }
    };
}

generate_tests!(u8, u8::MIN, u8::MAX);
generate_tests!(u16, u16::MIN, u16::MAX);
generate_tests!(u32, u32::MIN, u32::MAX);
generate_tests!(u64, u64::MIN, u64::MAX);
generate_tests!(u128, u128::MIN, u128::MAX);
generate_tests!(usize, usize::MIN, usize::MAX);
