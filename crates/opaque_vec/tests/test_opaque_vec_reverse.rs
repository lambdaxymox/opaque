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
    let mut vec = OpaqueVec::new::<T>();
    for value in values.iter().rev().cloned() {
        vec.push::<T>(value);
    }

    vec
}

fn run_test_opaque_vec_reverse<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let expected = expected(values);
    let result = {
        let mut vec = OpaqueVec::from(values);
        vec.reverse::<T>();

        vec
    };

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_reverse_empty() {
    let values: [i32; 0] = [];

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_reverse(&values);
}

#[test]
fn test_opaque_vec_reverse64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_reverse(&values);
}
