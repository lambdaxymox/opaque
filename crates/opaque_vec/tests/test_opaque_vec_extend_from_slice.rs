#![feature(allocator_api)]
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn expected<T>(values: &[T], extension_values: &[T]) -> OpaqueVec
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
{
    let mut vec = OpaqueVec::from(values);
    for extension_value in extension_values.iter() {
        vec.push::<T, alloc::Global>(extension_value.clone());
    }

    vec
}

fn result<T>(values: &[T], extension_values: &[T]) -> OpaqueVec
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
{
    let mut vec = OpaqueVec::from(values);
    vec.extend_from_slice::<T, alloc::Global>(extension_values);

    vec
}

fn run_test_opaque_vec_extend_from_slice<T>(values: &[T], extension_values: &[T])
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
{
    let expected = expected(values, extension_values);
    let result = result(values, extension_values);

    assert_eq!(result.as_slice::<T, alloc::Global>(), expected.as_slice::<T, alloc::Global>());
}

fn run_test_opaque_vec_extend_from_slice_values<T>(values: &[T], extension_values: &[T])
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        let extension_slice = &extension_values[0..slice.len()];
        run_test_opaque_vec_extend_from_slice(slice, extension_slice);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr, $const_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_clone_len_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let extension_values = opaque_vec_testing::constant_values::<$typ, $max_array_size>($const_spec);
                run_test_opaque_vec_extend_from_slice_values(&values, &extension_values);
            }

            #[test]
            fn test_opaque_vec_clone_len_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let extension_values = opaque_vec_testing::constant_values::<$typ, $max_array_size>($const_spec);
                run_test_opaque_vec_extend_from_slice_values(&values, &extension_values);
            }
        }
    };
}

generate_tests!(
    i8,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i8::MIN, 0),
    opaque_vec_testing::ConstantValuesSpec::new(i8::MAX)
);
generate_tests!(
    i16,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i16::MIN, 0),
    opaque_vec_testing::ConstantValuesSpec::new(i16::MAX)
);
generate_tests!(
    i32,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i32::MIN, 0),
    opaque_vec_testing::ConstantValuesSpec::new(i32::MAX)
);
generate_tests!(
    i64,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i64::MIN, 0),
    opaque_vec_testing::ConstantValuesSpec::new(i64::MAX)
);
generate_tests!(
    i128,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i128::MIN, 0),
    opaque_vec_testing::ConstantValuesSpec::new(i128::MAX)
);
generate_tests!(
    isize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(isize::MIN, 0),
    opaque_vec_testing::ConstantValuesSpec::new(isize::MAX)
);

generate_tests!(
    u8,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(u8::MAX)
);
generate_tests!(
    u16,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(u16::MAX)
);
generate_tests!(
    u32,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(u32::MAX)
);
generate_tests!(
    u64,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(u64::MAX)
);
generate_tests!(
    u128,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u128::MIN, u128::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(u128::MAX)
);
generate_tests!(
    usize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(usize::MAX)
);
