use opaque_vec::OpaqueVec;

use std::fmt;

fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}

fn run_test_opaque_vec_as_mut_slice<T>(values: &mut [T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::from(&mut *values);

    let expected = values;
    let result = vec.as_mut_slice::<T>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_as_mut_slice_empty1() {
    let values: [i32; 0] = [];
    let mut vec = OpaqueVec::from(&values);

    assert!(vec.as_mut_slice::<i32>().is_empty());
}

#[test]
fn test_opaque_vec_as_mut_slice_empty2() {
    let mut values: [i32; 0] = [];
    let mut vec = OpaqueVec::from(&values);

    let expected = values.as_mut_slice();
    let result = vec.as_mut_slice::<i32>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_as_mut_slice1() {
    let mut values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice2() {
    let mut values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice3() {
    let mut values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice4() {
    let mut values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice5() {
    let mut values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice6() {
    let mut values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice7() {
    let mut values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice8() {
    let mut values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice9() {
    let mut values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice10() {
    let mut values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice11() {
    let mut values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice12() {
    let mut values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice13() {
    let mut values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice14() {
    let mut values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice15() {
    let mut values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice16() {
    let mut values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice32() {
    let mut values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}

#[test]
fn test_opaque_vec_as_mut_slice64() {
    let mut values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_as_mut_slice(&mut values)
}
