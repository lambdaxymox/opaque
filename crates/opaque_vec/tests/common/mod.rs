use opaque_vec::OpaqueVec;
use std::{alloc, any};

pub fn from_slice_in<T, A>(values: &[T], alloc: A) -> OpaqueVec
where
    T: any::Any + Clone,
    A: alloc::Allocator + any::Any,
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
    A: alloc::Allocator + any::Any,
{
    from_slice_in::<T, A>(&[], alloc)
}
