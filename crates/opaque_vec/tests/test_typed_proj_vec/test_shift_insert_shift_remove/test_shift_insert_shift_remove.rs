use crate::common;

use core::any;
use core::fmt;
use core::iter;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_shift_insert_shift_remove<T, A>(values: &[T], alloc: A, new_value: T)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let expected = common::typed_proj_vec::from_slice_in(values, alloc.clone());
    let mut result = common::typed_proj_vec::from_slice_in(values, alloc.clone());

    assert_eq!(result.as_slice(), expected.as_slice());

    for i in 0..values.len() {
        result.shift_insert(i, new_value.clone());
        result.shift_remove(i);

        assert_eq!(result.as_slice(), expected.as_slice());
    }
}

fn run_test_typed_proj_vec_shift_insert_shift_remove_values<T, A>(values: &[T], alloc: A, new_value: T)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator = ovt::PrefixGenerator::new_only_nonempty(values);
    for slice in iterator {
        run_test_typed_proj_vec_shift_insert_shift_remove(slice, alloc.clone(), new_value.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_array_size:expr, $new_value:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_vec_shift_insert_shift_remove_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                let new_value: $typ = $new_value;
                run_test_typed_proj_vec_shift_insert_shift_remove_values(&values, alloc, new_value);
            }

            #[test]
            fn test_typed_proj_vec_shift_insert_shift_remove_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                let new_value: $typ = $new_value;
                run_test_typed_proj_vec_shift_insert_shift_remove_values(&values, alloc, new_value);
            }
        }
    };
}

generate_tests!(
    unit,
    (),
    128,
    (),
    opaque_vec_testing::RangeValuesSpec::new(Box::new(iter::repeat(()))),
    opaque_vec_testing::AlternatingValuesSpec::new((), ())
);
generate_tests!(
    u8,
    u8,
    128,
    u8::MAX,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX)
);
generate_tests!(
    u16,
    u16,
    128,
    u16::MAX,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX)
);
generate_tests!(
    u32,
    u32,
    128,
    u32::MAX,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX)
);
generate_tests!(
    u64,
    u64,
    128,
    u64::MAX,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX)
);
generate_tests!(
    usize,
    usize,
    128,
    usize::MAX,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
generate_tests!(
    string,
    String,
    128,
    usize::MAX.to_string(),
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ovt::StringRangeFrom::new(0))),
    opaque_vec_testing::AlternatingValuesSpec::new(String::from("foo"), String::from("bar"))
);
