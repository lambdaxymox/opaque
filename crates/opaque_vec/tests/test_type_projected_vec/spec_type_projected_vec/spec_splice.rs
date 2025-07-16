use crate::common::projected::{
    SingleBoundedValue,
    strategy_type_projected_vec_len,
};
use opaque_vec::TypeProjectedVec;

use core::any;
use core::fmt;
use core::ops;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn strategy_vec_range(max_length: usize) -> impl Strategy<Value = ops::Range<usize>> {
    Just(0..max_length)
}

fn strategy_prop_splice_equivalent_to_drain_and_shift_insert<T, A>(
    max_length: usize,
    max_splice_length: usize,
) -> impl Strategy<Value = (TypeProjectedVec<T, A>, ops::Range<usize>, TypeProjectedVec<T, A>)>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (1..=max_length, 0..=max_splice_length).prop_flat_map(move |(length, splice_length)| {
        let vec_gen = strategy_type_projected_vec_len(length);
        let range_gen = strategy_vec_range(length);
        let splice_vec_gen = strategy_type_projected_vec_len(splice_length);

        (vec_gen, range_gen, splice_vec_gen)
    })
}

fn prop_splice_equivalent_to_drain_and_shift_insert<T, A>(
    values: TypeProjectedVec<T, A>,
    range: ops::Range<usize>,
    splice_values: TypeProjectedVec<T, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let expected = {
        let mut vec = values.clone();
        vec.drain(range.clone());
        for i in 0..splice_values.len() {
            vec.shift_insert(range.start + i, splice_values[i].clone());
        }

        vec
    };
    let result = {
        let mut vec = values.clone();
        vec.splice(range.clone(), splice_values);

        vec
    };

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_splice_len<T, A>(
    values: TypeProjectedVec<T, A>,
    range: ops::Range<usize>,
    splice_values: TypeProjectedVec<T, A>,
) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let vec = {
        let mut vec = values.clone();
        vec.splice(range.clone(), splice_values.as_slice().iter().cloned());

        vec
    };
    let expected = values.len() - range.len() + splice_values.len();
    let result = vec.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $max_splice_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_splice_equivalent_to_drain_and_shift_insert(
                    (values, range, splice_values) in super::$vec_gen::<$typ, $alloc_typ>($max_length, $max_splice_length),
                ) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    let range: super::ops::Range<usize> = range;
                    let splice_values: super::TypeProjectedVec<$typ, $alloc_typ> = splice_values;
                    super::prop_splice_equivalent_to_drain_and_shift_insert(values, range, splice_values)?
                }

                #[test]
                fn prop_splice_len(
                    (values, range, splice_values) in super::$vec_gen::<$typ, $alloc_typ>($max_length, $max_splice_length),
                ) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    let range: super::ops::Range<usize> = range;
                    let splice_values: super::TypeProjectedVec<$typ, $alloc_typ> = splice_values;
                    super::prop_splice_len(values, range, splice_values)?
                }
            }
        }
    };
}

generate_props!(
    unit,
    (),
    alloc::Global,
    32,
    16,
    strategy_prop_splice_equivalent_to_drain_and_shift_insert
);
generate_props!(
    u8,
    u8,
    alloc::Global,
    32,
    16,
    strategy_prop_splice_equivalent_to_drain_and_shift_insert
);
generate_props!(
    u16,
    u16,
    alloc::Global,
    32,
    16,
    strategy_prop_splice_equivalent_to_drain_and_shift_insert
);
generate_props!(
    u32,
    u32,
    alloc::Global,
    32,
    16,
    strategy_prop_splice_equivalent_to_drain_and_shift_insert
);
generate_props!(
    u64,
    u64,
    alloc::Global,
    32,
    16,
    strategy_prop_splice_equivalent_to_drain_and_shift_insert
);
generate_props!(
    usize,
    usize,
    alloc::Global,
    32,
    16,
    strategy_prop_splice_equivalent_to_drain_and_shift_insert
);
generate_props!(
    string,
    String,
    alloc::Global,
    32,
    16,
    strategy_prop_splice_equivalent_to_drain_and_shift_insert
);
