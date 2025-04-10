mod common;

use opaque_vec::OpaqueVec;

use core::fmt;

use common::array_generators as ag;

/*
fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}
 */

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
    for len in 0..values.len() {
        run_test_opaque_vec_clone(&values[0..len]);
    }
}

fn run_test_opaque_vec_clone_occupy_disjoint_memory_locations_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + TryFrom<usize> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    for len in 0..values.len() {
        run_test_opaque_vec_clone_occupy_disjoint_memory_locations(&values[0..len]);
    }
}

fn run_test_opaque_vec_clone_occupy_disjoint_memory_regions_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + TryFrom<usize> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    for len in 0..values.len() {
        run_test_opaque_vec_clone_occupy_disjoint_memory_regions(&values[0..len]);
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
                let values = ag::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_clone_values(&values);
            }

            #[test]
            fn test_opaque_vec_clone_alternating_values() {
                let values = ag::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_vec_clone_values(&values);
            }

            #[test]
            fn test_opaque_vec_clone_occupy_disjoint_memory_locations() {
                let values = ag::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_clone_occupy_disjoint_memory_locations(&values);
            }

            #[test]
            fn test_opaque_vec_clone_occupy_disjoint_memory_regions() {
                let values = ag::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_clone_occupy_disjoint_memory_regions(&values);
            }
        }
    };
}

generate_tests!(i8,    128,  ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i8::MIN,    0));
generate_tests!(i16,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i16::MIN,   0));
generate_tests!(i32,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i32::MIN,   0));
generate_tests!(i64,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i64::MIN,   0));
generate_tests!(i128,  1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i128::MIN,  0));
generate_tests!(isize, 1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(isize::MIN, 0));

generate_tests!(u8,    128,  ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u8::MIN,    u8::MAX));
generate_tests!(u16,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u16::MIN,   u16::MAX));
generate_tests!(u32,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u32::MIN,   u32::MAX));
generate_tests!(u64,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u64::MIN,   u64::MAX));
generate_tests!(u128,  1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u128::MIN,  u128::MAX));
generate_tests!(usize, 1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(usize::MIN, usize::MAX));
/*

#[test]
fn test_opaque_vec_clone_empty1() {
    let values: [i32; 0] = [];
    let mut vec = OpaqueVec::from(&values);

    assert!(vec.as_slice::<i32>().is_empty());
}

#[test]
fn test_opaque_vec_clone_empty2() {
    let values: [i32; 0] = [];
    let mut vec = OpaqueVec::from(&values);

    let expected = values.as_slice();
    let result = vec.as_slice::<i32>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_clone1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_clone(&values)
}
*/
