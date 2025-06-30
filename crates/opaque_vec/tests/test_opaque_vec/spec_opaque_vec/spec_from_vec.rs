use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_from_vec<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &[T], alloc: A) -> Vec<T, A>
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = Vec::with_capacity_in(values.len(), alloc);
        vec.extend(values.iter().cloned());

        vec
    }

    let expected_values = expected(values.as_slice::<T, A>(), values.allocator::<T, A>().allocator().clone());
    let vec = OpaqueVec::from(expected_values.clone());

    let expected = expected_values.as_slice();
    let result = vec.as_slice::<T, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_from_vec(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_from_vec::<$typ, $alloc_typ>(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(u8, u8, alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(u16, u16, alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(u32, u32, alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(u64, u64, alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(usize, usize, alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(string, String, alloc::Global, 128, strategy_type_erased_vec_max_len);
