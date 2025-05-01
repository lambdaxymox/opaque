#![feature(allocator_api)]
mod common;

use core::fmt;

use opaque_vec_testing as ovt;

fn run_test_opaque_blob_vec_truncate_as_slice_length_greater_than_or_equal_to<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + TryFrom<usize> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    let base_opaque_vec = common::from_typed_slice(values);
    let min_len = values.len();
    let max_len = 10 * values.len();
    for len in min_len..max_len {
        let mut opaque_blob_vec = base_opaque_vec.clone();

        opaque_blob_vec.truncate(len);

        let expected = values;
        let result = common::as_slice::<T>(&opaque_blob_vec);

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_blob_vec_truncate_as_slice_length_greater_than_or_equal_to_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + TryFrom<usize> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_opaque_blob_vec_truncate_as_slice_length_greater_than_or_equal_to(slice);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_truncate_as_slice_length_greater_than_or_equal_to_empty() {
                let values: [$typ; 0] = [];

                run_test_opaque_blob_vec_truncate_as_slice_length_greater_than_or_equal_to(&values);
            }

            #[test]
            fn test_opaque_blob_vec_truncate_as_slice_length_greater_than_or_equal_to_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_blob_vec_truncate_as_slice_length_greater_than_or_equal_to_values(&values);
            }

            #[test]
            fn test_opaque_blob_vec_truncate_as_slice_length_greater_than_or_equal_to_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_blob_vec_truncate_as_slice_length_greater_than_or_equal_to_values(&values);
            }
        }
    };
}

generate_tests!(
    i8,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i8::MIN, 0)
);
generate_tests!(
    i16,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i16::MIN, 0)
);
generate_tests!(
    i32,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i32::MIN, 0)
);
generate_tests!(
    i64,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i64::MIN, 0)
);
generate_tests!(
    i128,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i128::MIN, 0)
);
generate_tests!(
    isize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(isize::MIN, 0)
);

generate_tests!(
    u8,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX)
);
generate_tests!(
    u16,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX)
);
generate_tests!(
    u32,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX)
);
generate_tests!(
    u64,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX)
);
generate_tests!(
    u128,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u128::MIN, u128::MAX)
);
generate_tests!(
    usize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
