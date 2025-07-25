// Portions of this file are derived from `indexmap`,
// Copyright (c) 2016--2017 The indexmap Developers
// Licensed under either of
//   * Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
//   * MIT license (http://opensource.org/licenses/MIT)
// at your option.
use crate::equivalent::Equivalent;
use crate::get_disjoint_mut_error::GetDisjointMutError;
use crate::range_ops;
use crate::slice_eq;

use alloc_crate::boxed::Box;
use core::any;
use core::cmp;
use core::error;
use core::fmt;
use core::iter;
use core::mem;
use core::ops;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use hashbrown::hash_table;

use opaque_alloc::TypeProjectedAlloc;
use opaque_error::{
    TryReserveError,
    TryReserveErrorKind,
};
use opaque_hash::{
    TypeErasedBuildHasher,
    TypeProjectedBuildHasher,
};
use opaque_vec::{
    TypeErasedVec,
    TypeProjectedVec,
};

pub(crate) struct Drain<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: opaque_vec::Drain<'a, Bucket<K, V>, A>,
}

impl<'a, K, V, A> Drain<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    const fn new(iter: opaque_vec::Drain<'a, Bucket<K, V>, A>) -> Self {
        Self { iter }
    }

    pub(crate) fn as_slice(&self) -> &Slice<K, V> {
        Slice::from_slice(self.iter.as_slice())
    }
}

impl<K, V, A> Iterator for Drain<'_, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::key_value)
    }
}

impl<K, V, A> DoubleEndedIterator for Drain<'_, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::key_value)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::key_value)
    }
}

impl<K, V, A> ExactSizeIterator for Drain<'_, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn len(&self) -> usize {
        <opaque_vec::Drain<'_, Bucket<K, V>, A> as ExactSizeIterator>::len(&self.iter)
    }
}

impl<K, V, A> iter::FusedIterator for Drain<'_, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<K, V, A> fmt::Debug for Drain<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iterator = self.iter.as_slice().iter().map(Bucket::refs);
        formatter.debug_list().entries(iterator).finish()
    }
}

pub(crate) struct Extract<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    map: &'a mut TypeProjectedIndexMapCore<K, V, A>,
    new_len: usize,
    current: usize,
    end: usize,
}

impl<K, V, A> Drop for Extract<'_, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn drop(&mut self) {
        let old_len = self.map.indices.len();
        let mut new_len = self.new_len;

        debug_assert!(new_len <= self.current);
        debug_assert!(self.current <= self.end);
        debug_assert!(self.current <= old_len);
        debug_assert!(old_len <= self.map.entries.capacity());

        // SAFETY: We assume `new_len` and `current` were correctly maintained by the iterator.
        // So `entries[new_len..current]` were extracted, but the rest before and after are valid.
        unsafe {
            if new_len == self.current {
                // Nothing was extracted, so any remaining items can be left in place.
                new_len = old_len;
            } else if self.current < old_len {
                // Need to shift the remaining items down.
                let tail_len = old_len - self.current;
                let base = self.map.entries.as_mut_ptr();
                let src = base.add(self.current);
                let dest = base.add(new_len);
                src.copy_to(dest, tail_len);
                new_len += tail_len;
            }
            self.map.entries.set_len(new_len);
        }

        if new_len != old_len {
            // We don't keep track of *which* items were extracted, so reindex everything.
            self.map.rebuild_hash_table();
        }
    }
}

impl<K, V, A> Extract<'_, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn extract_if<F>(&mut self, mut filter: F) -> Option<Bucket<K, V>>
    where
        F: FnMut(&mut Bucket<K, V>) -> bool,
    {
        debug_assert!(self.end <= self.map.entries.capacity());

        let base = self.map.entries.as_mut_ptr();
        while self.current < self.end {
            // SAFETY: We're maintaining both indices within bounds of the original entries, so
            // 0..new_len and current..indices.len() are always valid items for our Drop to keep.
            unsafe {
                let item = base.add(self.current);
                if filter(&mut *item) {
                    // Extract it!
                    self.current += 1;
                    return Some(item.read());
                } else {
                    // Keep it, shifting it down if needed.
                    if self.new_len != self.current {
                        debug_assert!(self.new_len < self.current);
                        let dest = base.add(self.new_len);
                        item.copy_to_nonoverlapping(dest, 1);
                    }
                    self.current += 1;
                    self.new_len += 1;
                }
            }
        }
        None
    }

    pub(crate) fn remaining(&self) -> usize {
        self.end - self.current
    }
}

pub(crate) struct Keys<'a, K, V> {
    iter: std::slice::Iter<'a, Bucket<K, V>>,
}

impl<'a, K, V> Keys<'a, K, V> {
    fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self { iter: entries.iter() }
    }
}

impl<'a, K, V> Clone for Keys<'a, K, V> {
    fn clone(&self) -> Self {
        Keys { iter: self.iter.clone() }
    }
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::key_ref)
    }
}

impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::key_ref)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::key_ref)
    }
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> iter::FusedIterator for Keys<'a, K, V> {}

impl<'a, K, V> fmt::Debug for Keys<'a, K, V>
where
    K: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

impl<'a, K, V> ops::Index<usize> for Keys<'a, K, V> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.iter.as_slice()[index].key
    }
}

impl<'a, K, V> Default for Keys<'a, K, V> {
    fn default() -> Self {
        Self {
            iter: Default::default(),
        }
    }
}

pub(crate) struct IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: opaque_vec::IntoIter<Bucket<K, V>, A>,
}

impl<K, V, A> IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn new(entries: TypeProjectedVec<Bucket<K, V>, A>) -> Self {
        Self {
            iter: entries.into_iter(),
        }
    }
}

impl<K, V, A> Iterator for IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::key)
    }
}

impl<K, V, A> DoubleEndedIterator for IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::key)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::key)
    }
}

impl<K, V, A> ExactSizeIterator for IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V, A> iter::FusedIterator for IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<K, V, A> fmt::Debug for IntoKeys<K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iterator = self.iter.as_slice().iter().map(Bucket::key_ref);
        formatter.debug_list().entries(iterator).finish()
    }
}

impl<K, V, A> Default for IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Self {
            iter: TypeProjectedVec::new_in(Default::default()).into_iter(),
        }
    }
}

pub(crate) struct Values<'a, K, V> {
    iter: std::slice::Iter<'a, Bucket<K, V>>,
}

impl<'a, K, V> Values<'a, K, V> {
    fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self { iter: entries.iter() }
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::value_ref)
    }
}

impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::value_ref)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::value_ref)
    }
}

impl<'a, K, V> ExactSizeIterator for Values<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> iter::FusedIterator for Values<'a, K, V> {}

impl<K, V> Clone for Values<'_, K, V> {
    fn clone(&self) -> Self {
        Values { iter: self.iter.clone() }
    }
}

impl<K, V> fmt::Debug for Values<'_, K, V>
where
    V: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

impl<K, V> Default for Values<'_, K, V> {
    fn default() -> Self {
        Self { iter: [].iter() }
    }
}

pub(crate) struct ValuesMut<'a, K, V> {
    iter: core::slice::IterMut<'a, Bucket<K, V>>,
}

impl<'a, K, V> ValuesMut<'a, K, V> {
    fn new(entries: &'a mut [Bucket<K, V>]) -> Self {
        Self {
            iter: entries.iter_mut(),
        }
    }
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::value_mut)
    }
}

impl<K, V> DoubleEndedIterator for ValuesMut<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::value_mut)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::value_mut)
    }
}

impl<'a, K, V> ExactSizeIterator for ValuesMut<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> iter::FusedIterator for ValuesMut<'a, K, V> {}

impl<K, V> fmt::Debug for ValuesMut<'_, K, V>
where
    V: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iterator = self.iter.as_slice().iter().map(Bucket::value_ref);
        formatter.debug_list().entries(iterator).finish()
    }
}

impl<K, V> Default for ValuesMut<'_, K, V> {
    fn default() -> Self {
        Self { iter: [].iter_mut() }
    }
}

pub(crate) struct IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: opaque_vec::IntoIter<Bucket<K, V>, A>,
}

impl<K, V, A> IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn new(entries: TypeProjectedVec<Bucket<K, V>, A>) -> Self {
        Self {
            iter: entries.into_iter(),
        }
    }
}

impl<K, V, A> Iterator for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::value)
    }
}

impl<K, V, A> DoubleEndedIterator for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::value)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::value)
    }
}

impl<K, V, A> ExactSizeIterator for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V, A> iter::FusedIterator for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<K, V, A> fmt::Debug for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iterator = self.iter.as_slice().iter().map(Bucket::value_ref);
        formatter.debug_list().entries(iterator).finish()
    }
}

impl<K, V, A> Default for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Self {
            iter: TypeProjectedVec::new_in(Default::default()).into_iter(),
        }
    }
}

#[repr(transparent)]
pub(crate) struct Slice<K, V> {
    entries: [Bucket<K, V>],
}

impl<K, V> Slice<K, V> {
    #[inline]
    pub(crate) const fn from_slice(entries: &[Bucket<K, V>]) -> &Self {
        unsafe { &*(entries as *const [Bucket<K, V>] as *const Self) }
    }

    #[inline]
    pub(crate) const fn from_slice_mut(entries: &mut [Bucket<K, V>]) -> &mut Self {
        unsafe { &mut *(entries as *mut [Bucket<K, V>] as *mut Self) }
    }
}

#[cfg(feature = "nightly")]
impl<K, V> Slice<K, V> {
    fn from_boxed_slice<A>(entries: Box<[Bucket<K, V>], A>) -> Box<Self, A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        unsafe {
            let (ptr, alloc) = Box::into_raw_with_allocator(entries);

            Box::from_raw_in(ptr as *mut Self, alloc)
        }
    }

    fn into_boxed_slice<A>(self: Box<Self, A>) -> Box<[Bucket<K, V>], A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        unsafe {
            let (ptr, alloc) = Box::into_raw_with_allocator(self);

            Box::from_raw_in(ptr as *mut [Bucket<K, V>], alloc)
        }
    }

    pub(crate) fn into_entries<A>(self: Box<Self, TypeProjectedAlloc<A>>) -> TypeProjectedVec<Bucket<K, V>, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        unsafe {
            let len = self.entries.len();
            let capacity = len;
            let (ptr, alloc) = Box::into_raw_with_allocator(self.into_boxed_slice());

            TypeProjectedVec::from_raw_parts_proj_in(ptr as *mut Bucket<K, V>, len, capacity, alloc)
        }
    }
}

impl<K, V> Slice<K, V> {
    pub(crate) fn to_entries_in<A>(&self, alloc: TypeProjectedAlloc<A>) -> TypeProjectedVec<Bucket<K, V>, A>
    where
        K: any::Any + Clone,
        V: any::Any + Clone,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut entries = TypeProjectedVec::with_capacity_proj_in(self.len(), alloc);
        entries.extend_from_slice(&self.entries);

        entries
    }
}

#[cfg(feature = "nightly")]
impl<K, V> Slice<K, V> {
    pub(crate) fn from_entries_in<A>(vec: TypeProjectedVec<Bucket<K, V>, A>) -> Box<Self, TypeProjectedAlloc<A>>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let boxed_slice_inner = Box::from(vec);
        let boxed_slice = unsafe {
            let (_ptr, alloc) = Box::into_raw_with_allocator(boxed_slice_inner);
            let ptr = _ptr as *mut Self;
            Box::from_raw_in(ptr, alloc)
        };

        boxed_slice
    }
}

impl<K, V> Slice<K, V> {
    #[inline]
    pub(crate) const fn as_entries(&self) -> &[Bucket<K, V>] {
        &self.entries
    }

    pub(crate) const fn new<'a>() -> &'a Self {
        Self::from_slice(&[])
    }

    pub(crate) const fn new_mut<'a>() -> &'a mut Self {
        Self::from_slice_mut(&mut [])
    }

