use crate::vec_inner::TypedProjVecInner;

use core::any;
use core::fmt;
use core::slice;
use core::ops;
use alloc_crate::alloc;

use opaque_alloc::TypedProjAlloc;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ExtractIf<'a, T, F, A = alloc::Global>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    vec: &'a mut TypedProjVecInner<T, A>,
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

impl<'a, T, F, A> ExtractIf<'a, T, F, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn new<R>(vec: &'a mut TypedProjVecInner<T, A>, pred: F, range: R) -> Self
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

    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
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
