#![feature(const_eval_select)]
#![feature(allocator_api)]
#![feature(structural_match)]
#![feature(alloc_layout_extra)]
#![feature(optimize_attribute)]
#![feature(slice_range)]
extern crate core;

use core::cmp;
use core::ops;
use core::slice;
use std::alloc::{
    Allocator,
    Global,
    Layout,
};
use std::fmt;
use std::mem::{
    ManuallyDrop,
    MaybeUninit,
};
use std::borrow;
use std::ptr::NonNull;

use opaque_blob_vec::OpaqueBlobVec;

use std::any::TypeId;
use std::marker::PhantomData;

use core::iter::FusedIterator;
use opaque_alloc::OpaqueAlloc;
use opaque_error;


pub struct IntoIter<T, A> {
    opaque_vec: OpaqueVecInner,
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


#[cfg(not(no_global_oom_handling))]
impl<T, A> Clone for IntoIter<T, A>
where
    T: Clone + 'static,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self {
        Self {
            opaque_vec: self.opaque_vec.clone::<T>(),
            _marker: self._marker,
        }
    }
}

pub struct Drain<'a, T, A>
where
    T: 'static,
    A: Allocator,
{
    /*
    /// Index of tail to preserve
    pub(crate) tail_start: usize,
    /// Length of tail
    pub(crate) tail_len: usize,
    /// Current remaining range to remove
    pub(crate) iter: slice::Iter<'a, T>,
    pub(crate) vec: NonNull<Vec<T, A>>,
     */
    /// Index of tail to preserve
    pub(crate) tail_start: usize,
    /// Length of tail
    pub(crate) tail_len: usize,
    /// Current remaining range to remove
    pub(crate) iter: slice::Iter<'a, T>,
    pub(crate) vec: NonNull<OpaqueVecInner>,
    _marker: core::marker::PhantomData<A>,
}

impl<T: fmt::Debug, A: Allocator> fmt::Debug for Drain<'_, T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Drain").field(&self.iter.as_slice()).finish()
    }
}

impl<'a, T, A> Drain<'a, T, A>
where
    T: 'static,
    A: Allocator,
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
    pub fn allocator(&self) -> &OpaqueAlloc {
        unsafe { self.vec.as_ref().allocator() }
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
            // if !T::IS_ZST {
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
            // }


            source_vec.set_len(start + unyielded_len + this.tail_len);
        }
    }
}

impl<'a, T, A: Allocator> AsRef<[T]> for Drain<'a, T, A>
where
    T: 'static,
    A: Allocator,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

unsafe impl<T: Sync, A: Sync + Allocator> Sync for Drain<'_, T, A> {}
unsafe impl<T: Send, A: Send + Allocator> Send for Drain<'_, T, A> {}

impl<T, A> Iterator for Drain<'_, T, A>
where
    T: 'static,
    A: Allocator,
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
    T: 'static,
    A: Allocator,
{
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back().map(|elt| unsafe { core::ptr::read(elt as *const _) })
    }
}

impl<T, A> Drop for Drain<'_, T, A>
where
    T: 'static,
    A: Allocator,
{
    fn drop(&mut self) {
        /// Moves back the un-`Drain`ed elements to restore the original `Vec`.
        struct DropGuard<'r, 'a, T: 'static, A: Allocator>(&'r mut Drain<'a, T, A>);

        impl<'r, 'a, T, A> Drop for DropGuard<'r, 'a, T, A>
        where
            T: 'static,
            A: Allocator,
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

        /*
        if T::IS_ZST {
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
         */

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

impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A> {
    /*
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
     */
}

impl<T, A> FusedIterator for Drain<'_, T, A>
where
    T: 'static,
    A: Allocator,
{
}

#[derive(Debug)]
pub struct Splice<'a, I, A>
where
    I: Iterator + 'a,
    A: Allocator + 'a,
    <I as Iterator>::Item: 'static,
{
    drain: Drain<'a, I::Item, A>,
    replace_with: I,
}

impl<I, A> Iterator for Splice<'_, I, A>
where
    I: Iterator,
    A: Allocator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.drain.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.drain.size_hint()
    }
}

impl<I: Iterator, A: Allocator> DoubleEndedIterator for Splice<'_, I, A> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.drain.next_back()
    }
}

impl<I: Iterator, A: Allocator> ExactSizeIterator for Splice<'_, I, A> {}

impl<I: Iterator, A: Allocator> Drop for Splice<'_, I, A> {
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
impl<T, A: Allocator> Drain<'_, T, A> {
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
    T: 'static,
    A: Allocator,
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

impl<'a, T, F, A: Allocator> ExtractIf<'a, T, F, A> {
    fn new<R: ops::RangeBounds<usize>>(vec: &'a mut OpaqueVecInner, pred: F, range: R) -> Self {
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
    pub fn allocator(&self) -> &OpaqueAlloc {
        self.vec.allocator()
    }
}

impl<T, F, A> Iterator for ExtractIf<'_, T, F, A>
where
    T: 'static,
    F: FnMut(&mut T) -> bool,
    A: Allocator,
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
    T: 'static,
    A: Allocator,
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
    opaque_vec: &'a mut OpaqueVecInner,
    _marker: core::marker::PhantomData<T>,
}

