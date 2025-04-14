#![feature(allocator_api)]
mod common;

use common::opaque_blob_vec_utils as utils;

use core::fmt;

use common::array_generators as ag;

fn run_test_opaque_blob_vec_clear_len<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut opaque_blob_vec = utils::from_typed_slice(values);

    assert_eq!(opaque_blob_vec.len(), values.len());

    opaque_blob_vec.clear();

    let result = opaque_blob_vec.len();
    let expected = 0;

    assert_eq!(result, expected);
}

fn run_test_opaque_blob_vec_clear_len_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + TryFrom<usize> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    for len in 0..values.len() {
        let prefix_values = &values[0..len];
        run_test_opaque_blob_vec_clear_len(prefix_values);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_clear_len_empty() {
                let values: [$typ; 0] = [];

                run_test_opaque_blob_vec_clear_len(&values);
            }

            #[test]
            fn test_opaque_blob_vec_clear_len_alternating_values() {
                let values = ag::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_blob_vec_clear_len_values(&values);
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
