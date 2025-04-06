#![feature(const_eval_select)]
#![feature(allocator_api)]
#![feature(structural_match)]
#![feature(alloc_layout_extra)]
#![feature(optimize_attribute)]
extern crate core;

use std::alloc::{Allocator, Layout, Global};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ptr::NonNull;
use std::fmt;

mod range_types;
mod unique;
mod opaque_vec_memory;
mod try_reserve_error;
mod opaque_vec_inner;

use crate::try_reserve_error::TryReserveError;
use crate::opaque_vec_inner::OpaqueVecInner;

use std::alloc;
use std::any::TypeId;
use std::marker::PhantomData;

use opaque_alloc::OpaqueAlloc;

#[derive(Clone)]
pub struct Iter<'a, T> {
    slice: std::slice::Iter<'a, T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.next()
    }
}

pub struct OpaqueVec {
    data: OpaqueVecInner,
    type_id: TypeId,
}

impl OpaqueVec {
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_in<T, A>(alloc: A) -> Self
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let opaque_alloc = OpaqueAlloc::new(alloc);
        let element_layout = Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let data = OpaqueVecInner::new_in(opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();

        Self { data, type_id, }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in<T, A>(capacity: usize, alloc: A) -> Self
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let opaque_alloc = OpaqueAlloc::new::<A>(alloc);
        let element_layout = Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let data = OpaqueVecInner::with_capacity_in(capacity, opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();

        Self { data, type_id, }
    }

    #[inline]
    pub fn try_with_capacity_in<T, A>(capacity: usize, alloc: A) -> Result<Self, TryReserveError>
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let opaque_alloc = OpaqueAlloc::new::<A>(alloc);
        let element_layout = Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let data = OpaqueVecInner::try_with_capacity_in(capacity, opaque_alloc, element_layout, drop_fn)?;
        let type_id = TypeId::of::<T>();

        Ok(Self { data, type_id, })
    }

    #[inline]
    pub unsafe fn from_raw_parts_in<T, A>(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let opaque_alloc = OpaqueAlloc::new(alloc);
        let element_layout = Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let ptr_bytes = ptr.cast::<u8>();
        let data = OpaqueVecInner::from_raw_parts_in(ptr_bytes, length, capacity, opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();

        Self { data, type_id, }
    }

    #[inline]
    pub unsafe fn from_parts_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let opaque_alloc = OpaqueAlloc::new::<A>(alloc);
        let element_layout = Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let ptr_bytes = ptr.cast::<u8>();
        let data = OpaqueVecInner::from_parts_in(ptr_bytes, length, capacity, opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();

        Self { data, type_id, }
    }

    #[inline]
    pub const fn allocator(&self) -> &OpaqueAlloc {
        self.data.allocator()
    }
}

impl OpaqueVec {
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new<T>() -> Self
    where
        T: 'static,
    {
        Self::new_in::<T, Global>(Global)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity<T>(capacity: usize) -> Self
    where
        T: 'static,
    {
        Self::with_capacity_in::<T, Global>(capacity, Global)
    }

    #[inline]
    pub fn try_with_capacity<T>(capacity: usize) -> Result<Self, TryReserveError>
    where
        T: 'static,
    {
        Self::try_with_capacity_in::<T, Global>(capacity, Global)
    }

    #[inline]
    pub unsafe fn from_raw_parts<T>(ptr: *mut T, length: usize, capacity: usize) -> Self
    where
        T: 'static,
    {
        let opaque_alloc = OpaqueAlloc::new::<Global>(Global);

        Self::from_raw_parts_in(ptr, length, capacity, opaque_alloc)
    }

    #[inline]
    pub unsafe fn from_parts<T>(ptr: NonNull<T>, length: usize, capacity: usize) -> Self
    where
        T: 'static,
    {
        let opaque_alloc = OpaqueAlloc::new::<Global>(Global);

        Self::from_parts_in(ptr, length, capacity, opaque_alloc)
    }
}

impl OpaqueVec {
    #[inline]
    pub const fn element_layout(&self) -> Layout {
        self.data.element_layout()
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.data.capacity()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    unsafe fn set_len(&mut self, new_len: usize) {
        self.data.set_len(new_len);
    }
}

impl OpaqueVec {
    #[inline]
    #[must_use]
    pub fn get_unchecked<T>(&self, index: usize) -> &T
    where
        T: 'static,
    {
        let ptr = self.data.get_unchecked(index);

        // SAFETY:
        // (1) The size of T matches the expected element size.
        // (2) We assume that the caller has ensured that `index` is within bounds.
        unsafe {
            &*ptr.as_ptr().cast::<T>()
        }
    }

