use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_opaque_vec_into_iter_take<T, A>(values: &[T], count: usize, alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc.clone());
    vec.extend::<_, T, A>(values.iter().cloned());

    let mut expected = OpaqueVec::new_in::<T, A>(alloc.clone());
    for value in values.iter().cloned().take(count) {
        expected.push::<T, A>(value);
    }

    let mut result = OpaqueVec::new_in::<T, A>(alloc.clone());
    result.extend::<_, T, A>(vec.into_iter::<T, A>().take(count));

    assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());
}

fn run_test_opaque_vec_into_iter_take_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    for count in 0..values.len() {
        run_test_opaque_vec_into_iter_take(values, count, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_vec_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_vec_into_iter_take_empty() {
                let values: [$typ; 0] = [];
                let alloc = alloc::Global;
                run_test_opaque_vec_into_iter_take_values(&values, alloc);
            }

            #[test]
            fn test_opaque_vec_into_iter_take_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_vec_size>($range_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_into_iter_take_values(&values, alloc);
            }

            #[test]
            fn test_opaque_vec_into_iter_take_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_vec_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_into_iter_take_values(&values, alloc);
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
generate_tests!(
    string,
    String,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ovt::StringRangeFrom::new(0))),
    opaque_vec_testing::AlternatingValuesSpec::new(String::from("foo"), String::from("bar"))
);
