use opaque_vec::OpaqueVec;

use std::fmt;

fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}

fn run_test_opaque_vec_as_slice<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::from(values);

    let expected = values;
    let result = vec.as_slice::<T>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_as_slice_empty1() {
    let values: [i32; 0] = [];
    let mut vec = OpaqueVec::from(&values);

    assert!(vec.as_slice::<i32>().is_empty());
}

#[test]
fn test_opaque_vec_as_slice_empty2() {
    let values: [i32; 0] = [];
    let mut vec = OpaqueVec::from(&values);

    let expected = values.as_slice();
    let result = vec.as_slice::<i32>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_as_slice1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_as_slice(&values)
}

#[test]
fn test_opaque_vec_as_slice64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_as_slice(&values)
}
