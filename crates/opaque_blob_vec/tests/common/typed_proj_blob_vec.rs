use core::alloc::Layout;
use core::any;
use core::fmt;
use core::ptr::NonNull;
use std::alloc;

use opaque_alloc::TypedProjAlloc;
use opaque_blob_vec::TypedProjBlobVec;

unsafe fn drop_fn<T>(value: NonNull<u8>)
where
    T: any::Any + fmt::Debug,
{
    {
        let value_ref: &T = &*value.cast::<T>().as_ptr();

        eprintln!("Dropping value `{:?}` at memory location: `{:?}`", value_ref, value);
    }

    let to_drop = value.as_ptr() as *mut T;

    core::ptr::drop_in_place(to_drop)
}

pub(crate) fn new_in<T, A>(alloc: A) -> TypedProjBlobVec<A>
where
    T: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator,
{
    let alloc = TypedProjAlloc::new(alloc);
    let element_layout = Layout::new::<T>();
    let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));

    TypedProjBlobVec::new_in(alloc, element_layout, drop_fn)
}

pub(crate) fn from_slice_in<T, A>(values: &[T], alloc: A) -> TypedProjBlobVec<A>
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator,
{
    let mut vec = new_in::<T, A>(alloc);
    for value in values.iter() {
        let value_ptr: NonNull<u8> = NonNull::from(value).cast::<u8>();
        vec.push(value_ptr);
    }

    vec
}

pub(crate) fn as_slice<T, A>(proj_blob_vec: &TypedProjBlobVec<A>) -> &[T]
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    let ptr = proj_blob_vec.as_ptr() as *const T;
    let len = proj_blob_vec.len();

    unsafe { core::slice::from_raw_parts(ptr, len) }
}

pub(crate) fn as_mut_slice<T, A>(proj_blob_vec: &mut TypedProjBlobVec<A>) -> &mut [T]
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    let ptr = proj_blob_vec.as_mut_ptr() as *mut T;
    let len = proj_blob_vec.len();

    unsafe { core::slice::from_raw_parts_mut(ptr, len) }
}

#[inline]
pub(crate) fn clone<T, A>(proj_blob_vec: &TypedProjBlobVec<A>) -> TypedProjBlobVec<A>
where
    T: any::Any + fmt::Debug + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    let new_alloc = {
        let proj_old_alloc = proj_blob_vec.allocator();
        Clone::clone(proj_old_alloc)
    };
    let new_element_layout = proj_blob_vec.element_layout();
    let new_capacity = proj_blob_vec.capacity();
    let new_drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));

    let new_proj_blob_vec = unsafe {
        let mut _new_proj_blob_vec = TypedProjBlobVec::with_capacity_in(new_capacity, new_alloc, new_element_layout, new_drop_fn);
        let length = proj_blob_vec.len();
        let data_ptr = NonNull::new_unchecked(proj_blob_vec.as_ptr() as *mut u8);
        _new_proj_blob_vec.append(data_ptr, length);
        _new_proj_blob_vec
    };

    new_proj_blob_vec
}
