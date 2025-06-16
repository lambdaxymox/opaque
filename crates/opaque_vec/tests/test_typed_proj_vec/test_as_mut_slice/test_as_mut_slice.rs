use crate::common;

use core::any;
use core::fmt;
use core::iter;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_as_mut_slice<T, A>(values: &mut [T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = common::typed_proj_vec::from_slice_in(values, alloc);

    let expected = values;
    let result = vec.as_mut_slice();

    assert_eq!(result, expected);
}

fn run_test_typed_proj_vec_as_mut_slice_values<T, A>(values: &mut [T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator = ovt::PrefixGenerator::new(values);
    for slice in iterator {
        let mut cloned_slice = Vec::from(slice);
        run_test_typed_proj_vec_as_mut_slice::<T, A>(&mut cloned_slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_vec_as_mut_slice_empty() {
                let mut values: [$typ; 0] = [];
                let alloc = alloc::Global;
                run_test_typed_proj_vec_as_mut_slice(&mut values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_as_mut_slice_range_values() {
                let mut values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_as_mut_slice_values(&mut values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_as_mut_slice_alternating_values() {
                let mut values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_as_mut_slice_values(&mut values, alloc);
            }
        }
    };
}

generate_tests!(
    unit,
    (),
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(iter::repeat(()))),
    opaque_vec_testing::AlternatingValuesSpec::new((), ())
);
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
generate_tests!(
    string,
    String,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ovt::StringRangeFrom::new(0))),
    opaque_vec_testing::AlternatingValuesSpec::new(String::from("foo"), String::from("bar"))
);
