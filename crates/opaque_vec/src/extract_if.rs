// Portions of this file are derived from `rust`,
// Copyright (c) <year> The Rust Project Contributors
// Licensed under either of
//   * Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
//   * MIT license (http://opensource.org/licenses/MIT)
// at your option.
use crate::vec_inner::TypeProjectedVecInner;

use core::any;
use core::fmt;
use core::ops;
use core::slice;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use opaque_alloc::TypeProjectedAlloc;

/// An iterator that extracts the elements of a vector that satisfy a predicate.
///
/// Extracting iterators are created by the [`TypeProjectedVec::extract_if`] and
/// [`TypeErasedVec::extract_if`] methods.
///
/// # Examples
///
/// Using an extracting iterator on a type-projected vector.
///
/// ```
/// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
/// # use opaque_vec::TypeProjectedVec;
/// #
/// # #[cfg(feature = "nightly")]
/// # use std::alloc::Global;
/// #
/// # #[cfg(not(feature = "nightly"))]
/// # use opaque_allocator_api::alloc::Global;
/// #
/// fn is_even(value: &mut i32) -> bool {
///     *value % 2 == 0
/// }
///
/// let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
/// let iterator = vec.extract_if(.., is_even);
/// let extracted: TypeProjectedVec<i32> = iterator.collect();
///
/// assert_eq!(extracted.as_slice(), &[2, 4, 6, 8, 10]);
/// assert_eq!(vec.as_slice(), &[1, 3, 5, 7, 9]);
/// ```
///
/// Using an extracting iterator on a type-erased vector.
///
/// ```
/// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
/// # use opaque_vec::TypeErasedVec;
/// #
/// # #[cfg(feature = "nightly")]
/// # use std::alloc::Global;
/// #
/// # #[cfg(not(feature = "nightly"))]
/// # use opaque_allocator_api::alloc::Global;
/// #
/// fn is_even(value: &mut i32) -> bool {
///     *value % 2 == 0
/// }
///
/// let mut vec = {
///     let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
///     TypeErasedVec::from(array)
/// };
/// #
/// # assert!(vec.has_element_type::<i32>());
/// # assert!(vec.has_allocator_type::<Global>());
/// #
/// let iterator = vec.extract_if::<_, _, i32, Global>(.., is_even);
/// let extracted: TypeErasedVec = iterator.collect();
/// #
/// # assert!(extracted.has_element_type::<i32>());
/// # assert!(extracted.has_allocator_type::<Global>());
/// #
///
/// assert_eq!(extracted.as_slice::<i32, Global>(), &[2, 4, 6, 8, 10]);
/// assert_eq!(vec.as_slice::<i32, Global>(), &[1, 3, 5, 7, 9]);
/// ```
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ExtractIf<'a, T, F, A = alloc::Global>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    vec: &'a mut TypeProjectedVecInner<T, A>,
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
}

#[cfg(feature = "nightly")]
impl<'a, T, F, A> ExtractIf<'a, T, F, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Construct a new extraction iterator.
    #[inline]
    pub(crate) fn new<R>(vec: &'a mut TypeProjectedVecInner<T, A>, pred: F, range: R) -> Self
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
        }
    }
}

#[cfg(not(feature = "nightly"))]
impl<'a, T, F, A> ExtractIf<'a, T, F, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Construct a new extraction iterator.
    #[inline]
    pub(crate) fn new<R>(vec: &'a mut TypeProjectedVecInner<T, A>, pred: F, range: R) -> Self
    where
        R: ops::RangeBounds<usize>,
    {
        let old_len = vec.len();
        let ops::Range { start, end } = opaque_polyfill::slice_range::range(range, ..old_len);

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
        }
    }
}

impl<'a, T, F, A> ExtractIf<'a, T, F, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Get the underlying allocator of the extracting iterator.
    ///
    /// # Examples
    ///
    /// Getting the allocator from the extracting iterator of a type-projected vector.
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_vec::TypeProjectedVec;
    /// # use opaque_alloc::TypeProjectedAlloc;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// fn is_even(value: &mut i32) -> bool {
    ///     *value % 2 == 0
    /// }
    ///
    /// let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    /// let iterator = vec.extract_if(.., is_even);
    ///
    /// let alloc: &TypeProjectedAlloc<Global> = iterator.allocator();
    /// ```
    ///
    /// Getting the allocator from the extracting iterator of a type-erased vector.
    ///
    /// ```
    /// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
    /// # use opaque_vec::TypeErasedVec;
    /// # use opaque_alloc::TypeProjectedAlloc;
    /// #
    /// # #[cfg(feature = "nightly")]
    /// # use std::alloc::Global;
    /// #
    /// # #[cfg(not(feature = "nightly"))]
    /// # use opaque_allocator_api::alloc::Global;
    /// #
    /// fn is_even(value: &mut i32) -> bool {
    ///     *value % 2 == 0
    /// }
    ///
    /// let mut vec = {
    ///     let array: [i32; 5] = [1, 2, 3, 4, 5];
    ///     TypeErasedVec::from(array)
    /// };
    /// #
    /// # assert!(vec.has_element_type::<i32>());
    /// # assert!(vec.has_allocator_type::<Global>());
    /// #
    /// let iterator = vec.extract_if::<_, _, i32, Global>(.., is_even);
    ///
    /// let alloc: &TypeProjectedAlloc<Global> = iterator.allocator();
    /// ```
    #[inline]
    pub fn allocator(&self) -> &TypeProjectedAlloc<A> {
        self.vec.allocator()
    }
}

impl<T, F, A> Iterator for ExtractIf<'_, T, F, A>
where
    T: any::Any,
    F: FnMut(&mut T) -> bool,
    A: any::Any + alloc::Allocator + Send + Sync,
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
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn drop(&mut self) {
        unsafe {
            if self.idx < self.old_len && self.del > 0 {
                let ptr = self.vec.as_mut_ptr();
                let src = ptr.add(self.idx);
                let dst = src.sub(self.del);
                let tail_len = self.old_len - self.idx;
                src.copy_to(dst, tail_len);
            }
            self.vec.set_len(self.old_len - self.del);
        }
    }
}

impl<T, F, A> fmt::Debug for ExtractIf<'_, T, F, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "ExtractIf {{ .. }}")
    }
}
