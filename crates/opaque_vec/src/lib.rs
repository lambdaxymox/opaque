#![deny(unsafe_op_in_unsafe_fn)]
#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(optimize_attribute)]
#![feature(slice_range)]
mod into_iter;
mod drain;
mod splice;
mod extract_if;
mod zst;

pub use crate::into_iter::*;
pub use crate::drain::*;
pub use crate::splice::*;
pub use crate::extract_if::*;

use core::any;
use core::cmp;
use core::hash;
use core::mem;
use core::ops;
use core::slice;
use core::fmt;
use core::ptr::NonNull;
use core::marker;
use std::mem::{
    ManuallyDrop,
    MaybeUninit,
};
use std::alloc;
use std::borrow;

use opaque_blob_vec::{OpaqueBlobVec, TypedProjBlobVec};
use opaque_alloc::TypedProjAlloc;
use opaque_error;

#[repr(C)]
struct TypedProjVecInner<T, A>
where
    A: any::Any + alloc::Allocator,
{
    data: TypedProjBlobVec<A>,
    element_type_id: any::TypeId,
    allocator_type_id: any::TypeId,
    _marker: marker::PhantomData<(T, A)>,
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new_proj_in(proj_alloc: TypedProjAlloc<A>) -> Self {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            unsafe {
                core::ptr::drop_in_place(to_drop)
            }
        }

        let element_layout = alloc::Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let data = TypedProjBlobVec::new_in(proj_alloc, element_layout, drop_fn);
        let element_type_id = any::TypeId::of::<T>();
        let allocator_type_id = any::TypeId::of::<A>();

        Self { data, element_type_id, allocator_type_id, _marker: marker::PhantomData, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;
            unsafe {
                core::ptr::drop_in_place(to_drop)
            }
        }

        let element_layout = alloc::Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let data = TypedProjBlobVec::with_capacity_in(capacity, proj_alloc, element_layout, drop_fn);
        let element_type_id = any::TypeId::of::<T>();
        let allocator_type_id = any::TypeId::of::<A>();

        Self { data, element_type_id, allocator_type_id, _marker: marker::PhantomData, }
    }

    #[inline]
    pub(crate) fn try_with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Result<Self, opaque_error::TryReserveError> {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            unsafe {
                core::ptr::drop_in_place(to_drop)
            }
        }

        let element_layout = alloc::Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let data = TypedProjBlobVec::try_with_capacity_in(capacity, proj_alloc, element_layout, drop_fn)?;
        let element_type_id = any::TypeId::of::<T>();
        let allocator_type_id = any::TypeId::of::<A>();

        Ok(Self { data, element_type_id, allocator_type_id, _marker: marker::PhantomData, })
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts_proj_in(ptr: *mut T, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;
            unsafe {
                core::ptr::drop_in_place(to_drop)
            }
        }

        let element_layout = alloc::Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let ptr_bytes = ptr.cast::<u8>();
        let data = unsafe {
            TypedProjBlobVec::from_raw_parts_in(ptr_bytes, length, capacity, proj_alloc, element_layout, drop_fn)
        };
        let element_type_id = any::TypeId::of::<T>();
        let allocator_type_id = any::TypeId::of::<A>();

        Self { data, element_type_id, allocator_type_id, _marker: marker::PhantomData, }
    }

    #[inline]
    pub(crate) unsafe fn from_parts_proj_in(ptr: NonNull<T>, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            unsafe {
                core::ptr::drop_in_place(to_drop)
            }
        }

        let element_layout = alloc::Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let ptr_bytes = ptr.cast::<u8>();
        let data = unsafe {
            TypedProjBlobVec::from_parts_in(ptr_bytes, length, capacity, proj_alloc, element_layout, drop_fn)
        };
        let element_type_id = any::TypeId::of::<T>();
        let allocator_type_id = any::TypeId::of::<A>();

        Self { data, element_type_id, allocator_type_id, _marker: marker::PhantomData, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new_in(alloc: A) -> Self {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::new_proj_in(proj_alloc)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::with_capacity_proj_in(capacity, proj_alloc)
    }

    #[inline]
    pub(crate) fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Self, opaque_error::TryReserveError> {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::try_with_capacity_proj_in(capacity, proj_alloc)
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts_in(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self {
        let proj_alloc = TypedProjAlloc::new(alloc);

        unsafe {
            Self::from_raw_parts_proj_in(ptr, length, capacity, proj_alloc)
        }
    }

    #[inline]
    pub(crate) unsafe fn from_parts_in(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self {
        let proj_alloc = TypedProjAlloc::new(alloc);

        unsafe {
            Self::from_parts_proj_in(ptr, length, capacity, proj_alloc)
        }
    }
}

impl<T> TypedProjVecInner<T, alloc::Global>
where
    T: any::Any,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new() -> Self {
        Self::new_in(alloc::Global)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, alloc::Global)
    }

    #[inline]
    pub(crate) fn try_with_capacity(capacity: usize) -> Result<Self, opaque_error::TryReserveError> {
        Self::try_with_capacity_in(capacity, alloc::Global)
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        unsafe {
            Self::from_raw_parts_in(ptr, length, capacity, alloc::Global)
        }
    }

    #[inline]
    pub(crate) unsafe fn from_parts(ptr: NonNull<T>, length: usize, capacity: usize) -> Self {
        unsafe {
            Self::from_parts_in(ptr, length, capacity, alloc::Global)
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub(crate) const fn capacity(&self) -> usize {
        self.data.capacity()
    }

    #[inline]
    pub(crate) const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub(crate) const fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub(crate) fn as_byte_slice(&self) -> &[u8] {
        self.data.as_byte_slice()
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub(crate) fn allocator(&self) -> &TypedProjAlloc<A> {
        self.data.allocator()
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    unsafe fn set_len(&mut self, new_len: usize) {
        self.data.set_len(new_len);
    }

    #[inline]
    pub(crate) fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    #[inline]
    pub(crate) fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    #[inline]
    #[must_use]
    pub(crate) fn get_unchecked(&self, index: usize) -> &T {
        let ptr = self.data.get_unchecked(index);

        // SAFETY:
        // (1) The size of T matches the expected element size.
        // (2) We assume that the caller has ensured that `index` is within bounds.
        unsafe { &*ptr.as_ptr().cast::<T>() }
    }

    #[inline]
    #[must_use]
    pub(crate) fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        let ptr = self.data.get_mut_unchecked(index);

        // SAFETY:
        // (1) The size of T matches the expected element size.
        // (2) We assume that the caller has ensured that `index` is within bounds.
        unsafe { &mut *ptr.as_ptr().cast::<T>() }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn push(&mut self, value: T) {
        let mut me = ManuallyDrop::new(value);
        let value_ptr = unsafe { NonNull::new_unchecked(&mut *me as *mut T as *mut u8) };

        self.data.push(value_ptr);
    }

    #[inline]
    pub(crate) fn pop(&mut self) -> Option<T> {
        if self.data.len() == 0 {
            None
        } else {
            let last_value = unsafe {
                let last_index = self.data.len() - 1;
                let last_value_ptr = self.data.swap_remove_forget_unchecked(last_index);
                let _last_value = last_value_ptr.cast::<T>().read();

                _last_value
            };

            Some(last_value)
        }
    }

    #[inline]
    pub(crate) fn replace_insert(&mut self, index: usize, value: T) {
        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("replace_insert index out of bounds: Got index `{index}`. Need index `{index}` <= len, where len is `{length}`.");
        }

        let length = self.len();
        if index > length {
            index_out_of_bounds_failure(index, length);
        }

        let mut me = ManuallyDrop::new(value);
        let value_ptr = unsafe { NonNull::new_unchecked(&mut *me as *mut T as *mut u8) };

        self.data.replace_insert(index, value_ptr);
    }

    #[inline]
    pub(crate) fn shift_insert(&mut self, index: usize, value: T) {
        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("shift_insert index out of bounds: Got index `{index}`. Need index `{index}` <= len, where len is `{length}`.");
        }

        let length = self.len();
        if index > length {
            index_out_of_bounds_failure(index, length);
        }

        let mut me = ManuallyDrop::new(value);
        let value_ptr = unsafe { NonNull::new_unchecked(&mut *me as *mut T as *mut u8) };

        self.data.shift_insert(index, value_ptr);
    }

    #[inline]
    pub(crate) fn swap_remove(&mut self, index: usize) -> T {
        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("swap_remove index out of bounds: Got `{index}`, length is `{length}`.");
        }

        let length = self.len();
        if index >= length {
            index_out_of_bounds_failure(index, length);
        }

        // index < self.len()
        let value = unsafe {
            let ptr = self.data.get_unchecked(index);
            let _value = ptr.cast::<T>().read();
            _value
        };

        let _ = self.data.swap_remove_forget_unchecked(index);

        value
    }

    #[inline]
    pub(crate) fn shift_remove(&mut self, index: usize) -> T {
        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("shift_remove index out of bounds: Got `{index}`, length is `{length}`.");
        }

        let length = self.len();
        if index >= length {
            index_out_of_bounds_failure(index, length);
        }

        // index < self.len()
        let value = unsafe {
            let ptr = self.data.get_unchecked(index);
            let _value = ptr.cast::<T>().read();
            _value
        };

        // SAFETY:
        let _ = self.data.shift_remove_forget_unchecked(index);

        value
    }

    #[inline]
    pub(crate) fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.as_slice().contains(value)
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.data.as_ptr() as *const T
    }

    #[inline]
    pub(crate) const fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr() as *mut T
    }

    #[inline]
    pub(crate) const fn as_non_null(&mut self) -> NonNull<T> {
        // SAFETY: A [`TypedProjVecInner`] always holds a non-null pointer.
        self.data.as_non_null().cast::<T>()
    }

    #[inline]
    pub(crate) fn as_slice(&self) -> &[T] {
        unsafe {
            let data_ptr = self.data.as_ptr() as *const T;
            let len = self.data.len();

            slice::from_raw_parts(data_ptr, len)
        }
    }

    #[inline]
    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            let data_ptr = self.data.as_mut_ptr() as *mut T;
            let len = self.data.len();

            slice::from_raw_parts_mut(data_ptr, len)
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_raw_parts(self) -> (*mut T, usize, usize) {
        let mut me = ManuallyDrop::new(self);
        let ptr = me.as_mut_ptr();
        let len = me.len();
        let capacity = me.capacity();

        (ptr, len, capacity)
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_parts(self) -> (NonNull<T>, usize, usize) {
        let mut me = ManuallyDrop::new(self);

        // SAFETY: An `OpaqueVec` always has a non-null pointer.
        let ptr = unsafe { NonNull::new_unchecked(me.as_mut_ptr()) };
        let len = me.len();
        let capacity = me.capacity();

        (ptr, len, capacity)
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_raw_parts_with_alloc(self) -> (*mut T, usize, usize, TypedProjAlloc<A>)
    where
        A: alloc::Allocator,
    {
        let mut me = ManuallyDrop::new(self);
        let ptr = me.as_mut_ptr();
        let len = me.len();
        let capacity = me.capacity();
        let alloc = unsafe { core::ptr::read(me.allocator()) };

        (ptr, len, capacity, alloc)
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_parts_with_alloc(self) -> (NonNull<T>, usize, usize, TypedProjAlloc<A>)
    where
        A: alloc::Allocator,
    {
        let mut me = ManuallyDrop::new(self);

        // SAFETY: An `OpaqueVec` always has a non-null pointer.
        let ptr = unsafe { NonNull::new_unchecked(me.as_mut_ptr()) };
        let len = me.len();
        let capacity = me.capacity();
        let alloc = unsafe { core::ptr::read(me.allocator()) };

        (ptr, len, capacity, alloc)
    }

    #[inline]
    pub(crate) fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        unsafe {
            let ptr = self.as_mut_ptr().add(self.len()) as *mut MaybeUninit<T>;
            let len = self.capacity() - self.len();

            slice::from_raw_parts_mut(ptr, len)
        }
    }

    pub(crate) fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
    where
        A: alloc::Allocator,
        R: ops::RangeBounds<usize>,
    {
        // Memory safety
        //
        // When the Drain is first created, it shortens the length of
        // the source vector to make sure no uninitialized or moved-from elements
        // are accessible at all if the Drain's destructor never gets to run.
        //
        // Drain will ptr::read out the values to remove.
        // When finished, remaining tail of the vec is copied back to cover
        // the hole, and the vector length is restored to the new length.
        //
        let len = self.len();
        let ops::Range { start, end } = core::slice::range(range, ..len);

        unsafe {
            // set self.vec length's to start, to be safe in case Drain is leaked
            self.set_len(start);
            let range_slice = slice::from_raw_parts(self.as_ptr().add(start), end - start);

            Drain::from_parts(end, len - end, range_slice.iter(), NonNull::from(self))
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    #[must_use]
    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        if index >= self.data.len() {
            return None;
        }

        let ptr = self.get_unchecked(index);

        Some(ptr)
    }

    #[inline]
    #[must_use]
    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.data.len() {
            return None;
        }

        let ptr = self.get_mut_unchecked(index);

        Some(ptr)
    }

    #[inline]
    pub(crate) fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
        if self.data.len() == self.data.capacity() {
            return Err(value);
        }

        self.push(value);

        Ok(())
    }

    #[inline]
    #[track_caller]
    pub(crate) fn append(&mut self, other: &mut Self) {
        unsafe {
            let ptr = NonNull::new_unchecked(other.as_mut_slice().as_mut_ptr().cast::<u8>());
            let count = other.len();

            self.data.append(ptr, count);
            other.set_len(0);
        }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn into_boxed_slice(mut self) -> Box<[T], TypedProjAlloc<A>>
    where
        A: alloc::Allocator,
    {
        unsafe {
            self.shrink_to_fit();
            let mut me = ManuallyDrop::new(self);
            let len = me.len();
            let ptr = me.as_mut_ptr();
            let slice_ptr = std::ptr::slice_from_raw_parts_mut(ptr, len);
            let alloc = core::ptr::read(me.allocator());

            Box::from_raw_in(slice_ptr, alloc)
        }
    }

    #[inline]
    pub(crate) fn split_off(&mut self, at: usize) -> Self
    where
        A: alloc::Allocator + Clone,
    {
        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds(at: usize, len: usize) -> ! {
            panic!("`at` split index (is {at}) should be <= len (is {len})");
        }

        if at > self.len() {
            index_out_of_bounds(at, self.len());
        }

        let other_len = self.len() - at;
        let mut other = {
            let cloned_alloc = self.allocator().clone();
            let box_alloc = cloned_alloc.into_boxed_alloc();
            let split_alloc = TypedProjAlloc::from_boxed_alloc(box_alloc);

            TypedProjVecInner::with_capacity_proj_in(other_len, split_alloc)
        };

        // Unsafely `set_len` and copy items to `other`.
        unsafe {
            self.set_len(at);
            other.set_len(other_len);

            core::ptr::copy_nonoverlapping(self.as_ptr().add(at), other.as_mut_ptr(), other.len());
        }

        other
    }

    #[inline]
    pub(crate) fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        A: alloc::Allocator,
        F: FnMut() -> T,
    {
        let len = self.len();
        if new_len > len {
            self.extend::<_>(core::iter::repeat_with(f).take(new_len - len));
        } else {
            self.truncate(new_len);
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub(crate) fn try_reserve(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.data.try_reserve(additional)
    }

    #[inline]
    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.data.try_reserve_exact(additional)
    }

    #[inline]
    #[track_caller]
    pub(crate) fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    #[inline]
    #[track_caller]
    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }

    #[inline]
    #[track_caller]
    pub(crate) fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    #[inline]
    #[track_caller]
    pub(crate) fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity);
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        self.data.clear();
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub(crate) fn extend_with(&mut self, count: usize, value: T)
    where
        T: Clone,
    {
        let value_ptr = unsafe { NonNull::new_unchecked(&value as *const T as *mut T as *mut u8) };

        self.data.extend_with(count, value_ptr);
    }

    #[inline]
    pub(crate) fn extend_from_iter<I>(&mut self, iterator: I)
    where
        T: Clone,
        I: Iterator<Item = T>,
    {
        for item in iterator {
            self.push(item);
        }
    }

    #[inline]
    pub(crate) fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.extend_from_iter::<_>(other.iter().cloned())
    }

    #[inline]
    pub(crate) fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        let len = self.len();

        if new_len > len {
            self.extend_with(new_len - len, value)
        } else {
            self.truncate(new_len);
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub(crate) fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.retain_mut::<_>(|elem| f(elem));
    }

    pub(crate) fn retain_mut<F>(&mut self, mut f: F)
    where
        A: alloc::Allocator,
        F: FnMut(&mut T) -> bool,
    {
        let original_len = self.len();

        if original_len == 0 {
            // Empty case: explicit return allows better optimization, vs letting compiler infer it
            return;
        }

        // Avoid double drop if the drop guard is not executed,
        // since we may make some holes during the process.
        unsafe { self.set_len(0) };

        // Vec: [Kept, Kept, Hole, Hole, Hole, Hole, Unchecked, Unchecked]
        //      |<-              processed len   ->| ^- next to check
        //                  |<-  deleted cnt     ->|
        //      |<-              original_len                          ->|
        // Kept: Elements which predicate returns true on.
        // Hole: Moved or dropped element slot.
        // Unchecked: Unchecked valid elements.
        //
        // This drop guard will be invoked when predicate or `drop` of element panicked.
        // It shifts unchecked elements to cover holes and `set_len` to the correct length.
        // In cases when predicate and `drop` never panic, it will be optimized out.
        struct BackshiftOnDrop<'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator,
        {
            v: &'a mut TypedProjVecInner<T, A>,
            processed_len: usize,
            deleted_cnt: usize,
            original_len: usize,
        }

        impl<T, A> Drop for BackshiftOnDrop<'_, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator,
        {
            fn drop(&mut self) {
                if self.deleted_cnt > 0 {
                    // SAFETY: Trailing unchecked items must be valid since we never touch them.
                    unsafe {
                        core::ptr::copy(
                            self.v.as_ptr().add(self.processed_len),
                            self.v.as_mut_ptr().add(self.processed_len - self.deleted_cnt),
                            self.original_len - self.processed_len,
                        );
                    }
                }
                // SAFETY: After filling holes, all items are in contiguous memory.
                unsafe {
                    self.v.set_len(self.original_len - self.deleted_cnt);
                }
            }
        }

        let mut g = BackshiftOnDrop {
            v: self,
            processed_len: 0,
            deleted_cnt: 0,
            original_len,
        };

        fn process_loop<F, T, A, const DELETED: bool>(original_len: usize, f: &mut F, g: &mut BackshiftOnDrop<'_, T, A>)
        where
            T: any::Any,
            A: any::Any + alloc::Allocator,
            F: FnMut(&mut T) -> bool,
        {
            while g.processed_len != original_len {
                // SAFETY: Unchecked element must be valid.
                let cur = unsafe { &mut *g.v.as_mut_ptr().add(g.processed_len) };
                if !f(cur) {
                    // Advance early to avoid double drop if `drop_in_place` panicked.
                    g.processed_len += 1;
                    g.deleted_cnt += 1;
                    // SAFETY: We never touch this element again after dropped.
                    unsafe { core::ptr::drop_in_place(cur) };
                    // We already advanced the counter.
                    if DELETED {
                        continue;
                    } else {
                        break;
                    }
                }
                if DELETED {
                    // SAFETY: `deleted_cnt` > 0, so the hole slot must not overlap with current element.
                    // We use copy for move, and never touch this element again.
                    unsafe {
                        let hole_slot = g.v.as_mut_ptr().add(g.processed_len - g.deleted_cnt);
                        core::ptr::copy_nonoverlapping(cur, hole_slot, 1);
                    }
                }
                g.processed_len += 1;
            }
        }

        // Stage 1: Nothing was deleted.
        process_loop::<F, T, A, false>(original_len, &mut f, &mut g);

        // Stage 2: Some elements were deleted.
        process_loop::<F, T, A, true>(original_len, &mut f, &mut g);

        // All item are processed. This can be optimized to `set_len` by LLVM.
        drop(g);
    }

    pub(crate) fn dedup_by<F>(&mut self, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        let len = self.len();
        if len <= 1 {
            return;
        }

        // Check if we ever want to remove anything.
        // This allows to use copy_non_overlapping in next cycle.
        // And avoids any memory writes if we don't need to remove anything.
        let mut first_duplicate_idx: usize = 1;
        let start = self.as_mut_ptr();
        while first_duplicate_idx != len {
            let found_duplicate = unsafe {
                // SAFETY: first_duplicate always in range [1..len)
                // Note that we start iteration from 1 so we never overflow.
                let prev = start.add(first_duplicate_idx.wrapping_sub(1));
                let current = start.add(first_duplicate_idx);
                // We explicitly say in docs that references are reversed.
                same_bucket(&mut *current, &mut *prev)
            };
            if found_duplicate {
                break;
            }
            first_duplicate_idx += 1;
        }
        // Don't need to remove anything.
        // We cannot get bigger than len.
        if first_duplicate_idx == len {
            return;
        }

        /* INVARIANT: vec.len() > read > write > write-1 >= 0 */
        struct FillGapOnDrop<'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator,
        {
            /* Offset of the element we want to check if it is duplicate */
            read: usize,

            /* Offset of the place where we want to place the non-duplicate
             * when we find it. */
            write: usize,

            /* The Vec that would need correction if `same_bucket` panicked */
            vec: &'a mut TypedProjVecInner<T, A>,
        }

        impl<'a, T, A> Drop for FillGapOnDrop<'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator,
        {
            fn drop(&mut self) {
                /* This code gets executed when `same_bucket` panics */

                /* SAFETY: invariant guarantees that `read - write`
                 * and `len - read` never overflow and that the copy is always
                 * in-bounds. */
                unsafe {
                    let ptr = self.vec.as_mut_ptr();
                    let len = self.vec.len();

                    /* How many items were left when `same_bucket` panicked.
                     * Basically vec[read..].len() */
                    let items_left = len.wrapping_sub(self.read);

                    /* Pointer to first item in vec[write..write+items_left] slice */
                    let dropped_ptr = ptr.add(self.write);
                    /* Pointer to first item in vec[read..] slice */
                    let valid_ptr = ptr.add(self.read);

                    /* Copy `vec[read..]` to `vec[write..write+items_left]`.
                     * The slices can overlap, so `copy_nonoverlapping` cannot be used */
                    core::ptr::copy(valid_ptr, dropped_ptr, items_left);

                    /* How many items have been already dropped
                     * Basically vec[read..write].len() */
                    let dropped = self.read.wrapping_sub(self.write);

                    self.vec.set_len(len - dropped);
                }
            }
        }

        /* Drop items while going through Vec, it should be more efficient than
         * doing slice partition_dedup + truncate */

        // Construct gap first and then drop item to avoid memory corruption if `T::drop` panics.
        let mut gap: FillGapOnDrop<'_, T, A> = FillGapOnDrop {
            read: first_duplicate_idx + 1,
            write: first_duplicate_idx,
            vec: self,
        };

        unsafe {
            // SAFETY: we checked that first_duplicate_idx in bounds before.
            // If drop panics, `gap` would remove this item without drop.
            core::ptr::drop_in_place(start.add(first_duplicate_idx));
        }

        /* SAFETY: Because of the invariant, read_ptr, prev_ptr and write_ptr
         * are always in-bounds and read_ptr never aliases prev_ptr */
        unsafe {
            while gap.read < len {
                let read_ptr = start.add(gap.read);
                let prev_ptr = start.add(gap.write.wrapping_sub(1));

                // We explicitly say in docs that references are reversed.
                let found_duplicate = same_bucket(&mut *read_ptr, &mut *prev_ptr);
                if found_duplicate {
                    // Increase `gap.read` now since the drop may panic.
                    gap.read += 1;
                    /* We have found duplicate, drop it in-place */
                    core::ptr::drop_in_place(read_ptr);
                } else {
                    let write_ptr = start.add(gap.write);

                    /* read_ptr cannot be equal to write_ptr because at this point
                     * we guaranteed to skip at least one element (before loop starts).
                     */
                    core::ptr::copy_nonoverlapping(read_ptr, write_ptr, 1);

                    /* We have filled that place, so go further */
                    gap.write += 1;
                    gap.read += 1;
                }
            }

            /* Technically we could let `gap` clean up with its Drop, but
             * when `same_bucket` is guaranteed to not panic, this bloats a little
             * the codegen, so we just do it manually */
            gap.vec.set_len(gap.write);
            mem::forget(gap);
        }
    }

    #[inline]
    pub(crate) fn dedup_by_key<F, K>(&mut self, mut key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.dedup_by::<_>(|a, b| key(a) == key(b))
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub(crate) fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
    where
        A: alloc::Allocator,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item=T>,
    {
        Splice::new(self.drain(range), replace_with.into_iter())
    }

    #[inline]
    pub(crate) fn extract_if<F, R>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, A>
    where
        A: alloc::Allocator,
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        ExtractIf::new(self, filter, range)
    }

    #[inline]
    pub(crate) fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }
}

