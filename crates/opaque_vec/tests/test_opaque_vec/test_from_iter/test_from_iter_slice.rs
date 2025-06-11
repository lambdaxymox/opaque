use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_opaque_vec_from_iter_slice<T>(expected: &[T])
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
{
    let vec = OpaqueVec::from_iter(expected.iter().cloned());
    let result = vec.as_slice::<T, alloc::Global>();

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_from_iter_slice_values<T>(values: &[T])
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_opaque_vec_from_iter_slice(slice);
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_vec_from_iter_slice_empty() {
                let values: [$typ; 0] = [];
                let vec = OpaqueVec::from_iter(values);

                let expected = values.as_slice();
                let result = vec.as_slice::<$typ, alloc::Global>();

                assert_eq!(result, expected);
            }

            #[test]
            fn test_opaque_vec_from_iter_slice_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_from_iter_slice_values(&values);
            }

            #[test]
            fn test_opaque_vec_from_iter_slice_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_vec_from_iter_slice_values(&values);
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
