use crate::common::projected::strategy_alloc;
use opaque_vec::TypeProjectedVec;

use core::any;
use core::fmt;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_pop_empty1<T, A>(alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec: TypeProjectedVec<T, A> = TypeProjectedVec::new_in(alloc);

    prop_assert!(vec.pop().is_none());

    Ok(())
}

fn prop_pop_empty2<T, A>(alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec: TypeProjectedVec<T, A> = TypeProjectedVec::new_in(alloc);

    for _ in 0..65536 {
        prop_assert!(vec.pop().is_none());
    }

    Ok(())
}

fn prop_pop_empty_is_empty1<T, A>(alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec: TypeProjectedVec<T, A> = TypeProjectedVec::new_in(alloc);

    prop_assert!(vec.is_empty());

    vec.pop();

    prop_assert!(vec.is_empty());

    Ok(())
}

fn prop_pop_empty_is_empty2<T, A>(alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec: TypeProjectedVec<T, A> = TypeProjectedVec::new_in(alloc);

    prop_assert!(vec.is_empty());

    for _ in 0..65536 {
        vec.pop();
    }

    prop_assert!(vec.is_empty());

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $alloc_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_pop_empty1(alloc in super::$alloc_gen::<$alloc_typ>()) {
                    let alloc: $alloc_typ = alloc;
                    super::prop_pop_empty1::<$typ, $alloc_typ>(alloc)?
                }

                #[test]
                fn prop_pop_empty2(alloc in super::$alloc_gen::<$alloc_typ>()) {
                    let alloc: $alloc_typ = alloc;
                    super::prop_pop_empty2::<$typ, $alloc_typ>(alloc)?
                }

                #[test]
                fn prop_pop_empty_is_empty1(alloc in super::$alloc_gen::<$alloc_typ>()) {
                    let alloc: $alloc_typ = alloc;
                    super::prop_pop_empty_is_empty1::<$typ, $alloc_typ>(alloc)?
                }

                #[test]
                fn prop_pop_empty_is_empty2(alloc in super::$alloc_gen::<$alloc_typ>()) {
                    let alloc: $alloc_typ = alloc;
                    super::prop_pop_empty_is_empty2::<$typ, $alloc_typ>(alloc)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 32, strategy_alloc);
generate_props!(u8, u8, alloc::Global, 32, strategy_alloc);
generate_props!(u16, u16, alloc::Global, 32, strategy_alloc);
generate_props!(u32, u32, alloc::Global, 32, strategy_alloc);
generate_props!(u64, u64, alloc::Global, 32, strategy_alloc);
generate_props!(usize, usize, alloc::Global, 32, strategy_alloc);
generate_props!(string, String, alloc::Global, 32, strategy_alloc);
