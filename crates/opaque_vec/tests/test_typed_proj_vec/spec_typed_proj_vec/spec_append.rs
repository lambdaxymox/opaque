use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_append_as_slice_source<T, A>(values1: TypedProjVec<T, A>, values2: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut source = values1.clone();
    let mut destination = values2.clone();

    source.append(&mut destination);

    for i in 0..values1.len() {
        prop_assert_eq!(&source[i], &values1[i]);
    }

    for j in 0..values2.len() {
        prop_assert_eq!(&source[values1.len() + j], &values2[j]);
    }

    Ok(())
}

fn prop_append_as_slice_destination<T, A>(values1: TypedProjVec<T, A>, values2: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut source = values1.clone();
    let mut destination = values2.clone();

    source.append(&mut destination);

    prop_assert!(destination.is_empty());

    Ok(())
}

fn prop_append_len_source<T, A>(values1: TypedProjVec<T, A>, values2: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut source = values1.clone();
    let mut destination = values2.clone();

    source.append(&mut destination);

    let expected = values1.len() + values2.len();
    let result = source.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_append_len_destination<T, A>(values1: TypedProjVec<T, A>, values2: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut source = values1.clone();
    let mut destination = values2.clone();

    source.append(&mut destination);

    let expected = 0;
    let result = destination.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_append_as_slice_source(
                    values1 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                    values2 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                ) {
                    let values1: super::TypedProjVec<$typ, alloc::Global> = values1;
                    let values2: super::TypedProjVec<$typ, alloc::Global> = values2;
                    super::prop_append_as_slice_source(values1, values2)?
                }

                #[test]
                fn prop_append_as_slice_destination(
                    values1 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                    values2 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                ) {
                    let values1: super::TypedProjVec<$typ, alloc::Global> = values1;
                    let values2: super::TypedProjVec<$typ, alloc::Global> = values2;
                    super::prop_append_as_slice_destination(values1, values2)?
                }

                #[test]
                fn prop_append_len_source(
                    values1 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                    values2 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                ) {
                    let values1: super::TypedProjVec<$typ, alloc::Global> = values1;
                    let values2: super::TypedProjVec<$typ, alloc::Global> = values2;
                    super::prop_append_len_source(values1, values2)?
                }

                #[test]
                fn prop_append_len_destination(
                    values1 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                    values2 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                ) {
                    let values1: super::TypedProjVec<$typ, alloc::Global> = values1;
                    let values2: super::TypedProjVec<$typ, alloc::Global> = values2;
                    super::prop_append_len_destination(values1, values2)?
                }
            }
        }
    };
}

generate_props!(unit, (), 128, strategy_type_projected_vec_max_len);
generate_props!(u8, u8, 128, strategy_type_projected_vec_max_len);
generate_props!(u16, u16, 128, strategy_type_projected_vec_max_len);
generate_props!(u32, u32, 128, strategy_type_projected_vec_max_len);
generate_props!(u64, u64, 128, strategy_type_projected_vec_max_len);
generate_props!(usize, usize, 128, strategy_type_projected_vec_max_len);
generate_props!(string, String, 128, strategy_type_projected_vec_max_len);
