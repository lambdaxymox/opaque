#![feature(allocator_api)]
mod common;

use opaque_blob_vec::OpaqueBlobVec;

use core::fmt;
use core::ptr::NonNull;

fn expected<T>(values: &[T]) -> OpaqueBlobVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut opaque_blob_vec = common::new_opaque_blob_vec::<T>();
    for value in values.iter().take(values.len() - 1) {
        let value_cloned = value.clone();
        let value_ptr = NonNull::from(&value_cloned).cast::<u8>();
        opaque_blob_vec.push(value_ptr);
    }

    if !opaque_blob_vec.is_empty() {
        let value = values[values.len() - 1].clone();
        let value_ptr = NonNull::from(&value).cast::<u8>();
        opaque_blob_vec.replace_insert(0, value_ptr);
    }

    opaque_blob_vec
}

fn run_test_opaque_blob_vec_swap_remove_forget_unchecked_start<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut opaque_blob_vec = common::from_typed_slice(values);

    for i in 0..values.len() {
        let last_index = values.len() - i;
        let expected_opaque_blob_vec = expected(&values[0..last_index]);
        let _ = unsafe {
            let ptr = opaque_blob_vec.swap_remove_forget_unchecked(0).cast::<T>();
            ptr.read()
        };
        let expected = common::as_slice::<T>(&expected_opaque_blob_vec);
        let result = common::as_slice::<T>(&opaque_blob_vec);

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_blob_vec_swap_remove_forget_unchecked_start_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    for len in 0..values.len() {
        let prefix_values = &values[0..len];
        run_test_opaque_blob_vec_swap_remove_forget_unchecked_start(prefix_values);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_swap_remove_forget_unchecked_start_alternating_values() {
                let values = opaque_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_blob_vec_swap_remove_forget_unchecked_start_values(&values);
            }
        }
    };
}

generate_tests!(u8, 128, opaque_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX));
generate_tests!(u16, 128, opaque_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX));
generate_tests!(u32, 128, opaque_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX));
generate_tests!(u64, 128, opaque_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX));
generate_tests!(u128, 128, opaque_testing::AlternatingValuesSpec::new(u128::MIN, u128::MAX));
generate_tests!(usize, 128, opaque_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX));
