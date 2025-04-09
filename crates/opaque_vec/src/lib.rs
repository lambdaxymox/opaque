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
use core::slice;
use core::ops;

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

use core::iter::FusedIterator;

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

pub struct IntoIter<T, A> {
    opaque_vec: OpaqueVec,
    _marker: core::marker::PhantomData<(T, A)>,
}

impl<T, A> fmt::Debug for IntoIter<T, A>
where
    T: fmt::Debug + 'static,
    A: Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

impl<T, A> IntoIter<T, A>
where
    T: 'static,
    A: Allocator,
{
    pub fn as_slice(&self) -> &[T] {
        self.opaque_vec.as_slice::<T>()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.opaque_vec.as_mut_slice::<T>()
    }
}

impl<T> IntoIter<T, OpaqueAlloc>
where
    T: 'static,
{
    #[inline]
    pub fn allocator(&self) -> &OpaqueAlloc {
        self.opaque_vec.allocator()
    }
}

impl<T, A> AsRef<[T]> for IntoIter<T, A>
where
    T: 'static,
    A: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, A: Allocator> Iterator for IntoIter<T, A> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        todo!()
    }
}

impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        todo!()
    }
}

impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A> {}
impl<T, A: Allocator> FusedIterator for IntoIter<T, A> {}

/*
impl<T, A> Default for IntoIter<T, A>
where
    A: Allocator + Default,
{
    fn default() -> Self {
        OpaqueVec::new_in(Default::default()).into_iter()
    }
}
 */