impl<T, A> Extend<T> for TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter.into_iter() {
            self.push(item);
        }
    }
}

impl<'a, T, A> Extend<&'a T> for TypedProjVecInner<T, A>
where
    T: any::Any + Copy,
    A: any::Any + alloc::Allocator,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        for item in iter.into_iter() {
            self.push(*item);
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub(crate) fn clone(&self) -> Self
    where
        T: Clone,
        A: Clone,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            unsafe {
                core::ptr::drop_in_place(to_drop)
            }
        }

        let new_data = {
            let new_alloc = {
                let proj_old_alloc = self.data.allocator();
                Clone::clone(proj_old_alloc)
            };
            let new_element_layout = self.data.element_layout();
            let new_capacity = self.data.capacity();
            let new_drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
            let new_data = unsafe {
                let mut _new_data = TypedProjBlobVec::with_capacity_in(new_capacity, new_alloc, new_element_layout, new_drop_fn);
                let length = self.data.len();
                let old_data_ptr = NonNull::new_unchecked(self.data.as_ptr() as *mut u8);
                _new_data.append(old_data_ptr, length);
                _new_data
            };

            new_data
        };
        let new_type_id = self.element_type_id;
        let new_alloc_type_id = self.allocator_type_id;

        Self {
            data: new_data,
            element_type_id: new_type_id,
            allocator_type_id: new_alloc_type_id,
            _marker: marker::PhantomData,
        }
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    #[inline]
    pub(crate) fn from_slice_in(slice: &[T], alloc: A) -> TypedProjVecInner<T, A> {
        struct DropGuard<'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Clone,
        {
            vec: &'a mut TypedProjVecInner<T, A>,
            num_init: usize,
        }

        impl<'a, T, A> Drop for DropGuard<'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Clone,
        {
            #[inline]
            fn drop(&mut self) {
                // SAFETY:
                // items were marked initialized in the loop below
                unsafe {
                    self.vec.set_len(self.num_init);
                }
            }
        }

        let mut vec: TypedProjVecInner<T, A> = TypedProjVecInner::with_capacity_in(slice.len(), alloc);
        let mut guard = DropGuard {
            vec: &mut vec,
            num_init: 0,
        };
        let slots = guard.vec.spare_capacity_mut();
        // .take(slots.len()) is necessary for LLVM to remove bounds checks
        // and has better codegen than zip.
        for (i, b) in slice.iter().enumerate().take(slots.len()) {
            guard.num_init = i;
            slots[i].write(b.clone());
        }

        mem::forget(guard);

        // SAFETY:
        // the vec was allocated and initialized above to at least this length.
        unsafe {
            vec.set_len(slice.len());
        }

        vec
    }
}

