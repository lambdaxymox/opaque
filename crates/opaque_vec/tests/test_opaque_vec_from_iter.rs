use opaque_vec::OpaqueVec;

use std::fmt;

fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}

fn run_test_opaque_vec_from_iter_slice<T>(expected: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let vec = OpaqueVec::from_iter(expected.iter().cloned());
    let result = vec.as_slice::<T>();

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_from_iter_array<const N: usize, T>(expected: [T; N])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let vec = OpaqueVec::from_iter(expected.iter().cloned());
    let result = vec.as_slice::<T>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_from_iter_slice_empty() {
    let values: [i32; 0] = [];
    let mut vec = OpaqueVec::from_iter(values);

    let expected = values.as_slice();
    let result = vec.as_slice::<i32>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_from_iter_slice1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_slice64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_from_iter_slice(&values)
}

#[test]
fn test_opaque_vec_from_iter_array1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_from_iter_array(values)
}

#[test]
fn test_opaque_vec_from_iter_array64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_from_iter_array(values)
}