impl<'a, T> Extender<'a, T> {
    #[inline]
    const fn new(opaque_vec: &'a mut OpaqueVecInner) -> Self {
        Self {
            opaque_vec,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<'a, T> Extend<T> for Extender<'a, T>
where
    T: 'static,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter.into_iter() {
            self.opaque_vec.push::<T>(item);
        }
    }
}

impl<'a, 'b, T> Extend<&'b T> for Extender<'a, T>
where
    T: Copy + 'static,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'b T>,
    {
        for item in iter.into_iter() {
            self.opaque_vec.push::<T>(*item);
        }
    }
}

pub struct OpaqueVecInner {
    data: OpaqueBlobVec,
    type_id: TypeId,
}

impl OpaqueVecInner {
    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new_in<T, A>(alloc: A) -> Self
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
        let data = OpaqueBlobVec::new_in(opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();

        Self { data, type_id }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity_in<T, A>(capacity: usize, alloc: A) -> Self
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
        let data = OpaqueBlobVec::with_capacity_in(capacity, opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();

        Self { data, type_id }
    }

    #[inline]
    pub(crate) fn try_with_capacity_in<T, A>(capacity: usize, alloc: A) -> Result<Self, opaque_error::TryReserveError>
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
        let data = OpaqueBlobVec::try_with_capacity_in(capacity, opaque_alloc, element_layout, drop_fn)?;
        let type_id = TypeId::of::<T>();

        Ok(Self { data, type_id })
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts_in<T, A>(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self
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
        let data = OpaqueBlobVec::from_raw_parts_in(ptr_bytes, length, capacity, opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();

        Self { data, type_id }
    }

    #[inline]
    pub(crate) unsafe fn from_parts_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self
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
        let data = OpaqueBlobVec::from_parts_in(ptr_bytes, length, capacity, opaque_alloc, element_layout, drop_fn);
        let type_id = TypeId::of::<T>();

        Self { data, type_id }
    }

    #[inline]
    pub(crate) const fn allocator(&self) -> &OpaqueAlloc {
        self.data.allocator()
    }
}

impl OpaqueVecInner {
    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn new<T>() -> Self
    where
        T: 'static,
    {
        Self::new_in::<T, Global>(Global)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub(crate) fn with_capacity<T>(capacity: usize) -> Self
    where
        T: 'static,
    {
        Self::with_capacity_in::<T, Global>(capacity, Global)
    }

    #[inline]
    pub(crate) fn try_with_capacity<T>(capacity: usize) -> Result<Self, opaque_error::TryReserveError>
    where
        T: 'static,
    {
        Self::try_with_capacity_in::<T, Global>(capacity, Global)
    }

    #[inline]
    pub(crate) unsafe fn from_raw_parts<T>(ptr: *mut T, length: usize, capacity: usize) -> Self
    where
        T: 'static,
    {
        let opaque_alloc = OpaqueAlloc::new::<Global>(Global);

        Self::from_raw_parts_in(ptr, length, capacity, opaque_alloc)
    }

    #[inline]
    pub(crate) unsafe fn from_parts<T>(ptr: NonNull<T>, length: usize, capacity: usize) -> Self
    where
        T: 'static,
    {
        let opaque_alloc = OpaqueAlloc::new::<Global>(Global);

        Self::from_parts_in(ptr, length, capacity, opaque_alloc)
    }
}

impl OpaqueVecInner {
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
        T: 'static,
    {
        self.as_slice::<T>().iter()
    }

    #[inline]
    pub(crate) fn iter_mut<T>(&mut self) -> slice::IterMut<'_, T>
    where
        T: 'static,
    {
        self.as_mut_slice::<T>().iter_mut()
    }

    #[inline]
    #[must_use]
    pub(crate) fn get_unchecked<T>(&self, index: usize) -> &T
    where
        T: 'static,
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
        T: 'static,
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
        T: 'static,
    {
        let mut me = ManuallyDrop::new(value);
        let value_ptr = unsafe { NonNull::new_unchecked(&mut *me as *mut T as *mut u8) };

        self.data.push(value_ptr);
    }

    #[inline]
    pub(crate) fn pop<T>(&mut self) -> Option<T>
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
    pub(crate) fn replace_insert<T>(&mut self, index: usize, value: T)
    where
        T: 'static,
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
        T: 'static,
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
        T: 'static,
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
        T: 'static,
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
        T: PartialEq + 'static,
    {
        self.as_slice::<T>().contains(value)
    }

    #[inline]
    pub const fn as_ptr<T>(&self) -> *const T
    where
        T: 'static,
    {
        self.data.as_ptr() as *const T
    }

    #[inline]
    pub(crate) const fn as_mut_ptr<T>(&mut self) -> *mut T
    where
        T: 'static,
    {
        self.data.as_mut_ptr() as *mut T
    }

    #[inline]
    pub(crate) const fn as_non_null<T>(&mut self) -> NonNull<T>
    where
        T: 'static,
    {
        // SAFETY: An [`OpaqueVec`] always holds a non-null pointer.
        self.data.as_non_null().cast::<T>()
    }

