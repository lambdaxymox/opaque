use opaque_vec::OpaqueVec;

use std::fmt;

fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}

fn run_test_opaque_vec_clone_len<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let vec = OpaqueVec::from(values);
    let cloned_vec = vec.clone();

    let expected = vec.len();
    let result = cloned_vec.len();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_clone_len_empty() {
    let values: [i32; 0] = [];
    let vec = OpaqueVec::from(values);
    let cloned_vec = vec.clone();

    let expected = vec.len();
    let result = cloned_vec.len();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_clone_len1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_clone_len(&values)
}

#[test]
fn test_opaque_vec_clone_len64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_clone_len(&values)
}
