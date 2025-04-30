use opaque_vec::OpaqueVec;

use core::fmt;

use opaque_vec_testing as ovt;

fn run_test_opaque_vec_clone<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let vec = OpaqueVec::from(values);
    let cloned_vec = vec.clone();

    let expected = vec.as_slice::<T>();
    let result = cloned_vec.as_slice::<T>();

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_clone_occupy_disjoint_memory_locations<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let vec1 = OpaqueVec::from(values);
    let vec2 = vec1.clone();

    assert_ne!(vec1.as_ptr::<T>(), vec2.as_ptr::<T>());
}

fn run_test_opaque_vec_clone_occupy_disjoint_memory_regions<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let vec1 = OpaqueVec::from(values);
    let vec2 = vec1.clone();

    let ptr_start1 = vec1.as_ptr::<T>() as usize;
    let ptr_start2 = vec2.as_ptr::<T>() as usize;
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

fn run_test_opaque_vec_clone_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + TryFrom<usize> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_opaque_vec_clone(slice);
    }
}

fn run_test_opaque_vec_clone_occupy_disjoint_memory_locations_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + TryFrom<usize> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_opaque_vec_clone_occupy_disjoint_memory_locations(slice);
    }
}

fn run_test_opaque_vec_clone_occupy_disjoint_memory_regions_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + TryFrom<usize> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_opaque_vec_clone_occupy_disjoint_memory_regions(slice);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_clone_empty() {
                let values: [$typ; 0] = [];

                run_test_opaque_vec_clone(&values);
            }

            #[test]
            fn test_opaque_vec_clone_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_clone_values(&values);
            }

            #[test]
            fn test_opaque_vec_clone_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_vec_clone_values(&values);
            }

            #[test]
            fn test_opaque_vec_clone_occupy_disjoint_memory_locations() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_clone_occupy_disjoint_memory_locations(&values);
            }

            #[test]
            fn test_opaque_vec_clone_occupy_disjoint_memory_regions() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_clone_occupy_disjoint_memory_regions(&values);
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
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i16::MIN, 0)
);
generate_tests!(
    i32,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i32::MIN, 0)
);
generate_tests!(
    i64,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i64::MIN, 0)
);
generate_tests!(
    i128,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i128::MIN, 0)
);
generate_tests!(
    isize,
    1024,
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
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX)
);
generate_tests!(
    u32,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX)
);
generate_tests!(
    u64,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX)
);
generate_tests!(
    u128,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u128::MIN, u128::MAX)
);
generate_tests!(
    usize,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
