#![feature(const_eval_select)]
#![feature(allocator_api)]
#![feature(structural_match)]
#![feature(alloc_layout_extra)]
#![feature(optimize_attribute)]
#![feature(slice_range)]
extern crate core;

use core::any;
use core::cmp;
use core::hash;
use core::ops;
use core::slice;
use std::alloc::{
    Allocator,
    Global,
    Layout,
};
use std::{fmt, mem};
use std::mem::{
    ManuallyDrop,
    MaybeUninit,
};
use std::borrow;
use std::ptr::NonNull;

use core::any::TypeId;
use core::marker::PhantomData;
use core::iter::FusedIterator;

use opaque_blob_vec::OpaqueBlobVec;
use opaque_alloc::{OpaqueAlloc, TypedProjAlloc};
use opaque_error;

#[inline(always)]
const fn is_zst<T>() -> bool {
    core::mem::size_of::<T>() == 0
}

#[inline(always)]
const fn assuming_non_null<T>(item: *const T) -> NonNull<T> {
    unsafe { *(item as *const NonNull<T>) }
}

#[inline(always)]
const fn assuming_non_null_mut<T>(item: *const T) -> NonNull<T> {
    unsafe { *(item as *mut NonNull<T>) }
}

pub struct IntoIter<T, A> {
    buf: NonNull<T>,
    cap: usize,
    // the drop impl reconstructs a RawVec from buf, cap and alloc
    // to avoid dropping the allocator twice we need to wrap it into ManuallyDrop
    alloc: ManuallyDrop<TypedProjAlloc<A>>,
    ptr: NonNull<T>,
    /// If T is a ZST, this is actually ptr+len. This encoding is picked so that
    /// ptr == end is a quick test for the Iterator being empty, that works
    /// for both ZST and non-ZST.
    /// For non-ZSTs the pointer is treated as `NonNull<T>`
    end: *const T,
    _marker: core::marker::PhantomData<(T, A)>,
}

impl<T, A> fmt::Debug for IntoIter<T, A>
where
    T: any::Any + fmt::Debug,
    A: Allocator + any::Any,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

impl<T, A> IntoIter<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len()) }
    }

    fn as_raw_mut_slice(&mut self) -> *mut [T] {
        core::ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len())
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { &mut *self.as_raw_mut_slice() }
    }

    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
        &self.alloc
    }
}

impl<T, A> AsRef<[T]> for IntoIter<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

unsafe impl<T, A> Send for IntoIter<T, A>
where
    T: any::Any + Send,
    A: Allocator + any::Any + Send,
{
}
unsafe impl<T, A> Sync for IntoIter<T, A>
where
    T: any::Any + Sync,
    A: Allocator + any::Any + Sync,
{
}

impl<T, A> Iterator for IntoIter<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let ptr = if is_zst::<T>() {
            if self.ptr.as_ptr() == self.end as *mut T {
                return None;
            }
            // `ptr` has to stay where it is to remain aligned, so we reduce the length by 1 by
            // reducing the `end`.
            self.end = self.end.wrapping_byte_sub(1);
            self.ptr
        } else {
            if self.ptr == assuming_non_null(self.end) {
                return None;
            }
            let old = self.ptr;
            self.ptr = unsafe { old.add(1) };
            old
        };

        Some(unsafe { ptr.read() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = if is_zst::<T>() {
            self.end.addr().wrapping_sub(self.ptr.as_ptr().addr())
        } else {
            unsafe { assuming_non_null_mut(self.end).offset_from_unsigned(self.ptr) }
        };

        (exact, Some(exact))
    }

    /*
    #[inline]
    fn advance_by(&mut self, n: usize) -> Result<(), core::num::NonZero<usize>> {
        let step_size = self.len().min(n);
        let to_drop = core::ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), step_size);
        if is_zst::<T>() {
            // See `next` for why we sub `end` here.
            self.end = self.end.wrapping_byte_sub(step_size);
        } else {
            // SAFETY: the min() above ensures that step_size is in bounds
            self.ptr = unsafe { self.ptr.add(step_size) };
        }
        // SAFETY: the min() above ensures that step_size is in bounds
        unsafe {
            core::ptr::drop_in_place(to_drop);
        }

        core::num::NonZero::new(n - step_size).map_or(Ok(()), Err)
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn next_chunk<const N: usize>(&mut self) -> Result<[T; N], core::array::IntoIter<T, N>> {
        let mut raw_ary = [const { MaybeUninit::uninit() }; N];

        let len = self.len();

        if is_zst::<T>() {
            if len < N {
                self.forget_remaining_elements();
                // Safety: ZSTs can be conjured ex nihilo, only the amount has to be correct
                return Err(unsafe { core::array::IntoIter::new_unchecked(raw_ary, 0..len) });
            }

            self.end = self.end.wrapping_byte_sub(N);
            // Safety: ditto
            return Ok(unsafe { raw_ary.transpose().assume_init() });
        }

        if len < N {
            // Safety: `len` indicates that this many elements are available and we just checked that
            // it fits into the array.
            unsafe {
                core::ptr::copy_nonoverlapping(self.ptr.as_ptr(), raw_ary.as_mut_ptr() as *mut T, len);
                self.forget_remaining_elements();
                return Err(core::array::IntoIter::new_unchecked(raw_ary, 0..len));
            }
        }

        // Safety: `len` is larger than the array size. Copy a fixed amount here to fully initialize
        // the array.
        unsafe {
            core::ptr::copy_nonoverlapping(self.ptr.as_ptr(), raw_ary.as_mut_ptr() as *mut T, N);
            self.ptr = self.ptr.add(N);
            Ok(raw_ary.transpose().assume_init())
        }
    }

    fn fold<B, F>(mut self, mut accum: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        if is_zst::<T>() {
            while self.ptr.as_ptr() != self.end.cast_mut() {
                // SAFETY: we just checked that `self.ptr` is in bounds.
                let tmp = unsafe { self.ptr.read() };
                // See `next` for why we subtract from `end` here.
                self.end = self.end.wrapping_byte_sub(1);
                accum = f(accum, tmp);
            }
        } else {
            // SAFETY: `self.end` can only be null if `T` is a ZST.
            while self.ptr != assuming_non_null(self.end) {
                // SAFETY: we just checked that `self.ptr` is in bounds.
                let tmp = unsafe { self.ptr.read() };
                // SAFETY: the maximum this can be is `self.end`.
                // Increment `self.ptr` first to avoid double dropping in the event of a panic.
                self.ptr = unsafe { self.ptr.add(1) };
                accum = f(accum, tmp);
            }
        }
        accum
    }

    fn try_fold<B, F, R>(&mut self, mut accum: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: core::ops::Try<Output = B>,
    {
        if is_zst::<T>() {
            while self.ptr.as_ptr() != self.end.cast_mut() {
                // SAFETY: we just checked that `self.ptr` is in bounds.
                let tmp = unsafe { self.ptr.read() };
                // See `next` for why we subtract from `end` here.
                self.end = self.end.wrapping_byte_sub(1);
                accum = f(accum, tmp)?;
            }
        } else {
            // SAFETY: `self.end` can only be null if `T` is a ZST.
            while self.ptr != assuming_non_null(self.end) {
                // SAFETY: we just checked that `self.ptr` is in bounds.
                let tmp = unsafe { self.ptr.read() };
                // SAFETY: the maximum this can be is `self.end`.
                // Increment `self.ptr` first to avoid double dropping in the event of a panic.
                self.ptr = unsafe { self.ptr.add(1) };
                accum = f(accum, tmp)?;
            }
        }

        R::from_output(accum)
    }
     */
}

impl<T, A> DoubleEndedIterator for IntoIter<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if is_zst::<T>() {
            if self.ptr.as_ptr() == self.end as *mut _ {
                return None;
            }
            // See above for why 'ptr.offset' isn't used
            self.end = self.end.wrapping_byte_sub(1);
            // Note that even though this is next_back() we're reading from `self.ptr`, not
            // `self.end`. We track our length using the byte offset from `self.ptr` to `self.end`,
            // so the end pointer may not be suitably aligned for T.
            Some(unsafe { core::ptr::read(self.ptr.as_ptr()) })
        } else {
            if self.ptr == assuming_non_null(self.end) {
                return None;
            }
            unsafe {
                self.end = self.end.sub(1);
                Some(core::ptr::read(self.end))
            }
        }
    }

    /*
    #[inline]
    fn advance_back_by(&mut self, n: usize) -> Result<(), core::num::NonZero<usize>> {
        let step_size = self.len().min(n);
        if is_zst::<T>() {
            // SAFETY: same as for advance_by()
            self.end = self.end.wrapping_byte_sub(step_size);
        } else {
            // SAFETY: same as for advance_by()
            self.end = unsafe { self.end.sub(step_size) };
        }
        let to_drop = core::ptr::slice_from_raw_parts_mut(self.end as *mut T, step_size);
        // SAFETY: same as for advance_by()
        unsafe {
            core::ptr::drop_in_place(to_drop);
        }

        core::num::NonZero::new(n - step_size).map_or(Ok(()), Err)
    }
    */
}

impl<T, A> ExactSizeIterator for IntoIter<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    /*
    fn is_empty(&self) -> bool {
        if is_zst::<T>() {
            self.ptr.as_ptr() == self.end as *mut _
        } else {
            self.ptr == assuming_non_null(self.end)
        }
    }
    */
}
impl<T, A> FusedIterator for IntoIter<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
}

