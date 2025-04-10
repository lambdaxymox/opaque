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

fn negative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = -((i as i32) + 1)
    }

    prefix
}
*/

fn run_test_opaque_vec_shift_insert_contains_same_index1<T>(value: T)
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();

    assert!(!vec.contains::<T>(&value));

    vec.shift_insert::<T>(0, value.clone());

    assert!(vec.contains::<T>(&value));
}

fn run_test_opaque_vec_shift_insert_contains_same_index2<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();
    for value in values.iter() {
        assert!(!vec.contains::<T>(&value));
    }

    for value in values.iter().cloned() {
        vec.shift_insert::<T>(0, value);
    }

    for value in values.iter() {
        assert!(vec.contains::<T>(&value));
    }
}

fn run_test_opaque_vec_shift_insert_contains_same_index2_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    for len in 0..values.len() {
        let prefix_values = &values[0..len];
        run_test_opaque_vec_shift_insert_contains_same_index2(prefix_values);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $single_value:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_shift_insert_contains_same_index1() {
                run_test_opaque_vec_shift_insert_contains_same_index1($single_value);
            }

            #[test]
            fn test_opaque_vec_shift_insert_contains_same_index2_range_values() {
                let values = ag::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_shift_insert_contains_same_index2_values(&values);
            }

            #[test]
            fn test_opaque_vec_shift_insert_contains_same_index2_alternating_values() {
                let values = ag::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_vec_shift_insert_contains_same_index2_values(&values);
            }
        }
    };
}

generate_tests!(i8,    128,  i8::MAX,    ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i8::MIN,    0));
generate_tests!(i16,   1024, i16::MAX,   ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i16::MIN,   0));
generate_tests!(i32,   1024, i32::MAX,   ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i32::MIN,   0));
generate_tests!(i64,   1024, i64::MAX,   ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i64::MIN,   0));
generate_tests!(i128,  1024, i128::MAX,  ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i128::MIN,  0));
generate_tests!(isize, 1024, isize::MAX, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(isize::MIN, 0));

generate_tests!(u8,    128,  u8::MAX,    ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u8::MIN,    u8::MAX));
generate_tests!(u16,   1024, u16::MAX,   ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u16::MIN,   u16::MAX));
generate_tests!(u32,   1024, u32::MAX,   ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u32::MIN,   u32::MAX));
generate_tests!(u64,   1024, u64::MAX,   ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u64::MIN,   u64::MAX));
generate_tests!(u128,  1024, u128::MAX,  ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u128::MIN,  u128::MAX));
generate_tests!(usize, 1024, usize::MAX, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(usize::MIN, usize::MAX));

/*
#[test]
fn test_opaque_vec_shift_insert_contains1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_shift_insert_contains(&values);
}

#[test]
fn test_opaque_vec_shift_insert_contains64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_shift_insert_contains(&values);
}
*/