impl<T, A> TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub(crate) fn from_boxed_slice(box_slice: Box<[T], A>) -> TypedProjVecInner<T, A> {
        let length = box_slice.len();
        let capacity = box_slice.len();
        let (ptr, alloc) = {
            let (slice_ptr, _alloc) = Box::into_non_null_with_allocator(box_slice);
            let _ptr: NonNull<T> = unsafe { NonNull::new_unchecked(slice_ptr.as_ptr() as *mut T) };
            (_ptr, _alloc)
        };
        let vec = unsafe {
            TypedProjVecInner::from_parts_in(ptr, length, capacity, alloc)
        };

        vec
    }
}

impl<T> From<&[T]> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any + Clone,
{
    fn from(slice: &[T]) -> Self {
        Self::from_slice_in(slice, alloc::Global::default())
    }
}

impl<T> From<&mut [T]> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any + Clone,
{
    fn from(slice: &mut [T]) -> Self {
        Self::from_slice_in(slice, alloc::Global::default())
    }
}

impl<const N: usize, T> From<&[T; N]> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any + Clone,
{
    fn from(array: &[T; N]) -> Self {
        Self::from(array.as_slice())
    }
}

impl<const N: usize, T> From<&mut [T; N]> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any + Clone,
{
    fn from(array: &mut [T; N]) -> Self {
        Self::from(array.as_mut_slice())
    }
}