#[cfg(not(no_global_oom_handling))]
impl<T, A> Clone for IntoIter<T, A>
where
    T: any::Any + Clone,
    A: Allocator + any::Any + Clone,
{
    fn clone(&self) -> Self {
        let alloc = Clone::clone(ops::Deref::deref(&self.alloc));
        let read_alloc = ManuallyDrop::new(unsafe { core::ptr::read(&alloc) });
        let inner = private::to_opaque_vec(self.as_slice(), alloc);

        unsafe {
            let mut me = ManuallyDrop::new(inner);
            let data_ptr = me.as_non_null::<T>();
            let begin = data_ptr.as_ptr();
            let end = if is_zst::<T>() {
                begin.wrapping_byte_add(me.len())
            } else {
                begin.add(me.len()) as *const T
            };
            let cap = me.capacity();

            IntoIter { buf: data_ptr, cap, alloc: read_alloc, ptr: data_ptr, end, _marker: PhantomData, }
        }
    }
}
/*
unsafe impl<T, A> Drop for IntoIter<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn drop(&mut self) {
        struct DropGuard<'a, T, A: Allocator>(&'a mut IntoIter<T, A>);

        impl<T, A> Drop for DropGuard<'_, T, A>
        where
            A: Allocator,
        {
            fn drop(&mut self) {
                unsafe {
                    // `IntoIter::alloc` is not used anymore after this and will be dropped by RawVec
                    let alloc = ManuallyDrop::take(&mut self.0.alloc);
                    let _ = OpaqueVecInner::from_raw_parts_in(self.0.buf.as_ptr(), self.0.len(), self.0.cap, alloc);
                }
            }
        }

        let guard = DropGuard(self);
        // destroy the remaining elements
        unsafe {
            core::ptr::drop_in_place(guard.0.as_raw_mut_slice());
        }
        // now `guard` will be dropped and do the rest
    }
}
*/


pub struct Drain<'a, T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    /// Index of tail to preserve
    pub(crate) tail_start: usize,
    /// Length of tail
    pub(crate) tail_len: usize,
    /// Current remaining range to remove
    pub(crate) iter: slice::Iter<'a, T>,
    pub(crate) vec: NonNull<OpaqueVecInner>,
    _marker: core::marker::PhantomData<A>,
}

impl<T, A> fmt::Debug for Drain<'_, T, A>
where
    T: any::Any + fmt::Debug,
    A: Allocator + any::Any,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Drain").field(&self.iter.as_slice()).finish()
    }
}

impl<'a, T, A> Drain<'a, T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.iter.as_slice()
    }

    /*
    #[must_use]
    #[inline]
    pub fn allocator(&self) -> &A {
        unsafe { self.vec.as_ref().allocator() }
    }
    */

    #[must_use]
    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
        unsafe { self.vec.as_ref().allocator::<T, A>() }
    }

    pub fn keep_rest(self) {
        // At this moment layout looks like this:
        //
        // [head] [yielded by next] [unyielded] [yielded by next_back] [tail]
        //        ^-- start         \_________/-- unyielded_len        \____/-- self.tail_len
        //                          ^-- unyielded_ptr                  ^-- tail
        //
        // Normally `Drop` impl would drop [unyielded] and then move [tail] to the `start`.
        // Here we want to
        // 1. Move [unyielded] to `start`
        // 2. Move [tail] to a new start at `start + len(unyielded)`
        // 3. Update length of the original vec to `len(head) + len(unyielded) + len(tail)`
        //    a. In case of ZST, this is the only thing we want to do
        // 4. Do *not* drop self, as everything is put in a consistent state already, there is nothing to do
        let mut this = ManuallyDrop::new(self);

        unsafe {
            let source_vec = this.vec.as_mut();

            let start = source_vec.len();
            let tail = this.tail_start;

            let unyielded_len = this.iter.len();
            let unyielded_ptr = this.iter.as_slice().as_ptr();

            // ZSTs have no identity, so we don't need to move them around.
            if !is_zst::<T>() {
                let start_ptr = source_vec.as_mut_ptr::<T>().add(start);

                // memmove back unyielded elements
                if unyielded_ptr != start_ptr {
                    let src = unyielded_ptr;
                    let dst = start_ptr;

                    core::ptr::copy(src, dst, unyielded_len);
                }

                // memmove back untouched tail
                if tail != (start + unyielded_len) {
                    let src = source_vec.as_ptr::<T>().add(tail);
                    let dst = start_ptr.add(unyielded_len);
                    core::ptr::copy(src, dst, this.tail_len);
                }
            }

            source_vec.set_len(start + unyielded_len + this.tail_len);
        }
    }
}

impl<'a, T, A: Allocator> AsRef<[T]> for Drain<'a, T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

unsafe impl<T, A> Sync for Drain<'_, T, A>
where
    T: any::Any + Sync,
    A: Allocator + any::Any + Sync,
{
}

unsafe impl<T: Send, A: Send + Allocator> Send for Drain<'_, T, A>
where
    T: any::Any + Send,
    A: Allocator + any::Any + Send,
{
}

impl<T, A> Iterator for Drain<'_, T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next().map(|elt| unsafe { core::ptr::read(elt as *const _) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, A> DoubleEndedIterator for Drain<'_, T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back().map(|elt| unsafe { core::ptr::read(elt as *const _) })
    }
}

impl<T, A> Drop for Drain<'_, T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn drop(&mut self) {
        /// Moves back the un-`Drain`ed elements to restore the original `Vec`.
        struct DropGuard<'r, 'a, T: any::Any, A: Allocator + any::Any>(&'r mut Drain<'a, T, A>);

        impl<'r, 'a, T, A> Drop for DropGuard<'r, 'a, T, A>
        where
            T: any::Any,
            A: Allocator + any::Any,
        {
            fn drop(&mut self) {
                if self.0.tail_len > 0 {
                    unsafe {
                        let source_vec = self.0.vec.as_mut();
                        // memmove back untouched tail, update to new length
                        let start = source_vec.len();
                        let tail = self.0.tail_start;
                        if tail != start {
                            let src = source_vec.as_ptr::<T>().add(tail);
                            let dst = source_vec.as_mut_ptr::<T>().add(start);
                            core::ptr::copy(src, dst, self.0.tail_len);
                        }
                        source_vec.set_len(start + self.0.tail_len);
                    }
                }
            }
        }

        let iter = core::mem::take(&mut self.iter);
        let drop_len = iter.len();

        let mut vec = self.vec;

        if is_zst::<T>() {
            // ZSTs have no identity, so we don't need to move them around, we only need to drop the correct amount.
            // this can be achieved by manipulating the Vec length instead of moving values out from `iter`.
            unsafe {
                let vec = vec.as_mut();
                let old_len = vec.len();
                vec.set_len(old_len + drop_len + self.tail_len);
                vec.truncate(old_len + self.tail_len);
            }

            return;
        }

        // ensure elements are moved back into their appropriate places, even when drop_in_place panics
        let _guard = DropGuard(self);

        if drop_len == 0 {
            return;
        }

        // as_slice() must only be called when iter.len() is > 0 because
        // it also gets touched by vec::Splice which may turn it into a dangling pointer
        // which would make it and the vec pointer point to different allocations which would
        // lead to invalid pointer arithmetic below.
        let drop_ptr = iter.as_slice().as_ptr();

        unsafe {
            // drop_ptr comes from a slice::Iter which only gives us a &[T] but for drop_in_place
            // a pointer with mutable provenance is necessary. Therefore we must reconstruct
            // it from the original vec but also avoid creating a &mut to the front since that could
            // invalidate raw pointers to it which some unsafe code might rely on.
            let vec_ptr = vec.as_mut().as_mut_ptr();
            let drop_offset = drop_ptr.offset_from_unsigned(vec_ptr);
            let to_drop = core::ptr::slice_from_raw_parts_mut(vec_ptr.add(drop_offset), drop_len);
            core::ptr::drop_in_place(to_drop);
        }
    }
}

impl<T, A> ExactSizeIterator for Drain<'_, T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    /*
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
     */
}

impl<T, A> FusedIterator for Drain<'_, T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
}

#[derive(Debug)]
pub struct Splice<'a, I, A>
where
    I: Iterator + 'a,
    A: Allocator + any::Any + 'a,
    <I as Iterator>::Item: any::Any,
{
    drain: Drain<'a, I::Item, A>,
    replace_with: I,
}

impl<I, A> Iterator for Splice<'_, I, A>
where
    I: Iterator,
    I::Item: any::Any,
    A: Allocator + any::Any,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.drain.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.drain.size_hint()
    }
}

impl<I, A> DoubleEndedIterator for Splice<'_, I, A>
where
    I: Iterator,
    I::Item: any::Any,
    A: Allocator + any::Any,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.drain.next_back()
    }
}

impl<I, A> ExactSizeIterator for Splice<'_, I, A>
where
    I: Iterator,
    I::Item: any::Any,
    A: Allocator + any::Any,
{
}

impl<I, A> Drop for Splice<'_, I, A>
where
    I: Iterator,
    I::Item: any::Any,
    A: Allocator + any::Any,
{
    #[track_caller]
    fn drop(&mut self) {
        self.drain.by_ref().for_each(drop);
        // At this point draining is done and the only remaining tasks are splicing
        // and moving things into the final place.
        // Which means we can replace the slice::Iter with pointers that won't point to deallocated
        // memory, so that Drain::drop is still allowed to call iter.len(), otherwise it would break
        // the ptr.sub_ptr contract.
        self.drain.iter = (&[]).iter();

        unsafe {
            if self.drain.tail_len == 0 {
                self.drain.vec.as_mut().extend(self.replace_with.by_ref());
                return;
            }

            // First fill the range left by drain().
            if !self.drain.fill(&mut self.replace_with) {
                return;
            }

            // There may be more elements. Use the lower bound as an estimate.
            // FIXME: Is the upper bound a better guess? Or something else?
            let (lower_bound, _upper_bound) = self.replace_with.size_hint();
            if lower_bound > 0 {
                self.drain.move_tail(lower_bound);
                if !self.drain.fill(&mut self.replace_with) {
                    return;
                }
            }

            // Collect any remaining elements.
            // This is a zero-length vector which does not allocate if `lower_bound` was exact.
            let mut collected = self.replace_with.by_ref().collect::<Vec<I::Item>>().into_iter();
            // Now we have an exact count.
            if collected.len() > 0 {
                self.drain.move_tail(collected.len());
                let filled = self.drain.fill(&mut collected);
                debug_assert!(filled);
                debug_assert_eq!(collected.len(), 0);
            }
        }
        // Let `Drain::drop` move the tail back if necessary and restore `vec.len`.
    }
}

