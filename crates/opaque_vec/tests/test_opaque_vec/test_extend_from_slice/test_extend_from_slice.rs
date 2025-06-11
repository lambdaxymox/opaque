use crate::common;

use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn expected<T, A>(values: &[T], extension_values: &[T], alloc: A) -> OpaqueVec
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = common::opaque_vec::from_slice_in(values, alloc);
    for extension_value in extension_values.iter() {
        vec.push::<T, A>(extension_value.clone());
    }

    vec
}

fn result<T, A>(values: &[T], extension_values: &[T], alloc: A) -> OpaqueVec
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = common::opaque_vec::from_slice_in(values, alloc);
    vec.extend_from_slice::<T, A>(extension_values);

    vec
}

fn run_test_opaque_vec_extend_from_slice<T, A>(values: &[T], extension_values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let expected = expected(values, extension_values, alloc.clone());
    let result = result(values, extension_values, alloc.clone());

    assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());
}

fn run_test_opaque_vec_extend_from_slice_values<T, A>(values: &[T], extension_values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        let extension_slice = &extension_values[0..slice.len()];
        run_test_opaque_vec_extend_from_slice(slice, extension_slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_array_size:expr, $range_spec:expr, $alt_spec:expr, $const_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_vec_clone_len_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let extension_values = opaque_vec_testing::constant_values::<$typ, $max_array_size>($const_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_extend_from_slice_values(&values, &extension_values, alloc);
            }

            #[test]
            fn test_opaque_vec_clone_len_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let extension_values = opaque_vec_testing::constant_values::<$typ, $max_array_size>($const_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_extend_from_slice_values(&values, &extension_values, alloc);
            }
        }
    };
}

generate_tests!(
    u8,
    u8,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(u8::MAX)
);
generate_tests!(
    u16,
    u16,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(u16::MAX)
);
generate_tests!(
    u32,
    u32,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(u32::MAX)
);
generate_tests!(
    u64,
    u64,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(u64::MAX)
);
generate_tests!(
    usize,
    usize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX),
    opaque_vec_testing::ConstantValuesSpec::new(usize::MAX)
);
