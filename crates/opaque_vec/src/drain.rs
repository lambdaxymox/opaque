use crate::vec_inner::TypedProjVecInner;

use core::fmt;
use core::any;
use core::iter;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;
use core::slice;
use alloc_crate::alloc;
use std::ptr;

use opaque_alloc::TypedProjAlloc;

/// A draining iterator for [`TypedProjVec`] and [`OpaqueVec`].
///
/// Draining iterators are created by the [`TypedProjVec::drain`] and [`OpaqueVec::drain`] methods.
///
/// # Examples
///
/// Using a draining iterator on a type-projected vector.
///
/// ```
/// # #![feature(allocator_api)]
/// # use opaque_vec::TypedProjVec;
/// # use std::alloc::Global;
/// #
/// let mut result = TypedProjVec::from([1, i32::MAX, i32::MAX, i32::MAX, 2, 3]);
/// let expected = TypedProjVec::from([1, 2, 3]);
/// result.drain(1..4);
///
/// assert_eq!(result, expected);
/// ```
///
/// Using a draining iterator on a type-erased vector.
///
/// ```
/// # #![feature(allocator_api)]
/// # use opaque_vec::OpaqueVec;
/// # use std::alloc::Global;
/// #
/// let mut result = OpaqueVec::from([1, i32::MAX, i32::MAX, i32::MAX, 2, 3]);
/// #
/// # assert!(result.has_element_type::<i32>());
/// # assert!(result.has_allocator_type::<Global>());
/// #
/// let expected = OpaqueVec::from([1, 2, 3]);
/// #
/// # assert!(expected.has_element_type::<i32>());
/// # assert!(expected.has_allocator_type::<Global>());
/// #
/// result.drain::<_, i32, Global>(1..4);
///
/// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
/// ```
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
    /// Construct a new draining iterator from its constituent components.
    #[inline]
    pub(crate) const fn from_parts(tail_start: usize, tail_len: usize, iter: slice::Iter<'a, T>, vec: NonNull<TypedProjVecInner<T, A>>) -> Self {
        Self {
            tail_start,
            tail_len,
            iter,
            vec,
        }
    }

    /// Returns a slice of the remaining items in the draining iterator.
    ///
    /// # Examples
    ///
    /// Getting a slice of remaining elements from a draining iterator of a type-projected vector.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_vec::TypedProjVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let mut vec = TypedProjVec::from([
    ///     "spam",
    ///     "eggs",
    ///     "bacon",
    ///     "baked beans",
    ///     "spam",
    /// ]);
    /// let mut drain = vec.drain(..);
    /// assert_eq!(drain.as_slice(), &["spam", "eggs", "bacon", "baked beans", "spam"]);
    /// let _ = drain.next().unwrap();
    /// assert_eq!(drain.as_slice(), &["eggs", "bacon", "baked beans", "spam"]);
    /// let _ = drain.next().unwrap();
    /// assert_eq!(drain.as_slice(), &["bacon", "baked beans", "spam"]);
    /// let _ = drain.next().unwrap();
    /// assert_eq!(drain.as_slice(), &["baked beans", "spam"]);
    /// let _ = drain.next().unwrap();
    /// assert_eq!(drain.as_slice(), &["spam"]);
    /// let _ = drain.next().unwrap();
    /// assert!(drain.as_slice().is_empty());
    /// ```
    ///
    /// Getting a slice of remaining elements from a draining iterator of a type-erased vector.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let mut vec = OpaqueVec::from([
    ///     "spam",
    ///     "eggs",
    ///     "bacon",
    ///     "baked beans",
    ///     "spam",
    /// ]);
    /// let mut drain = vec.drain::<_, &str, Global>(..);
    /// assert_eq!(drain.as_slice(), &["spam", "eggs", "bacon", "baked beans", "spam"]);
    /// let _ = drain.next().unwrap();
    /// assert_eq!(drain.as_slice(), &["eggs", "bacon", "baked beans", "spam"]);
    /// let _ = drain.next().unwrap();
    /// assert_eq!(drain.as_slice(), &["bacon", "baked beans", "spam"]);
    /// let _ = drain.next().unwrap();
    /// assert_eq!(drain.as_slice(), &["baked beans", "spam"]);
    /// let _ = drain.next().unwrap();
    /// assert_eq!(drain.as_slice(), &["spam"]);
    /// let _ = drain.next().unwrap();
    /// assert!(drain.as_slice().is_empty());
    /// ```
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.iter.as_slice()
    }

    /// Get the underlying type-projected memory allocator for the draining iterator.
    ///
    /// # Examples
    ///
    /// Using a draining iterator on a type-projected vector.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_vec::TypedProjVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let mut result = TypedProjVec::from([1, i32::MAX, i32::MAX, i32::MAX, 2, 3]);
    /// let iterator = result.drain(1..4);
    ///
    /// let alloc: &TypedProjAlloc<Global> = iterator.allocator();
    /// ```
    ///
    /// Using a draining iterator on a type-erased vector.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_vec::OpaqueVec;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let mut result = OpaqueVec::from([1, i32::MAX, i32::MAX, i32::MAX, 2, 3]);
    /// #
    /// # assert!(result.has_element_type::<i32>());
    /// # assert!(result.has_allocator_type::<Global>());
    /// #
    /// let iterator = result.drain::<_, i32, Global>(1..4);
    ///
    /// let alloc: &TypedProjAlloc<Global> = iterator.allocator();
    /// ```
    #[must_use]
    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
        unsafe { self.vec.as_ref().allocator() }
    }

    /// Keep the unyielded elements from the draining iterator.
    ///
    /// # Examples
    ///
    /// Using a draining iterator on a typed-projected vector.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_vec::TypedProjVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut vec = TypedProjVec::from([
    ///     "spam",
    ///     "eggs",
    ///     "bacon",
    ///     "baked beans",
    ///     "spam",
    /// ]);
    /// let mut iterator = vec.drain(1..4);
    ///
    /// assert_eq!(iterator.next(), Some("eggs"));
    /// assert_eq!(iterator.next(), Some("bacon"));
    /// assert_eq!(iterator.next(), Some("baked beans"));
    ///
    /// iterator.keep_rest();
    ///
    /// assert_eq!(vec.as_slice(), &["spam", "spam"]);
    /// ```
    ///
    /// Using a draining iterator on a type-erased vector.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_vec::OpaqueVec;
    /// # use std::alloc::Global;
    /// #
    /// let mut vec = OpaqueVec::from([
    ///     "spam",
    ///     "eggs",
    ///     "bacon",
    ///     "baked beans",
    ///     "spam",
    /// ]);
    /// #
    /// # assert!(vec.has_element_type::<&'static str>());
    /// # assert!(vec.has_allocator_type::<Global>());
    /// #
    /// let mut iterator = vec.drain::<_, &'static str, Global>(1..4);
    ///
    /// assert_eq!(iterator.next(), Some("eggs"));
    /// assert_eq!(iterator.next(), Some("bacon"));
    /// assert_eq!(iterator.next(), Some("baked beans"));
    ///
    /// iterator.keep_rest();
    ///
    /// assert_eq!(vec.as_slice::<&'static str, Global>(), &["spam", "spam"]);
    /// ```
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