    #[inline]
    pub(crate) const fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    pub(crate) const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.entries.get(index).map(Bucket::refs)
    }

    pub(crate) fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.entries.get_mut(index).map(Bucket::ref_mut)
    }

    pub(crate) fn get_range<R>(&self, range: R) -> Option<&Self>
    where
        R: ops::RangeBounds<usize>,
    {
        let range = range_ops::try_simplify_range(range, self.entries.len())?;

        self.entries.get(range).map(Slice::from_slice)
    }

    pub(crate) fn get_range_mut<R>(&mut self, range: R) -> Option<&mut Self>
    where
        R: ops::RangeBounds<usize>,
    {
        let range = range_ops::try_simplify_range(range, self.entries.len())?;

        self.entries.get_mut(range).map(Slice::from_slice_mut)
    }

    pub(crate) fn first(&self) -> Option<(&K, &V)> {
        self.entries.first().map(Bucket::refs)
    }

    pub(crate) fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.entries.first_mut().map(Bucket::ref_mut)
    }

    pub(crate) fn last(&self) -> Option<(&K, &V)> {
        self.entries.last().map(Bucket::refs)
    }

    pub(crate) fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        self.entries.last_mut().map(Bucket::ref_mut)
    }

    pub(crate) fn split_at(&self, index: usize) -> (&Self, &Self) {
        let (first, second) = self.entries.split_at(index);
        (Self::from_slice(first), Self::from_slice(second))
    }

    pub(crate) fn split_at_mut(&mut self, index: usize) -> (&mut Self, &mut Self) {
        let (first, second) = self.entries.split_at_mut(index);

        (Self::from_slice_mut(first), Self::from_slice_mut(second))
    }

    pub(crate) fn split_first(&self) -> Option<((&K, &V), &Self)> {
        if let [first, rest @ ..] = &self.entries {
            Some((first.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }

    pub(crate) fn split_first_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [first, rest @ ..] = &mut self.entries {
            Some((first.ref_mut(), Self::from_slice_mut(rest)))
        } else {
            None
        }
    }

    pub(crate) fn split_last(&self) -> Option<((&K, &V), &Self)> {
        if let [rest @ .., last] = &self.entries {
            Some((last.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }

    pub(crate) fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [rest @ .., last] = &mut self.entries {
            Some((last.ref_mut(), Self::from_slice_mut(rest)))
        } else {
            None
        }
    }

    pub(crate) fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(&self.entries)
    }

    pub(crate) fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(&mut self.entries)
    }

    pub(crate) fn keys(&self) -> Keys<'_, K, V> {
        Keys::new(&self.entries)
    }
}

#[cfg(feature = "nightly")]
impl<K, V> Slice<K, V> {
    pub(crate) fn into_keys<A>(self: Box<Self, TypeProjectedAlloc<A>>) -> IntoKeys<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        IntoKeys::new(self.into_entries())
    }
}

impl<K, V> Slice<K, V> {
    pub(crate) fn values(&self) -> Values<'_, K, V> {
        Values::new(&self.entries)
    }

    pub(crate) fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut::new(&mut self.entries)
    }
}

#[cfg(feature = "nightly")]
impl<K, V> Slice<K, V> {
    pub(crate) fn into_values<A>(self: Box<Self, TypeProjectedAlloc<A>>) -> IntoValues<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        IntoValues::new(self.into_entries())
    }
}

impl<K, V> Slice<K, V> {
    pub(crate) fn binary_search_keys(&self, key: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        self.binary_search_by(|p, _| p.cmp(key))
    }

    #[inline]
    pub(crate) fn binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> cmp::Ordering,
    {
        self.entries.binary_search_by(move |a| f(&a.key, &a.value))
    }

    #[inline]
    pub(crate) fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> B,
        B: Ord,
    {
        self.binary_search_by(|k, v| f(k, v).cmp(b))
    }

    #[must_use]
    pub(crate) fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {
        self.entries.partition_point(move |a| pred(&a.key, &a.value))
    }

    pub(crate) fn get_disjoint_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[(&K, &mut V); N], GetDisjointMutError> {
        let indices = indices.map(Some);
        let key_values = self.get_disjoint_opt_mut(indices)?;

        Ok(key_values.map(Option::unwrap))
    }

    pub(crate) fn get_disjoint_opt_mut<const N: usize>(
        &mut self,
        indices: [Option<usize>; N],
    ) -> Result<[Option<(&K, &mut V)>; N], GetDisjointMutError> {
        // SAFETY: We cannot allow duplicate indices as we would return several mutable references
        // to the same data.
        let len = self.len();
        for i in 0..N {
            if let Some(idx) = indices[i] {
                if idx >= len {
                    return Err(GetDisjointMutError::IndexOutOfBounds);
                } else if indices[..i].contains(&Some(idx)) {
                    return Err(GetDisjointMutError::OverlappingIndices);
                }
            }
        }

        let entries_ptr = self.entries.as_mut_ptr();
        let out = indices.map(|idx_opt| {
            match idx_opt {
                Some(idx) => {
                    // SAFETY: The base pointer is valid as it comes from a slice and the reference is always
                    // in bounds and unique as we have already checked the indices above.
                    let kv = unsafe { (*(entries_ptr.add(idx))).ref_mut() };
                    Some(kv)
                }
                None => None,
            }
        });

        Ok(out)
    }
}

impl<'a, K, V> IntoIterator for &'a Slice<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut Slice<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(feature = "nightly")]
impl<K, V, A> IntoIterator for Box<Slice<K, V>, TypeProjectedAlloc<A>>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V, A>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.into_entries())
    }
}

impl<K, V> Default for &'_ Slice<K, V> {
    fn default() -> Self {
        Slice::from_slice(&[])
    }
}


impl<K, V> Default for &'_ mut Slice<K, V> {
    fn default() -> Self {
        Slice::from_slice_mut(&mut [])
    }
}

#[cfg(feature = "nightly")]
impl<K, V, A> Default for Box<Slice<K, V>, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Slice::from_boxed_slice(Box::new_in([], Default::default()))
    }
}

#[cfg(feature = "nightly")]
impl<K, V, A> Clone for Box<Slice<K, V>, A>
where
    K: Clone,
    V: Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        let alloc = Box::<Slice<K, V>, A>::allocator(&self).clone();
        Slice::from_boxed_slice(self.entries.to_vec_in(alloc).into_boxed_slice())
    }
}

#[cfg(feature = "nightly")]
impl<K, V> From<&Slice<K, V>> for Box<Slice<K, V>, TypeProjectedAlloc<alloc::Global>>
where
    K: any::Any + Copy,
    V: any::Any + Copy,
{
    fn from(slice: &Slice<K, V>) -> Self {
        let alloc = TypeProjectedAlloc::new(alloc::Global);
        let entries = slice.to_entries_in(alloc);
        let boxed_entries: Box<[Bucket<K, V>], TypeProjectedAlloc<alloc::Global>> = entries.into_boxed_slice();

        Slice::from_boxed_slice(boxed_entries)
    }
}

impl<K, V> fmt::Debug for Slice<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self).finish()
    }
}

impl<K, V, K2, V2> PartialEq<Slice<K2, V2>> for Slice<K, V>
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &Slice<K2, V2>) -> bool {
        slice_eq::slice_eq(&self.entries, &other.entries, |b1, b2| {
            b1.key == b2.key && b1.value == b2.value
        })
    }
}

impl<K, V, K2, V2> PartialEq<[(K2, V2)]> for Slice<K, V>
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &[(K2, V2)]) -> bool {
        slice_eq::slice_eq(&self.entries, other, |b, t| b.key == t.0 && b.value == t.1)
    }
}

impl<K, V, K2, V2> PartialEq<Slice<K2, V2>> for [(K, V)]
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &Slice<K2, V2>) -> bool {
        slice_eq::slice_eq(self, &other.entries, |t, b| t.0 == b.key && t.1 == b.value)
    }
}

impl<K, V, K2, V2, const N: usize> PartialEq<[(K2, V2); N]> for Slice<K, V>
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &[(K2, V2); N]) -> bool {
        <Self as PartialEq<[_]>>::eq(self, other)
    }
}

impl<K, V, const N: usize, K2, V2> PartialEq<Slice<K2, V2>> for [(K, V); N]
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &Slice<K2, V2>) -> bool {
        <[_] as PartialEq<_>>::eq(self, other)
    }
}

impl<K, V> Eq for Slice<K, V>
where
    K: Eq,
    V: Eq,
{
}

impl<K, V> PartialOrd for Slice<K, V>
where
    K: PartialOrd,
    V: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<K, V> Ord for Slice<K, V>
where
    K: Ord,
    V: Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<K, V> hash::Hash for Slice<K, V>
where
    K: hash::Hash,
    V: hash::Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.len().hash(state);
        for (key, value) in self {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl<K, V> ops::Index<usize> for Slice<K, V> {
    type Output = Bucket<K, V>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<K, V> ops::IndexMut<usize> for Slice<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

macro_rules! impl_index_for_index_map_inner_slice {
    ($($range:ty),*) => {$(
        impl<K, V> ops::Index<$range> for Slice<K, V> {
            type Output = Slice<K, V>;

            fn index(&self, range: $range) -> &Self {
                Self::from_slice(&self.entries[range])
            }
        }

        impl<K, V> ops::IndexMut<$range> for Slice<K, V> {
            fn index_mut(&mut self, range: $range) -> &mut Self {
                Self::from_slice_mut(&mut self.entries[range])
            }
        }
    )*}
}

impl_index_for_index_map_inner_slice!(
    ops::Range<usize>,
    ops::RangeFrom<usize>,
    ops::RangeFull,
    ops::RangeInclusive<usize>,
    ops::RangeTo<usize>,
    ops::RangeToInclusive<usize>,
    (ops::Bound<usize>, ops::Bound<usize>)
);

pub(crate) struct Iter<'a, K, V> {
    iter: std::slice::Iter<'a, Bucket<K, V>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    #[inline]
    fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self { iter: entries.iter() }
    }

    pub(crate) fn as_slice(&self) -> &Slice<K, V> {
        Slice::from_slice(self.iter.as_slice())
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::refs)
    }
}

impl<K, V> DoubleEndedIterator for Iter<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::refs)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::refs)
    }
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> iter::FusedIterator for Iter<'_, K, V> {}

impl<K, V> Clone for Iter<'_, K, V> {
    fn clone(&self) -> Self {
        Iter { iter: self.iter.clone() }
    }
}

impl<K, V> fmt::Debug for Iter<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

impl<K, V> Default for Iter<'_, K, V> {
    fn default() -> Self {
        Self { iter: [].iter() }
    }
}

pub(crate) struct IterMut<'a, K, V> {
    iter: std::slice::IterMut<'a, Bucket<K, V>>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    #[inline]
    fn new(entries: &'a mut [Bucket<K, V>]) -> Self {
        Self {
            iter: entries.iter_mut(),
        }
    }

    #[cfg(feature = "nightly")]
    pub(crate) fn as_slice_mut(&'a mut self) -> &'a mut Slice<K, V> {
        Slice::from_slice_mut(self.iter.as_mut_slice())
    }

    #[cfg(not(feature = "nightly"))]
    pub(crate) fn as_slice_mut(&'a mut self) -> &'a mut Slice<K, V> {
        todo!(
            "This method cannot be implemented on Rust stable until the feature \
            `slice_iter_mut_as_mut_slice` stabilizes. \
            See `https://github.com/rust-lang/rust/issues/93079`."
        )
    }

    pub(crate) fn into_slice_mut(self) -> &'a mut Slice<K, V> {
        Slice::from_slice_mut(self.iter.into_slice())
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::ref_mut)
    }
}

impl<K, V> DoubleEndedIterator for IterMut<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::ref_mut)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::ref_mut)
    }
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> iter::FusedIterator for IterMut<'_, K, V> {}

impl<K, V> fmt::Debug for IterMut<'_, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iterator = self.iter.as_slice().iter().map(Bucket::refs);
        formatter.debug_list().entries(iterator).finish()
    }
}

impl<K, V> Default for IterMut<'_, K, V> {
    fn default() -> Self {
        Self { iter: [].iter_mut() }
    }
}

#[derive(Clone)]
pub(crate) struct IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: opaque_vec::IntoIter<Bucket<K, V>, A>,
}

impl<K, V, A> IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn new(entries: TypeProjectedVec<Bucket<K, V>, A>) -> Self {
        Self {
            iter: entries.into_iter(),
        }
    }

    pub(crate) fn as_slice(&self) -> &Slice<K, V> {
        Slice::from_slice(self.iter.as_slice())
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
        Slice::from_slice_mut(self.iter.as_mut_slice())
    }
}

impl<K, V, A> Iterator for IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::key_value)
    }
}

impl<K, V, A> DoubleEndedIterator for IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::key_value)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::key_value)
    }
}

impl<K, V, A> ExactSizeIterator for IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V, A> iter::FusedIterator for IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<K, V, A> fmt::Debug for IntoIter<K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iterator = self.iter.as_slice().iter().map(Bucket::refs);
        formatter.debug_list().entries(iterator).finish()
    }
}

impl<K, V, A> Default for IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Self {
            iter: TypeProjectedVec::new_in(Default::default()).into_iter(),
        }
    }
}

pub(crate) struct Splice<'a, I, K, V, S, A>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    map: &'a mut TypeProjectedIndexMapInner<K, V, S, A>,
    tail: TypeProjectedIndexMapCore<K, V, A>,
    drain: opaque_vec::IntoIter<Bucket<K, V>, A>,
    replace_with: I,
}

impl<'a, I, K, V, S, A> Splice<'a, I, K, V, S, A>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    #[track_caller]
    fn new<R>(map: &'a mut TypeProjectedIndexMapInner<K, V, S, A>, range: R, replace_with: I) -> Self
    where
        R: ops::RangeBounds<usize>,
    {
        let (tail, drain) = map.inner.split_splice::<R>(range);
        Self {
            map,
            tail,
            drain,
            replace_with,
        }
    }
}