    #[inline]
    #[must_use]
    pub fn get_mut_unchecked<T>(&mut self, index: usize) -> &mut T
    where
        T: 'static,
    {
        let ptr = self.data.get_mut_unchecked(index);

        // SAFETY:
        // (1) The size of T matches the expected element size.
        // (2) We assume that the caller has ensured that `index` is within bounds.
        unsafe {
            &mut *ptr.as_ptr().cast::<T>()
        }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn push_unchecked<T>(&mut self, value: T)
    where
        T: 'static,
    {
        let mut me = ManuallyDrop::new(value);
        let value_ptr = unsafe {
            NonNull::new_unchecked(&mut *me as *mut T as *mut u8)
        };

        self.data.push(value_ptr);
    }

    #[inline]
    pub(crate) fn pop_unchecked<T>(&mut self) -> Option<T>
    where
        T: 'static,
    {
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

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub(crate) fn replace_insert_unchecked<T>(&mut self, index: usize, value: T)
    where
        T: 'static,
    {
        let mut me = ManuallyDrop::new(value);
        let value_ptr = unsafe {
            NonNull::new_unchecked(&mut *me as *mut T as *mut u8)
        };

        self.data.replace_insert(index, value_ptr);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shift_insert_unchecked<T>(&mut self, index: usize, value: T)
    where
        T: 'static,
    {
        let mut me = ManuallyDrop::new(value);
        let value_ptr = unsafe {
            NonNull::new_unchecked(&mut *me as *mut T as *mut u8)
        };

        self.data.shift_insert(index, value_ptr);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub(crate) fn swap_remove_unchecked<T>(&mut self, index: usize) -> T
    where
        T: 'static,
    {
        // index < self.len()
        let value = unsafe {
            let ptr = self.data.get_unchecked(index);
            let _value = ptr.cast::<T>().read();
            _value
        };

        // SAFETY:
        let _ = self.data.swap_remove_forget_unchecked(index);

        value
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shift_remove_unchecked<T>(&mut self, index: usize) -> T
    where
        T: 'static,
    {
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

    pub(crate) fn contains_unchecked<T>(&self, value: &T) -> bool
    where
        T: PartialEq + 'static,
    {
        for other_value in self.iter::<T>() {
            if value == other_value {
                return true;
            }
        }

        false
    }

    #[inline]
    pub const fn as_ptr_unchecked<T>(&self) -> *const T
    where
        T: 'static,
    {
        self.data.as_ptr() as *const T
    }

    #[inline]
    pub(crate) const fn as_mut_ptr_unchecked<T>(&mut self) -> *mut T
    where
        T: 'static,
    {
        self.data.as_mut_ptr() as *mut T
    }

    #[inline]
    pub(crate) const fn as_non_null_unchecked<T>(&mut self) -> NonNull<T>
    where
        T: 'static,
    {
        // SAFETY: An [`OpaqueVec`] always holds a non-null pointer.
        self.data.as_non_null().cast::<T>()
    }

    pub(crate) fn as_slice_unchecked<T>(&self) -> &[T]
    where
        T: 'static,
    {
        if self.data.is_empty() {
            return &[];
        }

        let slice = unsafe {
            let data_ptr = self.data.as_ptr() as *const T;
            let len = self.data.len();

            core::slice::from_raw_parts(data_ptr, len)
        };

        slice
    }

    pub(crate) fn as_mut_slice_unchecked<T>(&mut self) -> &mut [T]
    where
        T: 'static,
    {
        if self.data.is_empty() {
            return &mut [];
        }

        let slice = unsafe {
            let data_ptr = self.data.as_mut_ptr() as *mut T;
            let len = self.data.len();

            std::slice::from_raw_parts_mut(data_ptr, len)
        };

        slice
    }

    #[must_use]
    pub(crate) fn into_raw_parts_unchecked<T>(self) -> (*mut T, usize, usize)
    where
        T: 'static,
    {
        let mut me = ManuallyDrop::new(self);
        let ptr = me.as_mut_ptr();
        let len = me.len();
        let capacity = me.capacity();

        (ptr, len, capacity)
    }

    #[must_use]
    pub(crate) fn into_parts_unchecked<T>(self) -> (NonNull<T>, usize, usize)
    where
        T: 'static,
    {
        let mut me = ManuallyDrop::new(self);

        // SAFETY: An `OpaqueVec` always has a non-null pointer.
        let ptr = unsafe { NonNull::new_unchecked(me.as_mut_ptr()) };
        let len = me.len();
        let capacity = me.capacity();

        (ptr, len, capacity)
    }

    #[must_use]
    pub(crate) fn into_raw_parts_with_alloc_unchecked<T>(self) -> (*mut T, usize, usize, OpaqueAlloc)
    where
        T: 'static,
    {
        let mut me = ManuallyDrop::new(self);
        let ptr = me.as_mut_ptr();
        let len = me.len();
        let capacity = me.capacity();
        let alloc = unsafe { core::ptr::read(me.allocator()) };

        (ptr, len, capacity, alloc)
    }

    #[must_use]
    pub(crate) fn into_parts_with_alloc_unchecked<T>(self) -> (NonNull<T>, usize, usize, OpaqueAlloc)
    where
        T: 'static,
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
    pub(crate) fn spare_capacity_mut_unchecked<T>(&mut self) -> &mut [MaybeUninit<T>]
    where
        T: 'static,
    {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut_ptr::<T>().add(self.len()) as *mut MaybeUninit<T>,
                self.capacity() - self.len(),
            )
        }
    }
}

impl OpaqueVec {
    #[inline]
    pub fn has_element_type<T>(&self) -> bool
    where
        T: 'static,
    {
        TypeId::of::<T>() == self.type_id
    }

    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: Allocator + Clone + 'static,
    {
        self.allocator().is_type::<A>()
    }

    #[inline]
    #[track_caller]
    fn ensure_element_type<T>(&self)
    where
        T: 'static,
    {
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: TypeId, type_id_other: TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_element_type::<T>() {
            type_check_failed(self.type_id, TypeId::of::<T>());
        }
    }

    #[inline]
    #[must_use]
    pub fn get<T>(&self, index: usize) -> Option<&T>
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        if index >= self.data.len() {
            return None;
        }

        let ptr = self.get_unchecked(index);

        Some(ptr)
    }

    #[inline]
    #[must_use]
    pub fn get_mut<T>(&mut self, index: usize) -> Option<&mut T>
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        if index >= self.data.len() {
            return None;
        }

        let ptr = self.get_mut_unchecked(index);

        Some(ptr)
    }

    #[inline]
    #[track_caller]
    pub fn push<T>(&mut self, value: T)
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.push_unchecked::<T>(value);
    }

    #[inline]
    pub fn pop<T>(&mut self) -> Option<T>
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.pop_unchecked::<T>()
    }