impl<T, A> From<Vec<T, A>> for TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn from(vec: Vec<T, A>) -> Self {
        let (ptr, length, capacity, alloc) = vec.into_parts_with_alloc();
        let inner = unsafe {
            TypedProjVecInner::from_parts_in(ptr, length, capacity, alloc)
        };

        inner
    }
}

impl<T, A> From<&Vec<T, A>> for TypedProjVecInner<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    fn from(vec: &Vec<T, A>) -> Self {
        Self::from(vec.clone())
    }
}

impl<T, A> From<&mut Vec<T, A>> for TypedProjVecInner<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    fn from(vec: &mut Vec<T, A>) -> Self {
        Self::from(vec.clone())
    }
}

impl<T, A> From<Box<[T], A>> for TypedProjVecInner<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn from(slice: Box<[T], A>) -> Self {
        Self::from_boxed_slice(slice)
    }
}

impl<const N: usize, T> From<[T; N]> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any,
{
    fn from(array: [T; N]) -> Self {
        Self::from_boxed_slice(Box::new(array))
    }
}

impl<T> FromIterator<T> for TypedProjVecInner<T, alloc::Global>
where
    T: any::Any,
{
    #[inline]
    #[track_caller]
    fn from_iter<I>(iter: I) -> TypedProjVecInner<T, alloc::Global>
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();

        let mut vec = TypedProjVecInner::with_capacity(lower);

        for item in iter {
            vec.push(item);
        }

        vec
    }
}

#[repr(C)]
struct OpaqueVecInner {
    data: OpaqueBlobVec,
    element_type_id: any::TypeId,
    allocator_type_id: any::TypeId,
}

impl OpaqueVecInner {
    #[inline(always)]
    pub(crate) fn as_proj_assuming_type<T, A>(&self) -> &TypedProjVecInner<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        debug_assert_eq!(self.element_type_id, any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id, any::TypeId::of::<A>());

        unsafe { &*(self as *const OpaqueVecInner as *const TypedProjVecInner<T, A>) }
    }

    #[inline(always)]
    pub(crate) fn as_proj_mut_assuming_type<T, A>(&mut self) -> &mut TypedProjVecInner<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        debug_assert_eq!(self.element_type_id, any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id, any::TypeId::of::<A>());

        unsafe { &mut *(self as *mut OpaqueVecInner as *mut TypedProjVecInner<T, A>) }
    }

    #[inline(always)]
    pub(crate) fn into_proj_assuming_type<T, A>(self) -> TypedProjVecInner<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        debug_assert_eq!(self.element_type_id, any::TypeId::of::<T>());
        debug_assert_eq!(self.allocator_type_id, any::TypeId::of::<A>());

        TypedProjVecInner {
            data: self.data.into_proj::<A>(),
            element_type_id: self.element_type_id,
            allocator_type_id: self.allocator_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) fn from_proj<T, A>(proj_self: TypedProjVecInner<T, A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        Self {
            data: OpaqueBlobVec::from_proj(proj_self.data),
            element_type_id: proj_self.element_type_id,
            allocator_type_id: proj_self.allocator_type_id,
        }
    }
}

impl OpaqueVecInner {
    #[inline]
    pub(crate) const fn element_type_id(&self) -> any::TypeId {
        self.element_type_id
    }

    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> any::TypeId {
        self.allocator_type_id
    }
}

impl OpaqueVecInner {
    #[inline]
    pub(crate) const fn element_layout(&self) -> alloc::Layout {
        self.data.element_layout()
    }

    #[inline]
    pub(crate) const fn capacity(&self) -> usize {
        self.data.capacity()
    }

    #[inline]
    pub(crate) const fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub(crate) const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub(crate) fn as_byte_slice(&self) -> &[u8] {
        self.data.as_byte_slice()
    }
}

