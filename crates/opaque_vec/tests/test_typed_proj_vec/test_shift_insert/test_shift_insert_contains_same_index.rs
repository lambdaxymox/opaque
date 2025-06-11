use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_shift_insert_contains_same_index1<T, A>(value: T, alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::new_in(alloc);

    assert!(!vec.contains(&value));

    vec.shift_insert(0, value.clone());

    assert!(vec.contains(&value));
}

fn run_test_typed_proj_vec_shift_insert_contains_same_index2<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::<T, A>::new_in(alloc);
    for value in values.iter() {
        assert!(!vec.contains(&value));
    }

    for value in values.iter().cloned() {
        vec.shift_insert(0, value);
    }

    for value in values.iter() {
        assert!(vec.contains(&value));
    }
}

fn run_test_typed_proj_vec_shift_insert_contains_same_index2_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_shift_insert_contains_same_index2(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_array_size:expr, $single_value:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_vec_shift_insert_contains_same_index1() {
                let single_value: $typ = $single_value;
                let alloc = alloc::Global;
                run_test_typed_proj_vec_shift_insert_contains_same_index1(single_value, alloc);
            }

            #[test]
            fn test_typed_proj_vec_shift_insert_contains_same_index2_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_shift_insert_contains_same_index2_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_shift_insert_contains_same_index2_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_shift_insert_contains_same_index2_values(&values, alloc);
            }
        }
    };
}

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
