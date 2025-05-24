use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn expected<T, A>(values: &[T], alloc: A) -> Vec<T, A>
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = Vec::with_capacity_in(values.len(), alloc);
    for value in values.iter() {
        vec.push(value.clone());
    }

    vec
}

fn run_test_typed_proj_vec_from_boxed_slice<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let expected_vec = expected(values, alloc);
    let vec = TypedProjVec::from(expected_vec.clone());

    let expected = expected_vec.as_slice();
    let result = vec.as_slice();

    assert_eq!(result, expected);
}

fn run_test_typed_proj_vec_from_boxed_slice_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_from_boxed_slice(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_from_boxed_slice_range_values() {
                let alloc = alloc::Global;
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                run_test_typed_proj_vec_from_boxed_slice_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_from_boxed_slice_alternating_values() {
                let alloc = alloc::Global;
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_typed_proj_vec_from_boxed_slice_values(&values, alloc);
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
