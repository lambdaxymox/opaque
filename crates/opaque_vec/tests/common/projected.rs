use opaque_vec::TypeProjectedVec;

use core::any;
use core::fmt;
use core::ops;
use std::string::{
    String,
    ToString,
};

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

pub fn shift_insert_slice<T, A>(values: &[T], slice: &[T], start: usize, alloc: A) -> TypeProjectedVec<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypeProjectedVec::new_in(alloc);
    for value in values.iter().cloned() {
        vec.push(value);
    }

    for (i, value) in slice.iter().cloned().enumerate() {
        vec.shift_insert(start + i, value);
    }

    vec
}

pub trait SingleBoundedValue: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary {
    fn bounded_any() -> impl Strategy<Value = Self>;
}

impl SingleBoundedValue for () {
    fn bounded_any() -> impl Strategy<Value = Self> {
        any::<()>()
    }
}
impl SingleBoundedValue for u8 {
    fn bounded_any() -> impl Strategy<Value = Self> {
        any::<u8>()
    }
}
impl SingleBoundedValue for u16 {
    fn bounded_any() -> impl Strategy<Value = Self> {
        any::<u16>()
    }
}
impl SingleBoundedValue for u32 {
    fn bounded_any() -> impl Strategy<Value = Self> {
        any::<u32>()
    }
}
impl SingleBoundedValue for u64 {
    fn bounded_any() -> impl Strategy<Value = Self> {
        any::<u64>()
    }
}
impl SingleBoundedValue for usize {
    fn bounded_any() -> impl Strategy<Value = Self> {
        any::<usize>()
    }
}
impl SingleBoundedValue for String {
    fn bounded_any() -> impl Strategy<Value = Self> {
        any::<usize>().prop_map(|value| value.to_string())
    }
}

pub fn strategy_bounded_value<T>() -> impl Strategy<Value = T>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
{
    <T as SingleBoundedValue>::bounded_any()
}

pub fn strategy_array<T, const N: usize>() -> impl Strategy<Value = [T; N]>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
{
    prop::array::uniform(strategy_bounded_value::<T>())
}

pub fn strategy_alloc<A>() -> impl Strategy<Value = A>
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    Just(A::default())
}

pub fn strategy_type_projected_vec_len<T, A>(length: usize) -> impl Strategy<Value = TypeProjectedVec<T, A>>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (
        proptest::collection::vec(strategy_bounded_value::<T>(), length),
        strategy_alloc::<A>(),
    )
        .prop_map(move |(values, alloc)| {
            let mut opaque_vec = TypeProjectedVec::new_in(alloc);
            opaque_vec.extend(values);

            opaque_vec
        })
}

pub fn strategy_type_projected_vec_max_len<T, A>(max_length: usize) -> impl Strategy<Value = TypeProjectedVec<T, A>>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (0..=max_length).prop_flat_map(move |length| strategy_type_projected_vec_len(length))
}

pub fn strategy_type_projected_vec_max_len_nonempty<T, A>(max_length: usize) -> impl Strategy<Value = TypeProjectedVec<T, A>>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn clamped_interval(max_length: usize) -> ops::RangeInclusive<usize> {
        if max_length == 0 { 1..=1 } else { 1..=max_length }
    }

    clamped_interval(max_length).prop_flat_map(move |length| strategy_type_projected_vec_len(length))
}
