use core::alloc::Layout;
use core::fmt;
use core::ptr::NonNull;
use opaque_alloc::OpaqueAlloc;
use opaque_blob_vec::OpaqueBlobVec;

pub fn new_opaque_blob_vec<T>() -> OpaqueBlobVec
where
    T: fmt::Debug + 'static,
{
    unsafe fn drop_fn<T>(value: NonNull<u8>)
    where
        T: core::fmt::Debug + 'static,
    {
        {
            let value_ref: &T = &*value.cast::<T>().as_ptr();

            eprintln!("Dropping value `{:?}` at memory location: `{:?}`", value_ref, value);
        }

        let to_drop = value.as_ptr() as *mut T;

        core::ptr::drop_in_place(to_drop)
    }

    let alloc = OpaqueAlloc::new(std::alloc::Global);
    let element_layout = Layout::new::<T>();
    let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));

    OpaqueBlobVec::new_in(alloc, element_layout, drop_fn)
}

pub fn from_typed_slice<T>(values: &[T]) -> OpaqueBlobVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = new_opaque_blob_vec::<T>();
    for value in values.iter() {
        let value_ptr: NonNull<u8> = NonNull::from(value).cast::<u8>();
        vec.push(value_ptr);
    }

    vec
}

pub fn as_slice<T>(opaque_blob_vec: &OpaqueBlobVec) -> &[T]
where
    T: 'static,
{
    let ptr = opaque_blob_vec.as_ptr() as *const T;
    let len = opaque_blob_vec.len();

    unsafe { core::slice::from_raw_parts(ptr, len) }
}

pub fn as_mut_slice<T>(opaque_blob_vec: &mut OpaqueBlobVec) -> &mut [T] {
    let ptr = opaque_blob_vec.as_mut_ptr() as *mut T;
    let len = opaque_blob_vec.len();

    unsafe { core::slice::from_raw_parts_mut(ptr, len) }
}
