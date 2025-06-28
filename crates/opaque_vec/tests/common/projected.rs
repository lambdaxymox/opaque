use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use core::ops;
use std::alloc;

use proptest::prelude::*;

pub fn shift_insert_slice<T, A>(values: &[T], slice: &[T], start: usize, alloc: A) -> TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::new_in(alloc);
    for value in values.iter().cloned() {
        vec.push(value);
    }

    for (i, value) in slice.iter().cloned().enumerate() {
        vec.shift_insert(start + i, value);
    }

    vec
}

pub fn strategy_array<T, const N: usize>() -> impl Strategy<Value = [T; N]>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
{
    prop::array::uniform(any::<T>())
}

pub fn strategy_alloc<A>() -> impl Strategy<Value = A>
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    Just(A::default())
}

pub fn strategy_type_projected_vec_len<T, A>(length: usize) -> impl Strategy<Value = TypedProjVec<T, A>>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (proptest::collection::vec(any::<T>(), length), strategy_alloc::<A>())
        .prop_map(move |(values, alloc)| {
            let mut opaque_vec = TypedProjVec::new_in(alloc);
            opaque_vec.extend(values);

            opaque_vec
        })
}

pub fn strategy_type_projected_vec_max_len<T, A>(max_length: usize) -> impl Strategy<Value = TypedProjVec<T, A>>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (0..=max_length).prop_flat_map(move |length| strategy_type_projected_vec_len(length))
}

pub fn strategy_type_projected_vec_max_len_nonempty<T, A>(max_length: usize) -> impl Strategy<Value = TypedProjVec<T, A>>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn clamped_interval(max_length: usize) -> ops::RangeInclusive<usize> {
        if max_length == 0 {
            1..=1
        } else {
            1..=max_length
        }
    }

    clamped_interval(max_length).prop_flat_map(move |length| strategy_type_projected_vec_len(length))
}