impl<I, K, V, S, A> Drop for Splice<'_, I, K, V, S, A>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn drop(&mut self) {
        // Finish draining unconsumed items. We don't strictly *have* to do this
        // manually, since we already split it into separate memory, but it will
        // match the drop order of `vec::Splice` items this way.
        let _ = self.drain.nth(usize::MAX);

        // Now insert all the new items. If a key matches an existing entry, it
        // keeps the original position and only replaces the value, like `insert`.
        while let Some((key, value)) = self.replace_with.next() {
            // Since the tail is disjoint, we can try to update it first,
            // or else insert (update or append) the primary map.
            let hash = self.map.hash(&key);
            if let Some(i) = self.tail.get_index_of(hash, &key) {
                self.tail.as_entries_mut()[i].value = value;
            } else {
                self.map.inner.insert_full(hash, key, value);
            }
        }

        // Finally, re-append the tail
        self.map.inner.append_unchecked(&mut self.tail);
    }
}

impl<I, K, V, S, A> Iterator for Splice<'_, I, K, V, S, A>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.drain.next().map(Bucket::key_value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.drain.size_hint()
    }
}

impl<I, K, V, S, A> DoubleEndedIterator for Splice<'_, I, K, V, S, A>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.drain.next_back().map(Bucket::key_value)
    }
}

impl<I, K, V, S, A> ExactSizeIterator for Splice<'_, I, K, V, S, A>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn len(&self) -> usize {
        self.drain.len()
    }
}

impl<I, K, V, S, A> iter::FusedIterator for Splice<'_, I, K, V, S, A>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<I, K, V, S, A> fmt::Debug for Splice<'_, I, K, V, S, A>
where
    I: fmt::Debug + Iterator<Item = (K, V)>,
    K: any::Any + fmt::Debug + hash::Hash + Eq,
    V: any::Any + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Follow `vec::Splice` in only printing the drain and replacement
        formatter
            .debug_struct("Splice")
            .field("drain", &self.drain)
            .field("replace_with", &self.replace_with)
            .finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) struct HashValue {
    value: usize,
}

impl HashValue {
    #[inline]
    const fn new(value: usize) -> Self {
        Self { value }
    }

    #[inline(always)]
    const fn get(self) -> u64 {
        self.value as u64
    }
}

#[derive(Copy, Debug)]
pub(crate) struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}

impl<K, V> Clone for Bucket<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Bucket {
            hash: self.hash,
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        self.hash = other.hash;
        self.key.clone_from(&other.key);
        self.value.clone_from(&other.value);
    }
}

impl<K, V> Bucket<K, V> {
    #[inline(always)]
    const fn new(hash: HashValue, key: K, value: V) -> Self {
        Self { hash, key, value }
    }

    pub(crate) const fn key_ref(&self) -> &K {
        &self.key
    }

    pub(crate) const fn value_ref(&self) -> &V {
        &self.value
    }

    pub(crate) const fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    pub(crate) fn key(self) -> K {
        self.key
    }

    pub(crate) fn value(self) -> V {
        self.value
    }

    pub(crate) fn key_value(self) -> (K, V) {
        (self.key, self.value)
    }

    pub(crate) const fn refs(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }

    pub(crate) fn ref_mut(&mut self) -> (&K, &mut V) {
        (&self.key, &mut self.value)
    }

    /*
    pub(crate) const fn muts(&mut self) -> (&mut K, &mut V) {
        (&mut self.key, &mut self.value)
    }
    */
}

#[inline(always)]
fn get_hash<K, V>(entries: &[Bucket<K, V>]) -> impl Fn(&usize) -> u64 + '_ {
    move |&i| entries[i].hash.get()
}

#[inline]
fn equivalent<'a, K, V, Q: ?Sized + Equivalent<K>>(key: &'a Q, entries: &'a [Bucket<K, V>]) -> impl Fn(&usize) -> bool + 'a {
    move |&i| Q::equivalent(key, &entries[i].key)
}

#[inline]
fn erase_index(table: &mut hashbrown::HashTable<usize>, hash: HashValue, index: usize) {
    if let Ok(entry) = table.find_entry(hash.get(), move |&i| i == index) {
        entry.remove();
    } else if cfg!(debug_assertions) {
        panic!("index not found");
    }
}

#[inline]
fn update_index(table: &mut hashbrown::HashTable<usize>, hash: HashValue, old: usize, new: usize) {
    let index = table.find_mut(hash.get(), move |&i| i == old).expect("index not found");
    *index = new;
}

fn insert_bulk_no_grow<K, V>(indices: &mut hashbrown::HashTable<usize>, entries: &[Bucket<K, V>]) {
    assert!(indices.capacity() - indices.len() >= entries.len());
    for entry in entries {
        indices.insert_unique(entry.hash.get(), indices.len(), |_| unreachable!());
    }
}

#[inline(always)]
const fn max_entries_capacity<K, V>() -> usize {
    (isize::MAX as usize) / mem::size_of::<Bucket<K, V>>()
}

fn reserve_entries<K, V, A>(entries: &mut TypeProjectedVec<Bucket<K, V>, A>, additional: usize, try_capacity: usize)
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    // Use a soft-limit on the maximum capacity, but if the caller explicitly
    // requested more, do it and let them have the resulting panic.
    let try_capacity = try_capacity.min(max_entries_capacity::<K, V>());
    let try_add = try_capacity - entries.len();
    if try_add > additional && entries.try_reserve_exact(try_add).is_ok() {
        return;
    }
    entries.reserve_exact(additional);
}

struct RefMut<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    indices: &'a mut hashbrown::HashTable<usize>,
    entries: &'a mut TypeProjectedVec<Bucket<K, V>, A>,
}

impl<'a, K, V, A> RefMut<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn new(indices: &'a mut hashbrown::HashTable<usize>, entries: &'a mut TypeProjectedVec<Bucket<K, V>, A>) -> Self {
        Self { indices, entries }
    }

    #[inline]
    fn reserve_entries(&mut self, additional: usize) {
        reserve_entries::<K, V, A>(self.entries, additional, self.indices.capacity());
    }

    fn insert_unique(self, hash: HashValue, key: K, value: V) -> OccupiedEntry<'a, K, V, A> {
        let i = self.indices.len();
        debug_assert_eq!(i, self.entries.len());
        let entry = self.indices.insert_unique(hash.get(), i, get_hash(self.entries.as_slice()));
        if self.entries.len() == self.entries.capacity() {
            // We can't call `indices.capacity()` while this `entry` has borrowed it, so we'll have
            // to amortize growth on our own. It's still an improvement over the basic `Vec::push`
            // doubling though, since we also consider `MAX_ENTRIES_CAPACITY`.
            reserve_entries::<K, V, A>(self.entries, 1, 2 * self.entries.capacity());
        }
        self.entries.push(Bucket::new(hash, key, value));

        OccupiedEntry::new(self.entries, entry)
    }

    fn shift_insert_unique(&mut self, index: usize, hash: HashValue, key: K, value: V) {
        let end = self.indices.len();
        assert!(index <= end);
        // Increment others first so we don't have duplicate indices.
        self.increment_indices(index, end);
        let entries = &*self.entries;
        self.indices.insert_unique(hash.get(), index, move |&i| {
            // Adjust for the incremented indices to find hashes.
            debug_assert_ne!(i, index);
            let i = if i < index { i } else { i - 1 };
            entries.as_slice()[i].hash.get()
        });
        if self.entries.len() == self.entries.capacity() {
            // Reserve our own capacity synced to the indices,
            // rather than letting `Vec::insert` just double it.
            self.reserve_entries(1);
        }
        self.entries.shift_insert(index, Bucket::new(hash, key, value));
    }

    fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        match self.entries.get(index) {
            Some(entry) => {
                erase_index(self.indices, entry.hash, index);
                Some(self.shift_remove_finish(index))
            }
            None => None,
        }
    }

    fn shift_remove_finish(&mut self, index: usize) -> (K, V) {
        // Correct indices that point to the entries that followed the removed entry.
        self.decrement_indices(index + 1, self.entries.len());

        // Use Vec::remove to actually remove the entry.
        let entry = self.entries.shift_remove(index);

        (entry.key, entry.value)
    }

    fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        match self.entries.get(index) {
            Some(entry) => {
                erase_index(self.indices, entry.hash, index);
                Some(self.swap_remove_finish(index))
            }
            None => None,
        }
    }

    fn swap_remove_finish(&mut self, index: usize) -> (K, V) {
        // use swap_remove, but then we need to update the index that points
        // to the other entry that has to move
        let entry = self.entries.swap_remove(index);

        // correct index that points to the entry that had to swap places
        if let Some(entry) = self.entries.get(index) {
            // was not last element
            // examine new element in `index` and find it in indices
            let last = self.entries.len();
            update_index(self.indices, entry.hash, last, index);
        }

        (entry.key, entry.value)
    }

    fn decrement_indices(&mut self, start: usize, end: usize) {
        // Use a heuristic between a full sweep vs. a `find()` for every shifted item.
        let shifted_entries = &self.entries.as_slice()[start..end];
        if shifted_entries.len() > self.indices.capacity() / 2 {
            // Shift all indices in range.
            for i in &mut *self.indices {
                if start <= *i && *i < end {
                    *i -= 1;
                }
            }
        } else {
            // Find each entry in range to shift its index.
            for (i, entry) in (start..end).zip(shifted_entries) {
                update_index(self.indices, entry.hash, i, i - 1);
            }
        }
    }

    fn increment_indices(&mut self, start: usize, end: usize) {
        // Use a heuristic between a full sweep vs. a `find()` for every shifted item.
        let shifted_entries = &self.entries.as_slice()[start..end];
        if shifted_entries.len() > self.indices.capacity() / 2 {
            // Shift all indices in range.
            for i in &mut *self.indices {
                if start <= *i && *i < end {
                    *i += 1;
                }
            }
        } else {
            // Find each entry in range to shift its index, updated in reverse so
            // we never have duplicated indices that might have a hash collision.
            for (i, entry) in (start..end).zip(shifted_entries).rev() {
                update_index(self.indices, entry.hash, i, i + 1);
            }
        }
    }

    #[track_caller]
    fn move_index(&mut self, from: usize, to: usize) {
        let from_hash = self.entries.as_slice()[from].hash;
        let _ = self.entries.as_slice()[to]; // explicit bounds check
        if from != to {
            // Use a sentinel index so other indices don't collide.
            update_index(self.indices, from_hash, from, usize::MAX);

            // Update all other indices and rotate the entry positions.
            if from < to {
                self.decrement_indices(from + 1, to + 1);
                self.entries.as_mut_slice()[from..=to].rotate_left(1);
            } else if to < from {
                self.increment_indices(to, from);
                self.entries.as_mut_slice()[to..=from].rotate_right(1);
            }

            // Change the sentinel index to its final position.
            update_index(self.indices, from_hash, usize::MAX, to);
        }
    }

    #[track_caller]
    fn swap_indices(&mut self, a: usize, b: usize) {
        // If they're equal and in-bounds, there's nothing to do.
        if a == b && a < self.entries.len() {
            return;
        }

        // We'll get a "nice" bounds-check from indexing `entries`,
        // and then we expect to find it in the table as well.
        match self.indices.get_many_mut(
            [self.entries.as_slice()[a].hash.get(), self.entries.as_slice()[b].hash.get()],
            move |i, &x| if i == 0 { x == a } else { x == b },
        ) {
            [Some(ref_a), Some(ref_b)] => {
                mem::swap(ref_a, ref_b);
                self.entries.as_mut_slice().swap(a, b);
            }
            _ => panic!("indices not found"),
        }
    }
}

#[repr(C)]
pub(crate) struct TypeProjectedIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    indices: hashbrown::HashTable<usize>,
    entries: TypeProjectedVec<Bucket<K, V>, A>,
    key_type_id: any::TypeId,
    value_type_id: any::TypeId,
    allocator_type_id: any::TypeId,
}

impl<K, V, A> TypeProjectedIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) const fn key_type_id(&self) -> any::TypeId {
        self.key_type_id
    }

    #[inline]
    pub(crate) const fn value_type_id(&self) -> any::TypeId {
        self.value_type_id
    }

    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> any::TypeId {
        self.allocator_type_id
    }
}

impl<K, V, A> TypeProjectedIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn new_proj_in(alloc: TypeProjectedAlloc<A>) -> Self {
        let indices = hashbrown::HashTable::new();
        let entries = TypeProjectedVec::new_proj_in(alloc);
        let key_type_id = any::TypeId::of::<K>();
        let value_type_id = any::TypeId::of::<V>();
        let allocator_type_id = any::TypeId::of::<A>();

        Self {
            indices,
            entries,
            key_type_id,
            value_type_id,
            allocator_type_id,
        }
    }

    pub(crate) fn with_capacity_proj_in(capacity: usize, alloc: TypeProjectedAlloc<A>) -> Self {
        let indices = hashbrown::HashTable::with_capacity(capacity);
        let entries = TypeProjectedVec::with_capacity_proj_in(capacity, alloc);
        let key_type_id = any::TypeId::of::<K>();
        let value_type_id = any::TypeId::of::<V>();
        let allocator_type_id = any::TypeId::of::<A>();

        Self {
            indices,
            entries,
            key_type_id,
            value_type_id,
            allocator_type_id,
        }
    }
}

impl<K, V, A> TypeProjectedIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn new_in(alloc: A) -> Self {
        let indices = hashbrown::HashTable::new();
        let entries = TypeProjectedVec::new_in(alloc);
        let key_type_id = any::TypeId::of::<K>();
        let value_type_id = any::TypeId::of::<V>();
        let allocator_type_id = any::TypeId::of::<A>();

