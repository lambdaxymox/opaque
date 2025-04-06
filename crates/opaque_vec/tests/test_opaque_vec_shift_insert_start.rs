use opaque_vec::OpaqueVec;

use std::fmt;

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

fn expected<T>(values: &[T]) -> OpaqueVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    OpaqueVec::from_iter(values.iter().rev().cloned())
}

fn run_test_opaque_vec_shift_insert_start<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();

    for value in values.iter().cloned() {
        vec.shift_insert::<T>(0, value);
    }

    let expected_vec = expected(values);
    let expected = expected_vec.as_slice::<T>();
    let result = vec.as_slice::<T>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_shift_insert_start1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_shift_insert_start(&values);
}

#[test]
fn test_opaque_vec_shift_insert_start64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_shift_insert_start(&values);
}