    #[inline]
    pub fn push_within_capacity<T>(&mut self, value: T) -> Result<(), T>
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        if self.data.len() == self.data.capacity() {
            return Err(value);
        }

        self.push_unchecked::<T>(value);

        Ok(())
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn replace_insert<T>(&mut self, index: usize, value: T)
    where
        T: 'static,
    {
        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("replace_insert index out of bounds: Got index `{index}`. Need index `{index}` <= len, where len is `{length}`.");
        }

        self.ensure_element_type::<T>();

        let length = self.len();
        if index > length {
            index_out_of_bounds_failure(index, length);
        }

        // SAFETY:
        self.replace_insert_unchecked::<T>(index, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shift_insert<T>(&mut self, index: usize, value: T)
    where
        T: 'static,
    {
        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("shift_insert index out of bounds: Got index `{index}`. Need index `{index}` <= len, where len is `{length}`.");
        }

        self.ensure_element_type::<T>();

        let length = self.len();
        if index > length {
            index_out_of_bounds_failure(index, length);
        }

        self.shift_insert_unchecked::<T>(index, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn swap_remove<T>(&mut self, index: usize) -> T
    where
        T: 'static,
    {
        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("swap_remove index out of bounds: Got `{index}`, length is `{length}`.");
        }

        self.ensure_element_type::<T>();

        let length = self.len();
        if index >= length {
            index_out_of_bounds_failure(index, length);
        }

        self.swap_remove_unchecked::<T>(index)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shift_remove<T>(&mut self, index: usize) -> T
    where
        T: 'static,
    {
        #[cold]
        #[track_caller]
        #[optimize(size)]
        fn index_out_of_bounds_failure(index: usize, length: usize) -> ! {
            panic!("shift_remove index out of bounds: Got `{index}`, length is `{length}`.");
        }

        self.ensure_element_type::<T>();

        let length = self.len();
        if index >= length {
            index_out_of_bounds_failure(index, length);
        }

        self.shift_remove_unchecked::<T>(index)
    }

    pub fn contains<T>(&self, value: &T) -> bool
    where
        T: PartialEq + 'static,
    {
        self.ensure_element_type::<T>();

        self.contains_unchecked::<T>(value)
    }

    pub fn iter<T>(&self) -> Iter<'_, T>
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        Iter {
            slice: self.as_slice::<T>().iter(),
        }
    }

    #[inline]
    pub fn as_ptr<T>(&self) -> *const T
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.as_ptr_unchecked::<T>()
    }