        Self {
            indices,
            entries,
            key_type_id,
            value_type_id,
            allocator_type_id,
        }
    }

    #[inline]
    pub(crate) fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let indices = hashbrown::HashTable::with_capacity(capacity);
        let entries = TypeProjectedVec::with_capacity_in(capacity, alloc);
        let key_type_id = any::TypeId::of::<K>();
        let value_type_id = any::TypeId::of::<V>();
        let allocator_type_id = any::TypeId::of::<A>();

        Self {
            indices,
            entries,
            key_type_id,
            value_type_id,
            allocator_type_id,
        }
    }
}

impl<K, V> TypeProjectedIndexMapCore<K, V, alloc::Global>
where
    K: any::Any,
    V: any::Any,
{
    #[inline]
    pub(crate) fn new() -> Self {
        Self::new_in(alloc::Global)
    }

    #[inline]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, alloc::Global)
    }
}

impl<K, V, A> TypeProjectedIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.indices.len()
    }

    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        Ord::min(self.indices.capacity(), self.entries.capacity())
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V, A> TypeProjectedIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn allocator(&self) -> &TypeProjectedAlloc<A> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.entries.allocator()
    }
}

impl<K, V, A> TypeProjectedIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn borrow_mut(&mut self) -> RefMut<'_, K, V, A> {
        RefMut::new(&mut self.indices, &mut self.entries)
    }

    pub(crate) fn clear(&mut self) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.indices.clear();
        self.entries.clear();
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if len < self.len() {
            self.erase_indices(len, self.entries.len());
            self.entries.truncate(len);
        }
    }

    #[track_caller]
    pub(crate) fn drain<R>(&mut self, range: R) -> opaque_vec::Drain<'_, Bucket<K, V>, A>
    where
        R: ops::RangeBounds<usize>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let range = range_ops::simplify_range(range, self.entries.len());
        self.erase_indices(range.start, range.end);

        self.entries.drain(range)
    }

    #[track_caller]
    pub(crate) fn extract<R>(&mut self, range: R) -> Extract<'_, K, V, A>
    where
        R: ops::RangeBounds<usize>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let range = range_ops::simplify_range(range, self.entries.len());

        // SAFETY: We must have consistent lengths to start, so that's a hard assertion.
        // Then the worst `set_len` can do is leak items if `ExtractCore` doesn't drop.
        assert_eq!(self.entries.len(), self.indices.len());
        unsafe {
            self.entries.set_len(range.start);
        }

        Extract {
            map: self,
            new_len: range.start,
            current: range.start,
            end: range.end,
        }
    }

    #[track_caller]
    pub(crate) fn split_off(&mut self, at: usize) -> Self
    where
        A: Clone,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let len = self.entries.len();
        assert!(
            at <= len,
            "index out of bounds: the len is {len} but the index is {at}. Expected index <= len"
        );

        self.erase_indices(at, self.entries.len());
        let entries = self.entries.split_off(at);

        let mut indices = hashbrown::HashTable::with_capacity(entries.len());
        insert_bulk_no_grow(&mut indices, entries.as_slice());

        let split_key_type_id = self.key_type_id;
        let split_value_type_id = self.value_type_id;
        let split_allocator_type_id = self.allocator_type_id;

        Self {
            indices,
            entries,
            key_type_id: split_key_type_id,
            value_type_id: split_value_type_id,
            allocator_type_id: split_allocator_type_id,
        }
    }

    #[track_caller]
    pub(crate) fn split_splice<R>(&mut self, range: R) -> (Self, opaque_vec::IntoIter<Bucket<K, V>, A>)
    where
        A: Clone,
        R: ops::RangeBounds<usize>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let range = range_ops::simplify_range(range, self.len());
        self.erase_indices(range.start, self.entries.len());
        let entries = self.entries.split_off(range.end);
        let drained = self.entries.split_off(range.start);

        // let mut indices = Indices::with_capacity(entries.len());
        let mut indices = hashbrown::HashTable::with_capacity(entries.len());
        insert_bulk_no_grow(&mut indices, entries.as_slice());

        let split_splice_key_type_id = self.key_type_id;
        let split_splice_value_type_id = self.value_type_id;
        let split_splice_allocator_type_id = self.allocator_type_id;

        (
            Self {
                indices,
                entries,
                key_type_id: split_splice_key_type_id,
                value_type_id: split_splice_value_type_id,
                allocator_type_id: split_splice_allocator_type_id,
            },
            drained.into_iter(),
        )
    }

    pub(crate) fn append_unchecked(&mut self, other: &mut Self) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.reserve(other.len());
        insert_bulk_no_grow(&mut self.indices, other.entries.as_slice());
        self.entries.append(&mut other.entries);
        other.indices.clear();
    }

    pub(crate) fn reserve(&mut self, additional: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.indices.reserve(additional, get_hash(self.entries.as_slice()));
        // Only grow entries if necessary, since we also round up capacity.
        if additional > self.entries.capacity() - self.entries.len() {
            self.borrow_mut().reserve_entries(additional);
        }
    }

    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.indices.reserve(additional, get_hash(self.entries.as_slice()));
        self.entries.reserve_exact(additional);
    }

    pub(crate) fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        fn from_hashbrown(error: hashbrown::TryReserveError) -> TryReserveError {
            let kind = match error {
                hashbrown::TryReserveError::CapacityOverflow => TryReserveErrorKind::CapacityOverflow,
                hashbrown::TryReserveError::AllocError { layout } => TryReserveErrorKind::AllocError { layout },
            };

            TryReserveError::from(kind)
        }

        self.indices
            .try_reserve(additional, get_hash::<K, V>(self.entries.as_slice()))
            .map_err(from_hashbrown)?;
        // Only grow entries if necessary, since we also round up capacity.
        if additional > self.entries.capacity() - self.entries.len() {
            self.try_reserve_entries(additional)
        } else {
            Ok(())
        }
    }

    /// The maximum capacity before the `entries` allocation would exceed `isize::MAX`. V>>()`.
    #[inline]
    const fn max_entries_capacity() -> usize {
        (isize::MAX as usize) / mem::size_of::<Bucket<K, V>>()
    }

    fn try_reserve_entries(&mut self, additional: usize) -> Result<(), TryReserveError> {
        // Use a soft-limit on the maximum capacity, but if the caller explicitly
        // requested more, do it and let them have the resulting error.
        let new_capacity = Ord::min(self.indices.capacity(), Self::max_entries_capacity());
        let try_add = new_capacity - self.entries.len();
        if try_add > additional && self.entries.try_reserve_exact(try_add).is_ok() {
            return Ok(());
        }

        self.entries.try_reserve_exact(additional)
    }

    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        fn from_hashbrown(error: hashbrown::TryReserveError) -> TryReserveError {
            let kind = match error {
                hashbrown::TryReserveError::CapacityOverflow => TryReserveErrorKind::CapacityOverflow,
                hashbrown::TryReserveError::AllocError { layout } => TryReserveErrorKind::AllocError { layout },
            };

            TryReserveError::from(kind)
        }

        self.indices
            .try_reserve(additional, get_hash(self.entries.as_slice()))
            .map_err(from_hashbrown)?;

        self.entries.try_reserve_exact(additional)
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.shrink_to(0);
    }

    pub(crate) fn shrink_to(&mut self, min_capacity: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.indices.shrink_to(min_capacity, get_hash(self.entries.as_slice()));
        self.entries.shrink_to(min_capacity);
    }

    pub(crate) fn pop(&mut self) -> Option<(K, V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if let Some(entry) = self.entries.pop() {
            let last = self.entries.len();
            erase_index(&mut self.indices, entry.hash, last);
            Some((entry.key, entry.value))
        } else {
            None
        }
    }

    pub(crate) fn get_index_of<Q>(&self, hash: HashValue, key: &Q) -> Option<usize>
    where
        Q: ?Sized + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let eq = equivalent(key, self.entries.as_slice());

        self.indices.find(hash.get(), eq).copied()
    }

    fn push_entry(&mut self, hash: HashValue, key: K, value: V) {
        if self.entries.len() == self.entries.capacity() {
            // Reserve our own capacity synced to the indices,
            // rather than letting `Vec::push` just double it.
            self.borrow_mut().reserve_entries(1);
        }

        self.entries.push(Bucket::new(hash, key, value));
    }

    pub(crate) fn insert_full(&mut self, hash: HashValue, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let eq = equivalent(&key, self.entries.as_slice());
        let hasher = get_hash(self.entries.as_slice());
        match self.indices.entry(hash.get(), eq, hasher) {
            hash_table::Entry::Occupied(entry) => {
                let i = *entry.get();

                (i, Some(mem::replace(&mut self.as_entries_mut()[i].value, value)))
            }
            hash_table::Entry::Vacant(entry) => {
                let i = self.entries.len();
                entry.insert(i);
                self.push_entry(hash, key, value);

                debug_assert_eq!(self.indices.len(), self.entries.len());

                (i, None)
            }
        }
    }

    pub(crate) fn replace_full(&mut self, hash: HashValue, key: K, value: V) -> (usize, Option<(K, V)>)
    where
        K: Eq,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let eq = equivalent(&key, &self.entries);
        let hasher = get_hash(&self.entries);
        match self.indices.entry(hash.get(), eq, hasher) {
            hash_table::Entry::Occupied(entry) => {
                let i = *entry.get();
                let entry = &mut self.entries[i];
                let kv = (mem::replace(&mut entry.key, key), mem::replace(&mut entry.value, value));
                (i, Some(kv))
            }
            hash_table::Entry::Vacant(entry) => {
                let i = self.entries.len();
                entry.insert(i);
                self.push_entry(hash, key, value);
                debug_assert_eq!(self.indices.len(), self.entries.len());
                (i, None)
            }
        }
    }

    pub(crate) fn shift_remove_full<Q>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let eq = equivalent(key, self.entries.as_slice());
        match self.indices.find_entry(hash.get(), eq) {
            Ok(entry) => {
                let (index, _) = entry.remove();
                let (key, value) = self.borrow_mut().shift_remove_finish(index);
                Some((index, key, value))
            }
            Err(_) => None,
        }
    }

    #[inline]
    pub(crate) fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.borrow_mut().shift_remove_index(index)
    }

    #[inline]
    #[track_caller]
    pub(crate) fn move_index(&mut self, from: usize, to: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.borrow_mut().move_index(from, to);
    }

    #[inline]
    #[track_caller]
    pub(crate) fn swap_indices(&mut self, a: usize, b: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.borrow_mut().swap_indices(a, b);
    }

    pub(crate) fn swap_remove_full<Q>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let eq = equivalent(key, self.entries.as_slice());
        match self.indices.find_entry(hash.get(), eq) {
            Ok(entry) => {
                let (index, _) = entry.remove();
                let (key, value) = self.borrow_mut().swap_remove_finish(index);
                Some((index, key, value))
            }
            Err(_) => None,
        }
    }

    #[inline]
    pub(crate) fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.borrow_mut().swap_remove_index(index)
    }

    fn erase_indices(&mut self, start: usize, end: usize) {
        let (init, shifted_entries) = self.entries.as_slice().split_at(end);
        let (start_entries, erased_entries) = init.split_at(start);

        let erased = erased_entries.len();
        let shifted = shifted_entries.len();
        let half_capacity = self.indices.capacity() / 2;

        // Use a heuristic between different strategies
        if erased == 0 {
            // Degenerate case, nothing to do
        } else if start + shifted < half_capacity && start < erased {
            // Reinsert everything, as there are few kept indices
            self.indices.clear();

            // Reinsert stable indices, then shifted indices
            insert_bulk_no_grow(&mut self.indices, start_entries);
            insert_bulk_no_grow(&mut self.indices, shifted_entries);
        } else if erased + shifted < half_capacity {
            // Find each affected index, as there are few to adjust

            // Find erased indices
            for (i, entry) in (start..).zip(erased_entries) {
                erase_index(&mut self.indices, entry.hash, i);
            }

            // Find shifted indices
            for ((new, old), entry) in (start..).zip(end..).zip(shifted_entries) {
                update_index(&mut self.indices, entry.hash, old, new);
            }
        } else {
            // Sweep the whole table for adjustments
            let offset = end - start;
            self.indices.retain(move |i| {
                if *i >= end {
                    *i -= offset;
                    true
                } else {
                    *i < start
                }
            });
        }

        debug_assert_eq!(self.indices.len(), start + shifted);
    }

    pub(crate) fn retain_in_order<F>(&mut self, mut keep: F)
    where
        F: FnMut(&mut K, &mut V) -> bool,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.entries
            .retain_mut(|entry: &mut Bucket<K, V>| keep(&mut entry.key, &mut entry.value));

        if self.entries.len() < self.indices.len() {
            self.rebuild_hash_table();
        }
    }

    fn rebuild_hash_table(&mut self) {
        self.indices.clear();
        insert_bulk_no_grow(&mut self.indices, self.entries.as_slice());
    }

    pub(crate) fn reverse(&mut self) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.entries.reverse();

        // No need to save hash indices, can easily calculate what they should
        // be, given that this is an in-place reversal.
        let len = self.entries.len();
        for i in &mut self.indices {
            *i = len - *i - 1;
        }
    }
}

