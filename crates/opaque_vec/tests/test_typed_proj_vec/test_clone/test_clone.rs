use crate::common;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_typed_proj_vec_clone<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let vec = common::typed_proj_vec::from_slice_in(values, alloc);
    let cloned_vec = vec.clone();

    let expected = vec.as_slice();
    let result = cloned_vec.as_slice();

    assert_eq!(result, expected);
}

fn run_test_typed_proj_vec_clone_occupy_disjoint_memory_locations<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let vec1 = common::typed_proj_vec::from_slice_in(values, alloc);
    let vec2 = vec1.clone();

    assert_ne!(vec1.as_ptr(), vec2.as_ptr());
}

fn run_test_typed_proj_vec_clone_occupy_disjoint_memory_regions<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let vec1 = common::typed_proj_vec::from_slice_in(values, alloc);
    let vec2 = vec1.clone();

    let ptr_start1 = vec1.as_ptr() as usize;
    let ptr_start2 = vec2.as_ptr() as usize;
    let ptr_end1 = {
        let len1 = vec1.len() * std::mem::size_of::<T>();
        ptr_start1 + len1
    };
    let ptr_end2 = {
        let len2 = vec2.len() * std::mem::size_of::<T>();
        ptr_start2 + len2
    };

    assert!(ptr_end1 <= ptr_start2 || ptr_end2 <= ptr_start1);
}

fn run_test_typed_proj_vec_clone_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_clone(slice, alloc.clone());
    }
}

fn run_test_typed_proj_vec_clone_occupy_disjoint_memory_locations_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_clone_occupy_disjoint_memory_locations(slice, alloc.clone());
    }
}

fn run_test_typed_proj_vec_clone_occupy_disjoint_memory_regions_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_vec_clone_occupy_disjoint_memory_regions(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_clone_empty() {
                let values: [$typ; 0] = [];
                let alloc = alloc::Global;
                run_test_typed_proj_vec_clone(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_clone_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_clone_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_clone_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_clone_values(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_clone_occupy_disjoint_memory_locations() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_clone_occupy_disjoint_memory_locations(&values, alloc);
            }

            #[test]
            fn test_typed_proj_vec_clone_occupy_disjoint_memory_regions() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_vec_clone_occupy_disjoint_memory_regions(&values, alloc);
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