    #[inline]
    pub fn as_mut_ptr<T>(&mut self) -> *mut T
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.as_mut_ptr_unchecked::<T>()
    }

    #[inline]
    pub fn as_non_null<T>(&mut self) -> NonNull<T>
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.as_non_null_unchecked::<T>()
    }

    pub fn as_slice<T>(&self) -> &[T]
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.as_slice_unchecked::<T>()
    }

    pub fn as_mut_slice<T>(&mut self) -> &mut [T]
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.as_mut_slice_unchecked::<T>()
    }

    pub fn as_byte_slice(&self) -> &[u8] {
        self.data.as_byte_slice()
    }

    #[must_use]
    pub fn into_raw_parts<T>(self) -> (*mut T, usize, usize)
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.into_raw_parts_unchecked::<T>()
    }

    #[must_use]
    pub fn into_parts<T>(self) -> (NonNull<T>, usize, usize)
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.into_parts_unchecked::<T>()
    }

    #[must_use]
    pub fn into_raw_parts_with_alloc<T>(self) -> (*mut T, usize, usize, OpaqueAlloc)
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.into_raw_parts_with_alloc_unchecked::<T>()
    }

    pub fn into_parts_with_alloc<T>(self) -> (NonNull<T>, usize, usize, OpaqueAlloc)
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.into_parts_with_alloc_unchecked::<T>()
    }

    #[inline]
    pub fn spare_capacity_mut<T>(&mut self) -> &mut [MaybeUninit<T>]
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        self.spare_capacity_mut_unchecked::<T>()
    }
}

