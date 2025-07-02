use core::ptr::NonNull;

#[inline]
#[must_use]
pub const fn as_non_null_ptr<T>(slf: NonNull<[T]>) -> NonNull<T> {
    slf.cast()
}

#[inline]
#[must_use]
pub const fn as_mut_ptr<T>(slf: NonNull<[T]>) -> *mut T {
    as_non_null_ptr(slf).as_ptr()
}

