use imgui_vulkan_renderer_opaque_vec::OpaqueVec;

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

fn run_test_opaque_vec_swap_remove_len<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let values_vec = OpaqueVec::from(values);

    for i in 0..values.len() {
        let result_vec = {
            let mut vec = values_vec.clone();
            vec.swap_remove::<T>(i);
            vec
        };

        let expected = values.len() - 1;
        let result = result_vec.len();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_opaque_vec_swap_remove_len1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_swap_remove_len(&values);
}

#[test]
fn test_opaque_vec_swap_remove_len64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_swap_remove_len(&values);
}