impl OpaqueVec {
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.data.try_reserve(additional)
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.data.try_reserve_exact(additional)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }

    #[track_caller]
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl OpaqueVec {
    #[inline]
    fn extend_with_unchecked<T>(&mut self, count: usize, value: T)
    where
        T: Clone + 'static,
    {
        self.ensure_element_type::<T>();

        let value_ptr = unsafe {
            NonNull::new_unchecked(&value as *const T as *mut T as *mut u8)
        };

        self.data.extend_with(count, value_ptr);
    }

    #[inline]
    fn extend_from_iter_unechecked<T, I>(&mut self, mut iterator: I)
    where
        T: Clone + 'static,
        I: Iterator<Item = T>,
    {
        self.ensure_element_type::<T>();

        let mut non_null_iterator = iterator.map(|item| unsafe {
            NonNull::new_unchecked(&item as *const T as *mut T as *mut u8)
        });
        self.data.extend_from_iter(non_null_iterator);
    }

    #[inline]
    pub fn extend_from_slice_unchecked<T>(&mut self, other: &[T])
    where
        T: Clone + 'static,
    {
        self.ensure_element_type::<T>();

        self.extend_from_iter::<T, _>(other.iter().cloned())
    }

    #[inline]
    pub fn resize_unchecked<T>(&mut self, new_len: usize, value: T)
    where
        T: Clone + 'static,
    {
        self.ensure_element_type::<T>();

        let len = self.len();

        if new_len > len {
            self.extend_with(new_len - len, value)
        } else {
            self.truncate(new_len);
        }
    }
}

impl OpaqueVec {
    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn extend_with<T>(&mut self, count: usize, value: T)
    where
        T: Clone + 'static,
    {
        self.ensure_element_type::<T>();

        self.extend_with_unchecked::<T>(count, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn extend_from_iter<T, I>(&mut self, mut iterator: I)
    where
        T: Clone + 'static,
        I: Iterator<Item = T>,
    {
        self.ensure_element_type::<T>();

        self.extend_from_iter_unechecked::<T, _>(iterator)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn extend_from_slice<T>(&mut self, other: &[T])
    where
        T: Clone + 'static,
    {
        self.ensure_element_type::<T>();

        self.extend_from_slice_unchecked::<T>(other);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn resize<T>(&mut self, new_len: usize, value: T)
    where
        T: Clone + 'static,
    {
        self.ensure_element_type::<T>();

        self.resize_unchecked::<T>(new_len, value);
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }
}

struct DebugDisplayDataFormatter<'a> {
    inner: &'a OpaqueVec,
}

impl<'a> DebugDisplayDataFormatter<'a> {
    #[inline]
    const fn new(inner: &'a OpaqueVec) -> Self {
        Self {
            inner,
        }
    }

    fn fmt_data(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let slice = self.inner.as_byte_slice();
        let element_size = self.inner.element_layout().size();

        write!(formatter, "[")?;

        let mut it = slice.chunks(element_size).peekable();
        while let Some(chunk) = it.next() {
            write!(formatter, "{:?}", chunk)?;
            if it.peek().is_some() {
                write!(formatter, ", ")?;
            }
        }

        write!(formatter, "]")
    }
}

impl<'a> fmt::Debug for DebugDisplayDataFormatter<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_data(formatter)
    }
}

impl fmt::Debug for OpaqueVec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_display = DebugDisplayDataFormatter::new(&self);

        formatter.debug_struct("OpaqueVec")
            .field("element_layout", &self.element_layout())
            .field("capacity", &self.capacity())
            .field("length", &self.len())
            .field("data", &data_display)
            .finish()
    }
}

impl fmt::Display for OpaqueVec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_display = DebugDisplayDataFormatter::new(&self);

        data_display.fmt_data(formatter)
    }
}