#[repr(transparent)]
pub struct TypedProjVec<T, A = alloc::Global>
where
    A: any::Any + alloc::Allocator,
{
    inner: TypedProjVecInner<T, A>,
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_proj_in(proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = TypedProjVecInner::new_proj_in(proj_alloc);

        Self { inner, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = TypedProjVecInner::with_capacity_proj_in(capacity, proj_alloc);

        Self { inner, }
    }

    #[inline]
    pub fn try_with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Result<Self, opaque_error::TryReserveError> {
        let inner = TypedProjVecInner::try_with_capacity_proj_in(capacity, proj_alloc)?;

        Ok(Self { inner, })
    }

    #[inline]
    pub unsafe fn from_raw_parts_proj_in(ptr: *mut T, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_raw_parts_proj_in(ptr, length, capacity, proj_alloc)
        };

        Self { inner, }
    }

    #[inline]
    pub unsafe fn from_parts_proj_in(ptr: NonNull<T>, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_parts_proj_in(ptr, length, capacity, proj_alloc)
        };

        Self { inner, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_in(alloc: A) -> Self {
        let inner = TypedProjVecInner::new_in(alloc);

        Self { inner, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let inner = TypedProjVecInner::with_capacity_in(capacity, alloc);

        Self { inner, }
    }

    #[inline]
    pub fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Self, opaque_error::TryReserveError> {
        let inner = TypedProjVecInner::try_with_capacity_in(capacity, alloc)?;

        Ok(Self { inner, })
    }

    #[inline]
    pub unsafe fn from_raw_parts_in(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_raw_parts_in(ptr, length, capacity, alloc)
        };

        Self { inner, }
    }

    #[inline]
    pub unsafe fn from_parts_in(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_parts_in(ptr, length, capacity, alloc)
        };

        Self { inner, }
    }
}

impl<T> TypedProjVec<T, alloc::Global>
where
    T: any::Any,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new() -> Self {
        let inner = TypedProjVecInner::new();

        Self { inner, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity(capacity: usize) -> Self {
        let inner = TypedProjVecInner::with_capacity(capacity);

        Self { inner, }
    }

    #[inline]
    pub fn try_with_capacity(capacity: usize) -> Result<Self, opaque_error::TryReserveError> {
        let inner = TypedProjVecInner::try_with_capacity(capacity)?;

        Ok(Self { inner, })
    }

    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_raw_parts(ptr, length, capacity)
        };

        Self { inner, }
    }

    #[inline]
    pub unsafe fn from_parts(ptr: NonNull<T>, length: usize, capacity: usize) -> Self {
        let inner = unsafe {
            TypedProjVecInner::from_parts(ptr, length, capacity)
        };

        Self { inner, }
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_byte_slice(&self) -> &[u8] {
        self.inner.as_byte_slice()
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
        self.inner.allocator()
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    unsafe fn set_len(&mut self, new_len: usize) {
        unsafe {
            self.inner.set_len(new_len)
        }
    }

    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index)
    }

    #[inline]
    #[track_caller]
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    #[inline]
    pub fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
        self.inner.push_within_capacity(value)
    }

    #[track_caller]
    pub fn replace_insert(&mut self, index: usize, value: T) {
        self.inner.replace_insert(index, value);
    }

    #[track_caller]
    pub fn shift_insert(&mut self, index: usize, value: T) {
        self.inner.shift_insert(index, value);
    }

    #[track_caller]
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.inner.swap_remove(index)
    }

    #[track_caller]
    pub fn shift_remove(&mut self, index: usize) -> T {
        self.inner.shift_remove(index)
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.inner.contains(value)
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.inner.iter_mut()
    }

    #[inline]
    #[track_caller]
    pub fn append(&mut self, other: &mut Self) {
        self.inner.append(&mut other.inner)
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
    where
        R: ops::RangeBounds<usize>,
    {
        self.inner.drain(range)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.as_mut_ptr()
    }

    #[inline]
    pub fn as_non_null(&mut self) -> NonNull<T> {
        self.inner.as_non_null()
    }

    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }

    #[must_use]
    pub fn into_raw_parts(self) -> (*mut T, usize, usize) {
        self.inner.into_raw_parts()
    }

    #[must_use]
    pub fn into_parts(self) -> (NonNull<T>, usize, usize) {
        self.inner.into_parts()
    }

    #[must_use]
    pub fn into_raw_parts_with_alloc(self) -> (*mut T, usize, usize, TypedProjAlloc<A>) {
        self.inner.into_raw_parts_with_alloc()
    }

    #[must_use]
    pub fn into_parts_with_alloc(self) -> (NonNull<T>, usize, usize, TypedProjAlloc<A>) {
        self.inner.into_parts_with_alloc()
    }

    #[track_caller]
    pub fn into_boxed_slice(self) -> Box<[T], TypedProjAlloc<A>> {
        self.inner.into_boxed_slice()
    }

    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    #[track_caller]
    pub fn split_off(&mut self, at: usize) -> Self
    where
        A: Clone,
    {
        let inner = self.inner.split_off(at);

        Self { inner, }
    }

    #[track_caller]
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T,
    {
        self.inner.resize_with(new_len, f)
    }

    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        self.inner.spare_capacity_mut()
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.inner.try_reserve(additional)
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    #[track_caller]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    #[track_caller]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    #[track_caller]
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    #[track_caller]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
    where
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        self.inner.splice::<R, I>(range, replace_with)
    }

    pub fn extract_if<F, R>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, A>
    where
        T: any::Any,
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        self.inner.extract_if::<F, R>(range, filter)
    }

    /*
    #[track_caller]
    fn extend_with(&mut self, count: usize, value: T)
    where
        T: Clone,
    {
        self.inner.extend_with(count, value);
    }

    #[track_caller]
    fn extend_from_iter<I>(&mut self, iterator: I)
    where
        T: Clone,
        I: Iterator<Item = T>,
    {
        self.inner.extend_from_iter::<I>(iterator)
    }
    */

    #[track_caller]
    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.inner.extend_from_slice(other);
    }

    #[track_caller]
    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        self.inner.resize(new_len, value);
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.inner.retain(|elem| f(elem));
    }

    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.inner.retain_mut(f)
    }

    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.inner.dedup_by_key(key)
    }

    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.inner.dedup_by(same_bucket)
    }
}

impl<T, A> ops::Deref for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, A> ops::DerefMut for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

/*
unsafe impl<T, A> ops::DerefPure for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
}
*/

impl<T, A> Clone for TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    fn clone(&self) -> Self {
        let cloned_inner = self.inner.clone();

        Self {
            inner: cloned_inner,
        }
    }
}

impl<T, A> hash::Hash for TypedProjVec<T, A>
where
    T: any::Any + hash::Hash,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        hash::Hash::hash(self.as_slice(), state)
    }
}

impl<T, I, A> ops::Index<I> for TypedProjVec<T, A>
where
    T: any::Any,
    I: slice::SliceIndex<[T]>,
    A: any::Any + alloc::Allocator,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        ops::Index::index(self.as_slice(), index)
    }
}

impl<T, I, A> ops::IndexMut<I> for TypedProjVec<T, A>
where
    T: any::Any,
    I: slice::SliceIndex<[T]>,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        ops::IndexMut::index_mut(self.as_mut_slice(), index)
    }
}

impl<T> FromIterator<T> for TypedProjVec<T, alloc::Global>
where
    T: any::Any,
{
    #[inline]
    #[track_caller]
    fn from_iter<I>(iter: I) -> TypedProjVec<T, alloc::Global>
    where
        I: IntoIterator<Item = T>,
    {
        let inner = TypedProjVecInner::from_iter(iter);

        Self { inner, }
    }
}

impl<T, A> IntoIterator for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    type Item = T;
    type IntoIter = IntoIter<T, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let mut me = ManuallyDrop::new(self);
            let alloc = ManuallyDrop::new(core::ptr::read(me.allocator()));
            let inner = me.as_non_null();
            let begin = inner.as_ptr();
            let end = if zst::is_zst::<T>() {
                begin.wrapping_byte_add(me.len())
            } else {
                begin.add(me.len()) as *const T
            };
            let cap = me.inner.capacity();

            IntoIter::from_parts(inner, cap, alloc, inner, end)
        }
    }
}

