use opaque_vec::OpaqueVec;

fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}

fn run_test_opaque_vec_push_len<T>(values: &[T])
where
    T: Clone + 'static,
{
    let mut vec = OpaqueVec::new::<T>();
    for value in values.iter().cloned() {
        vec.push::<T>(value);
    }

    let expected = values.len();
    let result = vec.len();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_push_len_empty() {
    let values: [i32; 0] = [];

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_push_len(&values);
}

#[test]
fn test_opaque_vec_push_len64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_push_len(&values);
}
