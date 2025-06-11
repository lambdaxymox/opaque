use crate::common;

use core::any;
use core::fmt;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_swap_remove_len<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let values_vec = common::typed_proj_vec::from_slice_in(values, alloc);

    for i in 0..values.len() {
        let result_vec = {
            let mut vec = values_vec.clone();
            vec.swap_remove(i);
            vec
        };

        let expected = values.len() - 1;
        let result = result_vec.len();

        assert_eq!(result, expected);
    }
}

fn run_test_typed_proj_vec_swap_remove_len_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_swap_remove_len(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_vec_swap_remove_len_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_swap_remove_len_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_swap_remove_len_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_swap_remove_len_values(&values, alloc);
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
