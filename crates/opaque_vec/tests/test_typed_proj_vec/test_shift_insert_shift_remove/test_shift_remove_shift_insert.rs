use crate::common;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_shift_remove_shift_insert<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let expected = common::typed_proj_vec::from_slice_in(values, alloc.clone());
    let mut result = common::typed_proj_vec::from_slice_in(values, alloc.clone());

    assert_eq!(result.as_slice(), expected.as_slice());

    for i in 0..values.len() {
        let removed_value = result.shift_remove(i);
        result.shift_insert(i, removed_value);

        assert_eq!(result.as_slice(), expected.as_slice());
    }
}

fn run_test_typed_proj_vec_shift_remove_shift_insert_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new_only_nonempty(values);
    for slice in iter {
        run_test_typed_proj_vec_shift_remove_shift_insert(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_shift_remove_shift_insert_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_shift_remove_shift_insert_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_shift_remove_shift_insert_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_shift_remove_shift_insert_values(&values, alloc);
            }
        }
    };
}

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
    usize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
