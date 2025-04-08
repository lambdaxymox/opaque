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

fn expected<T>(values: &[T], extension_values: &[T]) -> OpaqueVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::from(values);
    for extension_value in extension_values.iter() {
        vec.push::<T>(extension_value.clone());
    }

    vec
}

fn result<T>(values: &[T], extension_values: &[T]) -> OpaqueVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::from(values);
    vec.extend_from_slice::<T>(extension_values);

    vec
}

fn run_test_opaque_vec_extend_from_slice<T>(values: &[T], extension_values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let expected = expected(values, extension_values);
    let result = result(values, extension_values);

    assert_eq!(result.as_slice::<T>(), expected.as_slice::<T>());
}

#[test]
fn test_opaque_vec_extend_from_slice1() {
    let values = nonnegative_integer_values::<1>();
    let extension_values = OpaqueVec::from_iter((0..1).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice2() {
    let values = nonnegative_integer_values::<2>();
    let extension_values = OpaqueVec::from_iter((0..2).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice3() {
    let values = nonnegative_integer_values::<3>();
    let extension_values = OpaqueVec::from_iter((0..3).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice4() {
    let values = nonnegative_integer_values::<4>();
    let extension_values = OpaqueVec::from_iter((0..4).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice5() {
    let values = nonnegative_integer_values::<5>();
    let extension_values = OpaqueVec::from_iter((0..5).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice6() {
    let values = nonnegative_integer_values::<6>();
    let extension_values = OpaqueVec::from_iter((0..6).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice7() {
    let values = nonnegative_integer_values::<7>();
    let extension_values = OpaqueVec::from_iter((0..7).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice8() {
    let values = nonnegative_integer_values::<8>();
    let extension_values = OpaqueVec::from_iter((0..8).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice9() {
    let values = nonnegative_integer_values::<9>();
    let extension_values = OpaqueVec::from_iter((0..9).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice10() {
    let values = nonnegative_integer_values::<10>();
    let extension_values = OpaqueVec::from_iter((0..10).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice11() {
    let values = nonnegative_integer_values::<11>();
    let extension_values = OpaqueVec::from_iter((0..11).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice12() {
    let values = nonnegative_integer_values::<12>();
    let extension_values = OpaqueVec::from_iter((0..12).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice13() {
    let values = nonnegative_integer_values::<13>();
    let extension_values = OpaqueVec::from_iter((0..13).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice14() {
    let values = nonnegative_integer_values::<14>();
    let extension_values = OpaqueVec::from_iter((0..14).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice15() {
    let values = nonnegative_integer_values::<15>();
    let extension_values = OpaqueVec::from_iter((0..15).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice16() {
    let values = nonnegative_integer_values::<16>();
    let extension_values = OpaqueVec::from_iter((0..16).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice32() {
    let values = nonnegative_integer_values::<32>();
    let extension_values = OpaqueVec::from_iter((0..32).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice64() {
    let values = nonnegative_integer_values::<64>();
    let extension_values = OpaqueVec::from_iter((0..64).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}
