use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_into_iter_back_to_vec<T, A>(expected: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::new_in(alloc.clone());
    for value in expected.iter().cloned() {
        vec.push(value);
    }

    let mut cloned_vec = TypedProjVec::new_in(alloc.clone());
    for value in vec.into_iter() {
        cloned_vec.push(value);
    }

    let result = cloned_vec.as_slice();

    assert_eq!(result, expected);
}

fn run_test_typed_proj_vec_into_iter_back_to_vec_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_into_iter_back_to_vec(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_vec_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_into_iter_back_to_vec_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_vec_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_into_iter_back_to_vec_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_into_iter_back_to_vec_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_vec_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_into_iter_back_to_vec_values(&values, alloc);
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