/// Private helper methods for `Splice::drop`
impl<T, A> Drain<'_, T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    /// The range from `self.vec.len` to `self.tail_start` contains elements
    /// that have been moved out.
    /// Fill that range as much as possible with new elements from the `replace_with` iterator.
    /// Returns `true` if we filled the entire range. (`replace_with.next()` didn’t return `None`.)
    unsafe fn fill<I: Iterator<Item = T>>(&mut self, replace_with: &mut I) -> bool {
        let vec = unsafe { self.vec.as_mut() };
        let range_start = vec.len();
        let range_end = self.tail_start;
        let range_slice = unsafe { slice::from_raw_parts_mut(vec.as_mut_ptr::<T>().add(range_start), range_end - range_start) };

        for place in range_slice {
            if let Some(new_item) = replace_with.next() {
                unsafe {
                    core::ptr::write(place, new_item);
                    vec.set_len(vec.len() + 1);
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Makes room for inserting more elements before the tail.
    #[track_caller]
    unsafe fn move_tail(&mut self, additional: usize) {
        let vec = unsafe { self.vec.as_mut() };
        let len = self.tail_start + self.tail_len;
        vec.reserve(additional);

        let new_tail_start = self.tail_start + additional;
        unsafe {
            let src = vec.as_ptr::<T>().add(self.tail_start);
            let dst = vec.as_mut_ptr::<T>().add(new_tail_start);
            core::ptr::copy(src, dst, self.tail_len);
        }
        self.tail_start = new_tail_start;
    }
}

#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ExtractIf<'a, T, F, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    vec: &'a mut OpaqueVecInner,
    /// The index of the item that will be inspected by the next call to `next`.
    idx: usize,
    /// Elements at and beyond this point will be retained. Must be equal or smaller than `old_len`.
    end: usize,
    /// The number of items that have been drained (removed) thus far.
    del: usize,
    /// The original length of `vec` prior to draining.
    old_len: usize,
    /// The filter test predicate.
    pred: F,
    _marker: core::marker::PhantomData<(T, A)>,
}

impl<'a, T, F, A> ExtractIf<'a, T, F, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn new<R>(vec: &'a mut OpaqueVecInner, pred: F, range: R) -> Self
    where
        R: ops::RangeBounds<usize>,
    {
        let old_len = vec.len();
        let ops::Range { start, end } = slice::range(range, ..old_len);

        // Guard against the vec getting leaked (leak amplification)
        unsafe {
            vec.set_len(0);
        }

        ExtractIf {
            vec,
            idx: start,
            del: 0,
            end,
            old_len,
            pred,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
        self.vec.allocator::<T, A>()
    }
}

impl<T, F, A> Iterator for ExtractIf<'_, T, F, A>
where
    T: any::Any,
    F: FnMut(&mut T) -> bool,
    A: Allocator + any::Any,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        unsafe {
            while self.idx < self.end {
                let i = self.idx;
                let v = slice::from_raw_parts_mut(self.vec.as_mut_ptr(), self.old_len);
                let drained = (self.pred)(&mut v[i]);
                // Update the index *after* the predicate is called. If the index
                // is updated prior and the predicate panics, the element at this
                // index would be leaked.
                self.idx += 1;
                if drained {
                    self.del += 1;
                    return Some(core::ptr::read(&v[i]));
                } else if self.del > 0 {
                    let del = self.del;
                    let src: *const T = &v[i];
                    let dst: *mut T = &mut v[i - del];
                    core::ptr::copy_nonoverlapping(src, dst, 1);
                }
            }
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.end - self.idx))
    }
}

impl<T, F, A> Drop for ExtractIf<'_, T, F, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn drop(&mut self) {
        unsafe {
            if self.idx < self.old_len && self.del > 0 {
                let ptr = self.vec.as_mut_ptr::<T>();
                let src = ptr.add(self.idx);
                let dst = src.sub(self.del);
                let tail_len = self.old_len - self.idx;
                src.copy_to(dst, tail_len);
            }
            self.vec.set_len(self.old_len - self.del);
        }
    }
}

struct Extender<'a, T> {
    inner: &'a mut OpaqueVecInner,
    _marker: core::marker::PhantomData<T>,
}

impl<'a, T> Extender<'a, T> {
    #[inline]
    const fn new(inner: &'a mut OpaqueVecInner) -> Self {
        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<'a, T> Extend<T> for Extender<'a, T>
where
    T: any::Any,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter.into_iter() {
            self.inner.push::<T>(item);
        }
    }
}

impl<'a, 'b, T> Extend<&'b T> for Extender<'a, T>
where
    T: any::Any + Copy,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'b T>,
    {
        for item in iter.into_iter() {
            self.inner.push::<T>(*item);
        }
    }
}

pub struct OpaqueVecInner {
    data: OpaqueBlobVec,
    type_id: TypeId,
    alloc_type_id: TypeId,
}