impl<K, V, A> TypeProjectedIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn into_entries(self) -> TypeProjectedVec<Bucket<K, V>, A> {
        self.entries
    }

    #[inline]
    fn as_entries(&self) -> &[Bucket<K, V>] {
        self.entries.as_slice()
    }

    #[inline]
    fn as_entries_mut(&mut self) -> &mut [Bucket<K, V>] {
        self.entries.as_mut_slice()
    }

    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Bucket<K, V>]),
    {
        f(self.entries.as_mut_slice());

        self.rebuild_hash_table();
    }
}

impl<K, V, A> TypeProjectedIndexMapCore<K, V, A>
where
    K: any::Any + Eq,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn entry(&mut self, hash: HashValue, key: K) -> Entry<'_, K, V, A> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let entries = &mut self.entries;
        let eq = equivalent(&key, entries.as_slice());
        match self.indices.find_entry(hash.get(), eq) {
            Ok(index) => Entry::Occupied(OccupiedEntry { entries, index }),
            Err(absent) => Entry::Vacant(VacantEntry {
                map: RefMut::new(absent.into_table(), entries),
                hash,
                key,
            }),
        }
    }
}

impl<K, V, A> Clone for TypeProjectedIndexMapCore<K, V, A>
where
    K: any::Any + Clone,
    V: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let cloned_indices = self.indices.clone();
        let cloned_entries = self.entries.clone();
        let cloned_key_type_id = self.key_type_id;
        let cloned_value_type_id = self.value_type_id;
        let cloned_allocator_type_id = self.allocator_type_id;

        Self {
            indices: cloned_indices,
            entries: cloned_entries,
            key_type_id: cloned_key_type_id,
            value_type_id: cloned_value_type_id,
            allocator_type_id: cloned_allocator_type_id,
        }
    }

    /*
    fn clone_from(&mut self, other: &Self) {
        todo!()
    }
    */
}

#[repr(C)]
struct TypeErasedIndexMapCore {
    indices: hashbrown::HashTable<usize>,
    entries: TypeErasedVec,
    key_type_id: any::TypeId,
    value_type_id: any::TypeId,
    allocator_type_id: any::TypeId,
}

impl TypeErasedIndexMapCore {
    #[inline]
    pub(crate) const fn key_type_id(&self) -> any::TypeId {
        self.key_type_id
    }

    #[inline]
    pub(crate) const fn value_type_id(&self) -> any::TypeId {
        self.value_type_id
    }

    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> any::TypeId {
        self.allocator_type_id
    }
}

impl TypeErasedIndexMapCore {
    #[inline(always)]
    fn as_proj_assuming_type<K, V, A>(&self) -> &TypeProjectedIndexMapCore<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe { &*(self as *const TypeErasedIndexMapCore as *const TypeProjectedIndexMapCore<K, V, A>) }
    }

    #[inline(always)]
    fn as_proj_mut_assuming_type<K, V, A>(&mut self) -> &mut TypeProjectedIndexMapCore<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe { &mut *(self as *mut TypeErasedIndexMapCore as *mut TypeProjectedIndexMapCore<K, V, A>) }
    }

    #[inline(always)]
    fn into_proj_assuming_type<K, V, A>(self) -> TypeProjectedIndexMapCore<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        TypeProjectedIndexMapCore {
            indices: self.indices,
            entries: self.entries.into_proj::<Bucket<K, V>, A>(),
            key_type_id: self.key_type_id,
            value_type_id: self.value_type_id,
            allocator_type_id: self.allocator_type_id,
        }
    }

    #[inline(always)]
    fn from_proj_assuming_type<K, V, A>(proj_self: TypeProjectedIndexMapCore<K, V, A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        Self {
            indices: proj_self.indices,
            entries: TypeErasedVec::from_proj(proj_self.entries),
            key_type_id: proj_self.key_type_id,
            value_type_id: proj_self.value_type_id,
            allocator_type_id: proj_self.allocator_type_id,
        }
    }
}

impl TypeErasedIndexMapCore {
    #[inline]
    fn capacity(&self) -> usize {
        Ord::min(self.indices.capacity(), self.entries.capacity())
    }

    #[inline]
    fn len(&self) -> usize {
        self.indices.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub enum Entry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    Occupied(OccupiedEntry<'a, K, V, A>),
    Vacant(VacantEntry<'a, K, V, A>),
}

impl<'a, K, V, A> Entry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn index(&self) -> usize {
        match *self {
            Entry::Occupied(ref entry) => entry.index(),
            Entry::Vacant(ref entry) => entry.index(),
        }
    }

    pub(crate) fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, A> {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            Entry::Vacant(entry) => entry.insert_entry(value),
        }
    }

    pub(crate) fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    pub(crate) fn or_insert_with<F>(self, call: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(call()),
        }
    }

    pub(crate) fn or_insert_with_key<F>(self, call: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = call(&entry.key);
                entry.insert(value)
            }
        }
    }

    pub(crate) fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    pub(crate) fn and_modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        if let Entry::Occupied(entry) = &mut self {
            f(entry.get_mut());
        }
        self
    }

    pub(crate) fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(V::default()),
        }
    }
}

impl<K, V, A> fmt::Debug for Entry<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = formatter.debug_tuple("Entry");
        match self {
            Entry::Vacant(v) => tuple.field(v),
            Entry::Occupied(o) => tuple.field(o),
        };
        tuple.finish()
    }
}

pub(crate) struct OccupiedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    entries: &'a mut TypeProjectedVec<Bucket<K, V>, A>,
    index: hash_table::OccupiedEntry<'a, usize>,
}

impl<'a, K, V, A> OccupiedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn new(entries: &'a mut TypeProjectedVec<Bucket<K, V>, A>, index: hash_table::OccupiedEntry<'a, usize>) -> Self {
        Self { entries, index }
    }

    #[inline]
    pub(crate) fn index(&self) -> usize {
        *self.index.get()
    }

    #[inline]
    fn into_ref_mut(self) -> RefMut<'a, K, V, A> {
        RefMut::new(self.index.into_table(), self.entries)
    }

    pub(crate) fn key(&self) -> &K {
        &self.entries.as_slice()[self.index()].key
    }

    /*
    pub(crate) fn key_mut(&mut self) -> &mut K {
        let index = self.index();

        &mut self.entries.as_mut_slice()[index].key
    }
    */

    pub(crate) fn get(&self) -> &V {
        &self.entries.as_slice()[self.index()].value
    }

    pub(crate) fn get_mut(&mut self) -> &mut V {
        let index = self.index();

        &mut self.entries.as_mut_slice()[index].value
    }

    pub(crate) fn into_mut(self) -> &'a mut V {
        let index = self.index();

        &mut self.entries.as_mut_slice()[index].value
    }

    /*
    fn into_muts(self) -> (&'a mut K, &'a mut V) {
        let index = self.index();

        self.entries.as_mut_slice()[index].muts()
    }
    */

    pub(crate) fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }

    pub(crate) fn swap_remove(self) -> V {
        self.swap_remove_entry().1
    }

    pub(crate) fn shift_remove(self) -> V {
        self.shift_remove_entry().1
    }

    pub(crate) fn swap_remove_entry(self) -> (K, V) {
        let (index, entry) = self.index.remove();
        RefMut::<'_, K, V, A>::new(entry.into_table(), self.entries).swap_remove_finish(index)
    }

    pub(crate) fn shift_remove_entry(self) -> (K, V) {
        let (index, entry) = self.index.remove();
        RefMut::<'_, K, V, A>::new(entry.into_table(), self.entries).shift_remove_finish(index)
    }

    #[track_caller]
    pub(crate) fn move_index(self, to: usize) {
        let index = self.index();
        self.into_ref_mut().move_index(index, to);
    }

    pub(crate) fn swap_indices(self, other: usize) {
        let index = self.index();
        self.into_ref_mut().swap_indices(index, other);
    }
}

impl<K, V, A> fmt::Debug for OccupiedEntry<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

impl<'a, K, V, A> From<IndexedEntry<'a, K, V, A>> for OccupiedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn from(other: IndexedEntry<'a, K, V, A>) -> Self {
        let IndexedEntry {
            map: RefMut { indices, entries },
            index,
        } = other;
        let hash = entries.as_slice()[index].hash;
        let index = indices.find_entry(hash.get(), move |&i| i == index).expect("index not found");

        Self { entries, index }
    }
}

pub(crate) struct VacantEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    map: RefMut<'a, K, V, A>,
    hash: HashValue,
    key: K,
}

impl<'a, K, V, A> VacantEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn index(&self) -> usize {
        self.map.indices.len()
    }

    pub(crate) fn key(&self) -> &K {
        &self.key
    }

    /*
    pub(crate) fn key_mut(&mut self) -> &mut K {
        &mut self.key
    }
    */

    pub(crate) fn into_key(self) -> K {
        self.key
    }

    pub(crate) fn insert(self, value: V) -> &'a mut V {
        self.insert_entry(value).into_mut()
    }

    pub(crate) fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, A> {
        let Self { map, hash, key } = self;

        map.insert_unique(hash, key, value)
    }

    pub(crate) fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {
        let slice = Slice::from_slice(self.map.entries.as_slice());
        let i = slice.binary_search_keys(&self.key).unwrap_err();

        (i, self.shift_insert(i, value))
    }

    pub(crate) fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
        self.map.shift_insert_unique(index, self.hash, self.key, value);

        &mut self.map.entries.as_mut_slice()[index].value
    }
}

impl<K, V, A> fmt::Debug for VacantEntry<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}

pub(crate) struct IndexedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    map: RefMut<'a, K, V, A>,
    index: usize,
}

impl<'a, K, V, A> IndexedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn new(map: &'a mut TypeProjectedIndexMapCore<K, V, A>, index: usize) -> Self
    where
        K: Ord,
    {
        Self {
            map: map.borrow_mut(),
            index,
        }
    }

    #[inline]
    pub(crate) fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn key(&self) -> &K {
        &self.map.entries.as_slice()[self.index].key
    }

    /*
    pub(crate) fn key_mut(&mut self) -> &mut K {
        &mut self.map.entries.as_mut_slice()[self.index].key
    }
    */

    pub(crate) fn get(&self) -> &V {
        &self.map.entries.as_slice()[self.index].value
    }

    pub(crate) fn get_mut(&mut self) -> &mut V {
        &mut self.map.entries.as_mut_slice()[self.index].value
    }

    pub(crate) fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }

    pub(crate) fn into_mut(self) -> &'a mut V {
        &mut self.map.entries.as_mut_slice()[self.index].value
    }

    pub(crate) fn swap_remove_entry(mut self) -> (K, V) {
        self.map.swap_remove_index(self.index).unwrap()
    }

    pub(crate) fn shift_remove_entry(mut self) -> (K, V) {
        self.map.shift_remove_index(self.index).unwrap()
    }

    pub(crate) fn swap_remove(self) -> V {
        self.swap_remove_entry().1
    }

    pub(crate) fn shift_remove(self) -> V {
        self.shift_remove_entry().1
    }

    #[track_caller]
    pub(crate) fn move_index(mut self, to: usize) {
        self.map.move_index(self.index, to);
    }

    pub(crate) fn swap_indices(mut self, other: usize) {
        self.map.swap_indices(self.index, other);
    }
}

impl<K, V, A> fmt::Debug for IndexedEntry<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IndexedEntry")
            .field("index", &self.index)
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

impl<'a, K, V, A> From<OccupiedEntry<'a, K, V, A>> for IndexedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn from(other: OccupiedEntry<'a, K, V, A>) -> Self {
        Self {
            index: other.index(),
            map: other.into_ref_mut(),
        }
    }
}

#[cfg(test)]
mod entry_assert_send_sync {
    use super::*;

    #[test]
    fn test_assert_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<Entry<'_, i32, i32, alloc::Global>>();
        assert_send_sync::<OccupiedEntry<'_, i32, i32, alloc::Global>>();
        assert_send_sync::<VacantEntry<'_, i32, i32, alloc::Global>>();
        assert_send_sync::<IndexedEntry<'_, i32, i32, alloc::Global>>();
    }
}

