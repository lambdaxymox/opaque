use crate::common::erased::strategy_alloc;
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_pop_empty1<T, A>(alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    prop_assert!(vec.pop::<T, A>().is_none());

    Ok(())
}

fn prop_pop_empty2<T, A>(alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    for _ in 0..65536 {
        prop_assert!(vec.pop::<T, A>().is_none());
    }

    Ok(())
}

fn prop_pop_empty_is_empty1<T, A>(alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    prop_assert!(vec.is_empty());

    vec.pop::<T, A>();

    prop_assert!(vec.is_empty());

    Ok(())
}

fn prop_pop_empty_is_empty2<T, A>(alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    prop_assert!(vec.is_empty());

    for _ in 0..65536 {
        vec.pop::<T, A>();
    }

    prop_assert!(vec.is_empty());

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $max_length:expr, $alloc_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_pop_empty1(alloc in super::$alloc_gen::<alloc::Global>()) {
                    let alloc: alloc::Global = alloc;
                    super::prop_pop_empty1::<$typ, alloc::Global>(alloc)?
                }

                #[test]
                fn prop_pop_empty2(alloc in super::$alloc_gen::<alloc::Global>()) {
                    let alloc: alloc::Global = alloc;
                    super::prop_pop_empty2::<$typ, alloc::Global>(alloc)?
                }

                #[test]
                fn prop_pop_empty_is_empty1(alloc in super::$alloc_gen::<alloc::Global>()) {
                    let alloc: alloc::Global = alloc;
                    super::prop_pop_empty_is_empty1::<$typ, alloc::Global>(alloc)?
                }

                #[test]
                fn prop_pop_empty_is_empty2(alloc in super::$alloc_gen::<alloc::Global>()) {
                    let alloc: alloc::Global = alloc;
                    super::prop_pop_empty_is_empty2::<$typ, alloc::Global>(alloc)?
                }
            }
        }
    };
}

generate_props!(unit, (), 128, strategy_alloc);
generate_props!(u8, u8, 128, strategy_alloc);
generate_props!(u16, u16, 128, strategy_alloc);
generate_props!(u32, u32, 128, strategy_alloc);
generate_props!(u64, u64, 128, strategy_alloc);
generate_props!(usize, usize, 128, strategy_alloc);
generate_props!(string, String, 128, strategy_alloc);