impl<'a, T, A> IntoIterator for &'a TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, A> IntoIterator for &'a mut TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, A> Extend<T> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    #[track_caller]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.inner.extend(iter)
    }

    /*
    #[inline]
    #[track_caller]
    fn extend_one(&mut self, item: T) {
        self.inner.push(item);
    }
     */
    /*
    #[inline]
    #[track_caller]
    fn extend_reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }
    */
    /*
    #[inline]
    unsafe fn extend_one_unchecked(&mut self, item: T) {
        // SAFETY: Our preconditions ensure the space has been reserved, and `extend_reserve` is implemented correctly.
        unsafe {
            let len = self.len();
            core::ptr::write(self.as_mut_ptr().add(len), item);
            self.set_len(len + 1);
        }
    }
    */
}

impl<'a, T, A> Extend<&'a T> for TypedProjVec<T, A>
where
    T: any::Any + Copy + 'a,
    A: any::Any + alloc::Allocator,
{
    #[track_caller]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.inner.extend(iter.into_iter().copied())
    }

    /*
    #[inline]
    #[track_caller]
    fn extend_one(&mut self, &item: &'a T) {
        self.push(item);
    }
    */
    /*
    #[inline]
    #[track_caller]
    fn extend_reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }
    */
    /*
    #[inline]
    unsafe fn extend_one_unchecked(&mut self, &item: &'a T) {
        // SAFETY: Our preconditions ensure the space has been reserved, and `extend_reserve` is implemented correctly.
        unsafe {
            let len = self.len();
            core::ptr::write(self.as_mut_ptr().add(len), item);
            self.set_len(len + 1);
        }
    }
    */
}

impl<T, A1, A2> PartialEq<TypedProjVec<T, A2>> for TypedProjVec<T, A1>
where
    T: any::Any + PartialEq,
    A1: any::Any + alloc::Allocator,
    A2: any::Any + alloc::Allocator,
{
    fn eq(&self, other: &TypedProjVec<T, A2>) -> bool {
        PartialEq::eq(self.as_slice(), other.as_slice())
    }
}

impl<T, A1, A2> PartialOrd<TypedProjVec<T, A2>> for TypedProjVec<T, A1>
where
    T: any::Any + PartialOrd,
    A1: any::Any + alloc::Allocator,
    A2: any::Any + alloc::Allocator,
{
    #[inline]
    fn partial_cmp(&self, other: &TypedProjVec<T, A2>) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(self.as_slice(), other.as_slice())
    }
}

impl<T, A> Eq for TypedProjVec<T, A>
where
    T: any::Any + Eq,
    A: any::Any + alloc::Allocator,
{
}

impl<T, A> Ord for TypedProjVec<T, A>
where
    T: any::Any + Ord,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Ord::cmp(self.as_slice(), other.as_slice())
    }
}
/*
impl<T, A> Drop for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn drop(&mut self) {

    }
}
*/
impl<T, A> Default for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Default,
{
    fn default() -> TypedProjVec<T, A> {
        TypedProjVec::new_in(Default::default())
    }
}

impl<T, A> fmt::Debug for TypedProjVec<T, A>
where
    T: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl<T, A> AsRef<TypedProjVec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn as_ref(&self) -> &TypedProjVec<T, A> {
        self
    }
}

impl<T, A> AsMut<TypedProjVec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn as_mut(&mut self) -> &mut TypedProjVec<T, A> {
        self
    }
}

impl<T, A> AsRef<[T]> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, A> AsMut<[T]> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> From<&[T]> for TypedProjVec<T, alloc::Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &[T]) -> TypedProjVec<T, alloc::Global> {
        let inner = TypedProjVecInner::from(slice);

        Self { inner, }
    }
}

impl<T> From<&mut [T]> for TypedProjVec<T, alloc::Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &mut [T]) -> TypedProjVec<T, alloc::Global> {
        let inner = TypedProjVecInner::from(slice);

        Self { inner, }
    }
}

impl<T, const N: usize> From<&[T; N]> for TypedProjVec<T, alloc::Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &[T; N]) -> TypedProjVec<T, alloc::Global> {
        Self::from(slice.as_slice())
    }
}

impl<T, const N: usize> From<&mut [T; N]> for TypedProjVec<T, alloc::Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &mut [T; N]) -> TypedProjVec<T, alloc::Global> {
        Self::from(slice.as_mut_slice())
    }
}

impl<T, const N: usize> From<[T; N]> for TypedProjVec<T, alloc::Global>
where
    T: any::Any,
{
    #[track_caller]
    fn from(slice: [T; N]) -> TypedProjVec<T, alloc::Global> {
        let inner = TypedProjVecInner::from(slice);

        Self { inner, }
    }
}

impl<'a, T> From<borrow::Cow<'a, [T]>> for TypedProjVec<T, alloc::Global>
where
    T: any::Any,
    [T]: ToOwned<Owned = TypedProjVec<T, alloc::Global>>,
{
    #[track_caller]
    fn from(slice: borrow::Cow<'a, [T]>) -> TypedProjVec<T, alloc::Global> {
        slice.into_owned()
    }
}

impl<T, A> From<Box<[T], A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn from(slice: Box<[T], A>) -> Self {
        let inner = TypedProjVecInner::from(slice);

        Self { inner, }
    }
}

impl<T, A> From<Vec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[track_caller]
    fn from(vec: Vec<T, A>) -> Self {
        let inner = TypedProjVecInner::from(vec);

        Self { inner, }
    }
}

impl<T, A> From<&Vec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    #[track_caller]
    fn from(vec: &Vec<T, A>) -> Self {
        let inner = TypedProjVecInner::from(vec);

        Self { inner, }
    }
}

impl<T, A> From<&mut Vec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    #[track_caller]
    fn from(vec: &mut Vec<T, A>) -> Self {
        let inner = TypedProjVecInner::from(vec);

        Self { inner, }
    }
}

impl<T, A> From<TypedProjVec<T, A>> for Box<[T], TypedProjAlloc<A>>
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[track_caller]
    fn from(vec: TypedProjVec<T, A>) -> Self {
        vec.into_boxed_slice()
    }
}

impl From<&str> for TypedProjVec<u8, alloc::Global> {
    #[track_caller]
    fn from(st: &str) -> TypedProjVec<u8, alloc::Global> {
        From::from(st.as_bytes())
    }
}

impl<T, A, const N: usize> TryFrom<TypedProjVec<T, A>> for [T; N]
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    type Error = TypedProjVec<T, A>;

    fn try_from(mut vec: TypedProjVec<T, A>) -> Result<[T; N], TypedProjVec<T, A>> {
        if vec.len() != N {
            return Err(vec);
        }

        // SAFETY: `.set_len(0)` is always sound.
        unsafe { vec.set_len(0) };

        // SAFETY: A `Vec`'s pointer is always aligned properly, and
        // the alignment the array needs is the same as the items.
        // We checked earlier that we have sufficient items.
        // The items will not double-drop as the `set_len`
        // tells the `Vec` not to also drop them.
        let array = unsafe { core::ptr::read(vec.as_ptr() as *const [T; N]) };
        Ok(array)
    }
}

#[repr(transparent)]
pub struct OpaqueVec {
    inner: OpaqueVecInner,
}

impl OpaqueVec {
    #[inline]
    pub const fn element_type_id(&self) -> any::TypeId {
        self.inner.element_type_id()
    }

    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }

    #[inline]
    pub fn has_element_type<T>(&self) -> bool
    where
        T: any::Any,
    {
        self.inner.element_type_id() == any::TypeId::of::<T>()
    }

    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator,
    {
        self.inner.allocator_type_id() == any::TypeId::of::<A>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<T, A>(&self)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(st: &str, type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("{:?} type mismatch. Need `{:?}`, got `{:?}`", st, type_id_self, type_id_other);
        }

        if !self.has_element_type::<T>() {
            type_check_failed("Element", self.inner.element_type_id, any::TypeId::of::<T>());
        }

        if !self.has_allocator_type::<A>() {
            type_check_failed("Allocator", self.inner.allocator_type_id(), any::TypeId::of::<A>());
        }
    }
}

impl OpaqueVec {
    #[inline]
    pub fn as_proj<T, A>(&self) -> &TypedProjVec<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<T, A>();

        unsafe { &*(self as *const OpaqueVec as *const TypedProjVec<T, A>) }
    }

    #[inline]
    pub fn as_proj_mut<T, A>(&mut self) -> &mut TypedProjVec<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<T, A>();

        unsafe { &mut *(self as *mut OpaqueVec as *mut TypedProjVec<T, A>) }
    }

    #[inline]
    pub fn into_proj<T, A>(self) -> TypedProjVec<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<T, A>();

        TypedProjVec {
            inner: self.inner.into_proj_assuming_type::<T, A>(),
        }
    }

    #[inline]
    pub fn from_proj<T, A>(proj_self: TypedProjVec<T, A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        Self {
            inner: OpaqueVecInner::from_proj(proj_self.inner),
        }
    }
}

