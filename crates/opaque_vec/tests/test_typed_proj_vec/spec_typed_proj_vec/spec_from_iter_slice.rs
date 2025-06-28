use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;
use proptest::prelude::*;

fn prop_from_iter_slice<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let result_values = TypedProjVec::from_iter(values.as_slice().iter().cloned());
    let expected = values.as_slice();
    let result = result_values.as_slice();

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
                fn prop_from_iter_slice(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::TypedProjVec<$typ, alloc::Global> = values;
                    super::prop_from_iter_slice(values)?
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
