pub mod opaque_vec {
    use opaque_vec::OpaqueVec;
    use std::{alloc, any};

    pub fn from_slice_in<T, A>(values: &[T], alloc: A) -> OpaqueVec
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator,
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
        A: any::Any + alloc::Allocator,
    {
        from_slice_in::<T, A>(&[], alloc)
    }
}

pub mod typed_proj_vec {
    use opaque_vec::TypedProjVec;
    use std::{alloc, any};

    pub fn from_slice_in<T, A>(values: &[T], alloc: A) -> TypedProjVec<T, A>
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator,
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
        A: any::Any + alloc::Allocator,
    {
        from_slice_in::<T, A>(&[], alloc)
    }
}
