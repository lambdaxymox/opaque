use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use core::iter;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_opaque_vec_into_iter_back_to_vec<T, A>(expected: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc.clone());
    for value in expected.iter().cloned() {
        vec.push::<T, A>(value);
    }

    let mut cloned_vec = OpaqueVec::new_in::<T, A>(alloc.clone());
    for value in vec.into_iter::<T, A>() {
        cloned_vec.push::<T, A>(value);
    }

    let result = cloned_vec.as_slice::<T, A>();

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_into_iter_back_to_vec_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator = ovt::PrefixGenerator::new(values);
    for slice in iterator {
        run_test_opaque_vec_into_iter_back_to_vec(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_vec_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_vec_into_iter_back_to_vec_empty() {
                let values: [$typ; 0] = [];
                let alloc = alloc::Global;
                run_test_opaque_vec_into_iter_back_to_vec(&values, alloc);
            }

            #[test]
            fn test_opaque_vec_into_iter_back_to_vec_range_values() {
                let values = ovt::range_values::<$typ, $max_vec_size>($range_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_into_iter_back_to_vec_values(&values, alloc);
            }

            #[test]
            fn test_opaque_vec_into_iter_back_to_vec_alternating_values() {
                let values = ovt::alternating_values::<$typ, $max_vec_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_into_iter_back_to_vec_values(&values, alloc);
            }
        }
    };
}

generate_tests!(
    unit,
    (),
    128,
    ovt::RangeValuesSpec::new(Box::new(iter::repeat(()))),
    ovt::AlternatingValuesSpec::new((), ())
);
generate_tests!(
    u8,
    u8,
    128,
    ovt::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    ovt::AlternatingValuesSpec::new(u8::MIN, u8::MAX)
);
generate_tests!(
    u16,
    u16,
    128,
    ovt::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    ovt::AlternatingValuesSpec::new(u16::MIN, u16::MAX)
);
generate_tests!(
    u32,
    u32,
    128,
    ovt::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    ovt::AlternatingValuesSpec::new(u32::MIN, u32::MAX)
);
generate_tests!(
    u64,
    u64,
    128,
    ovt::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    ovt::AlternatingValuesSpec::new(u64::MIN, u64::MAX)
);
generate_tests!(
    usize,
    usize,
    128,
    ovt::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    ovt::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
generate_tests!(
    string,
    String,
    128,
    ovt::RangeValuesSpec::new(Box::new(ovt::StringRangeFrom::new(0))),
    ovt::AlternatingValuesSpec::new(String::from("foo"), String::from("bar"))
);