impl<T, A> Drain<'_, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) unsafe fn fill<I: Iterator<Item = T>>(&mut self, replace_with: &mut I) -> bool {
        let vec = unsafe { self.vec.as_mut() };
        let range_start = vec.len();
        let range_end = self.tail_start;
        let range_slice = unsafe {
            slice::from_raw_parts_mut(vec.as_mut_ptr().add(range_start), range_end - range_start)
        };

        for place in range_slice {
            if let Some(new_item) = replace_with.next() {
                unsafe {
                    ptr::write(place, new_item);
                    vec.set_len(vec.len() + 1);
                }
            } else {
                return false;
            }
        }

        true
    }

    #[track_caller]
    pub(crate) unsafe fn move_tail(&mut self, additional: usize) {
        let vec = unsafe { self.vec.as_mut() };
        let len = self.tail_start + self.tail_len;
        vec.reserve_with_length(len, additional);

        let new_tail_start = self.tail_start + additional;
        unsafe {
            let src = vec.as_ptr().add(self.tail_start);
            let dst = vec.as_mut_ptr().add(new_tail_start);
            ptr::copy(src, dst, self.tail_len);
        }
        self.tail_start = new_tail_start;
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
        /// Moves back the un-`Drain`ed elements to restore the original `TypedProjVec`.
        struct DropGuard<'r, 'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Send + Sync,
        {
            inner: &'r mut Drain<'a, T, A>
        }

        impl<'r, 'a, T, A> Drop for DropGuard<'r, 'a, T, A>
        where
            T: any::Any,
            A: any::Any + alloc::Allocator + Send + Sync,
        {
            fn drop(&mut self) {
                if self.inner.tail_len > 0 {
                    unsafe {
                        let source_vec = self.inner.vec.as_mut();
                        // memmove back untouched tail, update to new length
                        let start = source_vec.len();
                        let tail = self.inner.tail_start;
                        if tail != start {
                            let src = source_vec.as_ptr().add(tail);
                            let dst = source_vec.as_mut_ptr().add(start);
                            core::ptr::copy(src, dst, self.inner.tail_len);
                        }
                        source_vec.set_len(start + self.inner.tail_len);
                    }
                }
            }
        }

        let iterator = core::mem::take(&mut self.iter);
        let drop_len = iterator.len();

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
        let _guard = DropGuard { inner: self };

        if drop_len == 0 {
            return;
        }

        // as_slice() must only be called when iter.len() is > 0 because
        // it also gets touched by vec::Splice which may turn it into a dangling pointer
        // which would make it and the vec pointer point to different allocations which would
        // lead to invalid pointer arithmetic below.
        let drop_ptr = iterator.as_slice().as_ptr();

        unsafe {
            // drop_ptr comes from a slice::Iter which only gives us a &[T] but for drop_in_place
            // a pointer with mutable provenance is necessary. Therefore, we must reconstruct
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
