use imgui_vulkan_renderer_opaque_vec::OpaqueVec;

fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}

fn run_test_opaque_vec_push_contains<T>(values: &[T])
where
    T: PartialEq + Clone + 'static,
{
    let mut vec = OpaqueVec::new::<T>();

    for value in values.iter() {
        assert!(!vec.contains::<T>(value));
    }

    for value in values.iter().cloned() {
        vec.push::<T>(value);
    }

    for value in values.iter() {
        assert!(vec.contains::<T>(value));
    }
}

#[test]
fn test_opaque_vec_push_contains_empty() {
    let values: [i32; 0] = [];

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_push_contains(&values);
}

#[test]
fn test_opaque_vec_push_contains64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_push_contains(&values);
}
