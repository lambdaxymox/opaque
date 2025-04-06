use imgui_vulkan_renderer_opaque_vec::OpaqueVec;

use std::fmt;

fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}

fn run_test_opaque_vec_from_slice<T>(expected: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let vec = OpaqueVec::from(expected);
    let result = vec.as_slice::<T>();

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_from_array<const N: usize, T>(expected: [T; N])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let vec = OpaqueVec::from(expected.clone());
    let result = vec.as_slice::<T>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_from_slice_empty() {
    let values: [i32; 0] = [];
    let mut vec = OpaqueVec::from(values);

    let expected = values.as_slice();
    let result = vec.as_slice::<i32>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_from_slice1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_slice64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_from_slice(&values)
}

#[test]
fn test_opaque_vec_from_array1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_from_array(values)
}

#[test]
fn test_opaque_vec_from_array64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_from_array(values)
}