impl PartialEq<OpaqueVec> for OpaqueVec {
    fn eq(&self, other: &OpaqueVec) -> bool {
        (self.element_layout() == other.element_layout()) && (self.as_byte_slice() == other.as_byte_slice())
    }
}

impl Clone for OpaqueVec {
    fn clone(&self) -> Self {
        let new_inner = self.data.clone();
        let new_type_id = self.type_id;

        Self {
            data: new_inner,
            type_id: new_type_id,
        }
    }
}

mod private {
    use super::OpaqueVec;
    use std::alloc::Allocator;

    // We shouldn't add inline attribute to this since this is used in
    // `vec!` macro mostly and causes perf regression. See #71204 for
    // discussion and perf results.
    #[allow(missing_docs)]
    pub fn into_opaque_vec<T, A>(b: Box<[T], A>) -> OpaqueVec
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        unsafe {
            let len = b.len();
            let (b, alloc) = Box::into_raw_with_allocator(b);
            OpaqueVec::from_raw_parts_in(b as *mut T, len, len, alloc)
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[allow(missing_docs)]
    #[inline]
    pub fn to_opaque_vec<T, A>(slice: &[T], alloc: A) -> OpaqueVec
    where
        T: ConvertOpaqueVec,
        A: Allocator + Clone + 'static,
    {
        T::to_opaque_vec(slice, alloc)
    }

    #[cfg(not(no_global_oom_handling))]
    pub trait ConvertOpaqueVec {
        fn to_opaque_vec<A>(slice: &[Self], alloc: A) -> OpaqueVec
        where
            A: Allocator + Clone + 'static,
            Self: Sized;
    }

    #[cfg(not(no_global_oom_handling))]
    impl<T> ConvertOpaqueVec for T
    where
        T: Clone + 'static,
    {
        #[inline]
        fn to_opaque_vec<A>(slice: &[Self], alloc: A) -> OpaqueVec
        where
            A: Allocator + Clone + 'static,
        {
            struct DropGuard<'a> {
                vec: &'a mut OpaqueVec,
                num_init: usize,
            }
            
            impl<'a> Drop for DropGuard<'a> {
                #[inline]
                fn drop(&mut self) {
                    // SAFETY:
                    // items were marked initialized in the loop below
                    unsafe {
                        self.vec.set_len(self.num_init);
                    }
                }
            }
            
            let mut vec = OpaqueVec::with_capacity_in::<Self, A>(slice.len(), alloc);
            let mut guard = DropGuard { vec: &mut vec, num_init: 0 };
            let slots = guard.vec.spare_capacity_mut();
            // .take(slots.len()) is necessary for LLVM to remove bounds checks
            // and has better codegen than zip.
            for (i, b) in slice.iter().enumerate().take(slots.len()) {
                guard.num_init = i;
                slots[i].write(b.clone());
            }
            
            core::mem::forget(guard);
            
            // SAFETY:
            // the vec was allocated and initialized above to at least this length.
            unsafe {
                vec.set_len(slice.len());
            }
            
            vec
        }
    }
}

impl<T> From<&[T]> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(slice: &[T]) -> Self {
        private::to_opaque_vec::<T, Global>(slice, Global)
    }
}

impl<T> From<&mut [T]> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(slice: &mut [T]) -> Self {
        private::to_opaque_vec::<T, Global>(slice, Global)
    }
}

impl<const N: usize, T> From<&[T; N]> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(array: &[T; N]) -> Self {
        Self::from(array.as_slice())
    }
}

impl<const N: usize, T> From<&mut [T; N]> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(array: &mut [T; N]) -> Self {
        Self::from(array.as_mut_slice())
    }
}

impl<T> From<&Vec<T>> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(vec: &Vec<T>) -> Self {
        Self::from(vec.as_slice())
    }
}

impl<T> From<&mut Vec<T>> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(vec: &mut Vec<T>) -> Self {
        Self::from(vec.as_mut_slice())
    }
}

