use opaque_vec::OpaqueVec;

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

fn run_test_opaque_vec_replace_insert_len<T>(values: &[T])
where
    T: PartialEq + Clone + 'static,
{
    let mut vec = OpaqueVec::new::<T>();
    for (i, value) in values.iter().cloned().enumerate() {
        vec.replace_insert::<T>(i, value);
    }

    assert_eq!(vec.len(), values.len());
}

#[test]
fn test_opaque_vec_replace_insert_len_same_index1() {
    let mut vec = OpaqueVec::new::<u32>();

    assert!(vec.is_empty());

    for value in 0..65536 {
        vec.replace_insert::<u32>(0, value);
    }

    assert_eq!(vec.len(), 1);
}

#[test]
fn test_opaque_vec_replace_insert_len1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_replace_insert_len(&values);
}

#[test]
fn test_opaque_vec_replace_insert_len64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_replace_insert_len(&values);
}
