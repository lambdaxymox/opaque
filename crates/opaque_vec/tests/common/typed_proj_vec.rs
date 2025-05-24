use opaque_vec::TypedProjVec;
use std::{alloc, any};

pub fn from_slice_in<T, A>(values: &[T], alloc: A) -> TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let mut vec = TypedProjVec::new_in(alloc);
    for value in values.iter() {
        vec.push(value.clone());
    }

    vec
}

pub fn new_in<T, A>(alloc: A) -> TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    from_slice_in::<T, A>(&[], alloc)
}
