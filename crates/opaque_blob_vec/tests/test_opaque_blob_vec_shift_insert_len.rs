#![feature(allocator_api)]
mod common;

use common::array_generators as ag;
use core::fmt;
use std::ptr::NonNull;
use opaque_blob_vec::OpaqueBlobVec;
use crate::common::opaque_blob_vec_utils::new_opaque_blob_vec;

pub fn from_slice<T>(values: &[T]) -> OpaqueBlobVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = new_opaque_blob_vec::<T>();
    for i in 0..values.len() {
        let value: T = values[i].clone();
        let value_ptr: NonNull<u8> = NonNull::from(&value).cast::<u8>();
        vec.shift_insert(i, value_ptr);
    }

    vec
}

fn run_test_opaque_blob_vec_shift_insert_len<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let opaque_blob_vec = from_slice(values);
    let expected = values.len();
    let result = opaque_blob_vec.len();

    assert_eq!(result, expected);
}

fn run_test_opaque_blob_vec_shift_insert_len_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    for len in 0..values.len() {
        let prefix_values = &values[0..len];
        run_test_opaque_blob_vec_shift_insert_len(prefix_values);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_shift_insert_get_unchecked_alternating_values() {
                let values = ag::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_blob_vec_shift_insert_len_values(&values);
            }
        }
    };
}

generate_tests!(u8, 128, ag::AlternatingValuesSpec::new(u8::MIN, u8::MAX));
generate_tests!(u16, 1024, ag::AlternatingValuesSpec::new(u16::MIN, u16::MAX));
generate_tests!(u32, 1024, ag::AlternatingValuesSpec::new(u32::MIN, u32::MAX));
generate_tests!(u64, 1024, ag::AlternatingValuesSpec::new(u64::MIN, u64::MAX));
generate_tests!(u128, 1024, ag::AlternatingValuesSpec::new(u128::MIN, u128::MAX));
generate_tests!(usize, 1024, ag::AlternatingValuesSpec::new(usize::MIN, usize::MAX));