impl<T> From<Box<[T]>> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(slice: Box<[T]>) -> Self {
        Self::from(slice.as_ref())
    }
}

impl<const N: usize, T> From<[T; N]> for OpaqueVec
where
    T: 'static,
{
    fn from(array: [T; N]) -> Self {
        private::into_opaque_vec::<T, Global>(Box::new(array))
    }
}

impl<T> FromIterator<T> for OpaqueVec
where
    T: 'static,
{
    #[inline]
    #[track_caller]
    fn from_iter<I>(iter: I) -> OpaqueVec
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();

        let mut vec = OpaqueVec::with_capacity::<T>(lower);

        for item in iter {
            vec.push::<T>(item);
        }

        vec
    }
}

pub struct Map<'a, T> {
    opaque_vec: &'a OpaqueVec,
    _marker: PhantomData<T>,
}

impl<'a, T> Map<'a, T>
where
    T: 'static,
{
    #[inline]
    const fn new(opaque_vec: &'a OpaqueVec) -> Self {
        Self {
            opaque_vec,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub const fn element_layout(&self) -> Layout {
        self.opaque_vec.element_layout()
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.opaque_vec.capacity()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.opaque_vec.is_empty()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.opaque_vec.len()
    }

    #[inline]
    pub fn get_unchecked(&self, index: usize) -> &T {
        self.opaque_vec.get_unchecked(index)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.opaque_vec.len() {
            return None;
        }

        Some(self.opaque_vec.get_unchecked(index))
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.opaque_vec.iter::<T>()
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.opaque_vec.as_ptr_unchecked()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.opaque_vec.as_slice_unchecked()
    }

    #[inline]
    pub fn as_byte_slice(&self) -> &[u8] {
        self.opaque_vec.as_byte_slice()
    }
}

impl<'a, T> Map<'a, T>
where
    T: PartialEq + 'static,
{
    pub fn contains(&self, value: &T) -> bool {
        self.opaque_vec.contains(value)
    }
}

use core::ops;
use core::slice;
use crate::try_reserve_error::TryReserveErrorKind;

impl<'a, T, I: slice::SliceIndex<[T]>> ops::Index<I> for Map<'a, T>
where
    T: 'static,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        ops::Index::index(self.as_slice(), index)
    }
}

impl<'a, T> fmt::Debug for Map<'a, T>
where
    T: fmt::Debug + 'static,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(formatter)
    }
}

pub struct MapMut<'a, T> {
    opaque_vec: &'a mut OpaqueVec,
    _marker: PhantomData<T>,
}

impl<'a, T> MapMut<'a, T>
where
    T: 'static,
{
    #[inline]
    const fn new(opaque_vec: &'a mut OpaqueVec) -> Self {
        Self {
            opaque_vec,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub const fn element_layout(&self) -> Layout {
        self.opaque_vec.element_layout()
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.opaque_vec.capacity()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.opaque_vec.is_empty()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.opaque_vec.len()
    }

    #[inline]
    pub fn get_unchecked(&self, index: usize) -> &T {
        self.opaque_vec.get_unchecked(index)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.opaque_vec.len() {
            return None;
        }

        Some(self.opaque_vec.get_unchecked(index))
    }

    #[inline]
    pub fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        self.opaque_vec.get_mut_unchecked(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.opaque_vec.len() {
            return None;
        }

        Some(self.opaque_vec.get_mut_unchecked(index))
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.opaque_vec.iter::<T>()
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.opaque_vec.as_ptr_unchecked()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.opaque_vec.as_slice_unchecked()
    }

    #[inline]
    pub fn as_byte_slice(&self) -> &[u8] {
        self.opaque_vec.as_byte_slice()
    }
}

impl OpaqueVec {
    pub fn as_map<T>(&self) -> Map<'_, T>
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        Map::new(self)
    }

    pub fn as_map_mut<T>(&mut self) -> MapMut<'_, T>
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        MapMut::new(self)
    }
}
