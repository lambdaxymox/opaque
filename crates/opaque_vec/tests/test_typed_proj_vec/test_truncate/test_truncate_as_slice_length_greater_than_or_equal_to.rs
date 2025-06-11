use crate::common;

use core::any;
use core::fmt;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_truncate_as_slice_length_greater_than_or_equal_to<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let base_proj_vec = common::typed_proj_vec::from_slice_in(values, alloc);
    let min_len = values.len();
    let max_len = 10 * values.len();
    for len in min_len..max_len {
        let mut proj_vec = base_proj_vec.clone();

        proj_vec.truncate(len);

        let expected = &values[..];
        let result = proj_vec.as_slice();

        assert_eq!(result, expected);
    }
}

fn run_test_typed_proj_vec_truncate_as_slice_length_greater_than_or_equal_to_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_truncate_as_slice_length_greater_than_or_equal_to(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_vec_truncate_as_slice_length_greater_than_or_equal_to_empty() {
                let values: [$typ; 0] = [];
                let alloc = alloc::Global;
                run_test_typed_proj_vec_truncate_as_slice_length_greater_than_or_equal_to_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_truncate_as_slice_length_greater_than_or_equal_to_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_truncate_as_slice_length_greater_than_or_equal_to_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_truncate_as_slice_length_greater_than_or_equal_to_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_truncate_as_slice_length_greater_than_or_equal_to_values(&values, alloc);
            }
        }
    };
}

generate_tests!(
    u8,
    u8,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX)
);
generate_tests!(
    u16,
    u16,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX)
);
generate_tests!(
    u32,
    u32,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX)
);
generate_tests!(
    u64,
    u64,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX)
);
generate_tests!(
    usize,
    usize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
