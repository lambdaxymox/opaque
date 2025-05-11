use crate::common;

use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn expected<T, A>(values: &[T], alloc: A) -> TypedProjVec<T, A>
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut vec = TypedProjVec::new_in(alloc);
    for value in values.iter().skip(1).cloned() {
        vec.push(value);
    }

    vec
}

fn run_test_typed_proj_vec_shift_remove_start<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut vec = common::typed_proj_vec::from_slice_in(values, alloc.clone());

    for i in 0..values.len() {
        let new_vec = expected(&values[i..], alloc.clone());
        let _ = vec.shift_remove(0);
        let expected = new_vec.as_slice();
        let result = vec.as_slice();

        assert_eq!(result, expected);
    }
}

fn run_test_typed_proj_vec_shift_remove_start_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_shift_remove_start(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_shift_remove_end_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_shift_remove_start_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_shift_remove_end_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_shift_remove_start_values(&values, alloc);
            }
        }
    };
}

generate_tests!(
    i8,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i8::MIN, 0)
);
generate_tests!(
    i16,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i16::MIN, 0)
);
generate_tests!(
    i32,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i32::MIN, 0)
);
generate_tests!(
    i64,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i64::MIN, 0)
);
generate_tests!(
    i128,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i128::MIN, 0)
);
generate_tests!(
    isize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(isize::MIN, 0)
);

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
    u128,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u128::MIN, u128::MAX)
);
generate_tests!(
    usize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
