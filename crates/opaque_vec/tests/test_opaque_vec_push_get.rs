use opaque_vec::OpaqueVec;

use std::fmt;

fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}

fn run_test_opaque_vec_push_get<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();
    for value in values.iter().cloned() {
        vec.push::<T>(value);
    }

    for i in 0..vec.len() {
        let expected = Some(values[i].clone());
        let result = vec.get::<T>(i).cloned();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_opaque_vec_push_get1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_push_get(&values);
}

#[test]
fn test_opaque_vec_push_get64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_push_get(&values);
}