impl OpaqueVec {
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_proj_in<T, A>(proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_vec = TypedProjVec::<T, A>::new_proj_in(proj_alloc);

        Self::from_proj(proj_vec)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_proj_in<T, A>(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_vec = TypedProjVec::<T, A>::with_capacity_proj_in(capacity, proj_alloc);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub fn try_with_capacity_proj_in<T, A>(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Result<Self, opaque_error::TryReserveError>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_vec = TypedProjVec::<T, A>::try_with_capacity_proj_in(capacity, proj_alloc)?;

        Ok(Self::from_proj(proj_vec))
    }

    #[inline]
    pub unsafe fn from_raw_parts_proj_in<T, A>(ptr: *mut T, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, A>::from_raw_parts_proj_in(ptr, length, capacity, proj_alloc)
        };

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub unsafe fn from_parts_proj_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, A>::from_parts_proj_in(ptr, length, capacity, proj_alloc)
        };

        Self::from_proj(proj_vec)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_in<T, A>(alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_vec = TypedProjVec::<T, A>::new_in(alloc);

        Self::from_proj(proj_vec)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in<T, A>(capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_vec = TypedProjVec::<T, A>::with_capacity_in(capacity, alloc);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub fn try_with_capacity_in<T, A>(capacity: usize, alloc: A) -> Result<Self, opaque_error::TryReserveError>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_vec = TypedProjVec::<T, A>::try_with_capacity_in(capacity, alloc)?;

        Ok(Self::from_proj(proj_vec))
    }

    #[inline]
    pub unsafe fn from_raw_parts_in<T, A>(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, A>::from_raw_parts_in(ptr, length, capacity, alloc)
        };

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub unsafe fn from_parts_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, A>::from_parts_in(ptr, length, capacity, alloc)
        };

        Self::from_proj(proj_vec)
    }
}

impl OpaqueVec {
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new<T>() -> Self
    where
        T: any::Any,
    {
        let proj_vec = TypedProjVec::<T, alloc::Global>::new();

        Self::from_proj(proj_vec)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity<T>(capacity: usize) -> Self
    where
        T: any::Any,
    {
        let proj_vec = TypedProjVec::<T, alloc::Global>::with_capacity(capacity);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub fn try_with_capacity<T>(capacity: usize) -> Result<Self, opaque_error::TryReserveError>
    where
        T: any::Any,
    {
        let proj_vec = TypedProjVec::<T, alloc::Global>::try_with_capacity(capacity)?;

        Ok(Self::from_proj(proj_vec))
    }

    #[inline]
    pub unsafe fn from_raw_parts<T>(ptr: *mut T, length: usize, capacity: usize) -> Self
    where
        T: any::Any,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, alloc::Global>::from_raw_parts(ptr, length, capacity)
        };

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub unsafe fn from_parts<T>(ptr: NonNull<T>, length: usize, capacity: usize) -> Self
    where
        T: any::Any,
    {
        let proj_vec = unsafe {
            TypedProjVec::<T, alloc::Global>::from_parts(ptr, length, capacity)
        };

        Self::from_proj(proj_vec)
    }
}

impl OpaqueVec {
    #[inline]
    pub const fn element_layout(&self) -> alloc::Layout {
        self.inner.element_layout()
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn as_byte_slice(&self) -> &[u8] {
        self.inner.as_byte_slice()
    }
}

impl OpaqueVec {
    #[inline]
    pub fn allocator<T, A>(&self) -> &TypedProjAlloc<A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.allocator()
    }
}

impl OpaqueVec {
    #[inline]
    #[must_use]
    pub fn get<T, A>(&self, index: usize) -> Option<&T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.get(index)
    }

    #[inline]
    #[must_use]
    pub fn get_mut<T, A>(&mut self, index: usize) -> Option<&mut T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.get_mut(index)
    }

    #[inline]
    #[track_caller]
    pub fn push<T, A>(&mut self, value: T)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.push(value);
    }

    #[inline]
    pub fn pop<T, A>(&mut self) -> Option<T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.pop()
    }

    #[inline]
    pub fn push_within_capacity<T, A>(&mut self, value: T) -> Result<(), T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.push_within_capacity(value)
    }

    #[track_caller]
    pub fn replace_insert<T, A>(&mut self, index: usize, value: T)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.replace_insert(index, value);
    }

    #[track_caller]
    pub fn shift_insert<T, A>(&mut self, index: usize, value: T)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.shift_insert(index, value);
    }

    #[track_caller]
    pub fn swap_remove<T, A>(&mut self, index: usize) -> T
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.swap_remove(index)
    }

    #[track_caller]
    pub fn shift_remove<T, A>(&mut self, index: usize) -> T
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.shift_remove(index)
    }

    pub fn contains<T, A>(&self, value: &T) -> bool
    where
        T: any::Any + PartialEq,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.contains(value)
    }

    pub fn iter<T, A>(&self) -> slice::Iter<'_, T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.iter()
    }

    pub fn iter_mut<T, A>(&mut self) -> slice::IterMut<'_, T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.iter_mut()
    }

    pub fn into_iter<T, A>(self) -> IntoIter<T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.into_proj::<T, A>();

        proj_self.into_iter()
    }

    #[inline]
    #[track_caller]
    pub fn append<T, A>(&mut self, other: &mut Self)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        let proj_other = other.as_proj_mut::<T, A>();

        proj_self.append(proj_other)
    }

    pub fn drain<R, T, A>(&mut self, range: R) -> Drain<'_, T, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.drain(range)
    }

    #[inline]
    pub fn as_ptr<T, A>(&self) -> *const T
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr<T, A>(&mut self) -> *mut T
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.as_mut_ptr()
    }

    #[inline]
    pub fn as_non_null<T, A>(&mut self) -> NonNull<T>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.as_non_null()
    }

    pub fn as_slice<T, A>(&self) -> &[T]
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.as_slice()
    }

    pub fn as_mut_slice<T, A>(&mut self) -> &mut [T]
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.as_mut_slice()
    }

    #[must_use]
    pub fn into_raw_parts<T>(self) -> (*mut T, usize, usize)
    where
        T: any::Any,
    {
        let proj_self = self.into_proj::<T, alloc::Global>();

        proj_self.into_raw_parts()
    }

    #[must_use]
    pub fn into_parts<T>(self) -> (NonNull<T>, usize, usize)
    where
        T: any::Any,
    {
        let proj_self = self.into_proj::<T, alloc::Global>();

        proj_self.into_parts()
    }

    #[must_use]
    pub fn into_raw_parts_with_alloc<T, A>(self) -> (*mut T, usize, usize, TypedProjAlloc<A>)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.into_proj::<T, A>();

        proj_self.into_raw_parts_with_alloc()
    }

    #[must_use]
    pub fn into_parts_with_alloc<T, A>(self) -> (NonNull<T>, usize, usize, TypedProjAlloc<A>)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.into_proj::<T, A>();

        proj_self.into_parts_with_alloc()
    }

    #[track_caller]
    pub fn into_boxed_slice<T, A>(self) -> Box<[T], TypedProjAlloc<A>>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.into_proj::<T, A>();

        proj_self.into_boxed_slice()
    }

    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    #[track_caller]
    pub fn split_off<T, A>(&mut self, at: usize) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Clone,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        let proj_split_off = proj_self.split_off(at);

        Self::from_proj(proj_split_off)
    }

    #[track_caller]
    pub fn resize_with<F, T, A>(&mut self, new_len: usize, f: F)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
        F: FnMut() -> T,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.resize_with(new_len, f)
    }

    #[inline]
    pub fn spare_capacity_mut<T, A>(&mut self) -> &mut [MaybeUninit<T>]
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.spare_capacity_mut()
    }
}

impl OpaqueVec {
    pub fn try_reserve<T, A>(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.try_reserve(additional)
    }

    pub fn try_reserve_exact<T, A>(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.try_reserve_exact(additional)
    }

