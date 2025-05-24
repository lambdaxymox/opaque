use core::any;
use core::fmt;
use core::iter;
use core::ops;
use core::slice;
use std::alloc;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

use opaque_alloc::TypedProjAlloc;

use crate::TypedProjVecInner;

#[inline(always)]
const fn assuming_non_null<T>(item: *const T) -> NonNull<T> {
    unsafe { *(item as *const NonNull<T>) }
}

#[inline(always)]
const fn assuming_non_null_mut<T>(item: *const T) -> NonNull<T> {
    unsafe { *(item as *mut NonNull<T>) }
}

pub struct IntoIter<T, A = alloc::Global>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
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
}

impl<T, A> fmt::Debug for IntoIter<T, A>
where
    T: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

impl<T, A> IntoIter<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) const fn from_parts(buf: NonNull<T>, cap: usize, alloc: ManuallyDrop<TypedProjAlloc<A>>, ptr: NonNull<T>, end: *const T) -> Self {
        Self { buf, cap, alloc, ptr, end, }
    }

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
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

unsafe impl<T, A> Send for IntoIter<T, A>
where
    T: any::Any + Send,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}
unsafe impl<T, A> Sync for IntoIter<T, A>
where
    T: any::Any + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, A> Iterator for IntoIter<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let ptr = if crate::zst::is_zst::<T>() {
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
        let exact = if crate::zst::is_zst::<T>() {
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
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        if crate::zst::is_zst::<T>() {
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
    A: any::Any + alloc::Allocator + Send + Sync,
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
impl<T, A> iter::FusedIterator for IntoIter<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, A> Clone for IntoIter<T, A>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        let alloc = Clone::clone(ops::Deref::deref(&self.alloc));
        let read_alloc = ManuallyDrop::new(unsafe { core::ptr::read(&alloc) });
        let inner = TypedProjVecInner::from_slice_in(self.as_slice(), alloc);

        unsafe {
            let mut me = ManuallyDrop::new(inner);
            let data_ptr = me.as_non_null();
            let begin = data_ptr.as_ptr();
            let end = if crate::zst::is_zst::<T>() {
                begin.wrapping_byte_add(me.len())
            } else {
                begin.add(me.len()) as *const T
            };
            let cap = me.capacity();

            IntoIter { buf: data_ptr, cap, alloc: read_alloc, ptr: data_ptr, end, }
        }
    }
}
/*
unsafe impl<T, A> Drop for IntoIter<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
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
