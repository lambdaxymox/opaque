use crate::common::erased::{
    strategy_type_erased_vec_len,
    strategy_type_erased_vec_max_len,
};
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn strategy_prop_into_iter_take<T, A>(max_length: usize) -> impl Strategy<Value = (OpaqueVec, usize)>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (1..=max_length).prop_flat_map(move |length| (strategy_type_erased_vec_len::<T, A>(length), 0..=length))
}

fn prop_into_iter_back_to_vec<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let vec = values.clone::<T, A>();
    let mut result = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for value in vec.into_iter::<T, A>() {
        result.push::<T, A>(value);
    }

    let expected = values.as_slice::<T, A>();
    let result = result.as_slice::<T, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_into_iter_take<T, A>((values, count): (OpaqueVec, usize)) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    vec.extend::<_, T, A>(values.iter::<T, A>().cloned());

    let mut expected = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for value in values.iter::<T, A>().cloned().take(count) {
        expected.push::<T, A>(value);
    }

    let mut result = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    result.extend::<_, T, A>(values.into_iter::<T, A>().take(count));

    prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());

    Ok(())
}

fn prop_into_iter_take_none<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    vec.extend::<_, T, A>(values.iter::<T, A>().cloned());

    let mut result = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    result.extend::<_, T, A>(values.into_iter::<T, A>().take(0));

    prop_assert!(result.is_empty());

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_into_iter_back_to_vec(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_into_iter_back_to_vec::<$typ, alloc::Global>(values)?
                }

                #[test]
                fn prop_into_iter_take(values in super::strategy_prop_into_iter_take::<$typ, alloc::Global>($max_length)) {
                    let values: (super::OpaqueVec, usize) = values;
                    super::prop_into_iter_take::<$typ, alloc::Global>(values)?
                }

                #[test]
                fn prop_into_iter_take_none(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_into_iter_take_none::<$typ, alloc::Global>(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), 128, strategy_type_erased_vec_max_len);
generate_props!(u8, u8, 128, strategy_type_erased_vec_max_len);
generate_props!(u16, u16, 128, strategy_type_erased_vec_max_len);
generate_props!(u32, u32, 128, strategy_type_erased_vec_max_len);
generate_props!(u64, u64, 128, strategy_type_erased_vec_max_len);
generate_props!(usize, usize, 128, strategy_type_erased_vec_max_len);
generate_props!(string, String, 128, strategy_type_erased_vec_max_len);
