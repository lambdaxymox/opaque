use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_from_slice<T>(expected: &[T])
where
    T: any::Any + PartialEq + Clone + fmt::Debug
{
    let vec = TypedProjVec::from(expected);
    let result = vec.as_slice();

    assert_eq!(result, expected);
}

fn run_test_typed_proj_vec_from_slice_values<T>(values: &[T])
where
    T: any::Any + PartialEq + Clone + fmt::Debug
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_from_slice(slice);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_from_slice_empty() {
                let values: [$typ; 0] = [];
                let vec = TypedProjVec::from(values);

                let expected = values.as_slice();
                let result = vec.as_slice();

                assert_eq!(result, expected);
            }

            #[test]
            fn test_typed_proj_vec_from_slice_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                run_test_typed_proj_vec_from_slice_values(&values);
            }

            #[test]
            fn test_typed_proj_vec_from_slice_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_typed_proj_vec_from_slice_values(&values);
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
