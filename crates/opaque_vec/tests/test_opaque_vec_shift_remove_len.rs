mod common;

use opaque_vec::OpaqueVec;

use core::fmt;

use common::array_generators as ag;

/*
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
 */

fn run_test_opaque_vec_shift_remove_len<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let values_vec = OpaqueVec::from(values);

    for i in 0..values.len() {
        let result_vec = {
            let mut vec = values_vec.clone();
            vec.shift_remove::<T>(i);
            vec
        };

        let expected = values.len() - 1;
        let result = result_vec.len();

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_vec_shift_remove_len_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    for len in 0..values.len() {
        let prefix_values = &values[0..len];
        run_test_opaque_vec_shift_remove_len(prefix_values);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_shift_remove_len_range_values() {
                let values = ag::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_shift_remove_len_values(&values);
            }

            #[test]
            fn test_opaque_vec_shift_remove_len_alternating_values() {
                let values = ag::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_vec_shift_remove_len_values(&values);
            }
        }
    };
}

generate_tests!(i8,    128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i8::MIN,    0));
generate_tests!(i16,   128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i16::MIN,   0));
generate_tests!(i32,   128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i32::MIN,   0));
generate_tests!(i64,   128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i64::MIN,   0));
generate_tests!(i128,  128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i128::MIN,  0));
generate_tests!(isize, 128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(isize::MIN, 0));

generate_tests!(u8,    128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u8::MIN,    u8::MAX));
generate_tests!(u16,   128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u16::MIN,   u16::MAX));
generate_tests!(u32,   128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u32::MIN,   u32::MAX));
generate_tests!(u64,   128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u64::MIN,   u64::MAX));
generate_tests!(u128,  128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u128::MIN,  u128::MAX));
generate_tests!(usize, 128, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(usize::MIN, usize::MAX));

/*
#[test]
fn test_opaque_vec_shift_remove_len1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_shift_remove_len(&values);
}

#[test]
fn test_opaque_vec_shift_remove_len64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_shift_remove_len(&values);
}
*/
