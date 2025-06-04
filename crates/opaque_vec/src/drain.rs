use core::fmt;
use core::any;
use core::iter;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;
use core::slice;
use alloc_crate::alloc;

use opaque_alloc::TypedProjAlloc;
use crate::TypedProjVecInner;

pub struct Drain<'a, T, A = alloc::Global>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Index of tail to preserve
    pub(crate) tail_start: usize,
    /// Length of tail
    pub(crate) tail_len: usize,
    /// Current remaining range to remove
    pub(crate) iter: slice::Iter<'a, T>,
    pub(crate) vec: NonNull<TypedProjVecInner<T, A>>,
}

impl<T, A> fmt::Debug for Drain<'_, T, A>
where
    T: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Drain").field(&self.iter.as_slice()).finish()
    }
}

impl<'a, T, A> Drain<'a, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) const fn from_parts(tail_start: usize, tail_len: usize, iter: slice::Iter<'a, T>, vec: NonNull<TypedProjVecInner<T, A>>) -> Self {
        Self {
            tail_start,
            tail_len,
            iter,
            vec,
        }
    }

    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.iter.as_slice()
    }

    #[must_use]
    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
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
            if !crate::zst::is_zst::<T>() {
                let start_ptr = source_vec.as_mut_ptr().add(start);

                // memmove back unyielded elements
                if unyielded_ptr != start_ptr {
                    let src = unyielded_ptr;
                    let dst = start_ptr;

                    core::ptr::copy(src, dst, unyielded_len);
                }

                // memmove back untouched tail
                if tail != (start + unyielded_len) {
                    let src = source_vec.as_ptr().add(tail);
                    let dst = start_ptr.add(unyielded_len);
                    core::ptr::copy(src, dst, this.tail_len);
                }
            }

            source_vec.set_len(start + unyielded_len + this.tail_len);
        }
    }
}

impl<'a, T, A> AsRef<[T]> for Drain<'a, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

unsafe impl<T, A> Sync for Drain<'_, T, A>
where
    T: any::Any + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

unsafe impl<T, A> Send for Drain<'_, T, A>
where
    T: any::Any + Send,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, A> Iterator for Drain<'_, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
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
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back().map(|elt| unsafe { core::ptr::read(elt as *const _) })
    }
}

impl<T, A> Drop for Drain<'_, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn drop(&mut self) {
        /// Moves back the un-`Drain`ed elements to restore the original `Vec`.
        struct DropGuard<'r, 'a, T: any::Any, A: any::Any + alloc::Allocator + Send + Sync>(&'r mut Drain<'a, T, A>);

        impl<'r, 'a, T, A> Drop for DropGuard<'r, 'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Send + Sync,
        {
            fn drop(&mut self) {
                if self.0.tail_len > 0 {
                    unsafe {
                        let source_vec = self.0.vec.as_mut();
                        // memmove back untouched tail, update to new length
                        let start = source_vec.len();
                        let tail = self.0.tail_start;
                        if tail != start {
                            let src = source_vec.as_ptr().add(tail);
                            let dst = source_vec.as_mut_ptr().add(start);
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

        if crate::zst::is_zst::<T>() {
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
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /*
    fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
     */
}

impl<T, A> iter::FusedIterator for Drain<'_, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}
