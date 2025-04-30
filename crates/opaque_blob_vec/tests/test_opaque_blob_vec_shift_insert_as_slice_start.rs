#![feature(allocator_api)]
mod common;

use core::fmt;
use core::ptr::NonNull;
use opaque_blob_vec::OpaqueBlobVec;

use opaque_vec_testing as ovt;

fn expected<T>(values: &[T]) -> OpaqueBlobVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut opaque_blob_vec = common::new_opaque_blob_vec::<T>();
    for value in values.iter().rev() {
        let value_ptr = NonNull::from(value).cast::<u8>();
        opaque_blob_vec.push(value_ptr);
    }

    opaque_blob_vec
}

fn result<T>(values: &[T]) -> OpaqueBlobVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut opaque_blob_vec = common::new_opaque_blob_vec::<T>();
    for value in values.iter() {
        let value_ptr = NonNull::from(value).cast::<u8>();
        opaque_blob_vec.shift_insert(0, value_ptr);
    }

    opaque_blob_vec
}

fn run_test_opaque_blob_vec_shift_insert_slice_start<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let expected_vec = expected(values);
    let result_vec = result(values);

    let expected = common::as_slice::<T>(&expected_vec);
    let result = common::as_slice::<T>(&result_vec);

    assert_eq!(result, expected);
}

fn run_test_opaque_blob_vec_shift_insert_slice_start_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_opaque_blob_vec_shift_insert_slice_start(slice);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_shift_insert_slice_start_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_blob_vec_shift_insert_slice_start_values(&values);
            }
        }
    };
}

generate_tests!(u8, 128, opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX));
generate_tests!(u16, 128, opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX));
generate_tests!(u32, 128, opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX));
generate_tests!(u64, 128, opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX));
generate_tests!(u128, 128, opaque_vec_testing::AlternatingValuesSpec::new(u128::MIN, u128::MAX));
generate_tests!(usize, 128, opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX));