    #[inline]
    pub(crate) fn as_slice<T>(&self) -> &[T]
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
    pub(crate) fn as_mut_slice<T>(&mut self) -> &mut [T]
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
    pub(crate) fn into_raw_parts<T>(self) -> (*mut T, usize, usize)
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
    pub(crate) fn into_parts<T>(self) -> (NonNull<T>, usize, usize)
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
    pub(crate) fn into_raw_parts_with_alloc<T>(self) -> (*mut T, usize, usize, OpaqueAlloc)
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
    pub(crate) fn into_parts_with_alloc<T>(self) -> (NonNull<T>, usize, usize, OpaqueAlloc)
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
    pub(crate) fn spare_capacity_mut<T>(&mut self) -> &mut [MaybeUninit<T>]
    where
        T: 'static,
    {
        unsafe {
            let ptr = self.as_mut_ptr::<T>().add(self.len()) as *mut MaybeUninit<T>;
            let len = self.capacity() - self.len();

            std::slice::from_raw_parts_mut(ptr, len)
        }
    }

    pub(crate) fn drain<R, T>(&mut self, range: R) -> Drain<'_, T, OpaqueAlloc>
    where
        T: 'static,
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
        T: 'static,
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
        T: 'static,
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
        T: 'static,
    {
        if self.data.len() == self.data.capacity() {
            return Err(value);
        }

        self.push::<T>(value);

        Ok(())
    }

