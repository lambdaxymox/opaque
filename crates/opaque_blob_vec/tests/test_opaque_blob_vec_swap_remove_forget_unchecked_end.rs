#![feature(allocator_api)]
mod common;

use core::fmt;

use opaque_vec_testing as ovt;

fn run_test_opaque_blob_vec_swap_remove_forget_unchecked_end<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut opaque_blob_vec = common::from_typed_slice(values);

    let last_index = opaque_blob_vec.len() - 1;
    let expected = &values[0..last_index];
    let _ = unsafe {
        let ptr = opaque_blob_vec.swap_remove_forget_unchecked(last_index).cast::<T>();
        ptr.read()
    };
    let result = common::as_slice::<T>(&opaque_blob_vec);

    assert_eq!(result, expected);
}

fn run_test_opaque_blob_vec_swap_remove_forget_unchecked_end_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let iter = ovt::PrefixGenerator::new_only_nonempty(values);
    for slice in iter {
        run_test_opaque_blob_vec_swap_remove_forget_unchecked_end(slice);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_swap_remove_forget_unchecked_end_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_blob_vec_swap_remove_forget_unchecked_end_values(&values);
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
