use opaque_vec::TypedProjVec;

use core::any;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_push_contains<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut vec = TypedProjVec::new_in(alloc);

    for value in values.iter() {
        assert!(!vec.contains(value));
    }

    for value in values.iter().cloned() {
        vec.push(value);
    }

    for value in values.iter() {
        assert!(vec.contains(value));
    }
}

fn run_test_typed_proj_vec_push_contains_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_push_contains(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_push_contains_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_push_contains_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_push_contains_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_push_contains_values(&values, alloc);
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