#[repr(C)]
pub(crate) struct TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: TypeProjectedIndexMapCore<K, V, A>,
    build_hasher: TypeProjectedBuildHasher<S>,
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) const fn key_type_id(&self) -> any::TypeId {
        self.inner.key_type_id()
    }

    #[inline]
    pub(crate) const fn value_type_id(&self) -> any::TypeId {
        self.inner.value_type_id()
    }

    #[inline]
    pub(crate) const fn build_hasher_type_id(&self) -> any::TypeId {
        self.build_hasher.build_hasher_type_id()
    }

    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn into_entries(self) -> TypeProjectedVec<Bucket<K, V>, A> {
        self.inner.into_entries()
    }

    #[inline]
    pub(crate) fn as_entries(&self) -> &[Bucket<K, V>] {
        self.inner.as_entries()
    }

    #[inline]
    pub(crate) fn as_entries_mut(&mut self) -> &mut [Bucket<K, V>] {
        self.inner.as_entries_mut()
    }

    pub(crate) fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Bucket<K, V>]),
    {
        self.inner.with_entries(f);
    }
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn with_hasher_proj_in(proj_build_hasher: TypeProjectedBuildHasher<S>, proj_alloc: TypeProjectedAlloc<A>) -> Self {
        let proj_inner = TypeProjectedIndexMapCore::<K, V, A>::new_proj_in(proj_alloc);

        Self {
            inner: proj_inner,
            build_hasher: proj_build_hasher,
        }
    }

    #[inline]
    pub(crate) fn with_capacity_and_hasher_proj_in(
        capacity: usize,
        proj_build_hasher: TypeProjectedBuildHasher<S>,
        proj_alloc: TypeProjectedAlloc<A>,
    ) -> Self {
        if capacity == 0 {
            Self::with_hasher_proj_in(proj_build_hasher, proj_alloc)
        } else {
            let proj_inner = TypeProjectedIndexMapCore::<K, V, A>::with_capacity_proj_in(capacity, proj_alloc);

            Self {
                inner: proj_inner,
                build_hasher: proj_build_hasher,
            }
        }
    }
}

#[cfg(feature = "std")]
impl<K, V, A> TypeProjectedIndexMapInner<K, V, hash::RandomState, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn new_proj_in(proj_alloc: TypeProjectedAlloc<A>) -> Self {
        let proj_inner = TypeProjectedIndexMapCore::<K, V, A>::new_proj_in(proj_alloc);
        let proj_build_hasher = TypeProjectedBuildHasher::new(hash::RandomState::new());

        Self {
            inner: proj_inner,
            build_hasher: proj_build_hasher,
        }
    }

    pub(crate) fn with_capacity_proj_in(capacity: usize, proj_alloc: TypeProjectedAlloc<A>) -> Self {
        let proj_inner = TypeProjectedIndexMapCore::<K, V, A>::with_capacity_proj_in(capacity, proj_alloc);
        let proj_build_hasher = TypeProjectedBuildHasher::new(hash::RandomState::new());

        Self {
            inner: proj_inner,
            build_hasher: proj_build_hasher,
        }
    }
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn with_hasher_in(build_hasher: S, alloc: A) -> Self {
        let proj_inner = TypeProjectedIndexMapCore::<K, V, A>::new_in(alloc);
        let proj_build_hasher = TypeProjectedBuildHasher::new(build_hasher);

        Self {
            inner: proj_inner,
            build_hasher: proj_build_hasher,
        }
    }

    #[inline]
    pub(crate) fn with_capacity_and_hasher_in(capacity: usize, build_hasher: S, alloc: A) -> Self {
        if capacity == 0 {
            Self::with_hasher_in(build_hasher, alloc)
        } else {
            let proj_inner = TypeProjectedIndexMapCore::<K, V, A>::with_capacity_in(capacity, alloc);
            let proj_build_hasher = TypeProjectedBuildHasher::new(build_hasher);

            Self {
                inner: proj_inner,
                build_hasher: proj_build_hasher,
            }
        }
    }
}

#[cfg(feature = "std")]
impl<K, V, A> TypeProjectedIndexMapInner<K, V, hash::RandomState, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn new_in(alloc: A) -> Self {
        let proj_inner = TypeProjectedIndexMapCore::<K, V, A>::new_in(alloc);
        let proj_build_hasher = TypeProjectedBuildHasher::<hash::RandomState>::new(hash::RandomState::default());

        Self {
            inner: proj_inner,
            build_hasher: proj_build_hasher,
        }
    }

    pub(crate) fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let proj_inner = TypeProjectedIndexMapCore::<K, V, A>::with_capacity_in(capacity, alloc);
        let proj_build_hasher = TypeProjectedBuildHasher::<hash::RandomState>::new(hash::RandomState::default());

        Self {
            inner: proj_inner,
            build_hasher: proj_build_hasher,
        }
    }
}

impl<K, V, S> TypeProjectedIndexMapInner<K, V, S, alloc::Global>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    #[inline]
    pub(crate) fn with_hasher(build_hasher: S) -> Self {
        Self::with_hasher_in(build_hasher, alloc::Global)
    }

    #[inline]
    pub(crate) fn with_capacity_and_hasher(capacity: usize, build_hasher: S) -> Self {
        Self::with_capacity_and_hasher_in(capacity, build_hasher, alloc::Global)
    }
}

#[cfg(feature = "std")]
impl<K, V> TypeProjectedIndexMapInner<K, V, hash::RandomState, alloc::Global>
where
    K: any::Any,
    V: any::Any,
{
    #[inline]
    pub(crate) fn new() -> Self {
        Self::new_in(alloc::Global)
    }

    #[inline]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, alloc::Global)
    }
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) const fn hasher(&self) -> &TypeProjectedBuildHasher<S> {
        &self.build_hasher
    }

    #[inline]
    pub(crate) fn allocator(&self) -> &TypeProjectedAlloc<A> {
        self.inner.allocator()
    }
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn hash<Q>(&self, key: &Q) -> HashValue
    where
        Q: ?Sized + hash::Hash,
    {
        let mut hasher = hash::BuildHasher::build_hasher(&self.build_hasher);
        key.hash(&mut hasher);

        HashValue::new(hash::Hasher::finish(&mut hasher) as usize)
    }

    pub(crate) fn replace_full(&mut self, key: K, value: V) -> (usize, Option<(K, V)>)
    where
        K: hash::Hash + Eq,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let hash = self.hash(&key);
        self.inner.replace_full(hash, key, value)
    }

    pub(crate) fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        match self.as_entries() {
            [] => None,
            [x] => key.equivalent(&x.key).then_some(0),
            _ => {
                let hash = self.hash(key);
                self.inner.get_index_of(hash, key)
            }
        }
    }

    pub(crate) fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.get_index_of::<Q>(key).is_some()
    }

    pub(crate) fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if let Some(index) = self.get_index_of::<Q>(key) {
            let entry = &self.as_entries()[index];
            Some(&entry.value)
        } else {
            None
        }
    }

    pub(crate) fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if let Some(i) = self.get_index_of::<Q>(key) {
            let entry = &self.as_entries()[i];
            Some((&entry.key, &entry.value))
        } else {
            None
        }
    }

    pub(crate) fn get_full<Q>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if let Some(i) = self.get_index_of::<Q>(key) {
            let entry = &self.as_entries()[i];
            Some((i, &entry.key, &entry.value))
        } else {
            None
        }
    }

    pub(crate) fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if let Some(i) = self.get_index_of::<Q>(key) {
            let entry = &mut self.as_entries_mut()[i];
            Some(&mut entry.value)
        } else {
            None
        }
    }

    pub(crate) fn get_key_value_mut<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if let Some(i) = self.get_index_of::<Q>(key) {
            let entry = &mut self.as_entries_mut()[i];
            Some((&entry.key, &mut entry.value))
        } else {
            None
        }
    }

    pub(crate) fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if let Some(i) = self.get_index_of::<Q>(key) {
            let entry = &mut self.as_entries_mut()[i];

            Some((i, &entry.key, &mut entry.value))
        } else {
            None
        }
    }

    pub(crate) fn get_disjoint_mut<Q, const N: usize>(&mut self, keys: [&Q; N]) -> [Option<&mut V>; N]
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let indices = keys.map(|key| self.get_index_of(key));
        match self.as_mut_slice().get_disjoint_opt_mut(indices) {
            Err(GetDisjointMutError::IndexOutOfBounds) => {
                unreachable!("Internal error: indices should never be OOB as we got them from get_index_of");
            }
            Err(GetDisjointMutError::OverlappingIndices) => {
                panic!("duplicate keys found");
            }
            Ok(key_values) => key_values.map(|kv_opt| kv_opt.map(|kv| kv.1)),
        }
    }

    pub(crate) fn keys(&self) -> Keys<'_, K, V> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        Keys::new(self.as_entries())
    }

    pub(crate) fn into_keys(self) -> IntoKeys<K, V, A> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        IntoKeys::new(self.into_entries())
    }

    pub(crate) fn iter(&self) -> Iter<'_, K, V> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        Iter::new(self.as_entries())
    }

    pub(crate) fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        IterMut::new(self.as_entries_mut())
    }

    pub(crate) fn values(&self) -> Values<'_, K, V> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        Values::new(self.as_entries())
    }

    pub(crate) fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        ValuesMut::new(self.as_entries_mut())
    }

    pub(crate) fn into_values(self) -> IntoValues<K, V, A> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        IntoValues::new(self.into_entries())
    }

    pub(crate) fn clear(&mut self) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.clear();
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.truncate(len);
    }

    #[track_caller]
    pub(crate) fn drain<R>(&mut self, range: R) -> Drain<'_, K, V, A>
    where
        R: ops::RangeBounds<usize>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        Drain::new(self.inner.drain::<R>(range))
    }

    #[track_caller]
    pub(crate) fn extract<R>(&mut self, range: R) -> Extract<'_, K, V, A>
    where
        R: ops::RangeBounds<usize>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.extract::<R>(range)
    }

    #[track_caller]
    pub(crate) fn split_off(&mut self, at: usize) -> Self
    where
        S: Clone,
        A: Clone,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        Self {
            inner: self.inner.split_off(at),
            build_hasher: self.build_hasher.clone(),
        }
    }


    pub(crate) fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        fn third<A, B, C>(triple: (A, B, C)) -> C {
            triple.2
        }

        self.swap_remove_full::<Q>(key).map(third)
    }

    pub(crate) fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        match self.swap_remove_full::<Q>(key) {
            Some((_, key, value)) => Some((key, value)),
            None => None,
        }
    }

    pub(crate) fn swap_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        match self.as_entries() {
            [x] if key.equivalent(&x.key) => {
                let (k, v) = self.inner.pop()?;
                Some((0, k, v))
            }
            [_] | [] => None,
            _ => {
                let hash = self.hash(key);
                self.inner.swap_remove_full::<Q>(hash, key)
            }
        }
    }

    pub(crate) fn shift_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        fn third<A, B, C>(triple: (A, B, C)) -> C {
            triple.2
        }

        self.shift_remove_full::<Q>(key).map(third)
    }

    pub(crate) fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        match self.shift_remove_full::<Q>(key) {
            Some((_, key, value)) => Some((key, value)),
            None => None,
        }
    }

    pub(crate) fn shift_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        match self.as_entries() {
            [x] if key.equivalent(&x.key) => {
                let (k, v) = self.inner.pop()?;
                Some((0, k, v))
            }
            [_] | [] => None,
            _ => {
                let hash = self.hash(key);

                self.inner.shift_remove_full::<Q>(hash, key)
            }
        }
    }

    pub(crate) fn as_slice(&self) -> &'_ Slice<K, V> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        Slice::from_slice(self.as_entries())
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        Slice::from_slice_mut(self.as_entries_mut())
    }
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Eq + hash::Hash,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.insert_full(key, value).1
    }

    pub(crate) fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let hash = self.hash(&key);

        self.inner.insert_full(hash, key, value)
    }

    pub(crate) fn insert_sorted(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash + Ord,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        match self.binary_search_keys(&key) {
            Ok(i) => {
                let destination = self.get_index_mut(i).unwrap().1;
                let old_value = mem::replace(destination, value);

                (i, Some(old_value))
            }
            Err(i) => self.insert_before(i, key, value),
        }
    }

    #[track_caller]
    pub(crate) fn insert_before(&mut self, mut index: usize, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let len = self.len();

        assert!(
            index <= len,
            "index out of bounds: the len is {len} but the index is {index}. Expected index <= len"
        );

        match self.entry(key) {
            Entry::Occupied(mut entry) => {
                if index > entry.index() {
                    // Some entries will shift down when this one moves up,
                    // so "insert before index" becomes "move to index - 1",
                    // keeping the entry at the original index unmoved.
                    index -= 1;
                }
                let old = mem::replace(entry.get_mut(), value);
                entry.move_index(index);

                (index, Some(old))
            }
            Entry::Vacant(entry) => {
                entry.shift_insert(index, value);

                (index, None)
            }
        }
    }

    #[track_caller]
    pub(crate) fn shift_insert(&mut self, index: usize, key: K, value: V) -> Option<V>
    where
        K: Eq + hash::Hash,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let len = self.len();
        match self.entry(key) {
            Entry::Occupied(mut entry) => {
                assert!(index < len, "index out of bounds: the len is {len} but the index is {index}");

                let old = mem::replace(entry.get_mut(), value);
                entry.move_index(index);

                Some(old)
            }
            Entry::Vacant(entry) => {
                assert!(
                    index <= len,
                    "index out of bounds: the len is {len} but the index is {index}. Expected index <= len"
                );

                entry.shift_insert(index, value);

                None
            }
        }
    }

    pub(crate) fn entry(&mut self, key: K) -> Entry<'_, K, V, A>
    where
        K: Eq + hash::Hash,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let hash = self.hash(&key);

        self.inner.entry(hash, key)
    }

    #[track_caller]
    pub(crate) fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, K, V, S, A>
    where
        K: Eq + hash::Hash,
        A: Clone,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = (K, V)>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        Splice::new(self, range, replace_with.into_iter())
    }
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn append<S2>(&mut self, other: &mut TypeProjectedIndexMapInner<K, V, S2, A>)
    where
        K: Eq + hash::Hash,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        debug_assert_eq!(other.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(other.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(other.build_hasher_type_id(), any::TypeId::of::<S2>());
        debug_assert_eq!(other.allocator_type_id(), any::TypeId::of::<A>());

        self.extend(other.drain::<_>(..));
    }
}