#[cfg(not(no_global_oom_handling))]
impl<T, A> Clone for IntoIter<T, A>
where
    T: Clone + 'static,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self {
        Self {
            opaque_vec: self.opaque_vec.clone(),
            _marker: self._marker,
        }
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    pub(crate) fn contains_unchecked<T>(&self, value: &T) -> bool
    where
        T: PartialEq + 'static,
    {
        self.as_slice::<T>().contains(value)
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

    #[inline]
    pub(crate) fn as_slice_unchecked<T>(&self) -> &[T]
    where
        T: 'static,
    {
        unsafe {
            let data_ptr = self.data.as_ptr() as *const T;
            let len = self.data.len();

            core::slice::from_raw_parts(data_ptr, len)
        }
    }

    #[inline]
    pub(crate) fn as_mut_slice_unchecked<T>(&mut self) -> &mut [T]
    where
        T: 'static,
    {
        unsafe {
            let data_ptr = self.data.as_mut_ptr() as *mut T;
            let len = self.data.len();

            core::slice::from_raw_parts_mut(data_ptr, len)
        }
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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
            let ptr = self.as_mut_ptr::<T>().add(self.len()) as *mut MaybeUninit<T>;
            let len = self.capacity() - self.len();

            std::slice::from_raw_parts_mut(ptr, len)
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

    pub fn into_iter<T>(self) -> IntoIter<T, OpaqueAlloc>
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();

        IntoIter {
            opaque_vec: self,
            _marker: PhantomData,
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
        let value_ptr = unsafe {
            NonNull::new_unchecked(&value as *const T as *mut T as *mut u8)
        };

        self.data.extend_with(count, value_ptr);
    }

    #[inline]
    fn extend_from_iter_unchecked<T, I>(&mut self, mut iterator: I)
    where
        T: Clone + 'static,
        I: Iterator<Item = T>,
    {
        for item in iterator {
            self.push_unchecked::<T>(item);
        }
    }

    #[inline]
    pub fn extend_from_slice_unchecked<T>(&mut self, other: &[T])
    where
        T: Clone + 'static,
    {
        self.extend_from_iter::<T, _>(other.iter().cloned())
    }

    #[inline]
    pub fn resize_unchecked<T>(&mut self, new_len: usize, value: T)
    where
        T: Clone + 'static,
    {
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
    fn extend_from_iter<T, I>(&mut self, iterator: I)
    where
        T: Clone + 'static,
        I: Iterator<Item = T>,
    {
        self.ensure_element_type::<T>();

        self.extend_from_iter_unchecked::<T, _>(iterator)
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

impl OpaqueVec {
    pub fn retain_unchecked<F, T>(&mut self, mut f: F)
    where
        T: 'static,
        F: FnMut(&T) -> bool,
    {
        self.retain_mut_unchecked(|elem| f(elem));
    }

    pub fn retain<F, T>(&mut self, mut f: F)
    where
        T: 'static,
        F: FnMut(&T) -> bool,
    {
        self.ensure_element_type::<T>();

        self.retain_unchecked(|elem| f(elem));
    }

    pub fn retain_mut_unchecked<F, T>(&mut self, mut f: F)
    where
        T: 'static,
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
            T: 'static,
            A: Allocator,
        {
            v: &'a mut OpaqueVec,
            processed_len: usize,
            deleted_cnt: usize,
            original_len: usize,
            _marker: PhantomData<(T, A)>,
        }

        impl<T, A> Drop for BackshiftOnDrop<'_, T, A>
        where
            T: 'static,
            A: Allocator,
        {
            fn drop(&mut self) {
                if self.deleted_cnt > 0 {
                    // SAFETY: Trailing unchecked items must be valid since we never touch them.
                    unsafe {
                        core::ptr::copy(
                            self.v.as_ptr::<T>().add(self.processed_len),
                            self.v.as_mut_ptr::<T>().add(self.processed_len - self.deleted_cnt),
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
            _marker: PhantomData,
        };

        fn process_loop<F, T, A, const DELETED: bool>(
            original_len: usize,
            f: &mut F,
            g: &mut BackshiftOnDrop<'_, T, A>,
        ) where
            T: 'static,
            A: Allocator,
            F: FnMut(&mut T) -> bool,
        {
            while g.processed_len != original_len {
                // SAFETY: Unchecked element must be valid.
                let cur = unsafe { &mut *g.v.as_mut_ptr::<T>().add(g.processed_len) };
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
                        let hole_slot = g.v.as_mut_ptr::<T>().add(g.processed_len - g.deleted_cnt);
                        core::ptr::copy_nonoverlapping(cur, hole_slot, 1);
                    }
                }
                g.processed_len += 1;
            }
        }

        // Stage 1: Nothing was deleted.
        process_loop::<F, T, OpaqueAlloc, false>(original_len, &mut f, &mut g);

        // Stage 2: Some elements were deleted.
        process_loop::<F, T, OpaqueAlloc, true>(original_len, &mut f, &mut g);

        // All item are processed. This can be optimized to `set_len` by LLVM.
        drop(g);
    }

    pub fn retain_mut<F, T>(&mut self, mut f: F)
    where
        T: 'static,
        F: FnMut(&mut T) -> bool,
    {
        self.ensure_element_type::<T>();

        self.retain_mut_unchecked::<F, T>(f)
    }

    #[inline]
    pub fn dedup_by_key<F, K, T>(&mut self, mut key: F)
    where
        T: 'static,
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.dedup_by(|a, b| key(a) == key(b))
    }

    pub fn dedup_by<F, T>(&mut self, mut same_bucket: F)
    where
        T: 'static,
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
        let start = self.as_mut_ptr::<T>();
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
            T: 'static,
            A: core::alloc::Allocator,
        {
            /* Offset of the element we want to check if it is duplicate */
            read: usize,

            /* Offset of the place where we want to place the non-duplicate
             * when we find it. */
            write: usize,

            /* The Vec that would need correction if `same_bucket` panicked */
            vec: &'a mut OpaqueVec,
            _marker: PhantomData<(T, A)>,
        }

        impl<'a, T, A> Drop for FillGapOnDrop<'a, T, A>
        where
            T: 'static,
            A: core::alloc::Allocator,
        {
            fn drop(&mut self) {
                /* This code gets executed when `same_bucket` panics */

                /* SAFETY: invariant guarantees that `read - write`
                 * and `len - read` never overflow and that the copy is always
                 * in-bounds. */
                unsafe {
                    let ptr = self.vec.as_mut_ptr::<T>();
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
        let mut gap: FillGapOnDrop<'_, T, OpaqueAlloc> = FillGapOnDrop {
            read: first_duplicate_idx + 1,
            write: first_duplicate_idx,
            vec: self,
            _marker: PhantomData,
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
            core::mem::forget(gap);
        }
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
        self.opaque_vec.as_ptr_unchecked::<T>()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.opaque_vec.as_slice_unchecked::<T>()
    }

    #[inline]
    pub fn as_byte_slice(&self) -> &[u8] {
        self.opaque_vec.as_byte_slice()
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.opaque_vec.as_mut_slice_unchecked::<T>()
    }
}

impl OpaqueVec {
    #[inline]
    pub fn reverse<T>(&mut self)
    where
        T: 'static,
    {
        self.ensure_element_type::<T>();
        self.as_mut_slice::<T>().reverse();
    }
}

impl<'a, T> MapMut<'a, T>
where
    T: PartialEq + 'static,
{
    pub fn contains(&self, value: &T) -> bool {
        self.opaque_vec.contains(value)
    }
}

impl<'a, T, I: slice::SliceIndex<[T]>> ops::Index<I> for MapMut<'a, T>
where
    T: 'static,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        ops::Index::index(self.as_slice(), index)
    }
}

impl<'a, T, I: slice::SliceIndex<[T]>> ops::IndexMut<I> for MapMut<'a, T>
where
    T: 'static,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        ops::IndexMut::index_mut(self.as_mut_slice(), index)
    }
}

impl<'a, T> fmt::Debug for MapMut<'a, T>
where
    T: fmt::Debug + 'static,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(formatter)
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
