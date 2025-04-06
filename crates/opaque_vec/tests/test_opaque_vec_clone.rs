use imgui_vulkan_renderer_opaque_vec::OpaqueVec;

use std::fmt;

fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}

fn run_test_opaque_vec_clone<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let vec = OpaqueVec::from(values);
    let cloned_vec = vec.clone();

    let expected = vec.as_slice::<T>();
    let result = cloned_vec.as_slice::<T>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_clone_occupy_disjoint_memory_locations() {
    let vec1 = {
        let mut _vec1 = OpaqueVec::new::<isize>();
        _vec1.push::<isize>(1);
        _vec1.push::<isize>(2);
        _vec1.push::<isize>(3);
        _vec1
    };
    let vec2 = vec1.clone();

    assert_ne!(vec1.as_ptr::<isize>(), vec2.as_ptr::<isize>());
}

#[test]
fn test_opaque_vec_clone_occupy_disjoint_memory_regions() {
    let vec1 = {
        let mut _vec1 = OpaqueVec::new::<isize>();
        _vec1.push::<isize>(1);
        _vec1.push::<isize>(2);
        _vec1.push::<isize>(3);
        _vec1
    };
    let vec2 = vec1.clone();

    let ptr_start1 = vec1.as_ptr::<isize>() as usize;
    let ptr_start2 = vec2.as_ptr::<isize>() as usize;
    let ptr_end1 = {
        let len1 = vec1.len() * std::mem::size_of::<isize>();
        ptr_start1 + len1
    };
    let ptr_end2 = {
        let len2 = vec2.len() * std::mem::size_of::<isize>();
        ptr_start2 + len2
    };

    assert!(ptr_end1 <= ptr_start2 || ptr_end2 <= ptr_start1);
}

#[test]
fn test_opaque_vec_clone_empty1() {
    let values: [i32; 0] = [];
    let mut vec = OpaqueVec::from(&values);

    assert!(vec.as_slice::<i32>().is_empty());
}

#[test]
fn test_opaque_vec_clone_empty2() {
    let values: [i32; 0] = [];
    let mut vec = OpaqueVec::from(&values);

    let expected = values.as_slice();
    let result = vec.as_slice::<i32>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_clone1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_clone(&values)
}

#[test]
fn test_opaque_vec_clone64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_clone(&values)
}