impl<K, V, S, A> Default for TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Self::with_capacity_and_hasher_in(0, S::default(), A::default())
    }
}

impl<K, V, S, A> Clone for TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any + Clone,
    V: any::Any + Clone,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let cloned_inner = self.inner.clone();
        let cloned_builder_hasher = self.build_hasher.clone();

        Self {
            inner: cloned_inner,
            build_hasher: cloned_builder_hasher,
        }
    }

    /*
    fn clone_from(&mut self, other: &Self) {
        todo!()
    }
    */
}

impl<K, V, S, A> Extend<(K, V)> for TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        // (Note: this is a copy of `std`/`hashbrown`'s reservation logic.)
        // Keys may be already present or show multiple times in the iterator.
        // Reserve the entire hint lower bound if the map is empty.
        // Otherwise, reserve half the hint (rounded up), so the map
        // will only resize twice in the worst case.
        let iterator = iterable.into_iter();
        let reserve_count = if self.is_empty() {
            iterator.size_hint().0
        } else {
            (iterator.size_hint().0 + 1) / 2
        };
        self.reserve(reserve_count);
        iterator.for_each(move |(k, v)| {
            self.insert(k, v);
        });
    }
}

impl<'a, K, V, S, A> Extend<(&'a K, &'a V)> for TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any + hash::Hash + Eq + Copy,
    V: any::Any + Copy,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (&'a K, &'a V)>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.extend(iterable.into_iter().map(|(&key, &value)| (key, value)));
    }
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[doc(alias = "pop_last")]
    pub(crate) fn pop(&mut self) -> Option<(K, V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.pop()
    }

    pub(crate) fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.retain_in_order::<_>(move |k, v| keep(k, v));
    }

    pub(crate) fn sort_keys(&mut self)
    where
        K: Ord,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.with_entries::<_>(move |entries| {
            entries.sort_by(move |a, b| K::cmp(&a.key, &b.key));
        });
    }

    pub(crate) fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.with_entries::<_>(move |entries| {
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
        });
    }

    pub(crate) fn sorted_by<F>(self, mut cmp: F) -> IntoIter<K, V, A>
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let mut entries = self.into_entries();
        entries
            .as_mut_slice()
            .sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));

        IntoIter::new(entries)
    }

    pub(crate) fn sort_unstable_keys(&mut self)
    where
        K: Ord,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.with_entries::<_>(move |entries| {
            entries.sort_unstable_by(move |a, b| K::cmp(&a.key, &b.key));
        });
    }

    pub(crate) fn sort_unstable_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.with_entries::<_>(move |entries| {
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
        });
    }

    #[inline]
    pub(crate) fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<K, V, A>
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let mut entries = self.into_entries();
        entries
            .as_mut_slice()
            .sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));

        IntoIter::new(entries)
    }

    pub(crate) fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
    where
        T: Ord,
        F: FnMut(&K, &V) -> T,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.with_entries::<_>(move |entries| {
            entries.sort_by_cached_key(move |a| sort_key(&a.key, &a.value));
        });
    }

    pub(crate) fn binary_search_keys(&self, key: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_slice().binary_search_keys(key)
    }

    #[inline]
    pub(crate) fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> cmp::Ordering,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_slice().binary_search_by(f)
    }

    #[inline]
    pub(crate) fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> B,
        B: Ord,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_slice().binary_search_by_key(b, f)
    }

    #[must_use]
    pub(crate) fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_slice().partition_point(pred)
    }

    pub(crate) fn reverse(&mut self) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.reverse();
    }

    pub(crate) fn reserve(&mut self, additional: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.reserve(additional);
    }

    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.reserve_exact(additional);
    }

    pub(crate) fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.try_reserve(additional)
    }

    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.try_reserve_exact(additional)
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.shrink_to_fit();
    }

    pub(crate) fn shrink_to(&mut self, min_capacity: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.shrink_to(min_capacity);
    }
}

#[cfg(feature = "nightly")]
impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn into_boxed_slice(self) -> Box<Slice<K, V>, TypeProjectedAlloc<A>> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        Slice::from_boxed_slice(self.into_entries().into_boxed_slice())
    }
}

impl<K, V, S, A> TypeProjectedIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_entries().get(index).map(Bucket::refs)
    }

    pub(crate) fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_entries_mut().get_mut(index).map(Bucket::ref_mut)
    }

    pub(crate) fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        if index >= self.len() {
            return None;
        }

        Some(IndexedEntry::new(&mut self.inner, index))
    }

    pub(crate) fn get_disjoint_indices_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[(&K, &mut V); N], GetDisjointMutError> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_mut_slice().get_disjoint_mut(indices)
    }

    pub(crate) fn get_range<R>(&self, range: R) -> Option<&Slice<K, V>>
    where
        R: ops::RangeBounds<usize>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let entries = self.as_entries();
        let range = range_ops::try_simplify_range(range, entries.len())?;
        entries.get(range).map(Slice::from_slice)
    }

    pub(crate) fn get_range_mut<R>(&mut self, range: R) -> Option<&mut Slice<K, V>>
    where
        R: ops::RangeBounds<usize>,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let entries = self.as_entries_mut();
        let range = range_ops::try_simplify_range(range, entries.len())?;
        entries.get_mut(range).map(Slice::from_slice_mut)
    }

    #[doc(alias = "first_key_value")]
    pub(crate) fn first(&self) -> Option<(&K, &V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_entries().first().map(Bucket::refs)
    }

    pub(crate) fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_entries_mut().first_mut().map(Bucket::ref_mut)
    }

    pub(crate) fn first_entry(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.get_index_entry(0)
    }

    #[doc(alias = "last_key_value")]
    pub(crate) fn last(&self) -> Option<(&K, &V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_entries().last().map(Bucket::refs)
    }

    pub(crate) fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.as_entries_mut().last_mut().map(Bucket::ref_mut)
    }

    pub(crate) fn last_entry(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.get_index_entry(self.len().checked_sub(1)?)
    }

    pub(crate) fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.swap_remove_index(index)
    }

    pub(crate) fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.shift_remove_index(index)
    }

    #[track_caller]
    pub(crate) fn move_index(&mut self, from: usize, to: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.move_index(from, to)
    }

    #[track_caller]
    pub(crate) fn swap_indices(&mut self, a: usize, b: usize) {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        self.inner.swap_indices(a, b)
    }
}

#[repr(C)]
pub(crate) struct TypeErasedIndexMapInner {
    inner: TypeErasedIndexMapCore,
    build_hasher: TypeErasedBuildHasher,
}

impl TypeErasedIndexMapInner {
    #[inline]
    pub(crate) const fn key_type_id(&self) -> any::TypeId {
        self.inner.key_type_id()
    }

    #[inline]
    pub(crate) const fn value_type_id(&self) -> any::TypeId {
        self.inner.value_type_id()
    }

    #[inline]
    pub(crate) const fn build_hasher_type_id(&self) -> any::TypeId {
        self.build_hasher.build_hasher_type_id()
    }

    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }
}

impl TypeErasedIndexMapInner {
    #[inline(always)]
    pub(crate) fn as_proj<K, V, S, A>(&self) -> &TypeProjectedIndexMapInner<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe { &*(self as *const TypeErasedIndexMapInner as *const TypeProjectedIndexMapInner<K, V, S, A>) }
    }

    #[inline(always)]
    pub(crate) fn as_proj_mut<K, V, S, A>(&mut self) -> &mut TypeProjectedIndexMapInner<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe { &mut *(self as *mut TypeErasedIndexMapInner as *mut TypeProjectedIndexMapInner<K, V, S, A>) }
    }

    #[inline(always)]
    pub(crate) fn into_proj<K, V, S, A>(self) -> TypeProjectedIndexMapInner<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.key_type_id(), any::TypeId::of::<K>());
        debug_assert_eq!(self.value_type_id(), any::TypeId::of::<V>());
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let proj_inner = self.inner.into_proj_assuming_type::<K, V, A>();
        let proj_build_hasher = self.build_hasher.into_proj::<S>();

        TypeProjectedIndexMapInner {
            inner: proj_inner,
            build_hasher: proj_build_hasher,
        }
    }

    #[inline(always)]
    pub(crate) fn from_proj<K, V, S, A>(proj_self: TypeProjectedIndexMapInner<K, V, S, A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let opaque_inner = TypeErasedIndexMapCore::from_proj_assuming_type::<K, V, A>(proj_self.inner);
        let opaque_build_hasher = TypeErasedBuildHasher::from_proj::<S>(proj_self.build_hasher);

        Self {
            inner: opaque_inner,
            build_hasher: opaque_build_hasher,
        }
    }
}

impl TypeErasedIndexMapInner {
    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

mod dummy {
    use super::*;
    use core::marker;
    use core::ptr::NonNull;

    #[allow(dead_code)]
    pub(super) struct DummyHasher {
        _do_not_construct: marker::PhantomData<()>,
    }

    impl hash::Hasher for DummyHasher {
        #[inline]
        fn finish(&self) -> u64 {
            panic!("[`DummyHasher::finish`] should never actually be called. Its purpose is to test struct layouts.");
        }

        #[inline]
        fn write(&mut self, _bytes: &[u8]) {
            panic!("[`DummyHasher::write`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }

    #[allow(dead_code)]
    pub(super) struct DummyBuildHasher {
        _do_not_construct: marker::PhantomData<()>,
    }

    impl hash::BuildHasher for DummyBuildHasher {
        type Hasher = DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            panic!("[`DummyBuildHasher::build_hasher`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }

    #[allow(dead_code)]
    pub(super) struct DummyAlloc {
        _do_not_construct: marker::PhantomData<()>,
    }

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, _layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            panic!("[`DummyAlloc::allocate`] should never actually be called. Its purpose is to test struct layouts.");
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: alloc::Layout) {
            panic!("[`DummyAlloc::deallocate`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }
}

mod layout_testing_types {
    use super::*;
    use core::marker;

    #[allow(dead_code)]
    pub(super) struct TangentSpace {
        tangent: [f32; 3],
        bitangent: [f32; 3],
        normal: [f32; 3],
        _do_not_construct: marker::PhantomData<()>,
    }

    #[allow(dead_code)]
    pub(super) struct SurfaceDifferential {
        dpdu: [f32; 3],
        dpdv: [f32; 3],
        _do_not_construct: marker::PhantomData<()>,
    }

    #[allow(dead_code)]
    pub(super) struct OctTreeNode {
        center: [f32; 3],
        extent: f32,
        children: [Option<Box<OctTreeNode>>; 8],
        occupancy: u8,
        _do_not_construct: marker::PhantomData<()>,
    }
}

#[cfg(test)]
mod index_map_core_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_type_erased_index_map_core_match_sizes<K, V, A>()
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::size_of::<TypeProjectedIndexMapCore<K, V, A>>();
        let result = mem::size_of::<TypeErasedIndexMapCore>();

        assert_eq!(result, expected, "Type Erased and Type Projected data types size mismatch");
    }

    fn run_test_type_erased_index_map_core_match_alignments<K, V, A>()
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::align_of::<TypeProjectedIndexMapCore<K, V, A>>();
        let result = mem::align_of::<TypeErasedIndexMapCore>();

        assert_eq!(
            result, expected,
            "Type Erased and Type Projected data types alignment mismatch"
        );
    }

    fn run_test_type_erased_index_map_core_match_offsets<K, V, A>()
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        assert_eq!(
            mem::offset_of!(TypeProjectedIndexMapCore<K, V, A>, indices),
            mem::offset_of!(TypeErasedIndexMapCore, indices),
            "Type Erased and Type Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypeProjectedIndexMapCore<K, V, A>, entries),
            mem::offset_of!(TypeErasedIndexMapCore, entries),
            "Type Erased and Type Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypeProjectedIndexMapCore<K, V, A>, key_type_id),
            mem::offset_of!(TypeErasedIndexMapCore, key_type_id),
            "Type Erased and Type Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypeProjectedIndexMapCore<K, V, A>, value_type_id),
            mem::offset_of!(TypeErasedIndexMapCore, value_type_id),
            "Type Erased and Type Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypeProjectedIndexMapCore<K, V, A>, allocator_type_id),
            mem::offset_of!(TypeErasedIndexMapCore, allocator_type_id),
            "Type Erased and Type Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $key_typ:ty, $value_typ:ty, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_type_erased_index_map_core_layout_match_sizes() {
                    run_test_type_erased_index_map_core_match_sizes::<$key_typ, $value_typ, $alloc_typ>();
                }

                #[test]
                fn test_type_erased_index_map_core_layout_match_alignments() {
                    run_test_type_erased_index_map_core_match_alignments::<$key_typ, $value_typ, $alloc_typ>();
                }

                #[test]
                fn test_type_erased_index_map_core_layout_match_offsets() {
                    run_test_type_erased_index_map_core_match_offsets::<$key_typ, $value_typ, $alloc_typ>();
                }
            }
        };
    }

