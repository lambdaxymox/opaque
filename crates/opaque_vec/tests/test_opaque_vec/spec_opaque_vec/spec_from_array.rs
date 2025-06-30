use crate::common::erased::strategy_array;
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_from_array_as_slice<const N: usize, T>(expected: [T; N]) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
{
    let vec = OpaqueVec::from(expected.clone());
    let result = vec.as_slice::<T, alloc::Global>();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $array_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            proptest! {
                #[test]
                fn prop_from_array_as_slice0(array in super::$array_gen::<$typ, 0>()) {
                    let array: [$typ; 0] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice1(array in super::$array_gen::<$typ, 1>()) {
                    let array: [$typ; 1] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice2(array in super::$array_gen::<$typ, 2>()) {
                    let array: [$typ; 2] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice3(array in super::$array_gen::<$typ, 3>()) {
                    let array: [$typ; 3] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice4(array in super::$array_gen::<$typ, 4>()) {
                    let array: [$typ; 4] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice5(array in super::$array_gen::<$typ, 5>()) {
                    let array: [$typ; 5] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice6(array in super::$array_gen::<$typ, 6>()) {
                    let array: [$typ; 6] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice7(array in super::$array_gen::<$typ, 7>()) {
                    let array: [$typ; 7] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice8(array in super::$array_gen::<$typ, 8>()) {
                    let array: [$typ; 8] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice9(array in super::$array_gen::<$typ, 9>()) {
                    let array: [$typ; 9] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice10(array in super::$array_gen::<$typ, 10>()) {
                    let array: [$typ; 10] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice11(array in super::$array_gen::<$typ, 11>()) {
                    let array: [$typ; 11] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice12(array in super::$array_gen::<$typ, 12>()) {
                    let array: [$typ; 12] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice13(array in super::$array_gen::<$typ, 13>()) {
                    let array: [$typ; 13] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice14(array in super::$array_gen::<$typ, 14>()) {
                    let array: [$typ; 14] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice15(array in super::$array_gen::<$typ, 15>()) {
                    let array: [$typ; 15] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice16(array in super::$array_gen::<$typ, 16>()) {
                    let array: [$typ; 16] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice32(array in super::$array_gen::<$typ, 32>()) {
                    let array: [$typ; 32] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice64(array in super::$array_gen::<$typ, 64>()) {
                    let array: [$typ; 64] = array;
                    super::prop_from_array_as_slice(array)?;
                }

                #[test]
                fn prop_from_array_as_slice128(array in super::$array_gen::<$typ, 128>()) {
                    let array: [$typ; 128] = array;
                    super::prop_from_array_as_slice(array)?;
                }
            }
        }
    };
}

generate_props!(unit,   (),     strategy_array);
generate_props!(u8,     u8,     strategy_array);
generate_props!(u16,    u16,    strategy_array);
generate_props!(u32,    u32,    strategy_array);
generate_props!(u64,    u64,    strategy_array);
generate_props!(usize,  usize,  strategy_array);
generate_props!(string, String, strategy_array);