    #[track_caller]
    pub fn reserve<T, A>(&mut self, additional: usize)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.reserve(additional);
    }

    #[track_caller]
    pub fn reserve_exact<T, A>(&mut self, additional: usize)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.reserve_exact(additional);
    }

    #[track_caller]
    #[inline]
    pub fn shrink_to_fit<T, A>(&mut self)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.shrink_to_fit();
    }

    #[track_caller]
    pub fn shrink_to<T, A>(&mut self, min_capacity: usize)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.shrink_to(min_capacity);
    }

    pub fn clear<T, A>(&mut self)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.clear();
    }
}

impl OpaqueVec {
    #[inline]
    pub fn splice<R, I, T, A>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.splice(range, replace_with)
    }

    pub fn extract_if<F, R, T, A>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.extract_if(range, filter)
    }

    /*
    #[track_caller]
    fn extend_with<T, A>(&mut self, count: usize, value: T)
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.extend_with(count, value);
    }

    #[track_caller]
    fn extend_from_iter<I, T, A>(&mut self, iterator: I)
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator,
        I: Iterator<Item = T>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.extend_from_iter(iterator);
    }
    */

    #[track_caller]
    pub fn extend_from_slice<T, A>(&mut self, other: &[T])
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.extend_from_slice(other);
    }

    #[track_caller]
    pub fn resize<T, A>(&mut self, new_len: usize, value: T)
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.resize(new_len, value);
    }

    #[inline]
    pub fn truncate<T, A>(&mut self, len: usize)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.truncate(len);
    }
}

impl OpaqueVec {
    pub fn retain<F, T, A>(&mut self, f: F)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
        F: FnMut(&T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.retain(f);
    }

    pub fn retain_mut<F, T, A>(&mut self, f: F)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
        F: FnMut(&mut T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.retain_mut(f);
    }

    #[inline]
    pub fn dedup_by_key<F, K, T, A>(&mut self, mut key: F)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.dedup_by_key(&mut key);
    }

    pub fn dedup_by<F, T, A>(&mut self, same_bucket: F)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
        F: FnMut(&mut T, &mut T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.dedup_by(same_bucket);
    }
}

impl OpaqueVec {
    #[inline]
    pub fn extend<I, T, A>(&mut self, iter: I)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
        I: IntoIterator<Item=T>,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.extend(iter);
    }

    #[inline]
    pub fn reverse<T, A>(&mut self)
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<T, A>();

        proj_self.reverse();
    }
}

impl OpaqueVec {
    #[inline]
    pub fn clone<T, A>(&self) -> Self
    where
        T: any::Any + Clone,
        A: any::Any + alloc::Allocator + Clone,
    {
        let proj_self = self.as_proj::<T, A>();
        let proj_cloned_self = Clone::clone(proj_self);
        let cloned_self = OpaqueVec::from_proj(proj_cloned_self);

        cloned_self
    }
}

impl fmt::Debug for OpaqueVec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpaqueVec")
            .finish()
    }
}

impl<T> From<&[T]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(slice: &[T]) -> Self {
        let proj_vec = TypedProjVec::from(slice);

        Self::from_proj(proj_vec)
    }
}

impl<T> From<&mut [T]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(slice: &mut [T]) -> Self {
        let proj_vec = TypedProjVec::from(slice);

        Self::from_proj(proj_vec)
    }
}

impl<const N: usize, T> From<&[T; N]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(array: &[T; N]) -> Self {
        let proj_vec = TypedProjVec::from(array);

        Self::from_proj(proj_vec)
    }
}

impl<const N: usize, T> From<&mut [T; N]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(array: &mut [T; N]) -> Self {
        let proj_vec = TypedProjVec::from(array);

        Self::from_proj(proj_vec)
    }
}

impl<T, A> From<Vec<T, A>> for OpaqueVec
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn from(vec: Vec<T, A>) -> Self {
        let proj_vec = TypedProjVec::from(vec);

        Self::from_proj(proj_vec)
    }
}

impl<T> From<&Vec<T>> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(vec: &Vec<T>) -> Self {
        let proj_vec = TypedProjVec::from(vec);

        Self::from_proj(proj_vec)
    }
}

impl<T> From<&mut Vec<T>> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(vec: &mut Vec<T>) -> Self {
        let proj_vec = TypedProjVec::from(vec);

        Self::from_proj(proj_vec)
    }
}

impl<T, A> From<Box<[T], A>> for OpaqueVec
where
    T: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn from(slice: Box<[T], A>) -> Self {
        let proj_vec = TypedProjVec::from(slice);

        Self::from_proj(proj_vec)
    }
}

impl<const N: usize, T> From<[T; N]> for OpaqueVec
where
    T: any::Any,
{
    fn from(array: [T; N]) -> Self {
        let proj_vec = TypedProjVec::from(array);

        Self::from_proj(proj_vec)
    }
}

impl From<&str> for OpaqueVec {
    #[track_caller]
    fn from(st: &str) -> Self {
        From::from(st.as_bytes())
    }
}

impl<T> FromIterator<T> for OpaqueVec
where
    T: any::Any,
{
    #[inline]
    #[track_caller]
    fn from_iter<I>(iter: I) -> OpaqueVec
    where
        I: IntoIterator<Item = T>,
    {
        let proj_vec = TypedProjVec::from_iter(iter);

        Self::from_proj(proj_vec)
    }
}

#[cfg(test)]
mod vec_inner_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_vec_inner_match_sizes<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::size_of::<TypedProjVecInner<T, A>>();
        let result = mem::size_of::<OpaqueVecInner>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_vec_inner_match_alignments<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::align_of::<TypedProjVecInner<T, A>>();
        let result = mem::align_of::<OpaqueVecInner>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_vec_inner_match_offsets<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        assert_eq!(
            mem::offset_of!(TypedProjVecInner<T, A>, data),
            mem::offset_of!(OpaqueVecInner, data),
            "Opaque and Typed Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypedProjVecInner<T, A>, element_type_id),
            mem::offset_of!(OpaqueVecInner, element_type_id),
            "Opaque and Typed Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypedProjVecInner<T, A>, allocator_type_id),
            mem::offset_of!(OpaqueVecInner, allocator_type_id),
            "Opaque and Typed Projected data types offsets mismatch"
        );
    }

    struct Pair(u8, u64);

    struct DummyAlloc {}

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            alloc::Global.allocate(layout)
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
            unsafe {
                alloc::Global.deallocate(ptr, layout)
            }
        }
    }

    macro_rules! layout_tests {
        ($module_name:ident, $element_typ:ty, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_opaque_vec_inner_layout_match_sizes() {
                    run_test_opaque_vec_inner_match_sizes::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_vec_inner_layout_match_alignments() {
                    run_test_opaque_vec_inner_match_alignments::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_vec_inner_layout_match_offsets() {
                    run_test_opaque_vec_inner_match_offsets::<$element_typ, $alloc_typ>();
                }
            }
        };
    }

    layout_tests!(u8_global, u8, alloc::Global);
    layout_tests!(pair_dummy_alloc, Pair, DummyAlloc);
    layout_tests!(unit_zst_dummy_alloc, (), DummyAlloc);
}

#[cfg(test)]
mod vec_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_vec_match_sizes<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::size_of::<TypedProjVec<T, A>>();
        let result = mem::size_of::<OpaqueVec>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_vec_match_alignments<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::align_of::<TypedProjVec<T, A>>();
        let result = mem::align_of::<OpaqueVec>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_vec_match_offsets<T, A>()
    where
        T: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::offset_of!(TypedProjVec<T, A>, inner);
        let result = mem::offset_of!(OpaqueVec, inner);

        assert_eq!(result, expected, "Opaque and Typed Projected data types offsets mismatch");
    }

    struct Pair(u8, u64);

    struct DummyAlloc {}

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            alloc::Global.allocate(layout)
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
            unsafe {
                alloc::Global.deallocate(ptr, layout)
            }
        }
    }

    macro_rules! layout_tests {
        ($module_name:ident, $element_typ:ty, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_opaque_vec_layout_match_sizes() {
                    run_test_opaque_vec_match_sizes::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_vec_layout_match_alignments() {
                    run_test_opaque_vec_match_alignments::<$element_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_vec_layout_match_offsets() {
                    run_test_opaque_vec_match_offsets::<$element_typ, $alloc_typ>();
                }
            }
        };
    }

    layout_tests!(u8_global, u8, alloc::Global);
    layout_tests!(pair_dummy_alloc, Pair, DummyAlloc);
    layout_tests!(unit_zst_dummy_alloc, (), DummyAlloc);
}
