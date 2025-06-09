use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_into_iter_take_none<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::new_in(alloc.clone());
    vec.extend(values.iter().cloned());

    let mut result = TypedProjVec::new_in(alloc.clone());
    result.extend(vec.into_iter().take(0));

    assert!(result.is_empty());
}

fn run_test_typed_proj_vec_into_iter_take_none_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_into_iter_take_none(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_vec_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_into_iter_take_none_empty() {
                let values: [$typ; 0] = [];
                let alloc = alloc::Global;
                run_test_typed_proj_vec_into_iter_take_none(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_into_iter_take_none_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_vec_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_into_iter_take_none_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_into_iter_take_none_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_vec_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_into_iter_take_none_values(&values, alloc);
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