impl OpaqueVecInner {
    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new_proj_in<T, A>(proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let opaque_alloc = OpaqueAlloc::from_proj::<A>(proj_alloc);
        let element_layout = Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let data = OpaqueBlobVec::new_in(opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();
        let alloc_type_id = TypeId::of::<A>();

        Self { data, type_id, alloc_type_id }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity_proj_in<T, A>(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let opaque_alloc = OpaqueAlloc::from_proj::<A>(proj_alloc);
        let element_layout = Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let data = OpaqueBlobVec::with_capacity_in(capacity, opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();
        let alloc_type_id = TypeId::of::<A>();

        Self { data, type_id, alloc_type_id }
    }

    #[inline]
    pub(crate) fn try_with_capacity_proj_in<T, A>(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Result<Self, opaque_error::TryReserveError>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let opaque_alloc = OpaqueAlloc::from_proj::<A>(proj_alloc);
        let element_layout = Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let data = OpaqueBlobVec::try_with_capacity_in(capacity, opaque_alloc, element_layout, drop_fn)?;
        let type_id = TypeId::of::<T>();
        let alloc_type_id = TypeId::of::<A>();

        Ok(Self { data, type_id, alloc_type_id })
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts_proj_in<T, A>(ptr: *mut T, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);
        let element_layout = Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let ptr_bytes = ptr.cast::<u8>();
        let data = OpaqueBlobVec::from_raw_parts_in(ptr_bytes, length, capacity, opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();
        let alloc_type_id = TypeId::of::<A>();

        Self { data, type_id, alloc_type_id }
    }

    #[inline]
    pub(crate) unsafe fn from_parts_proj_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let opaque_alloc = OpaqueAlloc::from_proj::<A>(proj_alloc);
        let element_layout = Layout::new::<T>();
        let drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
        let ptr_bytes = ptr.cast::<u8>();
        let data = OpaqueBlobVec::from_parts_in(ptr_bytes, length, capacity, opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();
        let alloc_type_id = TypeId::of::<A>();

        Self { data, type_id, alloc_type_id }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new_in<T, A>(alloc: A) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::new_proj_in::<T, A>(proj_alloc)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity_in<T, A>(capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::with_capacity_proj_in::<T, A>(capacity, proj_alloc)
    }

    #[inline]
    pub(crate) fn try_with_capacity_in<T, A>(capacity: usize, alloc: A) -> Result<Self, opaque_error::TryReserveError>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::try_with_capacity_proj_in::<T, A>(capacity, proj_alloc)
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts_in<T, A>(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::from_raw_parts_proj_in(ptr, length, capacity, proj_alloc)
    }

    #[inline]
    pub(crate) unsafe fn from_parts_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_alloc = TypedProjAlloc::new(alloc);

        Self::from_parts_proj_in(ptr, length, capacity, proj_alloc)
    }

    #[inline]
    pub(crate) fn allocator<T, A>(&self) -> &TypedProjAlloc<A>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        self.data.allocator().as_proj::<A>()
    }
}

impl OpaqueVecInner {
    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new<T>() -> Self
    where
        T: any::Any,
    {
        Self::new_in::<T, Global>(Global)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity<T>(capacity: usize) -> Self
    where
        T: any::Any,
    {
        Self::with_capacity_in::<T, Global>(capacity, Global)
    }

    #[inline]
    pub(crate) fn try_with_capacity<T>(capacity: usize) -> Result<Self, opaque_error::TryReserveError>
    where
        T: any::Any,
    {
        Self::try_with_capacity_in::<T, Global>(capacity, Global)
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts<T>(ptr: *mut T, length: usize, capacity: usize) -> Self
    where
        T: any::Any,
    {
        Self::from_raw_parts_in(ptr, length, capacity, Global)
    }

    #[inline]
    pub(crate) unsafe fn from_parts<T>(ptr: NonNull<T>, length: usize, capacity: usize) -> Self
    where
        T: any::Any,
    {
        Self::from_parts_in(ptr, length, capacity, Global)
    }
}

impl OpaqueVecInner {
    #[inline]
    pub fn has_element_type<T>(&self) -> bool
    where
        T: any::Any,
    {
        TypeId::of::<T>() == self.type_id
    }

    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: Allocator + any::Any,
    {
        TypeId::of::<A>() == self.alloc_type_id
    }
}

impl OpaqueVecInner {
    #[inline]
    pub(crate) const fn element_layout(&self) -> Layout {
        self.data.element_layout()
    }

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
    unsafe fn set_len(&mut self, new_len: usize) {
        self.data.set_len(new_len);
    }
}


impl OpaqueVecInner {
    #[inline]
    pub(crate) fn iter<T>(&self) -> slice::Iter<'_, T>
    where
        T: any::Any,
    {
        self.as_slice::<T>().iter()
    }

    #[inline]
    pub(crate) fn iter_mut<T>(&mut self) -> slice::IterMut<'_, T>
    where
        T: any::Any,
    {
        self.as_mut_slice::<T>().iter_mut()
    }

    #[inline]
    #[must_use]
    pub(crate) fn get_unchecked<T>(&self, index: usize) -> &T
    where
        T: any::Any,
    {
        let ptr = self.data.get_unchecked(index);

        // SAFETY:
        // (1) The size of T matches the expected element size.
        // (2) We assume that the caller has ensured that `index` is within bounds.
        unsafe { &*ptr.as_ptr().cast::<T>() }
    }

    #[inline]
    #[must_use]
    pub(crate) fn get_mut_unchecked<T>(&mut self, index: usize) -> &mut T
    where
        T: any::Any,
    {
        let ptr = self.data.get_mut_unchecked(index);

        // SAFETY:
        // (1) The size of T matches the expected element size.
        // (2) We assume that the caller has ensured that `index` is within bounds.
        unsafe { &mut *ptr.as_ptr().cast::<T>() }
    }

    #[inline]
    #[track_caller]
    pub(crate) fn push<T>(&mut self, value: T)
    where
        T: any::Any,
    {
        let mut me = ManuallyDrop::new(value);
        let value_ptr = unsafe { NonNull::new_unchecked(&mut *me as *mut T as *mut u8) };

        self.data.push(value_ptr);
    }

    #[inline]
    pub(crate) fn pop<T>(&mut self) -> Option<T>
    where
        T: any::Any,
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
    pub(crate) fn replace_insert<T>(&mut self, index: usize, value: T)
    where
        T: any::Any,
    {
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
    pub(crate) fn shift_insert<T>(&mut self, index: usize, value: T)
    where
        T: any::Any,
    {
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
    pub(crate) fn swap_remove<T>(&mut self, index: usize) -> T
    where
        T: any::Any,
    {
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
    pub(crate) fn shift_remove<T>(&mut self, index: usize) -> T
    where
        T: any::Any,
    {
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
    pub(crate) fn contains<T>(&self, value: &T) -> bool
    where
        T: any::Any + PartialEq,
    {
        self.as_slice::<T>().contains(value)
    }

    #[inline]
    pub const fn as_ptr<T>(&self) -> *const T
    where
        T: any::Any,
    {
        self.data.as_ptr() as *const T
    }

    #[inline]
    pub(crate) const fn as_mut_ptr<T>(&mut self) -> *mut T
    where
        T: any::Any,
    {
        self.data.as_mut_ptr() as *mut T
    }

    #[inline]
    pub(crate) const fn as_non_null<T>(&mut self) -> NonNull<T>
    where
        T: any::Any,
    {
        // SAFETY: An [`OpaqueVec`] always holds a non-null pointer.
        self.data.as_non_null().cast::<T>()
    }

    #[inline]
    pub(crate) fn as_slice<T>(&self) -> &[T]
    where
        T: any::Any,
    {
        unsafe {
            let data_ptr = self.data.as_ptr() as *const T;
            let len = self.data.len();

            core::slice::from_raw_parts(data_ptr, len)
        }
    }

    #[inline]
    pub(crate) fn as_mut_slice<T>(&mut self) -> &mut [T]
    where
        T: any::Any,
    {
        unsafe {
            let data_ptr = self.data.as_mut_ptr() as *mut T;
            let len = self.data.len();

            core::slice::from_raw_parts_mut(data_ptr, len)
        }
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_raw_parts<T>(self) -> (*mut T, usize, usize)
    where
        T: any::Any,
    {
        let mut me = ManuallyDrop::new(self);
        let ptr = me.as_mut_ptr();
        let len = me.len();
        let capacity = me.capacity();

        (ptr, len, capacity)
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_parts<T>(self) -> (NonNull<T>, usize, usize)
    where
        T: any::Any,
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
    pub(crate) fn into_raw_parts_with_alloc<T, A>(self) -> (*mut T, usize, usize, TypedProjAlloc<A>)
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let mut me = ManuallyDrop::new(self);
        let ptr = me.as_mut_ptr();
        let len = me.len();
        let capacity = me.capacity();
        let alloc = unsafe { core::ptr::read(me.allocator::<T, A>()) };

        (ptr, len, capacity, alloc)
    }

    #[inline]
    #[must_use]
    pub(crate) fn into_parts_with_alloc<T, A>(self) -> (NonNull<T>, usize, usize, TypedProjAlloc<A>)
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let mut me = ManuallyDrop::new(self);

        // SAFETY: An `OpaqueVec` always has a non-null pointer.
        let ptr = unsafe { NonNull::new_unchecked(me.as_mut_ptr()) };
        let len = me.len();
        let capacity = me.capacity();
        let alloc = unsafe { core::ptr::read(me.allocator::<T, A>()) };

        (ptr, len, capacity, alloc)
    }

    #[inline]
    pub(crate) fn spare_capacity_mut<T>(&mut self) -> &mut [MaybeUninit<T>]
    where
        T: any::Any,
    {
        unsafe {
            let ptr = self.as_mut_ptr::<T>().add(self.len()) as *mut MaybeUninit<T>;
            let len = self.capacity() - self.len();

            std::slice::from_raw_parts_mut(ptr, len)
        }
    }

    pub(crate) fn drain<R, T, A>(&mut self, range: R) -> Drain<'_, T, A>
    where
        T: any::Any,
        A: Allocator + any::Any,
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
            let range_slice = slice::from_raw_parts(self.as_ptr::<T>().add(start), end - start);
            Drain {
                tail_start: end,
                tail_len: len - end,
                iter: range_slice.iter(),
                vec: NonNull::from(self),
                _marker: PhantomData,
            }
        }
    }
}

impl OpaqueVecInner {
    #[inline]
    #[must_use]
    pub(crate) fn get<T>(&self, index: usize) -> Option<&T>
    where
        T: any::Any,
    {
        if index >= self.data.len() {
            return None;
        }

        let ptr = self.get_unchecked(index);

        Some(ptr)
    }

    #[inline]
    #[must_use]
    pub(crate) fn get_mut<T>(&mut self, index: usize) -> Option<&mut T>
    where
        T: any::Any,
    {
        if index >= self.data.len() {
            return None;
        }

        let ptr = self.get_mut_unchecked(index);

        Some(ptr)
    }

    #[inline]
    pub(crate) fn push_within_capacity<T>(&mut self, value: T) -> Result<(), T>
    where
        T: any::Any,
    {
        if self.data.len() == self.data.capacity() {
            return Err(value);
        }

        self.push::<T>(value);

        Ok(())
    }

    /*
    #[inline]
    pub(crate) fn into_iter<T>(self) -> IntoIter<T, OpaqueAlloc>
    where
        T: any::Any,
    {
        IntoIter {
            inner: self,
            _marker: PhantomData,
        }
    }
    */

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub(crate) fn append<T>(&mut self, other: &mut Self)
    where
        T: any::Any,
    {
        unsafe {
            let ptr = NonNull::new_unchecked(other.as_mut_slice::<T>().as_mut_ptr().cast::<u8>());
            let count = other.len();

            self.data.append(ptr, count);
            other.set_len(0);
        }
    }

    #[inline]
    pub(crate) fn as_byte_slice(&self) -> &[u8] {
        self.data.as_byte_slice()
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub(crate) fn into_boxed_slice<T, A>(mut self) -> Box<[T], TypedProjAlloc<A>>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        unsafe {
            self.shrink_to_fit();
            let mut me = ManuallyDrop::new(self);
            let len = me.len();
            let ptr = me.as_mut_ptr::<T>();
            let slice_ptr = std::ptr::slice_from_raw_parts_mut(ptr, len);
            let alloc = core::ptr::read(me.allocator::<T, A>());

            Box::from_raw_in(slice_ptr, alloc)
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    pub(crate) fn split_off<T, A>(&mut self, at: usize) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any + Clone,
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
            let cloned_alloc = self.allocator::<T, A>().clone();
            let box_alloc = cloned_alloc.into_box_alloc();
            let split_alloc = TypedProjAlloc::from_boxed_alloc(box_alloc);

            OpaqueVecInner::with_capacity_proj_in::<T, A>(other_len, split_alloc)
        };

        // Unsafely `set_len` and copy items to `other`.
        unsafe {
            self.set_len(at);
            other.set_len(other_len);

            core::ptr::copy_nonoverlapping(self.as_ptr::<T>().add(at), other.as_mut_ptr::<T>(), other.len());
        }

        other
    }

    #[inline]
    pub(crate) fn resize_with<F, T>(&mut self, new_len: usize, f: F)
    where
        T: any::Any,
        F: FnMut() -> T,
    {
        let len = self.len();
        if new_len > len {
            self.extend::<T, _>(core::iter::repeat_with(f).take(new_len - len));
        } else {
            self.truncate(new_len);
        }
    }
}

impl OpaqueVecInner {
    #[inline]
    pub(crate) fn try_reserve(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.data.try_reserve(additional)
    }

    #[inline]
    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.data.try_reserve_exact(additional)
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub(crate) fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    #[cfg(not(no_global_oom_handling))]
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

    #[cfg(not(no_global_oom_handling))]
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

impl OpaqueVecInner {
    #[inline]
    pub(crate) fn extend_with<T>(&mut self, count: usize, value: T)
    where
        T: any::Any + Clone,
    {
        let value_ptr = unsafe { NonNull::new_unchecked(&value as *const T as *mut T as *mut u8) };

        self.data.extend_with(count, value_ptr);
    }

    #[inline]
    pub(crate) fn extend_from_iter<T, I>(&mut self, mut iterator: I)
    where
        T: any::Any + Clone,
        I: Iterator<Item = T>,
    {
        for item in iterator {
            self.push::<T>(item);
        }
    }

    #[inline]
    pub(crate) fn extend_from_slice<T>(&mut self, other: &[T])
    where
        T: any::Any + Clone,
    {
        self.extend_from_iter::<T, _>(other.iter().cloned())
    }

    #[inline]
    pub(crate) fn resize<T>(&mut self, new_len: usize, value: T)
    where
        T: any::Any + Clone,
    {
        let len = self.len();

        if new_len > len {
            self.extend_with(new_len - len, value)
        } else {
            self.truncate(new_len);
        }
    }
}

impl OpaqueVecInner {
    pub(crate) fn retain<F, T, A>(&mut self, mut f: F)
    where
        T: any::Any,
        A: Allocator + any::Any,
        F: FnMut(&T) -> bool,
    {
        self.retain_mut::<_, T, A>(|elem| f(elem));
    }

    pub(crate) fn retain_mut<F, T, A>(&mut self, mut f: F)
    where
        T: any::Any,
        A: Allocator + any::Any,
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
            A: Allocator,
        {
            v: &'a mut OpaqueVecInner,
            processed_len: usize,
            deleted_cnt: usize,
            original_len: usize,
            _marker: PhantomData<(T, A)>,
        }

        impl<T, A> Drop for BackshiftOnDrop<'_, T, A>
        where
            T: any::Any,
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

        fn process_loop<F, T, A, const DELETED: bool>(original_len: usize, f: &mut F, g: &mut BackshiftOnDrop<'_, T, A>)
        where
            T: any::Any,
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
        process_loop::<F, T, A, false>(original_len, &mut f, &mut g);

        // Stage 2: Some elements were deleted.
        process_loop::<F, T, A, true>(original_len, &mut f, &mut g);

        // All item are processed. This can be optimized to `set_len` by LLVM.
        drop(g);
    }

    pub(crate) fn dedup_by<F, T, A>(&mut self, mut same_bucket: F)
    where
        T: any::Any,
        A: Allocator + any::Any,
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
            T: any::Any,
            A: core::alloc::Allocator,
        {
            /* Offset of the element we want to check if it is duplicate */
            read: usize,

            /* Offset of the place where we want to place the non-duplicate
             * when we find it. */
            write: usize,

            /* The Vec that would need correction if `same_bucket` panicked */
            vec: &'a mut OpaqueVecInner,
            _marker: PhantomData<(T, A)>,
        }

        impl<'a, T, A> Drop for FillGapOnDrop<'a, T, A>
        where
            T: any::Any,
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
        let mut gap: FillGapOnDrop<'_, T, A> = FillGapOnDrop {
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

    #[inline]
    pub(crate) fn dedup_by_key<F, K, T, A>(&mut self, mut key: F)
    where
        T: any::Any,
        A: Allocator + any::Any,
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.dedup_by::<_, T, A>(|a, b| key(a) == key(b))
    }
}

impl OpaqueVecInner {
    #[cfg(not(no_global_oom_handling))]
    #[inline]
    pub(crate) fn splice<R, I, T, A>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
    where
        T: any::Any,
        A: Allocator + any::Any,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item=T>,
    {
        Splice {
            drain: self.drain(range),
            replace_with: replace_with.into_iter(),
        }
    }

    #[inline]
    pub(crate) fn extract_if<F, R, T, A>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, A>
    where
        T: any::Any,
        A: Allocator + any::Any,
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        ExtractIf::new(self, filter, range)
    }

    #[inline]
    pub(crate) fn truncate(&mut self, len: usize) {
        self.data.truncate(len);
    }

    #[inline]
    pub(crate) fn extend<T, I>(&mut self, iter: I)
    where
        T: any::Any,
        I: IntoIterator<Item=T>,
    {
        let mut extender = Extender::new(self);
        extender.extend(iter)
    }

    /*
    #[inline]
    pub(crate) fn reverse<T>(&mut self)
    where
        T: any::Any,
    {
        self.as_mut_slice::<T>().reverse();
    }
    */
}

impl OpaqueVecInner {
    #[inline]
    pub(crate) fn clone<T, A>(&self) -> Self
    where
        T: any::Any + Clone,
        A: Allocator + any::Any + Clone,
    {
        unsafe fn drop_fn<T>(value: NonNull<u8>) {
            let to_drop = value.as_ptr() as *mut T;

            core::ptr::drop_in_place(to_drop)
        }

        let new_data = {
            let new_alloc = {
                let proj_old_alloc = self.data.allocator().as_proj::<A>();
                let proj_new_alloc = Clone::clone(proj_old_alloc);
                OpaqueAlloc::from_proj(proj_new_alloc)
            };
            let new_element_layout = self.data.element_layout();
            let new_capacity = self.data.capacity();
            let new_drop_fn = Some(drop_fn::<T> as unsafe fn(NonNull<u8>));
            let new_data = unsafe {
                let mut _new_data = OpaqueBlobVec::with_capacity_in(new_capacity, new_alloc, new_element_layout, new_drop_fn);
                let length = self.data.len();
                let old_data_ptr = NonNull::new_unchecked(self.data.as_ptr() as *mut u8);
                _new_data.append(old_data_ptr, length);
                _new_data
            };

            new_data
        };
        let new_type_id = self.type_id;
        let new_alloc_type_id = self.alloc_type_id;

        Self {
            data: new_data,
            type_id: new_type_id,
            alloc_type_id: new_alloc_type_id,
        }
    }
}

struct DebugDisplayDataFormatter<'a> {
    inner: &'a OpaqueVecInner,
}

impl<'a> DebugDisplayDataFormatter<'a> {
    #[inline]
    const fn new(inner: &'a OpaqueVecInner) -> Self {
        Self { inner }
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

impl fmt::Debug for OpaqueVecInner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_display = DebugDisplayDataFormatter::new(&self);

        formatter
            .debug_struct("OpaqueVecInner")
            .field("element_layout", &self.element_layout())
            .field("capacity", &self.capacity())
            .field("length", &self.len())
            .field("data", &data_display)
            .finish()
    }
}

impl fmt::Display for OpaqueVecInner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_display = DebugDisplayDataFormatter::new(&self);

        data_display.fmt_data(formatter)
    }
}

mod private {
    use super::OpaqueVecInner;
    use std::alloc::Allocator;
    use core::any;

    // We shouldn't add inline attribute to this since this is used in
    // `vec!` macro mostly and causes perf regression. See #71204 for
    // discussion and perf results.
    #[allow(missing_docs)]
    pub fn into_opaque_vec<T, A>(b: Box<[T], A>) -> OpaqueVecInner
    where
        T: any::Any,
        A: Allocator + any::Any + Clone,
    {
        unsafe {
            let len = b.len();
            let (b, alloc) = Box::into_raw_with_allocator(b);
            OpaqueVecInner::from_raw_parts_in(b as *mut T, len, len, alloc)
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[allow(missing_docs)]
    #[inline]
    pub fn to_opaque_vec<T, A>(slice: &[T], alloc: A) -> OpaqueVecInner
    where
        T: ConvertOpaqueVec,
        A: Allocator + any::Any + Clone,
    {
        T::to_opaque_vec(slice, alloc)
    }

    #[cfg(not(no_global_oom_handling))]
    pub trait ConvertOpaqueVec {
        fn to_opaque_vec<A>(slice: &[Self], alloc: A) -> OpaqueVecInner
        where
            A: Allocator + any::Any + Clone,
            Self: Sized;
    }

    #[cfg(not(no_global_oom_handling))]
    impl<T> ConvertOpaqueVec for T
    where
        T: any::Any + Clone,
    {
        #[inline]
        fn to_opaque_vec<A>(slice: &[Self], alloc: A) -> OpaqueVecInner
        where
            A: Allocator + any::Any + Clone,
        {
            struct DropGuard<'a> {
                vec: &'a mut OpaqueVecInner,
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

            let mut vec = OpaqueVecInner::with_capacity_in::<Self, A>(slice.len(), alloc);
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

impl<T> From<&[T]> for OpaqueVecInner
where
    T: any::Any + Clone,
{
    fn from(slice: &[T]) -> Self {
        private::to_opaque_vec::<T, Global>(slice, Global)
    }
}

impl<T> From<&mut [T]> for OpaqueVecInner
where
    T: any::Any + Clone,
{
    fn from(slice: &mut [T]) -> Self {
        private::to_opaque_vec::<T, Global>(slice, Global)
    }
}

impl<const N: usize, T> From<&[T; N]> for OpaqueVecInner
where
    T: any::Any + Clone,
{
    fn from(array: &[T; N]) -> Self {
        Self::from(array.as_slice())
    }
}

impl<const N: usize, T> From<&mut [T; N]> for OpaqueVecInner
where
    T: any::Any + Clone,
{
    fn from(array: &mut [T; N]) -> Self {
        Self::from(array.as_mut_slice())
    }
}

impl<T, A> From<&Vec<T, A>> for OpaqueVecInner
where
    T: any::Any + Clone,
    A: Allocator + any::Any + Clone,
{
    fn from(vec: &Vec<T, A>) -> Self {
        Self::from(vec.as_slice())
    }
}

impl<T, A> From<&mut Vec<T, A>> for OpaqueVecInner
where
    T: any::Any + Clone,
    A: Allocator + any::Any + Clone,
{
    fn from(vec: &mut Vec<T, A>) -> Self {
        Self::from(vec.as_mut_slice())
    }
}
/*
impl<T> From<Box<[T]>> for OpaqueVecInner
where
    T: any::Any + Clone,
{
    fn from(slice: Box<[T]>) -> Self {
        Self::from(slice.as_ref())
    }
}
*/

impl<T, A> From<Box<[T], A>> for OpaqueVecInner
where
    T: any::Any + Clone,
    A: Allocator + any::Any + Clone,
{
    fn from(slice: Box<[T], A>) -> Self {
        Self::from(slice.as_ref())
    }
}

impl<const N: usize, T> From<[T; N]> for OpaqueVecInner
where
    T: any::Any,
{
    fn from(array: [T; N]) -> Self {
        private::into_opaque_vec::<T, Global>(Box::new(array))
    }
}

impl<T> FromIterator<T> for OpaqueVecInner
where
    T: any::Any,
{
    #[inline]
    #[track_caller]
    fn from_iter<I>(iter: I) -> OpaqueVecInner
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let (lower, _) = iter.size_hint();

        let mut vec = OpaqueVecInner::with_capacity::<T>(lower);

        for item in iter {
            vec.push::<T>(item);
        }

        vec
    }
}

#[repr(transparent)]
pub struct TypedProjVec<T, A: Allocator> {
    inner: OpaqueVecInner,
    _marker: core::marker::PhantomData<(T, A)>,
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_proj_in(proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = OpaqueVecInner::new_proj_in::<T, A>(proj_alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = OpaqueVecInner::with_capacity_proj_in::<T, A>(capacity, proj_alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn try_with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Result<Self, opaque_error::TryReserveError> {
        let inner = OpaqueVecInner::try_with_capacity_proj_in::<T, A>(capacity, proj_alloc)?;

        Ok(Self {
            inner,
            _marker: core::marker::PhantomData,
        })
    }

    #[inline]
    pub unsafe fn from_raw_parts_proj_in(ptr: *mut T, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = OpaqueVecInner::from_raw_parts_proj_in::<T, A>(ptr, length, capacity, proj_alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub unsafe fn from_parts_proj_in(ptr: NonNull<T>, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let inner = OpaqueVecInner::from_parts_proj_in::<T, A>(ptr, length, capacity, proj_alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_in(alloc: A) -> Self {
        let inner = OpaqueVecInner::new_in::<T, A>(alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let inner = OpaqueVecInner::with_capacity_in::<T, A>(capacity, alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Self, opaque_error::TryReserveError> {
        let inner = OpaqueVecInner::try_with_capacity_in::<T, A>(capacity, alloc)?;

        Ok(Self {
            inner,
            _marker: core::marker::PhantomData,
        })
    }

    #[inline]
    pub unsafe fn from_raw_parts_in(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self {
        let inner = OpaqueVecInner::from_raw_parts_in::<T, A>(ptr, length, capacity, alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub unsafe fn from_parts_in(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self {
        let inner = OpaqueVecInner::from_parts_in::<T, A>(ptr, length, capacity, alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
        self.inner.allocator::<T, A>()
    }
}

impl<T> TypedProjVec<T, Global>
where
    T: any::Any,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new() -> Self {
        let inner = OpaqueVecInner::new::<T>();

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity(capacity: usize) -> Self {
        let inner = OpaqueVecInner::with_capacity::<T>(capacity);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn try_with_capacity(capacity: usize) -> Result<Self, opaque_error::TryReserveError> {
        let inner = OpaqueVecInner::try_with_capacity::<T>(capacity)?;

        Ok(Self {
            inner,
            _marker: core::marker::PhantomData,
        })
    }

    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        let inner = OpaqueVecInner::from_raw_parts::<T>(ptr, length, capacity);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub unsafe fn from_parts(ptr: NonNull<T>, length: usize, capacity: usize) -> Self {
        let inner = OpaqueVecInner::from_parts::<T>(ptr, length, capacity);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    #[inline]
    pub const fn element_layout(&self) -> Layout {
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
    unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len);
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get::<T>(index)
    }

    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut::<T>(index)
    }

    #[inline]
    #[track_caller]
    pub fn push(&mut self, value: T) {
        self.inner.push::<T>(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop::<T>()
    }

    #[inline]
    pub fn push_within_capacity(&mut self, value: T) -> Result<(), T> {
        self.inner.push_within_capacity::<T>(value)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn replace_insert(&mut self, index: usize, value: T) {
        self.inner.replace_insert::<T>(index, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shift_insert(&mut self, index: usize, value: T) {
        self.inner.shift_insert::<T>(index, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.inner.swap_remove::<T>(index)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shift_remove(&mut self, index: usize) -> T {
        self.inner.shift_remove::<T>(index)
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.inner.contains::<T>(value)
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.inner.iter::<T>()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.inner.iter_mut::<T>()
    }

    /*
    pub fn into_iter(self) -> IntoIter<T, OpaqueAlloc> {
        self.inner.into_iter::<T>()
    }
    */

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub fn append(&mut self, other: &mut Self) {
        self.inner.append::<T>(&mut other.inner)
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
    where
        R: ops::RangeBounds<usize>,
    {
        self.inner.drain::<R, T, A>(range)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.inner.as_ptr::<T>()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.as_mut_ptr::<T>()
    }

    #[inline]
    pub fn as_non_null(&mut self) -> NonNull<T> {
        self.inner.as_non_null::<T>()
    }

    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice::<T>()
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.as_mut_slice::<T>()
    }

    pub fn as_byte_slice(&self) -> &[u8] {
        self.inner.as_byte_slice()
    }

    #[must_use]
    pub fn into_raw_parts(self) -> (*mut T, usize, usize) {
        self.inner.into_raw_parts::<T>()
    }

    #[must_use]
    pub fn into_parts(self) -> (NonNull<T>, usize, usize) {
        self.inner.into_parts::<T>()
    }

    #[must_use]
    pub fn into_raw_parts_with_alloc(self) -> (*mut T, usize, usize, TypedProjAlloc<A>) {
        self.inner.into_raw_parts_with_alloc::<T, A>()
    }

    pub fn into_parts_with_alloc(self) -> (NonNull<T>, usize, usize, TypedProjAlloc<A>) {
        self.inner.into_parts_with_alloc::<T, A>()
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn into_boxed_slice(self) -> Box<[T], TypedProjAlloc<A>> {
        self.inner.into_boxed_slice::<T, A>()
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    #[track_caller]
    pub fn split_off(&mut self, at: usize) -> Self
    where
        A: Clone,
    {
        let inner = self.inner.split_off::<T, A>(at);

        Self {
            inner,
            _marker: PhantomData,
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T,
    {
        self.inner.resize_with::<F, T>(new_len, f)
    }

    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        self.inner.spare_capacity_mut::<T>()
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.inner.try_reserve(additional)
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    #[track_caller]
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    #[cfg(not(no_global_oom_handling))]
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
    A: Allocator + any::Any,
{
    #[cfg(not(no_global_oom_handling))]
    #[inline]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
    where
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        self.inner.splice::<R, I, T, A>(range, replace_with)
    }

    pub fn extract_if<F, R>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, A>
    where
        T: any::Any,
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        self.inner.extract_if::<F, R, T, A>(range, filter)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn extend_with(&mut self, count: usize, value: T)
    where
        T: Clone,
    {
        self.inner.extend_with::<T>(count, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn extend_from_iter<I>(&mut self, iterator: I)
    where
        T: Clone,
        I: Iterator<Item = T>,
    {
        self.inner.extend_from_iter::<T, _>(iterator)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.inner.extend_from_slice::<T>(other);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        self.inner.resize::<T>(new_len, value);
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }
}

impl<T, A> TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.inner.retain::<_, T, A>(|elem| f(elem));
    }

    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.inner.retain_mut::<F, T, A>(f)
    }

    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, mut key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.inner.dedup_by_key::<F, K, T, A>(key)
    }

    pub fn dedup_by<F>(&mut self, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.inner.dedup_by::<F, T, A>(same_bucket)
    }
}
/*
impl<T> TypedProjVec<T>
where
    T: any::Any,
{
    #[inline]
    pub fn reverse(&mut self)
    where
        T: any::Any,
    {
        self.inner.reverse::<T>()
    }
}
*/
impl<T, A> ops::Deref for TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
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
    A: Allocator + any::Any,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

/*
unsafe impl<T, A: Allocator> ops::DerefPure for Vec<T, A> {}
*/

impl<T, A> Clone for TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: Allocator + any::Any + Clone,
{
    fn clone(&self) -> Self {
        let cloned_inner = self.inner.clone::<T, A>();

        Self {
            inner: cloned_inner,
            _marker: PhantomData,
        }
    }
}

impl<T, A> hash::Hash for TypedProjVec<T, A>
where
    T: any::Any + hash::Hash,
    A: Allocator + any::Any,
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
    A: Allocator + any::Any,
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
    A: Allocator + any::Any,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        ops::IndexMut::index_mut(self.as_mut_slice(), index)
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T> FromIterator<T> for TypedProjVec<T, Global>
where
    T: any::Any,
{
    #[inline]
    #[track_caller]
    fn from_iter<I>(iter: I) -> TypedProjVec<T, Global>
    where
        I: IntoIterator<Item = T>,
    {
        let inner = OpaqueVecInner::from_iter(iter);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<T, A> IntoIterator for TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
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
            let end = if is_zst::<T>() {
                begin.wrapping_byte_add(me.len())
            } else {
                begin.add(me.len()) as *const T
            };
            let cap = me.inner.capacity();

            IntoIter { buf: inner, cap, alloc, ptr: inner, end, _marker: PhantomData, }
        }
    }
}

impl<'a, T, A> IntoIterator for &'a TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
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
    A: Allocator + any::Any,
{
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, A> Extend<T> for TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    #[inline]
    #[track_caller]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.inner.extend::<T, I>(iter)
    }

    /*
    #[inline]
    #[track_caller]
    fn extend_one(&mut self, item: T) {
        self.inner.push::<T>(item);
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

#[cfg(not(no_global_oom_handling))]
impl<'a, T, A> Extend<&'a T> for TypedProjVec<T, A>
where
    T: any::Any + Copy + 'a,
    A: Allocator + any::Any,
{
    #[track_caller]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        self.inner.extend::<T, _>(iter.into_iter().copied())
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
    A1: Allocator + any::Any,
    A2: Allocator + any::Any,
{
    fn eq(&self, other: &TypedProjVec<T, A2>) -> bool {
        PartialEq::eq(self.as_slice(), other.as_slice())
    }
}

impl<T, A1, A2> PartialOrd<TypedProjVec<T, A2>> for TypedProjVec<T, A1>
where
    T: any::Any + PartialOrd,
    A1: Allocator + any::Any,
    A2: Allocator + any::Any,
{
    #[inline]
    fn partial_cmp(&self, other: &TypedProjVec<T, A2>) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(self.as_slice(), other.as_slice())
    }
}

impl<T, A> Eq for TypedProjVec<T, A>
where
    T: any::Any + Eq,
    A: Allocator + any::Any,
{
}

impl<T, A> Ord for TypedProjVec<T, A>
where
    T: any::Any + Ord,
    A: Allocator + any::Any,
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
    A: Allocator + any::Any,
{
    fn drop(&mut self) {

    }
}
*/
impl<T> Default for TypedProjVec<T, Global>
where
    T: any::Any,
{
    fn default() -> TypedProjVec<T, Global> {
        TypedProjVec::new()
    }
}

impl<T, A> fmt::Debug for TypedProjVec<T, A>
where
    T: any::Any + fmt::Debug,
    A: Allocator + any::Any,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl<T, A> AsRef<TypedProjVec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn as_ref(&self) -> &TypedProjVec<T, A> {
        self
    }
}

impl<T, A> AsMut<TypedProjVec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn as_mut(&mut self) -> &mut TypedProjVec<T, A> {
        self
    }
}

impl<T, A> AsRef<[T]> for TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, A> AsMut<[T]> for TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T> From<&[T]> for TypedProjVec<T, Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &[T]) -> TypedProjVec<T, Global> {
        let inner = OpaqueVecInner::from(slice);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T> From<&mut [T]> for TypedProjVec<T, Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &mut [T]) -> TypedProjVec<T, Global> {
        let inner = OpaqueVecInner::from(slice);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, const N: usize> From<&[T; N]> for TypedProjVec<T, Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &[T; N]) -> TypedProjVec<T, Global> {
        Self::from(slice.as_slice())
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, const N: usize> From<&mut [T; N]> for TypedProjVec<T, Global>
where
    T: any::Any + Clone,
{
    #[track_caller]
    fn from(slice: &mut [T; N]) -> TypedProjVec<T, Global> {
        Self::from(slice.as_mut_slice())
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, const N: usize> From<[T; N]> for TypedProjVec<T, Global>
where
    T: any::Any,
{
    #[track_caller]
    fn from(slice: [T; N]) -> TypedProjVec<T, Global> {
        /*
        <[T]>::into_vec(Box::new(slice))
         */
        todo!()
    }
}

impl<'a, T> From<borrow::Cow<'a, [T]>> for TypedProjVec<T, Global>
where
    T: any::Any,
    [T]: ToOwned<Owned = TypedProjVec<T, Global>>,
{
    #[track_caller]
    fn from(slice: borrow::Cow<'a, [T]>) -> TypedProjVec<T, Global> {
        slice.into_owned()
    }
}

impl<T, A> From<Box<[T], A>> for TypedProjVec<T, A>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    fn from(slice: Box<[T], A>) -> Self {
        /*
        slice.into_vec()
         */
        todo!()
    }
}
/*
#[cfg(not(no_global_oom_handling))]
impl<T, A> From<Vec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: Allocator + any::Any,
{
    #[track_caller]
    fn from(vec: Vec<T, A>) -> Self {
        let inner = OpaqueVecInner::from(vec);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}
*/
#[cfg(not(no_global_oom_handling))]
impl<T, A> From<&Vec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: Allocator + any::Any + Clone,
{
    #[track_caller]
    fn from(vec: &Vec<T, A>) -> Self {
        let inner = OpaqueVecInner::from(vec);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, A> From<&mut Vec<T, A>> for TypedProjVec<T, A>
where
    T: any::Any + Clone,
    A: Allocator + any::Any + Clone,
{
    #[track_caller]
    fn from(vec: &mut Vec<T, A>) -> Self {
        let inner = OpaqueVecInner::from(vec);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, A> From<TypedProjVec<T, A>> for Box<[T], TypedProjAlloc<A>>
where
    T: any::Any,
    A: Allocator + any::Any,
{
    #[track_caller]
    fn from(vec: TypedProjVec<T, A>) -> Self {
        vec.into_boxed_slice()
    }
}

#[cfg(not(no_global_oom_handling))]
impl From<&str> for TypedProjVec<u8, Global> {
    #[track_caller]
    fn from(st: &str) -> TypedProjVec<u8, Global> {
        From::from(st.as_bytes())
    }
}

impl<T, A, const N: usize> TryFrom<TypedProjVec<T, A>> for [T; N]
where
    T: any::Any,
    A: Allocator + any::Any,
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
    #[must_use]
    #[track_caller]
    pub fn new_proj_in<T, A>(proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
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
        A: Allocator + any::Any,
    {
        let proj_vec = TypedProjVec::<T, A>::with_capacity_proj_in(capacity, proj_alloc);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub fn try_with_capacity_proj_in<T, A>(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Result<Self, opaque_error::TryReserveError>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_vec = TypedProjVec::<T, A>::try_with_capacity_proj_in(capacity, proj_alloc)?;

        Ok(Self::from_proj(proj_vec))
    }

    #[inline]
    pub unsafe fn from_raw_parts_proj_in<T, A>(ptr: *mut T, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_vec = TypedProjVec::<T, A>::from_raw_parts_proj_in(ptr, length, capacity, proj_alloc);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub unsafe fn from_parts_proj_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_vec = TypedProjVec::<T, A>::from_parts_proj_in(ptr, length, capacity, proj_alloc);

        Self::from_proj(proj_vec)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_in<T, A>(alloc: A) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
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
        A: Allocator + any::Any,
    {
        let proj_vec = TypedProjVec::<T, A>::with_capacity_in(capacity, alloc);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub fn try_with_capacity_in<T, A>(capacity: usize, alloc: A) -> Result<Self, opaque_error::TryReserveError>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_vec = TypedProjVec::<T, A>::try_with_capacity_in(capacity, alloc)?;

        Ok(Self::from_proj(proj_vec))
    }

    #[inline]
    pub unsafe fn from_raw_parts_in<T, A>(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_vec = TypedProjVec::<T, A>::from_raw_parts_in(ptr, length, capacity, alloc);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub unsafe fn from_parts_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_vec = TypedProjVec::<T, A>::from_parts_in(ptr, length, capacity, alloc);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub fn allocator<T, A>(&self) -> &TypedProjAlloc<A>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj::<T, A>();

        proj_self.allocator()
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
        let proj_vec = TypedProjVec::<T, Global>::new();

        Self::from_proj(proj_vec)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity<T>(capacity: usize) -> Self
    where
        T: any::Any,
    {
        let proj_vec = TypedProjVec::<T, Global>::with_capacity(capacity);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub fn try_with_capacity<T>(capacity: usize) -> Result<Self, opaque_error::TryReserveError>
    where
        T: any::Any,
    {
        let proj_vec = TypedProjVec::<T, Global>::try_with_capacity(capacity)?;

        Ok(Self::from_proj(proj_vec))
    }

    #[inline]
    pub unsafe fn from_raw_parts<T>(ptr: *mut T, length: usize, capacity: usize) -> Self
    where
        T: any::Any,
    {
        let proj_vec = TypedProjVec::<T, Global>::from_raw_parts(ptr, length, capacity);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub unsafe fn from_parts<T>(ptr: NonNull<T>, length: usize, capacity: usize) -> Self
    where
        T: any::Any,
    {
        let proj_vec = TypedProjVec::<T, Global>::from_parts(ptr, length, capacity);

        Self::from_proj(proj_vec)
    }
}

impl OpaqueVec {
    #[inline]
    pub fn has_element_type<T>(&self) -> bool
    where
        T: any::Any,
    {
        self.inner.has_element_type::<T>()
    }

    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: Allocator + any::Any + Clone,
    {
        self.inner.has_allocator_type::<A>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<T>(&self)
    where
        T: any::Any,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: TypeId, type_id_other: TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_element_type::<T>() {
            type_check_failed(self.inner.type_id, TypeId::of::<T>());
        }
    }
}

impl OpaqueVec {
    pub fn as_proj<T, A>(&self) -> &TypedProjVec<T, A>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        self.assert_type_safety::<T>();

        unsafe { &*(self as *const OpaqueVec as *const TypedProjVec<T, A>) }
    }

    pub fn as_proj_mut<T, A>(&mut self) -> &mut TypedProjVec<T, A>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        self.assert_type_safety::<T>();

        unsafe { &mut *(self as *mut OpaqueVec as *mut TypedProjVec<T, A>) }
    }

    pub fn into_proj<T, A>(self) -> TypedProjVec<T, A>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        self.assert_type_safety::<T>();

        TypedProjVec {
            inner: self.inner,
            _marker: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_proj<T, A>(proj_self: TypedProjVec<T, A>) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        Self {
            inner: proj_self.inner,
        }
    }
}

impl OpaqueVec {
    #[inline]
    pub const fn element_layout(&self) -> Layout {
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
    unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len);
    }
}

impl OpaqueVec {
    #[inline]
    #[must_use]
    pub fn get<T, A>(&self, index: usize) -> Option<&T>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj::<T, A>();
        proj_self.get(index)
    }

    #[inline]
    #[must_use]
    pub fn get_mut<T, A>(&mut self, index: usize) -> Option<&mut T>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.get_mut(index)
    }

    #[inline]
    #[track_caller]
    pub fn push<T, A>(&mut self, value: T)
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.push(value);
    }

    #[inline]
    pub fn pop<T, A>(&mut self) -> Option<T>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.pop()
    }

    #[inline]
    pub fn push_within_capacity<T, A>(&mut self, value: T) -> Result<(), T>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.push_within_capacity(value)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn replace_insert<T, A>(&mut self, index: usize, value: T)
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.replace_insert(index, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shift_insert<T, A>(&mut self, index: usize, value: T)
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.shift_insert(index, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn swap_remove<T, A>(&mut self, index: usize) -> T
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.swap_remove(index)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shift_remove<T, A>(&mut self, index: usize) -> T
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.shift_remove(index)
    }

    pub fn contains<T, A>(&self, value: &T) -> bool
    where
        T: any::Any + PartialEq,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj::<T, A>();
        proj_self.contains(value)
    }

    pub fn iter<T, A>(&self) -> slice::Iter<'_, T>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj::<T, A>();
        proj_self.iter()
    }

    pub fn iter_mut<T, A>(&mut self) -> slice::IterMut<'_, T>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.iter_mut()
    }

    pub fn into_iter<T, A>(self) -> IntoIter<T, A>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.into_proj::<T, A>();
        proj_self.into_iter()
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub fn append<T, A>(&mut self, other: &mut Self)
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        let proj_other = other.as_proj_mut::<T, A>();
        proj_self.append(proj_other)
    }

    pub fn drain<R, T, A>(&mut self, range: R) -> Drain<'_, T, A>
    where
        T: any::Any,
        A: Allocator + any::Any,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.drain(range)
    }

    #[inline]
    pub fn as_ptr<T, A>(&self) -> *const T
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj::<T, A>();
        proj_self.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr<T, A>(&mut self) -> *mut T
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.as_mut_ptr()
    }

    #[inline]
    pub fn as_non_null<T, A>(&mut self) -> NonNull<T>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.as_non_null()
    }

    pub fn as_slice<T, A>(&self) -> &[T]
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj::<T, A>();
        proj_self.as_slice()
    }

    pub fn as_mut_slice<T, A>(&mut self) -> &mut [T]
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.as_mut_slice()
    }

    pub fn as_byte_slice<T, A>(&self) -> &[u8]
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj::<T, A>();
        proj_self.as_byte_slice()
    }

    #[must_use]
    pub fn into_raw_parts<T>(self) -> (*mut T, usize, usize)
    where
        T: any::Any,
    {
        let proj_self = self.into_proj::<T, Global>();
        proj_self.into_raw_parts()
    }

    #[must_use]
    pub fn into_parts<T>(self) -> (NonNull<T>, usize, usize)
    where
        T: any::Any,
    {
        let proj_self = self.into_proj::<T, Global>();
        proj_self.into_parts()
    }

    #[must_use]
    pub fn into_raw_parts_with_alloc<T, A>(self) -> (*mut T, usize, usize, TypedProjAlloc<A>)
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.into_proj::<T, A>();
        proj_self.into_raw_parts_with_alloc()
    }

    pub fn into_parts_with_alloc<T, A>(self) -> (NonNull<T>, usize, usize, TypedProjAlloc<A>)
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.into_proj::<T, A>();
        proj_self.into_parts_with_alloc()
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn into_boxed_slice<T, A>(self) -> Box<[T], TypedProjAlloc<A>>
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.into_proj::<T, A>();
        proj_self.into_boxed_slice()
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    #[track_caller]
    pub fn split_off<T, A>(&mut self, at: usize) -> Self
    where
        T: any::Any,
        A: Allocator + any::Any + Clone,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        let proj_split_off = proj_self.split_off(at);

        Self::from_proj(proj_split_off)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn resize_with<F, T, A>(&mut self, new_len: usize, f: F)
    where
        T: any::Any,
        A: Allocator + any::Any,
        F: FnMut() -> T,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.resize_with(new_len, f)
    }

    #[inline]
    pub fn spare_capacity_mut<T, A>(&mut self) -> &mut [MaybeUninit<T>]
    where
        T: any::Any,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.spare_capacity_mut()
    }
}

impl OpaqueVec {
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.inner.try_reserve(additional)
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), opaque_error::TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    #[track_caller]
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl OpaqueVec {
    #[cfg(not(no_global_oom_handling))]
    #[inline]
    pub fn splice<R, I, T, A>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, A>
    where
        T: any::Any,
        A: Allocator + any::Any,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.splice(range, replace_with)
    }

    pub fn extract_if<F, R, T, A>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, A>
    where
        T: any::Any,
        A: Allocator + any::Any,
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.extract_if(range, filter)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn extend_with<T, A>(&mut self, count: usize, value: T)
    where
        T: any::Any + Clone,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.extend_with(count, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn extend_from_iter<T, I, A>(&mut self, iterator: I)
    where
        T: any::Any + Clone,
        A: Allocator + any::Any,
        I: Iterator<Item = T>,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.extend_from_iter(iterator);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn extend_from_slice<T, A>(&mut self, other: &[T])
    where
        T: any::Any + Clone,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.extend_from_slice(other);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn resize<T, A>(&mut self, new_len: usize, value: T)
    where
        T: any::Any + Clone,
        A: Allocator + any::Any,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.resize(new_len, value);
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }
}

impl OpaqueVec {
    pub fn retain<F, T, A>(&mut self, mut f: F)
    where
        T: any::Any,
        A: Allocator + any::Any,
        F: FnMut(&T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.retain(f);
    }

    pub fn retain_mut<F, T, A>(&mut self, mut f: F)
    where
        T: any::Any,
        A: Allocator + any::Any,
        F: FnMut(&mut T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.retain_mut(f);
    }

    #[inline]
    pub fn dedup_by_key<F, K, T, A>(&mut self, mut key: F)
    where
        T: any::Any,
        A: Allocator + any::Any,
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.dedup_by_key(&mut key);
    }

    pub fn dedup_by<F, T, A>(&mut self, mut same_bucket: F)
    where
        T: any::Any,
        A: Allocator + any::Any,
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
        A: Allocator + any::Any,
        I: IntoIterator<Item=T>,
    {
        let proj_self = self.as_proj_mut::<T, A>();
        proj_self.extend(iter);
    }

    #[inline]
    pub fn reverse<T, A>(&mut self)
    where
        T: any::Any,
        A: Allocator + any::Any,
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
        A: Allocator + any::Any + Clone,
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

impl fmt::Display for OpaqueVec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(formatter)
    }
}

impl<T> From<&[T]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(slice: &[T]) -> Self {
        let inner = OpaqueVecInner::from(slice);

        Self { inner, }
    }
}

impl<T> From<&mut [T]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(slice: &mut [T]) -> Self {
        let inner = OpaqueVecInner::from(slice);

        Self { inner,}
    }
}

impl<const N: usize, T> From<&[T; N]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(array: &[T; N]) -> Self {
        let inner = OpaqueVecInner::from(array);

        Self { inner, }
    }
}

impl<const N: usize, T> From<&mut [T; N]> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(array: &mut [T; N]) -> Self {
        let inner = OpaqueVecInner::from(array);

        Self { inner,}
    }
}

impl<T> From<&Vec<T>> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(vec: &Vec<T>) -> Self {
        let inner = OpaqueVecInner::from(vec);

        Self { inner, }
    }
}

impl<T> From<&mut Vec<T>> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(vec: &mut Vec<T>) -> Self {
        let inner = OpaqueVecInner::from(vec);

        Self { inner, }
    }
}
/*
impl<T> From<Box<[T]>> for OpaqueVec
where
    T: any::Any + Clone,
{
    fn from(slice: Box<[T]>) -> Self {
        let inner = OpaqueVecInner::from(slice);

        Self { inner, }
    }
}
*/
impl<T, A> From<Box<[T], A>> for OpaqueVec
where
    T: any::Any + Clone,
    A: Allocator + any::Any + Clone,
{
    fn from(slice: Box<[T], A>) -> Self {
        let inner = OpaqueVecInner::from(slice);

        Self { inner, }
    }
}

impl<const N: usize, T> From<[T; N]> for OpaqueVec
where
    T: any::Any,
{
    fn from(array: [T; N]) -> Self {
        let inner = OpaqueVecInner::from(array);

        Self { inner, }
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
        let inner = OpaqueVecInner::from_iter(iter);

        Self { inner, }
    }
}
