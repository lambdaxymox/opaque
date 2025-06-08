use opaque_vec::{OpaqueVec, TypedProjVec};
use std::{alloc, any};

pub fn from_slice_in<T, A>(values: &[T], alloc: A) -> OpaqueVec
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);
    for value in values.iter() {
        vec.push::<T, A>(value.clone());
    }

    vec
}

pub fn new_in<T, A>(alloc: A) -> OpaqueVec
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    from_slice_in::<T, A>(&[], alloc)
}

pub fn shift_insert_slice<T, A>(values: &[T], slice: &[T], start: usize, alloc: A) -> OpaqueVec
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);
    for value in values.iter().cloned() {
        vec.push::<T, A>(value);
    }

    for (i, value) in slice.iter().cloned().enumerate() {
        vec.shift_insert::<T, A>(start + i, value);
    }

    vec
}