    layout_tests!(unit_zst_unit_zst_global, (), (), alloc::Global);
    layout_tests!(unit_zst_u8_global, (), u8, alloc::Global);
    layout_tests!(unit_zst_u64_global, (), u64, alloc::Global);
    layout_tests!(unit_zst_str_global, (), &'static str, alloc::Global);
    layout_tests!(
        unit_zst_tangent_space_global,
        (),
        layout_testing_types::TangentSpace,
        alloc::Global
    );
    layout_tests!(
        unit_zst_surface_differential_global,
        (),
        layout_testing_types::SurfaceDifferential,
        alloc::Global
    );
    layout_tests!(
        unit_zst_oct_tree_node_global,
        (),
        layout_testing_types::OctTreeNode,
        alloc::Global
    );

    layout_tests!(u8_unit_zst_global, u8, (), alloc::Global);
    layout_tests!(u8_u8_global, u8, u8, alloc::Global);
    layout_tests!(u8_u64_global, u8, u64, alloc::Global);
    layout_tests!(u8_str_global, u8, &'static str, alloc::Global);
    layout_tests!(u8_tangent_space_global, u8, layout_testing_types::TangentSpace, alloc::Global);
    layout_tests!(
        u8_surface_differential_global,
        u8,
        layout_testing_types::SurfaceDifferential,
        alloc::Global
    );
    layout_tests!(u8_oct_tree_node_global, u8, layout_testing_types::OctTreeNode, alloc::Global);

    layout_tests!(u64_unit_zst_global, u64, (), alloc::Global);
    layout_tests!(u64_u8_global, u64, u8, alloc::Global);
    layout_tests!(u64_u64_global, u64, u64, alloc::Global);
    layout_tests!(u64_str_global, u64, &'static str, alloc::Global);
    layout_tests!(
        u64_tangent_space_global,
        u64,
        layout_testing_types::TangentSpace,
        alloc::Global
    );
    layout_tests!(
        u64_surface_differential_global,
        u64,
        layout_testing_types::SurfaceDifferential,
        alloc::Global
    );
    layout_tests!(
        u64_oct_tree_node_global,
        u64,
        layout_testing_types::OctTreeNode,
        alloc::Global
    );

    layout_tests!(unit_zst_unit_zst_dummy_alloc, (), (), dummy::DummyAlloc);
    layout_tests!(unit_zst_u8_dummy_alloc, (), u8, dummy::DummyAlloc);
    layout_tests!(unit_zst_u64_dummy_alloc, (), u64, dummy::DummyAlloc);
    layout_tests!(unit_zst_str_dummy_alloc, (), &'static str, dummy::DummyAlloc);
    layout_tests!(
        unit_zst_tangent_space_dummy_alloc,
        (),
        layout_testing_types::TangentSpace,
        dummy::DummyAlloc
    );
    layout_tests!(
        unit_zst_surface_differential_dummy_alloc,
        (),
        layout_testing_types::SurfaceDifferential,
        dummy::DummyAlloc
    );
    layout_tests!(
        unit_zst_oct_tree_node_dummy_alloc,
        (),
        layout_testing_types::OctTreeNode,
        dummy::DummyAlloc
    );

    layout_tests!(u8_unit_zst_dummy_alloc, u8, (), dummy::DummyAlloc);
    layout_tests!(u8_u8_dummy_alloc, u8, u8, dummy::DummyAlloc);
    layout_tests!(u8_u64_dummy_alloc, u8, u64, dummy::DummyAlloc);
    layout_tests!(u8_str_dummy_alloc, u8, &'static str, dummy::DummyAlloc);
    layout_tests!(
        u8_tangent_space_dummy_alloc,
        u8,
        layout_testing_types::TangentSpace,
        dummy::DummyAlloc
    );
    layout_tests!(
        u8_surface_differential_dummy_alloc,
        u8,
        layout_testing_types::SurfaceDifferential,
        dummy::DummyAlloc
    );
    layout_tests!(
        u8_oct_tree_node_dummy_alloc,
        u8,
        layout_testing_types::OctTreeNode,
        dummy::DummyAlloc
    );

    layout_tests!(u64_unit_zst_dummy_alloc, u64, (), dummy::DummyAlloc);
    layout_tests!(u64_u8_dummy_alloc, u64, u8, dummy::DummyAlloc);
    layout_tests!(u64_u64_dummy_alloc, u64, u64, dummy::DummyAlloc);
    layout_tests!(u64_str_dummy_alloc, u64, &'static str, dummy::DummyAlloc);
    layout_tests!(
        u64_tangent_space_dummy_alloc,
        u64,
        layout_testing_types::TangentSpace,
        dummy::DummyAlloc
    );
    layout_tests!(
        u64_surface_differential_dummy_alloc,
        u64,
        layout_testing_types::SurfaceDifferential,
        dummy::DummyAlloc
    );
    layout_tests!(
        u64_oct_tree_node_dummy_alloc,
        u64,
        layout_testing_types::OctTreeNode,
        dummy::DummyAlloc
    );
}

#[cfg(test)]
mod index_map_core_assert_send_sync {
    use super::*;

    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeProjectedIndexMapCore<i32, i32, alloc::Global>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeProjectedIndexMapCore<i32, i32, dummy::DummyAlloc>>();
    }
}

/*
#[cfg(test)]
mod index_map_core_assert_not_send_not_sync {
    use super::*;

    #[test]
    fn test_assert_not_send_not_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeErasedIndexMapCore>();
    }
}
*/

#[cfg(test)]
mod index_map_inner_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_type_erased_index_map_inner_match_sizes<K, V, S, A>()
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::size_of::<TypeProjectedIndexMapInner<K, V, S, A>>();
        let result = mem::size_of::<TypeErasedIndexMapInner>();

        assert_eq!(result, expected, "Type Erased and Type Projected data types size mismatch");
    }

    fn run_test_type_erased_index_map_inner_match_alignments<K, V, S, A>()
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::align_of::<TypeProjectedIndexMapInner<K, V, S, A>>();
        let result = mem::align_of::<TypeErasedIndexMapInner>();

        assert_eq!(
            result, expected,
            "Type Erased and Type Projected data types alignment mismatch"
        );
    }

    fn run_test_type_erased_index_map_inner_match_offsets<K, V, S, A>()
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        assert_eq!(
            mem::offset_of!(TypeProjectedIndexMapInner<K, V, S, A>, inner),
            mem::offset_of!(TypeErasedIndexMapInner, inner),
            "Type Erased and Type Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypeProjectedIndexMapInner<K, V, S, A>, build_hasher),
            mem::offset_of!(TypeErasedIndexMapInner, build_hasher),
            "Type Erased and Type Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $key_typ:ty, $value_typ:ty, $build_hasher_typ:ty, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_type_erased_index_map_inner_layout_match_sizes() {
                    run_test_type_erased_index_map_inner_match_sizes::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>();
                }

                #[test]
                fn test_type_erased_index_map_inner_layout_match_alignments() {
                    run_test_type_erased_index_map_inner_match_alignments::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(
                    );
                }

                #[test]
                fn test_type_erased_index_map_inner_layout_match_offsets() {
                    run_test_type_erased_index_map_inner_match_offsets::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>();
                }
            }
        };
    }

    #[cfg(feature = "std")]
    layout_tests!(unit_zst_unit_zstrandom_state_global, (), (), hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(unit_zst_u8random_state_global, (), u8, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(unit_zst_u64random_state_global, (), u64, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(
        unit_zst_strrandom_state_global,
        (),
        &'static str,
        hash::RandomState,
        alloc::Global
    );

    #[cfg(feature = "std")]
    layout_tests!(
        unit_zst_tangent_spacerandom_state_global,
        (),
        layout_testing_types::TangentSpace,
        hash::RandomState,
        alloc::Global
    );

    #[cfg(feature = "std")]
    layout_tests!(
        unit_zst_surface_differentialrandom_state_global,
        (),
        layout_testing_types::SurfaceDifferential,
        hash::RandomState,
        alloc::Global
    );

    #[cfg(feature = "std")]
    layout_tests!(
        unit_zst_oct_tree_noderandom_state_global,
        (),
        layout_testing_types::OctTreeNode,
        hash::RandomState,
        alloc::Global
    );

    #[cfg(feature = "std")]
    layout_tests!(u8_unit_zstrandom_state_global, u8, (), hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_u8random_state_global, u8, u8, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_u64random_state_global, u8, u64, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_strrandom_state_global, u8, &'static str, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(
        u8_tangent_spacerandom_state_global,
        u8,
        layout_testing_types::TangentSpace,
        hash::RandomState,
        alloc::Global
    );

    #[cfg(feature = "std")]
    layout_tests!(
        u8_surface_differentialrandom_state_global,
        u8,
        layout_testing_types::SurfaceDifferential,
        hash::RandomState,
        alloc::Global
    );

    #[cfg(feature = "std")]
    layout_tests!(
        u8_oct_tree_noderandom_state_global,
        u8,
        layout_testing_types::OctTreeNode,
        hash::RandomState,
        alloc::Global
    );

    #[cfg(feature = "std")]
    layout_tests!(u64_unit_zstrandom_state_global, u64, (), hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u64_u8random_state_global, u64, u8, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u64_u64random_state_global, u64, u64, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(
        u64_strrandom_state_global,
        u64,
        &'static str,
        hash::RandomState,
        alloc::Global
    );

    #[cfg(feature = "std")]
    layout_tests!(
        u64_tangent_spacerandom_state_global,
        u64,
        layout_testing_types::TangentSpace,
        hash::RandomState,
        alloc::Global
    );

    #[cfg(feature = "std")]
    layout_tests!(
        u64_surface_differentialrandom_state_global,
        u64,
        layout_testing_types::SurfaceDifferential,
        hash::RandomState,
        alloc::Global
    );

    #[cfg(feature = "std")]
    layout_tests!(
        u64_oct_tree_noderandom_state_global,
        u64,
        layout_testing_types::OctTreeNode,
        hash::RandomState,
        alloc::Global
    );

    layout_tests!(
        unit_zst_unit_zst_dummy_hasher_dummy_alloc,
        (),
        (),
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        unit_zst_u8_dummy_hasher_dummy_alloc,
        (),
        u8,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        unit_zst_u64_dummy_hasher_dummy_alloc,
        (),
        u64,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        unit_zst_str_dummy_hasher_dummy_alloc,
        (),
        &'static str,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        unit_zst_tangent_space_dummy_hasher_dummy_alloc,
        (),
        layout_testing_types::TangentSpace,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        unit_zst_surface_differential_dummy_hasher_dummy_alloc,
        (),
        layout_testing_types::SurfaceDifferential,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        unit_zst_oct_tree_node_dummy_hasher_dummy_alloc,
        (),
        layout_testing_types::OctTreeNode,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );

    layout_tests!(
        u8_unit_zst_dummy_hasher_dummy_alloc,
        u8,
        (),
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u8_u8_dummy_hasher_dummy_alloc,
        u8,
        u8,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u8_u64_dummy_hasher_dummy_alloc,
        u8,
        u64,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u8_str_dummy_hasher_dummy_alloc,
        u8,
        &'static str,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u8_tangent_space_dummy_hasher_dummy_alloc,
        u8,
        layout_testing_types::TangentSpace,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u8_surface_differential_dummy_hasher_dummy_alloc,
        u8,
        layout_testing_types::SurfaceDifferential,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u8_oct_tree_node_dummy_hasher_dummy_alloc,
        u8,
        layout_testing_types::OctTreeNode,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );

    layout_tests!(
        u64_unit_zst_dummy_hasher_dummy_alloc,
        u64,
        (),
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u64_u8_dummy_hasher_dummy_alloc,
        u64,
        u8,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u64_u64_dummy_hasher_dummy_alloc,
        u64,
        u64,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u64_str_dummy_hasher_dummy_alloc,
        u64,
        &'static str,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u64_tangent_space_dummy_hasher_dummy_alloc,
        u64,
        layout_testing_types::TangentSpace,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u64_surface_differential_dummy_hasher_dummy_alloc,
        u64,
        layout_testing_types::SurfaceDifferential,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
    layout_tests!(
        u64_oct_tree_node_dummy_hasher_dummy_alloc,
        u64,
        layout_testing_types::OctTreeNode,
        dummy::DummyBuildHasher,
        dummy::DummyAlloc
    );
}

#[cfg(test)]
mod index_map_inner_assert_send_sync {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeProjectedIndexMapInner<i32, i32, hash::RandomState, alloc::Global>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeProjectedIndexMapInner<i32, i32, dummy::DummyBuildHasher, alloc::Global>>();
    }
}

/*
#[cfg(test)]
mod index_map_inner_assert_not_send_not_sync {
    use super::*;

    #[test]
    fn test_assert_not_send_not_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeErasedIndexMapInner>();
    }
}
*/