    #[inline]
    pub(crate) fn into_iter<T>(self) -> IntoIter<T, OpaqueAlloc>
    where
        T: 'static,
    {
        IntoIter {
            opaque_vec: self,
            _marker: PhantomData,
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub(crate) fn append<T>(&mut self, other: &mut Self)
    where
        T: 'static,
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
    pub(crate) fn into_boxed_slice<T>(mut self) -> Box<[T], OpaqueAlloc>
    where
        T: 'static,
    {
        unsafe {
            self.shrink_to_fit();
            let mut me = ManuallyDrop::new(self);
            let len = me.len();
            let ptr = me.as_mut_ptr::<T>();
            let slice_ptr = std::ptr::slice_from_raw_parts_mut(ptr, len);
            let alloc = core::ptr::read(me.allocator());

            Box::from_raw_in(slice_ptr, alloc)
        }
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    pub(crate) fn split_off<T>(&mut self, at: usize) -> Self
    where
        T: 'static,
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
        let mut other = OpaqueVecInner::with_capacity_in::<T, OpaqueAlloc>(other_len, self.allocator().clone());

        // Unsafely `set_len` and copy items to `other`.
        unsafe {
            self.set_len(at);
            other.set_len(other_len);

            core::ptr::copy_nonoverlapping(self.as_ptr::<T>().add(at), other.as_mut_ptr(), other.len());
        }

        other
    }

    #[inline]
    pub(crate) fn resize_with<F, T>(&mut self, new_len: usize, f: F)
    where
        T: 'static,
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
        T: Clone + 'static,
    {
        let value_ptr = unsafe { NonNull::new_unchecked(&value as *const T as *mut T as *mut u8) };

        self.data.extend_with(count, value_ptr);
    }

    #[inline]
    pub(crate) fn extend_from_iter<T, I>(&mut self, mut iterator: I)
    where
        T: Clone + 'static,
        I: Iterator<Item = T>,
    {
        for item in iterator {
            self.push::<T>(item);
        }
    }

    #[inline]
    pub(crate) fn extend_from_slice<T>(&mut self, other: &[T])
    where
        T: Clone + 'static,
    {
        self.extend_from_iter::<T, _>(other.iter().cloned())
    }

    #[inline]
    pub(crate) fn resize<T>(&mut self, new_len: usize, value: T)
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

impl OpaqueVecInner {
    pub(crate) fn retain<F, T>(&mut self, mut f: F)
    where
        T: 'static,
        F: FnMut(&T) -> bool,
    {
        self.retain_mut(|elem| f(elem));
    }

    pub(crate) fn retain_mut<F, T>(&mut self, mut f: F)
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
            v: &'a mut OpaqueVecInner,
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

        fn process_loop<F, T, A, const DELETED: bool>(original_len: usize, f: &mut F, g: &mut BackshiftOnDrop<'_, T, A>)
        where
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

    pub(crate) fn dedup_by<F, T>(&mut self, mut same_bucket: F)
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
            vec: &'a mut OpaqueVecInner,
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

    #[inline]
    pub(crate) fn dedup_by_key<F, K, T>(&mut self, mut key: F)
    where
        T: 'static,
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.dedup_by::<_, T>(|a, b| key(a) == key(b))
    }
}

impl OpaqueVecInner {
    #[cfg(not(no_global_oom_handling))]
    #[inline]
    pub(crate) fn splice<R, I, T>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, OpaqueAlloc>
    where
        T: 'static,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        Splice {
            drain: self.drain(range),
            replace_with: replace_with.into_iter(),
        }
    }

    #[inline]
    pub(crate) fn extract_if<F, R, T>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, OpaqueAlloc>
    where
        T: 'static,
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
        T: 'static,
        I: IntoIterator<Item = T>,
    {
        let mut extender = Extender::new(self);
        extender.extend(iter)
    }

    #[inline]
    pub(crate) fn reverse<T>(&mut self)
    where
        T: 'static,
    {
        self.as_mut_slice::<T>().reverse();
    }

    #[inline]
    pub(crate) fn clone<T>(&self) -> Self
    where
        T: Clone + 'static,
    {
        let new_inner = self.data.clone();
        let new_type_id = self.type_id;

        Self {
            data: new_inner,
            type_id: new_type_id,
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

    // We shouldn't add inline attribute to this since this is used in
    // `vec!` macro mostly and causes perf regression. See #71204 for
    // discussion and perf results.
    #[allow(missing_docs)]
    pub fn into_opaque_vec<T, A>(b: Box<[T], A>) -> OpaqueVecInner
    where
        T: 'static,
        A: Allocator + Clone + 'static,
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
        A: Allocator + Clone + 'static,
    {
        T::to_opaque_vec(slice, alloc)
    }

    #[cfg(not(no_global_oom_handling))]
    pub trait ConvertOpaqueVec {
        fn to_opaque_vec<A>(slice: &[Self], alloc: A) -> OpaqueVecInner
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
        fn to_opaque_vec<A>(slice: &[Self], alloc: A) -> OpaqueVecInner
        where
            A: Allocator + Clone + 'static,
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
    T: Clone + 'static,
{
    fn from(slice: &[T]) -> Self {
        private::to_opaque_vec::<T, Global>(slice, Global)
    }
}

impl<T> From<&mut [T]> for OpaqueVecInner
where
    T: Clone + 'static,
{
    fn from(slice: &mut [T]) -> Self {
        private::to_opaque_vec::<T, Global>(slice, Global)
    }
}

impl<const N: usize, T> From<&[T; N]> for OpaqueVecInner
where
    T: Clone + 'static,
{
    fn from(array: &[T; N]) -> Self {
        Self::from(array.as_slice())
    }
}

impl<const N: usize, T> From<&mut [T; N]> for OpaqueVecInner
where
    T: Clone + 'static,
{
    fn from(array: &mut [T; N]) -> Self {
        Self::from(array.as_mut_slice())
    }
}

impl<T> From<&Vec<T>> for OpaqueVecInner
where
    T: Clone + 'static,
{
    fn from(vec: &Vec<T>) -> Self {
        Self::from(vec.as_slice())
    }
}

impl<T> From<&mut Vec<T>> for OpaqueVecInner
where
    T: Clone + 'static,
{
    fn from(vec: &mut Vec<T>) -> Self {
        Self::from(vec.as_mut_slice())
    }
}

impl<T> From<Box<[T]>> for OpaqueVecInner
where
    T: Clone + 'static,
{
    fn from(slice: Box<[T]>) -> Self {
        Self::from(slice.as_ref())
    }
}

impl<T> From<Box<[T], opaque_alloc::OpaqueAlloc>> for OpaqueVecInner
where
    T: Clone + 'static,
{
    fn from(slice: Box<[T], opaque_alloc::OpaqueAlloc>) -> Self {
        Self::from(slice.as_ref())
    }
}

impl<const N: usize, T> From<[T; N]> for OpaqueVecInner
where
    T: 'static,
{
    fn from(array: [T; N]) -> Self {
        private::into_opaque_vec::<T, Global>(Box::new(array))
    }
}

impl<T> FromIterator<T> for OpaqueVecInner
where
    T: 'static,
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
pub struct TypedProjVec<T> {
    inner: OpaqueVecInner,
    _marker: core::marker::PhantomData<T>,
}

impl<T> TypedProjVec<T>
where
    T: 'static,
{
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new_in<A>(alloc: A) -> Self
    where
        A: Allocator + Clone + 'static,
    {
        let inner = OpaqueVecInner::new_in::<T, A>(alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in<A>(capacity: usize, alloc: A) -> Self
    where
        A: Allocator + Clone + 'static,
    {
        let inner = OpaqueVecInner::with_capacity_in::<T, A>(capacity, alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn try_with_capacity_in<A>(capacity: usize, alloc: A) -> Result<Self, opaque_error::TryReserveError>
    where
        A: Allocator + Clone + 'static,
    {
        let inner = OpaqueVecInner::try_with_capacity_in::<T, A>(capacity, alloc)?;

        Ok(Self {
            inner,
            _marker: core::marker::PhantomData,
        })
    }

    #[inline]
    pub unsafe fn from_raw_parts_in<A>(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self
    where
        A: Allocator + Clone + 'static,
    {
        let inner = OpaqueVecInner::from_raw_parts_in::<T, A>(ptr, length, capacity, alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub unsafe fn from_parts_in<A>(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self
    where
        A: Allocator + Clone + 'static,
    {
        let inner = OpaqueVecInner::from_parts_in::<T, A>(ptr, length, capacity, alloc);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub const fn allocator(&self) -> &OpaqueAlloc {
        self.inner.allocator()
    }
}

impl<T> TypedProjVec<T>
where
    T: 'static,
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

impl<T> TypedProjVec<T>
where
    T: 'static,
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

impl<T> TypedProjVec<T>
where
    T: 'static,
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

    pub fn into_iter(self) -> IntoIter<T, OpaqueAlloc> {
        self.inner.into_iter::<T>()
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub fn append(&mut self, other: &mut Self) {
        self.inner.append::<T>(&mut other.inner)
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, OpaqueAlloc>
    where
        R: ops::RangeBounds<usize>,
    {
        self.inner.drain::<R, T>(range)
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
    pub fn into_raw_parts_with_alloc(self) -> (*mut T, usize, usize, OpaqueAlloc) {
        self.inner.into_raw_parts_with_alloc::<T>()
    }

    pub fn into_parts_with_alloc(self) -> (NonNull<T>, usize, usize, OpaqueAlloc) {
        self.inner.into_parts_with_alloc::<T>()
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn into_boxed_slice(self) -> Box<[T], OpaqueAlloc> {
        self.inner.into_boxed_slice::<T>()
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    #[track_caller]
    pub fn split_off(&mut self, at: usize) -> Self {
        let inner = self.inner.split_off::<T>(at);

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

impl<T> TypedProjVec<T>
where
    T: 'static,
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

impl<T> TypedProjVec<T>
where
    T: 'static,
{
    #[cfg(not(no_global_oom_handling))]
    #[inline]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, OpaqueAlloc>
    where
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        self.inner.splice::<R, I, T>(range, replace_with)
    }

    pub fn extract_if<F, R>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, OpaqueAlloc>
    where
        T: 'static,
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        self.inner.extract_if::<F, R, T>(range, filter)
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

impl<T> TypedProjVec<T>
where
    T: 'static,
{
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.inner.retain(|elem| f(elem));
    }

    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.inner.retain_mut::<F, T>(f)
    }

    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, mut key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.inner.dedup_by_key::<F, K, T>(key)
    }

    pub fn dedup_by<F>(&mut self, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.inner.dedup_by::<F, T>(same_bucket)
    }
}

impl<T> TypedProjVec<T>
where
    T: 'static,
{
    #[inline]
    pub fn reverse(&mut self)
    where
        T: 'static,
    {
        self.inner.reverse::<T>()
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, /* A */> Extend<T> for TypedProjVec<T, /* A */>
where
    T: 'static,
    /*
    A: Allocator,
    */
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
impl<'a, T, /* A */> Extend<&'a T> for TypedProjVec<T, /* A */>
where
    T: Copy + 'a + 'static,
    /*
    A: Allocator,
     */
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

impl<T> PartialEq<TypedProjVec<T>> for TypedProjVec<T>
where
    T: PartialEq + 'static,
{
    fn eq(&self, other: &TypedProjVec<T>) -> bool {
        PartialEq::eq(self.as_slice(), other.as_slice())
    }
}

impl<T, /* A1, A2 */> PartialOrd<TypedProjVec<T, /* A2 */>> for TypedProjVec<T, /* A1 */>
where
    T: PartialOrd + 'static,
    /*
    A1: Allocator,
    A2: Allocator,
     */
{
    #[inline]
    fn partial_cmp(&self, other: &TypedProjVec<T, /* A2 */>) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(self.as_slice(), other.as_slice())
    }
}

impl<T, /* A */> Eq for TypedProjVec<T, /* A */>
where
    T: Eq + 'static,
    /*
    A: Allocator,
     */
{
}

impl<T, /* A */> Ord for TypedProjVec<T, /* A */>
where
    T: Ord + 'static,
    /*
    A: Allocator,
     */
{
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Ord::cmp(self.as_slice(), other.as_slice())
    }
}
/*
impl<T, /* A */> Drop for TypedProjVec<T>
where
    /*
    A: Allocator,
     */
{
    fn drop(&mut self) {

    }
}
*/
impl<T> Default for TypedProjVec<T>
where
    T: 'static,
{
    fn default() -> TypedProjVec<T> {
        TypedProjVec::new()
    }
}

impl<T, /* A */> fmt::Debug for TypedProjVec<T>
where
    T: fmt::Debug + 'static,
    /*
    A: Allocator,
     */
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl<T, /* A */> AsRef<TypedProjVec<T, /* A */>> for TypedProjVec<T, /* A */>
where
    T: 'static,
    /*
    A: Allocator,
     */
{
    fn as_ref(&self) -> &TypedProjVec<T, /* A */> {
        self
    }
}

impl<T, /* A */> AsMut<TypedProjVec<T, /* A */>> for TypedProjVec<T, /* A */>
where
    T: 'static,
    /*
    A: Allocator,
    */
{
    fn as_mut(&mut self) -> &mut TypedProjVec<T, /* A */> {
        self
    }
}

impl<T, /* A */> AsRef<[T]> for TypedProjVec<T, /* A */>
where
    T: 'static,
    /*
    A: Allocator,
    */
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, /* A */> AsMut<[T]> for TypedProjVec<T, /* A */>
where
    T: 'static,
    /*
    A: Allocator,
    */
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T> From<&[T]> for TypedProjVec<T>
where
    T: Clone + 'static,
{
    #[track_caller]
    fn from(slice: &[T]) -> TypedProjVec<T> {
        let inner = OpaqueVecInner::from(slice);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T> From<&mut [T]> for TypedProjVec<T>
where
    T: Clone + 'static,
{
    #[track_caller]
    fn from(slice: &mut [T]) -> TypedProjVec<T> {
        let inner = OpaqueVecInner::from(slice);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, const N: usize> From<&[T; N]> for TypedProjVec<T>
where
    T: Clone + 'static,
{
    #[track_caller]
    fn from(slice: &[T; N]) -> TypedProjVec<T> {
        Self::from(slice.as_slice())
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, const N: usize> From<&mut [T; N]> for TypedProjVec<T>
where
    T: Clone + 'static,
{
    #[track_caller]
    fn from(slice: &mut [T; N]) -> TypedProjVec<T> {
        Self::from(slice.as_mut_slice())
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, const N: usize> From<[T; N]> for TypedProjVec<T>
where
    T: 'static,
{
    #[track_caller]
    fn from(slice: [T; N]) -> TypedProjVec<T> {
        /*
        <[T]>::into_vec(Box::new(slice))
         */
        todo!()
    }
}

impl<'a, T> From<borrow::Cow<'a, [T]>> for TypedProjVec<T>
where
    T: 'static,
    [T]: ToOwned<Owned = TypedProjVec<T>>,
{
    #[track_caller]
    fn from(slice: borrow::Cow<'a, [T]>) -> TypedProjVec<T> {
        slice.into_owned()
    }
}

impl<T /*A */> From<Box<[T], /* A */>> for TypedProjVec<T, /* A */>
where
    T: 'static,
{
    fn from(slice: Box<[T], /* A */>) -> Self {
        /*
        slice.into_vec()
         */
        todo!()
    }
}
/*
#[cfg(not(no_global_oom_handling))]
impl<T, /* A */> From<Vec<T, /* A */>> for TypedProjVec<T, /* A */>
where
    T: Clone + 'static,
    /*
    A: Allocator,
     */
{
    #[track_caller]
    fn from(vec: Vec<T, /* A */>) -> Self {
        let inner = OpaqueVecInner::from(vec);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}
*/

#[cfg(not(no_global_oom_handling))]
impl<T, /* A */> From<&Vec<T, /* A */>> for TypedProjVec<T, /* A */>
where
    T: Clone + 'static,
/*
A: Allocator,
 */
{
    #[track_caller]
    fn from(vec: &Vec<T, /* A */>) -> Self {
        let inner = OpaqueVecInner::from(vec);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, /* A */> From<&mut Vec<T, /* A */>> for TypedProjVec<T, /* A */>
where
    T: Clone + 'static,
/*
A: Allocator,
 */
{
    #[track_caller]
    fn from(vec: &mut Vec<T, /* A */>) -> Self {
        let inner = OpaqueVecInner::from(vec);

        Self {
            inner,
            _marker: core::marker::PhantomData,
        }
    }
}

#[cfg(not(no_global_oom_handling))]
impl<T, /* A */> From<TypedProjVec<T, /* A */>> for Box<[T], /* A */ OpaqueAlloc>
where
    T: 'static,
    /*
    A: Allocator,
     */
{
    #[track_caller]
    fn from(vec: TypedProjVec<T, /* A */>) -> Self {
        vec.into_boxed_slice()
    }
}

#[cfg(not(no_global_oom_handling))]
impl From<&str> for TypedProjVec<u8> {
    #[track_caller]
    fn from(st: &str) -> TypedProjVec<u8> {
        From::from(st.as_bytes())
    }
}

impl<T, /* A, */ const N: usize> TryFrom<TypedProjVec<T, /* A */>> for [T; N]
where
    T: 'static,
    /*
    A: Allocator,
     */
{
    type Error = TypedProjVec<T, /* A */>;

    fn try_from(mut vec: TypedProjVec<T, /* A */>) -> Result<[T; N], TypedProjVec<T, /* A */>> {
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

impl<T, /* A */> Clone for TypedProjVec<T, /* A */>
where
    T: Clone + 'static,
    /*
    A: Allocator,
     */
{
    fn clone(&self) -> Self {
        let cloned_inner = self.inner.clone::<T>();

        Self {
            inner: cloned_inner,
            _marker: PhantomData,
        }
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
    pub fn new_in<T, A>(alloc: A) -> Self
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        let inner = OpaqueVecInner::new_in::<T, A>(alloc);

        Self { inner }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity_in<T, A>(capacity: usize, alloc: A) -> Self
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        let inner = OpaqueVecInner::with_capacity_in::<T, A>(capacity, alloc);

        Self { inner, }
    }

    #[inline]
    pub fn try_with_capacity_in<T, A>(capacity: usize, alloc: A) -> Result<Self, opaque_error::TryReserveError>
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        let inner = OpaqueVecInner::try_with_capacity_in::<T, A>(capacity, alloc)?;

        Ok(Self { inner, })
    }

    #[inline]
    pub unsafe fn from_raw_parts_in<T, A>(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        let inner = OpaqueVecInner::from_raw_parts_in::<T, A>(ptr, length, capacity, alloc);

        Self { inner, }
    }

    #[inline]
    pub unsafe fn from_parts_in<T, A>(ptr: NonNull<T>, length: usize, capacity: usize, alloc: A) -> Self
    where
        T: 'static,
        A: Allocator + Clone + 'static,
    {
        let inner = OpaqueVecInner::from_parts_in::<T, A>(ptr, length, capacity, alloc);

        Self { inner, }
    }

    #[inline]
    pub const fn allocator(&self) -> &OpaqueAlloc {
        self.inner.allocator()
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
        let proj_vec = TypedProjVec::<T>::new();

        Self::from_proj(proj_vec)
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity<T>(capacity: usize) -> Self
    where
        T: 'static,
    {
        let proj_vec = TypedProjVec::<T>::with_capacity(capacity);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub fn try_with_capacity<T>(capacity: usize) -> Result<Self, opaque_error::TryReserveError>
    where
        T: 'static,
    {
        let proj_vec = TypedProjVec::<T>::try_with_capacity(capacity)?;

        Ok(Self::from_proj(proj_vec))
    }

    #[inline]
    pub unsafe fn from_raw_parts<T>(ptr: *mut T, length: usize, capacity: usize) -> Self
    where
        T: 'static,
    {
        let proj_vec = TypedProjVec::<T>::from_raw_parts(ptr, length, capacity);

        Self::from_proj(proj_vec)
    }

    #[inline]
    pub unsafe fn from_parts<T>(ptr: NonNull<T>, length: usize, capacity: usize) -> Self
    where
        T: 'static,
    {
        let proj_vec = TypedProjVec::<T>::from_parts(ptr, length, capacity);

        Self::from_proj(proj_vec)
    }
}

impl OpaqueVec {
    #[inline]
    pub fn has_element_type<T>(&self) -> bool
    where
        T: 'static,
    {
        self.inner.has_element_type::<T>()
    }

    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: Allocator + Clone + 'static,
    {
        self.inner.has_allocator_type::<A>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<T>(&self)
    where
        T: 'static,
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
    pub fn as_proj<T>(&self) -> &TypedProjVec<T>
    where
        T: 'static,
    {
        self.assert_type_safety::<T>();

        unsafe { &*(self as *const OpaqueVec as *const TypedProjVec<T>) }
    }

    pub fn as_proj_mut<T>(&mut self) -> &mut TypedProjVec<T>
    where
        T: 'static,
    {
        self.assert_type_safety::<T>();

        unsafe { &mut *(self as *mut OpaqueVec as *mut TypedProjVec<T>) }
    }

    pub fn into_proj<T>(self) -> TypedProjVec<T>
    where
        T: 'static,
    {
        self.assert_type_safety::<T>();

        TypedProjVec {
            inner: self.inner,
            _marker: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_proj<T>(proj_self: TypedProjVec<T>) -> Self {
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
    pub fn get<T>(&self, index: usize) -> Option<&T>
    where
        T: 'static,
    {
        let proj_self = self.as_proj::<T>();
        proj_self.get(index)
    }

    #[inline]
    #[must_use]
    pub fn get_mut<T>(&mut self, index: usize) -> Option<&mut T>
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.get_mut(index)
    }

    #[inline]
    #[track_caller]
    pub fn push<T>(&mut self, value: T)
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.push(value);
    }

    #[inline]
    pub fn pop<T>(&mut self) -> Option<T>
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.pop()
    }

    #[inline]
    pub fn push_within_capacity<T>(&mut self, value: T) -> Result<(), T>
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.push_within_capacity(value)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn replace_insert<T>(&mut self, index: usize, value: T)
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.replace_insert(index, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shift_insert<T>(&mut self, index: usize, value: T)
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.shift_insert(index, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn swap_remove<T>(&mut self, index: usize) -> T
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.swap_remove(index)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn shift_remove<T>(&mut self, index: usize) -> T
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.shift_remove(index)
    }

    pub fn contains<T>(&self, value: &T) -> bool
    where
        T: PartialEq + 'static,
    {
        let proj_self = self.as_proj::<T>();
        proj_self.contains(value)
    }

    pub fn iter<T>(&self) -> slice::Iter<'_, T>
    where
        T: 'static,
    {
        let proj_self = self.as_proj::<T>();
        proj_self.iter()
    }

    pub fn iter_mut<T>(&mut self) -> slice::IterMut<'_, T>
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.iter_mut()
    }

    pub fn into_iter<T>(self) -> IntoIter<T, OpaqueAlloc>
    where
        T: 'static,
    {
        let proj_self = self.into_proj::<T>();
        proj_self.into_iter()
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[track_caller]
    pub fn append<T>(&mut self, other: &mut Self)
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        let proj_other = other.as_proj_mut::<T>();
        proj_self.append(proj_other)
    }

    pub fn drain<R, T>(&mut self, range: R) -> Drain<'_, T, OpaqueAlloc>
    where
        T: 'static,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.drain(range)
    }

    #[inline]
    pub fn as_ptr<T>(&self) -> *const T
    where
        T: 'static,
    {
        let proj_self = self.as_proj::<T>();
        proj_self.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr<T>(&mut self) -> *mut T
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.as_mut_ptr()
    }

    #[inline]
    pub fn as_non_null<T>(&mut self) -> NonNull<T>
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.as_non_null()
    }

    pub fn as_slice<T>(&self) -> &[T]
    where
        T: 'static,
    {
        let proj_self = self.as_proj::<T>();
        proj_self.as_slice()
    }

    pub fn as_mut_slice<T>(&mut self) -> &mut [T]
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.as_mut_slice()
    }

    pub fn as_byte_slice<T>(&self) -> &[u8]
    where
        T: 'static,
    {
        let proj_self = self.as_proj::<T>();
        proj_self.as_byte_slice()
    }

    #[must_use]
    pub fn into_raw_parts<T>(self) -> (*mut T, usize, usize)
    where
        T: 'static,
    {
        let proj_self = self.into_proj::<T>();
        proj_self.into_raw_parts()
    }

    #[must_use]
    pub fn into_parts<T>(self) -> (NonNull<T>, usize, usize)
    where
        T: 'static,
    {
        let proj_self = self.into_proj::<T>();
        proj_self.into_parts()
    }

    #[must_use]
    pub fn into_raw_parts_with_alloc<T>(self) -> (*mut T, usize, usize, OpaqueAlloc)
    where
        T: 'static,
    {
        let proj_self = self.into_proj::<T>();
        proj_self.into_raw_parts_with_alloc()
    }

    pub fn into_parts_with_alloc<T>(self) -> (NonNull<T>, usize, usize, OpaqueAlloc)
    where
        T: 'static,
    {
        let proj_self = self.into_proj::<T>();
        proj_self.into_parts_with_alloc()
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn into_boxed_slice<T>(self) -> Box<[T], OpaqueAlloc>
    where
        T: 'static,
    {
        let proj_self = self.into_proj::<T>();
        proj_self.into_boxed_slice()
    }

    #[cfg(not(no_global_oom_handling))]
    #[inline]
    #[must_use = "use `.truncate()` if you don't need the other half"]
    #[track_caller]
    pub fn split_off<T>(&mut self, at: usize) -> Self
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        let proj_split_off = proj_self.split_off(at);

        Self::from_proj(proj_split_off)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn resize_with<F, T>(&mut self, new_len: usize, f: F)
    where
        T: 'static,
        F: FnMut() -> T,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.resize_with(new_len, f)
    }

    #[inline]
    pub fn spare_capacity_mut<T>(&mut self) -> &mut [MaybeUninit<T>]
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
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
    pub fn splice<R, I, T>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, OpaqueAlloc>
    where
        T: 'static,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.splice(range, replace_with)
    }

    pub fn extract_if<F, R, T>(&mut self, range: R, filter: F) -> ExtractIf<'_, T, F, OpaqueAlloc>
    where
        T: 'static,
        F: FnMut(&mut T) -> bool,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.extract_if(range, filter)
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn extend_with<T>(&mut self, count: usize, value: T)
    where
        T: Clone + 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.extend_with(count, value);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    fn extend_from_iter<T, I>(&mut self, iterator: I)
    where
        T: Clone + 'static,
        I: Iterator<Item = T>,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.extend_from_iter(iterator);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn extend_from_slice<T>(&mut self, other: &[T])
    where
        T: Clone + 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.extend_from_slice(other);
    }

    #[cfg(not(no_global_oom_handling))]
    #[track_caller]
    pub fn resize<T>(&mut self, new_len: usize, value: T)
    where
        T: Clone + 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.resize(new_len, value);
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }
}

impl OpaqueVec {
    pub fn retain<F, T>(&mut self, mut f: F)
    where
        T: 'static,
        F: FnMut(&T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.retain(f);
    }

    pub fn retain_mut<F, T>(&mut self, mut f: F)
    where
        T: 'static,
        F: FnMut(&mut T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.retain_mut(f);
    }

    #[inline]
    pub fn dedup_by_key<F, K, T>(&mut self, mut key: F)
    where
        T: 'static,
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.dedup_by_key(&mut key);
    }

    pub fn dedup_by<F, T>(&mut self, mut same_bucket: F)
    where
        T: 'static,
        F: FnMut(&mut T, &mut T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.dedup_by(same_bucket);
    }
}

impl OpaqueVec {
    #[inline]
    pub fn extend<T, I>(&mut self, iter: I)
    where
        T: 'static,
        I: IntoIterator<Item=T>,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.extend(iter);
    }

    #[inline]
    pub fn reverse<T>(&mut self)
    where
        T: 'static,
    {
        let proj_self = self.as_proj_mut::<T>();
        proj_self.reverse();
    }
}

impl OpaqueVec {
    #[inline]
    pub fn clone<T>(&self) -> Self
    where
        T: Clone + 'static,
    {
        let proj_self = self.as_proj::<T>();
        let proj_cloned_self = proj_self.clone();
        let cloned_self = OpaqueVec::from_proj(proj_cloned_self);

        cloned_self
    }
}

impl fmt::Debug for OpaqueVec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpaqueVec")
            .field("inner", &self.inner)
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
    T: Clone + 'static,
{
    fn from(slice: &[T]) -> Self {
        let inner = OpaqueVecInner::from(slice);

        Self { inner, }
    }
}

impl<T> From<&mut [T]> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(slice: &mut [T]) -> Self {
        let inner = OpaqueVecInner::from(slice);

        Self { inner,}
    }
}

impl<const N: usize, T> From<&[T; N]> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(array: &[T; N]) -> Self {
        let inner = OpaqueVecInner::from(array);

        Self { inner, }
    }
}

impl<const N: usize, T> From<&mut [T; N]> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(array: &mut [T; N]) -> Self {
        let inner = OpaqueVecInner::from(array);

        Self { inner,}
    }
}

impl<T> From<&Vec<T>> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(vec: &Vec<T>) -> Self {
        let inner = OpaqueVecInner::from(vec);

        Self { inner, }
    }
}

impl<T> From<&mut Vec<T>> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(vec: &mut Vec<T>) -> Self {
        let inner = OpaqueVecInner::from(vec);

        Self { inner, }
    }
}

impl<T> From<Box<[T]>> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(slice: Box<[T]>) -> Self {
        let inner = OpaqueVecInner::from(slice);

        Self { inner, }
    }
}

impl<T> From<Box<[T], opaque_alloc::OpaqueAlloc>> for OpaqueVec
where
    T: Clone + 'static,
{
    fn from(slice: Box<[T], opaque_alloc::OpaqueAlloc>) -> Self {
        let inner = OpaqueVecInner::from(slice);

        Self { inner, }
    }
}

impl<const N: usize, T> From<[T; N]> for OpaqueVec
where
    T: 'static,
{
    fn from(array: [T; N]) -> Self {
        let inner = OpaqueVecInner::from(array);

        Self { inner, }
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
        let inner = OpaqueVecInner::from_iter(iter);

        Self { inner, }
    }
}
