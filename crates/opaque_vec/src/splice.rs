use core::any;
use alloc_crate::vec::Vec;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use crate::drain::Drain;

/// An iterator that drains a slice of a vector, then splices a new slice in place of the drained
/// slice.
///
/// Splicing iterators are created by the [`TypedProjVec::splice`] and [`OpaqueVec::splice`] 
/// methods.
///
/// # Examples
///
/// Using a splicing iterator on a type-projected vector.
///
/// ```
/// # #![cfg_attr(feature = "nightly", feature(allocator_api))]
/// # use opaque_vec::TypedProjVec;
/// #
/// # #[cfg(feature = "nightly")]
/// # use std::alloc::Global;
/// #
/// # #[cfg(not(feature = "nightly"))]
/// # use opaque_allocator_api::alloc::Global;
/// #
/// let mut result = TypedProjVec::from([1, 2, 3, 4, 5]);
/// let splice_data: [i32; 5] = [7, 8, 9, 10, 11];
/// let expected = TypedProjVec::from([1, 7, 8, 9, 10, 11, 5]);
/// result.splice(1..4, splice_data);
///
/// assert_eq!(result, expected);
/// ```
///
/// Using a splicing iterator on a type-erased vector.
///
/// ```
/// #![cfg_attr(feature = "nightly", feature(allocator_api))]
/// # use opaque_vec::OpaqueVec;
/// #
/// # #[cfg(feature = "nightly")]
/// # use std::alloc::Global;
/// #
/// # #[cfg(not(feature = "nightly"))]
/// # use opaque_allocator_api::alloc::Global;
/// #
/// let mut result = {
///     let array: [i32; 5] = [1, 2, 3, 4, 5];
///     OpaqueVec::from(array)
/// };
/// #
/// # assert!(result.has_element_type::<i32>());
/// # assert!(result.has_allocator_type::<Global>());
/// #
/// let splice_data: [i32; 5] = [7, 8, 9, 10, 11];
/// let expected = {
///     let array: [i32; 7] = [1, 7, 8, 9, 10, 11, 5];
///     OpaqueVec::from(array)
/// };
/// #
/// # assert!(expected.has_element_type::<i32>());
/// # assert!(expected.has_allocator_type::<Global>());
/// #
/// result.splice::<_, _, i32, Global>(1..4, splice_data);
///
/// assert_eq!(result.as_slice::<i32, Global>(), expected.as_slice::<i32, Global>());
/// ```
#[derive(Debug)]
pub struct Splice<'a, I, A = alloc::Global>
where
    I: Iterator + 'a,
    <I as Iterator>::Item: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + 'a,
{
    drain: Drain<'a, I::Item, A>,
    replace_with: I,
}

impl<'a, I, A> Splice<'a, I, A>
where
    I: Iterator + 'a,
    <I as Iterator>::Item: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + 'a,
{
    /// Construct a new splicing iterator.
    #[inline]
    pub(crate) const fn new(drain: Drain<'a, I::Item, A>, replace_with: I) -> Self {
        Self { drain, replace_with }
    }
}

impl<I, A> Iterator for Splice<'_, I, A>
where
    I: Iterator,
    I::Item: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
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
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.drain.next_back()
    }
}

impl<I, A> ExactSizeIterator for Splice<'_, I, A>
where
    I: Iterator,
    I::Item: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<I, A> Drop for Splice<'_, I, A>
where
    I: Iterator,
    I::Item: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
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
