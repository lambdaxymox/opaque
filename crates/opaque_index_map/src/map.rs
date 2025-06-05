use crate::equivalent::Equivalent;
use crate::map_inner::{Bucket, OpaqueIndexMapInner, TypedProjIndexMapInner};
use crate::map_inner;

use core::any;
use core::cmp;
use core::fmt;
use core::iter;
use core::mem;
use core::ops;
use alloc_crate::alloc;
use alloc_crate::boxed::Box;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

use opaque_alloc::TypedProjAlloc;
use opaque_error::{
    TryReserveError,
};
use opaque_hash::{TypedProjBuildHasher};
use opaque_vec::TypedProjVec;

pub struct Drain<'a, K, V, A = alloc::Global>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: map_inner::Drain<'a, K, V, A>,
}

impl<'a, K, V, A> Drain<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    const fn new(iter: map_inner::Drain<'a, K, V, A>) -> Self {
        Self { iter }
    }

    pub fn as_slice(&self) -> &Slice<K, V> {
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
        self.iter.next()
    }
}

impl<K, V, A> DoubleEndedIterator for Drain<'_, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
    }
}

impl<K, V, A> ExactSizeIterator for Drain<'_, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn len(&self) -> usize {
        self.iter.len()
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
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

pub struct Keys<'a, K, V> {
    iter: map_inner::Keys<'a, K, V>,
}

impl<'a, K, V> Keys<'a, K, V> {
    fn new(iter: map_inner::Keys<'a, K, V>) -> Self {
        Self { iter, }
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
        self.iter.next()
    }
}

impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
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
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<'a, K, V> ops::Index<usize> for Keys<'a, K, V> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        self.iter.index(index)
    }
}

pub struct IntoKeys<K, V, A = alloc::Global>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: map_inner::IntoKeys<K, V, A>,
}

impl<K, V, A> IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn new(iter: map_inner::IntoKeys<K, V, A>) -> Self {
        Self { iter, }
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
        self.iter.next()
    }
}

impl<K, V, A> DoubleEndedIterator for IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
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
        fmt::Debug::fmt(&self.iter, formatter)
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
            iter: Default::default(),
        }
    }
}

pub struct Values<'a, K, V> {
    iter: map_inner::Values<'a, K, V>,
}

impl<'a, K, V> Values<'a, K, V> {
    #[inline]
    const fn new(iter: map_inner::Values<'a, K, V>) -> Self {
        Self { iter, }
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
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
        Self { iter: Default::default() }
    }
}

pub struct ValuesMut<'a, K, V> {
    iter: map_inner::ValuesMut<'a, K, V>,
}

impl<'a, K, V> ValuesMut<'a, K, V> {
    #[inline]
    const fn new(iter: map_inner::ValuesMut<'a, K, V>) -> Self {
        Self { iter, }
    }
}

impl<'a, K, V> Iterator for ValuesMut<'a, K, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<K, V> DoubleEndedIterator for ValuesMut<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
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
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<K, V> Default for ValuesMut<'_, K, V> {
    fn default() -> Self {
        Self { iter: Default::default() }
    }
}

pub struct IntoValues<K, V, A = alloc::Global>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: map_inner::IntoValues<K, V, A>,
}

impl<K, V, A> IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    const fn new(iter: map_inner::IntoValues<K, V, A>) -> Self {
        Self { iter, }
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
        self.iter.next()
    }
}

impl<K, V, A> DoubleEndedIterator for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
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
        fmt::Debug::fmt(&self.iter, formatter)
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
            iter: Default::default(),
        }
    }
}

#[repr(transparent)]
pub struct Slice<K, V> {
    entries: map_inner::Slice<K, V>,
}

impl<K, V> Slice<K, V> {
    #[inline]
    pub(crate) const fn from_slice(entries: &map_inner::Slice<K, V>) -> &Self {
        unsafe { &*(entries as *const map_inner::Slice<K, V> as *const Self) }
    }

    #[inline]
    pub(crate) const fn from_slice_mut(entries: &mut map_inner::Slice<K, V>) -> &mut Self {
        unsafe { &mut *(entries as *mut map_inner::Slice<K, V> as *mut Self) }
    }

    fn from_boxed_slice<A>(entries: Box<map_inner::Slice<K, V>, TypedProjAlloc<A>>) -> Box<Self, TypedProjAlloc<A>>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        unsafe {
            let (ptr, alloc) = Box::into_raw_with_allocator(entries);

            Box::from_raw_in(ptr as *mut Self, alloc)
        }
    }

    fn into_boxed_slice<A>(self: Box<Self, A>) -> Box<map_inner::Slice<K, V>, A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        unsafe {
            let (ptr, alloc) = Box::into_raw_with_allocator(self);

            Box::from_raw_in(ptr as *mut map_inner::Slice<K, V>, alloc)
        }
    }

    fn from_entries_in<A>(vec: TypedProjVec<Bucket<K, V>, A>) -> Box<Self, TypedProjAlloc<A>>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let boxed_slice_inner = map_inner::Slice::from_entries_in(vec);
        let boxed_slice = unsafe {
            let (_ptr, alloc) = Box::into_raw_with_allocator(boxed_slice_inner);
            let ptr = _ptr as *mut Self;
            Box::from_raw_in(ptr, alloc)
        };

        boxed_slice
    }

    pub const fn new<'a>() -> &'a Self {
        Self::from_slice(map_inner::Slice::new())
    }

    pub fn new_mut<'a>() -> &'a mut Self {
        Self::from_slice_mut(map_inner::Slice::new_mut())
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.entries.get_index(index)
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.entries.get_index_mut(index)
    }

    pub fn get_range<R>(&self, range: R) -> Option<&Self>
    where
        R: ops::RangeBounds<usize>,
    {
        self.entries.get_range(range).map(Slice::from_slice)
    }

    pub fn get_range_mut<R>(&mut self, range: R) -> Option<&mut Self>
    where
        R: ops::RangeBounds<usize>,
    {
        self.entries.get_range_mut(range).map(Slice::from_slice_mut)
    }

    pub fn first(&self) -> Option<(&K, &V)> {
        self.entries.first()
    }

    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.entries.first_mut()
    }

    pub fn last(&self) -> Option<(&K, &V)> {
        self.entries.last()
    }

    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        self.entries.last_mut()
    }

    pub fn split_at(&self, index: usize) -> (&Self, &Self) {
        let (first, second) = self.entries.split_at(index);

        (Self::from_slice(first), Self::from_slice(second))
    }

    pub fn split_at_mut(&mut self, index: usize) -> (&mut Self, &mut Self) {
        let (first, second) = self.entries.split_at_mut(index);

        (Self::from_slice_mut(first), Self::from_slice_mut(second))
    }

    pub fn split_first(&self) -> Option<((&K, &V), &Self)> {
        let (split, slice) = self.entries.split_first()?;

        Some((split, Self::from_slice(slice)))
    }

    pub fn split_first_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        let (split, slice) = self.entries.split_first_mut()?;

        Some((split, Self::from_slice_mut(slice)))
    }

    pub fn split_last(&self) -> Option<((&K, &V), &Self)> {
        let (split, slice) = self.entries.split_last()?;

        Some((split, Self::from_slice(slice)))

    }

    pub fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        let (split, slice) = self.entries.split_last_mut()?;

        Some((split, Slice::from_slice_mut(slice)))
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(self.entries.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(self.entries.iter_mut())
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys::new(self.entries.keys())
    }

    pub fn into_keys<A>(self: Box<Self, TypedProjAlloc<A>>) -> IntoKeys<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        IntoKeys::new(self.into_boxed_slice().into_keys())
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values::new(self.entries.values())
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut::new(self.entries.values_mut())
    }

    pub fn into_values<A>(self: Box<Self, TypedProjAlloc<A>>) -> IntoValues<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        IntoValues::new(self.into_boxed_slice().into_values())
    }

    pub fn binary_search_keys(&self, x: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        self.entries.binary_search_keys(x)
    }

    #[inline]
    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> cmp::Ordering,
    {
        self.entries.binary_search_by(f)
    }

    #[inline]
    pub fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> B,
        B: Ord,
    {
        self.entries.binary_search_by_key(b, f)
    }

    #[must_use]
    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {
        self.entries.partition_point(pred)
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

impl<K, V, A> IntoIterator for Box<Slice<K, V>, TypedProjAlloc<A>>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V, A>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.into_boxed_slice().into_iter())
    }
}

impl<K, V> Default for &'_ Slice<K, V> {
    fn default() -> Self {
        Slice::from_slice(Default::default())
    }
}


impl<K, V> Default for &'_ mut Slice<K, V> {
    fn default() -> Self {
        Slice::from_slice_mut(Default::default())
    }
}

impl<K, V, A> Default for Box<Slice<K, V>, TypedProjAlloc<A>>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Slice::from_boxed_slice(Box::default())
    }
}

impl<K, V, A> Clone for Box<Slice<K, V>, TypedProjAlloc<A>>
where
    K: any::Any + Clone,
    V: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        let alloc = Box::allocator(&self).clone();
        let entries = self.entries.to_entries_in(alloc);

        Slice::from_entries_in(entries)
    }
}

impl<K, V> From<&Slice<K, V>> for Box<Slice<K, V>, TypedProjAlloc<alloc::Global>>
where
    K: any::Any + Copy,
    V: any::Any + Copy,
{
    fn from(slice: &Slice<K, V>) -> Self {
        let boxed_entries: Box<map_inner::Slice<K, V>, TypedProjAlloc<alloc::Global>> = Box::from(&slice.entries);

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
        self.entries.eq(&other.entries)
    }
}

impl<K, V, K2, V2> PartialEq<[(K2, V2)]> for Slice<K, V>
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &[(K2, V2)]) -> bool {
        self.entries.eq(other)
    }
}

impl<K, V, K2, V2> PartialEq<Slice<K2, V2>> for [(K, V)]
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &Slice<K2, V2>) -> bool {
        self.eq(&other.entries)
    }
}

impl<K, V, K2, V2, const N: usize> PartialEq<[(K2, V2); N]> for Slice<K, V>
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &[(K2, V2); N]) -> bool {
        self.entries.eq(other)
    }
}

impl<K, V, const N: usize, K2, V2> PartialEq<Slice<K2, V2>> for [(K, V); N]
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &Slice<K2, V2>) -> bool {
        self.eq(&other.entries)
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
        Ord::cmp(&self.entries, &other.entries)
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
        self.entries.hash(state);
    }
}

impl<K, V> ops::Index<usize> for Slice<K, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.entries.index(index).value_ref()
    }
}

impl<K, V> ops::IndexMut<usize> for Slice<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.entries.index_mut(index).value_mut()
    }
}

macro_rules! impl_index_for_index_map_slice {
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

impl_index_for_index_map_slice!(
    ops::Range<usize>,
    ops::RangeFrom<usize>,
    ops::RangeFull,
    ops::RangeInclusive<usize>,
    ops::RangeTo<usize>,
    ops::RangeToInclusive<usize>,
    (ops::Bound<usize>, ops::Bound<usize>)
);


pub struct Iter<'a, K, V> {
    iter: map_inner::Iter<'a, K, V>,
}

impl<'a, K, V> Iter<'a, K, V> {
    #[inline]
    fn new(iter: map_inner::Iter<'a, K, V>) -> Self {
        Self { iter, }
    }

    pub fn as_slice(&self) -> &Slice<K, V> {
        Slice::from_slice(self.iter.as_slice())
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<K, V> DoubleEndedIterator for Iter<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
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
        Self { iter: Default::default() }
    }
}

pub struct IterMut<'a, K, V> {
    iter: map_inner::IterMut<'a, K, V>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    #[inline]
    const fn new(iter: map_inner::IterMut<'a, K, V>) -> Self {
        Self { iter, }
    }

    pub fn as_slice_mut(&'a mut self) -> &'a mut Slice<K, V> {
        Slice::from_slice_mut(self.iter.as_slice_mut())
    }

    pub fn into_slice_mut(self) -> &'a mut Slice<K, V> {
        Slice::from_slice_mut(self.iter.into_slice_mut())
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<K, V> DoubleEndedIterator for IterMut<'_, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
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
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<K, V> Default for IterMut<'_, K, V> {
    fn default() -> Self {
        Self { iter: Default::default() }
    }
}

#[derive(Clone)]
pub struct IntoIter<K, V, A = alloc::Global>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: map_inner::IntoIter<K, V, A>,
}

impl<K, V, A> IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    const fn new(iter: map_inner::IntoIter<K, V, A>) -> Self {
        Self { iter, }
    }

    pub fn as_slice(&self) -> &Slice<K, V> {
        Slice::from_slice(self.iter.as_slice())
    }

    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
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
        self.iter.next()
    }
}

impl<K, V, A> DoubleEndedIterator for IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n)
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
        fmt::Debug::fmt(&self.iter, formatter)
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
            iter: Default::default(),
        }
    }
}

#[cfg(feature = "std")]
pub struct Splice<'a, I, K, V, S = hash::RandomState, A = alloc::Global>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: map_inner::Splice<'a, I, K, V, S, A>,
}

#[cfg(not(feature = "std"))]
pub struct Splice<'a, I, K, V, S, A = alloc::Global>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: map_inner::Splice<'a, I, K, V, S, A>,
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
    #[inline]
    const fn new<R>(inner: map_inner::Splice<'a, I, K, V, S, A>) -> Self
    where
        R: ops::RangeBounds<usize>,
    {
        Self { inner, }
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
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
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
        self.inner.next_back()
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
        self.inner.len()
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
        fmt::Debug::fmt(&self.inner, formatter)
    }
}

pub enum Entry<'a, K, V, A = alloc::Global>
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
    pub fn index(&self) -> usize {
        match *self {
            Entry::Occupied(ref entry) => entry.index(),
            Entry::Vacant(ref entry) => entry.index(),
        }
    }

    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, A> {
        match self {
            Entry::Occupied(mut entry) => {
                entry.insert(value);
                entry
            }
            Entry::Vacant(entry) => entry.insert_entry(value),
        }
    }

    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    pub fn or_insert_with<F>(self, call: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(call()),
        }
    }

    pub fn or_insert_with_key<F>(self, call: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = call(&entry.key());
                entry.insert(value)
            }
        }
    }

    pub fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    pub fn and_modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        if let Entry::Occupied(entry) = &mut self {
            f(entry.get_mut());
        }
        self
    }

    pub fn or_default(self) -> &'a mut V
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

pub struct OccupiedEntry<'a, K, V, A = alloc::Global>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: map_inner::OccupiedEntry<'a, K, V, A>,
}

impl<'a, K, V, A> OccupiedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{

    #[inline]
    pub(crate) const fn new(inner: map_inner::OccupiedEntry<'a, K, V, A>) -> Self {
        Self { inner, }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.inner.index()
    }

    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /*
    pub(crate) fn key_mut(&mut self) -> &mut K {
        self.inner.key_mut()
    }
    */

    pub fn get(&self) -> &V {
        self.inner.get()
    }

    pub fn get_mut(&mut self) -> &mut V {
        self.inner.get_mut()
    }

    pub fn into_mut(self) -> &'a mut V {
        self.inner.into_mut()
    }

    pub fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }

    pub fn swap_remove(self) -> V {
        self.swap_remove_entry().1
    }

    pub fn shift_remove(self) -> V {
        self.shift_remove_entry().1
    }

    pub fn swap_remove_entry(self) -> (K, V) {
        self.inner.swap_remove_entry()
    }

    pub fn shift_remove_entry(self) -> (K, V) {
        self.inner.shift_remove_entry()
    }

    #[track_caller]
    pub fn move_index(self, to: usize) {
        self.inner.move_index(to);
    }

    pub fn swap_indices(self, other: usize) {
        self.inner.swap_indices(other);
    }
}

impl<K, V, A> fmt::Debug for OccupiedEntry<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("OccupiedEntry")
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
        Self::new(map_inner::OccupiedEntry::from(other.inner))
    }
}

pub struct VacantEntry<'a, K, V, A = alloc::Global>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: map_inner::VacantEntry<'a, K, V, A>,
}

impl<'a, K, V, A> VacantEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    const fn new(inner: map_inner::VacantEntry<'a, K, V, A>) -> Self {
        Self { inner, }
    }

    pub fn index(&self) -> usize {
        self.inner.index()
    }

    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /*
    pub(crate) fn key_mut(&mut self) -> &mut K {
        self.inner.key_mut()
    }
    */

    pub fn into_key(self) -> K {
        self.inner.into_key()
    }

    pub fn insert(self, value: V) -> &'a mut V {
        self.inner.insert(value)
    }

    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, A> {
        OccupiedEntry::new(self.inner.insert_entry(value))
    }

    pub fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {
        self.inner.insert_sorted(value)
    }

    pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
        self.inner.shift_insert(index, value)
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

pub struct IndexedEntry<'a, K, V, A = alloc::Global>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: map_inner::IndexedEntry<'a, K, V, A>,
}

impl<'a, K, V, A> IndexedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) fn new(inner: map_inner::IndexedEntry<'a, K, V, A>) -> Self
    where
        K: Ord,
    {
        Self {
            inner,
        }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.inner.index()
    }

    pub fn key(&self) -> &K {
        self.inner.key()
    }

    /*
    pub(crate) fn key_mut(&mut self) -> &mut K {
        self.inner.key_mut()
    }
    */

    pub fn get(&self) -> &V {
        self.inner.get()
    }

    pub fn get_mut(&mut self) -> &mut V {
        self.inner.get_mut()
    }

    pub fn insert(&mut self, value: V) -> V {
        self.inner.insert(value)
    }

    pub fn into_mut(self) -> &'a mut V {
        self.inner.into_mut()
    }

    pub fn swap_remove_entry(self) -> (K, V) {
        self.inner.swap_remove_entry()
    }

    pub fn shift_remove_entry(self) -> (K, V) {
        self.inner.shift_remove_entry()
    }

    pub fn swap_remove(self) -> V {
        self.inner.swap_remove()
    }

    pub fn shift_remove(self) -> V {
        self.inner.shift_remove()
    }

    #[track_caller]
    pub fn move_index(self, to: usize) {
        self.inner.move_index(to);
    }

    pub fn swap_indices(self, other: usize) {
        self.inner.swap_indices(other);
    }
}

impl<K, V, A> fmt::Debug for IndexedEntry<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("IndexedEntry")
            .field("index", &self.index())
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
            inner: map_inner::IndexedEntry::from(other.inner),
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

#[cfg(feature = "std")]
#[repr(transparent)]
pub struct TypedProjIndexMap<K, V, S = hash::RandomState, A = alloc::Global>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: TypedProjIndexMapInner<K, V, S, A>,
}

#[cfg(not(feature = "std"))]
#[repr(transparent)]
pub struct TypedProjIndexMap<K, V, S, A = alloc::Global>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: TypedProjIndexMapInner<K, V, S, A>,
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub fn with_hasher_proj_in(proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, S, A>::with_hasher_proj_in(proj_build_hasher, proj_alloc);

        Self {
            inner: proj_inner,
        }
    }

    #[inline]
    pub fn with_capacity_and_hasher_proj_in(capacity: usize, proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self {
        if capacity == 0 {
            Self::with_hasher_proj_in(proj_build_hasher, proj_alloc)
        } else {
            let proj_inner = TypedProjIndexMapInner::<K, V, S, A>::with_capacity_and_hasher_proj_in(capacity, proj_build_hasher, proj_alloc);

            Self {
                inner: proj_inner,
            }
        }
    }
}

#[cfg(feature = "std")]
impl<K, V, A> TypedProjIndexMap<K, V, hash::RandomState, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub fn new_proj_in(proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, hash::RandomState, A>::new_proj_in(proj_alloc);

        Self {
            inner : proj_inner,
        }
    }

    pub fn with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, hash::RandomState, A>::with_capacity_proj_in(capacity, proj_alloc);

        Self {
            inner: proj_inner,
        }
    }
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub fn with_hasher_in(build_hasher: S, alloc: A) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, S, A>::with_hasher_in(build_hasher, alloc);

        Self {
            inner: proj_inner,
        }
    }

    #[inline]
    pub fn with_capacity_and_hasher_in(capacity: usize, build_hasher: S, alloc: A) -> Self {
        if capacity == 0 {
            Self::with_hasher_in(build_hasher, alloc)
        } else {
            let proj_inner = TypedProjIndexMapInner::<K, V, S, A>::with_capacity_and_hasher_in(capacity, build_hasher, alloc);

            Self {
                inner: proj_inner,
            }
        }
    }
}

#[cfg(feature = "std")]
impl<K, V, A> TypedProjIndexMap<K, V, hash::RandomState, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub fn new_in(alloc: A) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, hash::RandomState, A>::new_in(alloc);

        Self {
            inner : proj_inner,
        }
    }

    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, hash::RandomState, A>::with_capacity_in(capacity, alloc);

        Self {
            inner: proj_inner,
        }
    }
}

impl<K, V, S> TypedProjIndexMap<K, V, S, alloc::Global>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    #[inline]
    pub fn with_hasher(build_hasher: S) -> Self {
        Self::with_hasher_in(build_hasher, alloc::Global)
    }

    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, build_hasher: S) -> Self {
        Self::with_capacity_and_hasher_in(capacity, build_hasher, alloc::Global)
    }
}

#[cfg(feature = "std")]
impl<K, V> TypedProjIndexMap<K, V, hash::RandomState, alloc::Global>
where
    K: any::Any,
    V: any::Any,
{
    #[inline]
    pub fn new() -> Self {
        let proj_inner = TypedProjIndexMapInner::new();

        Self {
            inner : proj_inner,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let proj_inner = TypedProjIndexMapInner::with_capacity(capacity);

        Self {
            inner : proj_inner,
        }
    }
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub const fn hasher(&self) -> &TypedProjBuildHasher<S> {
        self.inner.hasher()
    }

    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
        self.inner.allocator()
    }
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.get_index_of(key)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.contains_key(key)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.get(key)
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.get_key_value(key)
    }

    pub fn get_full<Q>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.get_full(key)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.get_mut(key)
    }

    pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.get_full_mut(key)
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys::new(self.inner.keys())
    }

    pub fn into_keys(self) -> IntoKeys<K, V, A> {
        IntoKeys::new(self.inner.into_keys())
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(self.inner.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(self.inner.iter_mut())
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values::new(self.inner.values())
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut::new(self.inner.values_mut())
    }

    pub fn into_values(self) -> IntoValues<K, V, A> {
        IntoValues::new(self.inner.into_values())
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    #[track_caller]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, K, V, A>
    where
        R: ops::RangeBounds<usize>,
    {
        Drain::new(self.inner.drain(range))
    }

    #[track_caller]
    pub fn split_off(&mut self, at: usize) -> Self
    where
        S: Clone,
        A: Clone,
    {
        Self {
            inner: self.inner.split_off(at),
        }
    }

    pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.swap_remove(key)
    }

    pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.swap_remove_entry(key)
    }

    pub fn swap_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.swap_remove_full(key)
    }

    pub fn shift_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.shift_remove(key)
    }

    pub fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.shift_remove_entry(key)
    }

    pub fn shift_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.inner.shift_remove_full(key)
    }

    pub fn as_slice(&self) -> &'_ Slice<K, V> {
        Slice::from_slice(self.inner.as_slice())
    }

    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
        Slice::from_slice_mut(self.inner.as_mut_slice())
    }
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Eq + hash::Hash,
    {
        self.inner.insert(key, value)
    }

    pub fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash,
    {
        self.inner.insert_full(key, value)
    }

    pub fn insert_sorted(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash + Ord,
    {
        self.inner.insert_sorted(key, value)
    }

    #[track_caller]
    pub fn insert_before(&mut self, index: usize, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash,
    {
        self.inner.insert_before(index, key, value)
    }

    #[track_caller]
    pub fn shift_insert(&mut self, index: usize, key: K, value: V) -> Option<V>
    where
        K: Eq + hash::Hash,
    {
        self.inner.shift_insert(index, key, value)
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, A>
    where
        K: Eq + hash::Hash,
    {
        match self.inner.entry(key) {
            map_inner::Entry::Occupied(occupied) => Entry::Occupied(OccupiedEntry::new(occupied)),
            map_inner::Entry::Vacant(vacant) => Entry::Vacant(VacantEntry::new(vacant)),
        }
    }

    #[track_caller]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, K, V, S, A>
    where
        K: Eq + hash::Hash,
        A: any::Any + alloc::Allocator + Clone,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = (K, V)>,
    {
        Splice::new::<R>(self.inner.splice(range, replace_with))
    }

    pub fn append<S2, A2>(&mut self, other: &mut TypedProjIndexMap<K, V, S2, A2>)
    where
        K: Eq + hash::Hash,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
        A2: any::Any + alloc::Allocator + Send + Sync,
    {
        self.inner.append(&mut other.inner);
    }
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[doc(alias = "pop_last")] // like `BTreeMap`
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.inner.pop()
    }

    pub fn retain<F>(&mut self, keep: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.inner.retain(keep);
    }

    pub fn sort_keys(&mut self)
    where
        K: Ord,
    {
        self.inner.sort_keys();
    }

    pub fn sort_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        self.inner.sort_by(cmp);
    }

    pub fn sorted_by<F>(self, cmp: F) -> IntoIter<K, V, A>
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        IntoIter::new(self.inner.sorted_by(cmp))
    }

    pub fn sort_unstable_keys(&mut self)
    where
        K: Ord,
    {
        self.inner.sort_unstable_keys();
    }

    pub fn sort_unstable_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        self.inner.sort_unstable_by(cmp);
    }

    #[inline]
    pub fn sorted_unstable_by<F>(self, cmp: F) -> IntoIter<K, V, A>
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        IntoIter::new(self.inner.sorted_unstable_by(cmp))
    }

    pub fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
    where
        T: Ord,
        F: FnMut(&K, &V) -> T,
    {
        self.inner.sort_by_cached_key(&mut sort_key);
    }

    pub fn binary_search_keys(&self, key: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        self.inner.binary_search_keys(key)
    }

    #[inline]
    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> cmp::Ordering,
    {
        self.inner.binary_search_by(f)
    }

    #[inline]
    pub fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> B,
        B: Ord,
    {
        self.inner.binary_search_by_key(b, f)
    }

    #[must_use]
    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {
        self.inner.partition_point(pred)
    }

    pub fn reverse(&mut self) {
        self.inner.reverse();
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }

    pub fn into_boxed_slice(self) -> Box<Slice<K, V>, TypedProjAlloc<A>> {
        Slice::from_boxed_slice(self.inner.into_boxed_slice())
    }

    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.inner.get_index(index)
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.inner.get_index_mut(index)
    }

    pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        self.inner.get_index_entry(index).map(IndexedEntry::new)
    }

    pub fn get_range<R>(&self, range: R) -> Option<&Slice<K, V>>
    where
        R: ops::RangeBounds<usize>,
    {
        self.inner.get_range(range).map(Slice::from_slice)
    }

    pub fn get_range_mut<R>(&mut self, range: R) -> Option<&mut Slice<K, V>>
    where
        R: ops::RangeBounds<usize>,
    {
        self.inner.get_range_mut(range).map(Slice::from_slice_mut)
    }

    #[doc(alias = "first_key_value")] // like `BTreeMap`
    pub fn first(&self) -> Option<(&K, &V)> {
        self.inner.first()
    }

    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.inner.first_mut()
    }

    pub fn first_entry(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        self.inner.first_entry().map(IndexedEntry::new)
    }

    #[doc(alias = "last_key_value")] // like `BTreeMap`
    pub fn last(&self) -> Option<(&K, &V)> {
        self.inner.last()
    }

    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        self.inner.last_mut()
    }

    pub fn last_entry(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        self.inner.last_entry().map(IndexedEntry::new)
    }

    pub fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.inner.swap_remove_index(index)
    }

    pub fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.inner.shift_remove_index(index)
    }

    #[track_caller]
    pub fn move_index(&mut self, from: usize, to: usize) {
        self.inner.move_index(from, to);
    }

    #[track_caller]
    pub fn swap_indices(&mut self, a: usize, b: usize) {
        self.inner.swap_indices(a, b)
    }
}

impl<Q, K, V, S, A> ops::Index<&Q> for TypedProjIndexMap<K, V, S, A>
where
    Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).expect("Entry not found for key")
    }
}


impl<Q, K, V, S, A> ops::IndexMut<&Q> for TypedProjIndexMap<K, V, S, A>
where
    Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn index_mut(&mut self, key: &Q) -> &mut Self::Output {
        self.get_mut(key).expect("Entry not found for key")
    }
}

impl<K, V, S, A> ops::Index<usize> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_index(index)
            .unwrap_or_else(|| {
                panic!(
                    "index out of bounds: the len is `{len}` but the index is `{index}`",
                    len = self.len()
                );
            })
            .1
    }
}

impl<K, V, S, A> ops::IndexMut<usize> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len: usize = self.len();

        self.get_index_mut(index)
            .unwrap_or_else(|| {
                panic!("index out of bounds: the len is `{len}` but the index is `{index}`");
            })
            .1
    }
}

macro_rules! impl_index_for_index_map {
    ($($range:ty),*) => {$(
        impl<K, V, S, A> ops::Index<$range> for TypedProjIndexMap<K, V, S, A>
        where
            K: any::Any,
            V: any::Any,
            S: any::Any + hash::BuildHasher + Send + Sync,
            S::Hasher: any::Any + hash::Hasher + Send + Sync,
            A: any::Any + alloc::Allocator + Send + Sync,
        {
            type Output = Slice<K, V>;

            fn index(&self, range: $range) -> &Self::Output {
                Slice::from_slice(self.inner.as_slice().index(range))
            }
        }

        impl<K, V, S, A> ops::IndexMut<$range> for TypedProjIndexMap<K, V, S, A>
        where
            K: any::Any,
            V: any::Any,
            S: any::Any + hash::BuildHasher + Send + Sync,
            S::Hasher: any::Any + hash::Hasher + Send + Sync,
            A: any::Any + alloc::Allocator + Send + Sync,
        {
            fn index_mut(&mut self, range: $range) -> &mut Self::Output {
                Slice::from_slice_mut(self.inner.as_mut_slice().index_mut(range))
            }
        }
    )*}
}

impl_index_for_index_map!(
    ops::Range<usize>,
    ops::RangeFrom<usize>,
    ops::RangeFull,
    ops::RangeInclusive<usize>,
    ops::RangeTo<usize>,
    ops::RangeToInclusive<usize>,
    (ops::Bound<usize>, ops::Bound<usize>)
);

impl<K, V, S, A> fmt::Debug for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V, S, A> Clone for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any + Clone,
    V: any::Any + Clone,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        let cloned_inner = Clone::clone(&self.inner);

        Self {
            inner: cloned_inner,
        }
    }

    fn clone_from(&mut self, other: &Self) {
        Clone::clone_from(&mut self.inner, &other.inner);
    }
}

impl<K, V, S, A> Extend<(K, V)> for TypedProjIndexMap<K, V, S, A>
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
        self.inner.extend(iterable);
    }
}

impl<'a, K, V, S, A> Extend<(&'a K, &'a V)> for TypedProjIndexMap<K, V, S, A>
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
        self.inner.extend(iterable);
    }
}

impl<K, V, S> FromIterator<(K, V)> for TypedProjIndexMap<K, V, S, alloc::Global>
where
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iterable: I) -> Self {
        let iter = iterable.into_iter();
        let (low, _) = iter.size_hint();
        let mut map = Self::with_capacity_and_hasher_in(low, S::default(), alloc::Global::default());
        map.extend(iter);

        map
    }
}

impl<K, V, S, const N: usize> From<[(K, V); N]> for TypedProjIndexMap<K, V, S, alloc::Global>
where
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn from(array: [(K, V); N]) -> Self {
        Self::from_iter(array)
    }
}

impl<K, V, S, A> Default for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Self { inner: TypedProjIndexMapInner::default(), }
    }
}

impl<K, V1, S1, V2, S2, A1, A2> PartialEq<TypedProjIndexMap<K, V2, S2, A2>> for TypedProjIndexMap<K, V1, S1, A1>
where
    K: any::Any + hash::Hash + Eq,
    V1: any::Any + PartialEq<V2>,
    V2: any::Any,
    S1: any::Any + hash::BuildHasher + Send + Sync,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A1: any::Any + alloc::Allocator + Send + Sync,
    A2: any::Any + alloc::Allocator + Send + Sync,
{
    fn eq(&self, other: &TypedProjIndexMap<K, V2, S2, A2>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .all(|(key, value)| other.get(key).map_or(false, |v| *value == *v))
    }
}

impl<K, V, S, A> Eq for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any + Eq + hash::Hash,
    V: any::Any + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<'a, K, V, S, A> IntoIterator for &'a TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, S, A> IntoIterator for &'a mut TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V, S, A> IntoIterator for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V, A>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(map_inner::IntoIter::new(self.inner.into_entries()))
    }
}

#[repr(transparent)]
pub struct OpaqueIndexMap {
    inner: OpaqueIndexMapInner,
}

impl OpaqueIndexMap {
    #[inline]
    pub const fn key_type_id(&self) -> any::TypeId {
        self.inner.key_type_id()
    }

    #[inline]
    pub const fn value_type_id(&self) -> any::TypeId {
        self.inner.value_type_id()
    }

    #[inline]
    pub const fn build_hasher_type_id<S>(&self) -> any::TypeId {
        self.inner.build_hasher_type_id()
    }

    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }

    #[inline]
    pub fn has_key_type<K>(&self) -> bool
    where
        K: any::Any,
    {
        self.inner.key_type_id() == any::TypeId::of::<K>()
    }

    #[inline]
    pub fn has_value_type<V>(&self) -> bool
    where
        V: any::Any,
    {
        self.inner.value_type_id() == any::TypeId::of::<V>()
    }

    #[inline]
    pub fn has_build_hasher_type<S>(&self) -> bool
    where
        S: any::Any,
    {
        self.inner.build_hasher_type_id() == any::TypeId::of::<S>()
    }

    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.inner.allocator_type_id() == any::TypeId::of::<A>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<K, V, S, A>(&self)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(st: &str, type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("{:?} type mismatch. Need `{:?}`, got `{:?}`", st, type_id_self, type_id_other);
        }

        if !self.has_key_type::<K>() {
            type_check_failed("Key", self.inner.key_type_id(), any::TypeId::of::<K>());
        }

        if !self.has_value_type::<V>() {
            type_check_failed("Value", self.inner.value_type_id(), any::TypeId::of::<V>());
        }

        if !self.has_build_hasher_type::<S>() {
            type_check_failed("BuildHasher", self.inner.build_hasher_type_id(), any::TypeId::of::<S>());
        }

        if !self.has_allocator_type::<A>() {
            type_check_failed("Allocator", self.inner.allocator_type_id(), any::TypeId::of::<A>());
        }
    }
}

impl OpaqueIndexMap {
    #[inline]
    pub fn as_proj<K, V, S, A>(&self) -> &TypedProjIndexMap<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<K, V, S, A>();

        unsafe { &*(self as *const OpaqueIndexMap as *const TypedProjIndexMap<K, V, S, A>) }
    }

    #[inline]
    pub fn as_proj_mut<K, V, S, A>(&mut self) -> &mut TypedProjIndexMap<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<K, V, S, A>();

        unsafe { &mut *(self as *mut OpaqueIndexMap as *mut TypedProjIndexMap<K, V, S, A>) }
    }

    #[inline]
    pub fn into_proj<K, V, S, A>(self) -> TypedProjIndexMap<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<K, V, S, A>();

        TypedProjIndexMap {
            inner: self.inner.into_proj::<K, V, S, A>(),
        }
    }

    #[inline]
    pub fn from_proj<K, V, S, A>(proj_self: TypedProjIndexMap<K, V, S, A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        Self {
            inner: OpaqueIndexMapInner::from_proj(proj_self.inner),
        }
    }
}

impl OpaqueIndexMap {
    #[inline]
    pub fn with_hasher_proj_in<K, V, S, A>(proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, S, A>::with_hasher_proj_in(proj_build_hasher, proj_alloc);

        Self::from_proj(proj_index_map)
    }

    #[inline]
    pub fn with_capacity_and_hasher_proj_in<K, V, S, A>(capacity: usize, proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, S, A>::with_capacity_and_hasher_proj_in(capacity, proj_build_hasher, proj_alloc);

        Self::from_proj(proj_index_map)
    }
}

#[cfg(feature = "std")]
impl OpaqueIndexMap {
    pub fn new_proj_in<K, V, A>(proj_alloc: TypedProjAlloc<A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, hash::RandomState, A>::new_proj_in(proj_alloc);

        Self::from_proj(proj_index_map)
    }

    pub fn with_capacity_proj_in<K, V, A>(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, hash::RandomState, A>::with_capacity_proj_in(capacity, proj_alloc);

        Self::from_proj(proj_index_map)
    }
}

impl OpaqueIndexMap {
    #[inline]
    pub fn with_hasher_in<K, V, S, A>(build_hasher: S, alloc: A) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, S, A>::with_hasher_in(build_hasher, alloc);

        Self::from_proj(proj_index_map)
    }

    #[inline]
    pub fn with_capacity_and_hasher_in<K, V, S, A>(capacity: usize, build_hasher: S, alloc: A) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, S, A>::with_capacity_and_hasher_in(capacity, build_hasher, alloc);

        Self::from_proj(proj_index_map)
    }
}

impl OpaqueIndexMap {
    pub fn new_in<K, V, A>(alloc: A) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, _, A>::new_in(alloc);

        Self::from_proj(proj_index_map)
    }

    pub fn with_capacity_in<K, V, A>(capacity: usize, alloc: A) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, _, A>::with_capacity_in(capacity, alloc);

        Self::from_proj(proj_index_map)
    }
}

impl OpaqueIndexMap {
    #[inline]
    pub fn with_hasher<K, V, S>(build_hasher: S) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, S, _>::with_hasher(build_hasher);

        Self::from_proj(proj_index_map)
    }

    #[inline]
    pub fn with_capacity_and_hasher<K, V, S>(capacity: usize, build_hasher: S) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, S, _>::with_capacity_and_hasher(capacity, build_hasher);

        Self::from_proj(proj_index_map)
    }
}

impl OpaqueIndexMap {
    #[inline]
    pub fn new<K, V>() -> Self
    where
        K: any::Any,
        V: any::Any,
    {
        Self::new_in::<K, V, alloc::Global>(alloc::Global)
    }

    #[inline]
    pub fn with_capacity<K, V>(capacity: usize) -> Self
    where
        K: any::Any,
        V: any::Any,
    {
        Self::with_capacity_in::<K, V, alloc::Global>(capacity, alloc::Global)
    }
}

impl OpaqueIndexMap {
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl OpaqueIndexMap {
    #[inline]
    pub fn hasher<K, V, S, A>(&self) -> &TypedProjBuildHasher<S>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.hasher()
    }

    #[inline]
    pub fn allocator<K, V, S, A>(&self) -> &TypedProjAlloc<A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.allocator()
    }
}

impl OpaqueIndexMap {
    pub fn get_index_of<Q, K, V, S, A>(&self, key: &Q) -> Option<usize>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get_index_of(key)
    }

    pub fn contains_key<Q, K, V, S, A>(&self, key: &Q) -> bool
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.contains_key(key)
    }

    pub fn get<Q, K, V, S, A>(&self, key: &Q) -> Option<&V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get(key)
    }

    pub fn get_key_value<Q, K, V, S, A>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get_key_value(key)
    }

    pub fn get_full<Q, K, V, S, A>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get_full(key)
    }

    pub fn get_mut<Q, K, V, S, A>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.get_mut(key)
    }

    pub fn get_full_mut<Q, K, V, S, A>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.get_full_mut(key)
    }

    pub fn keys<K, V, S, A>(&self) -> Keys<'_, K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.keys()
    }

    pub fn into_keys<K, V, S, A>(self) -> IntoKeys<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.into_proj::<K, V, S, A>();

        proj_self.into_keys()
    }

    pub fn iter<K, V, S, A>(&self) -> Iter<'_, K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.iter()
    }

    pub fn iter_mut<K, V, S, A>(&mut self) -> IterMut<'_, K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.iter_mut()
    }

    pub fn values<K, V, S, A>(&self) -> Values<'_, K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.values()
    }

    pub fn values_mut<K, V, S, A>(&mut self) -> ValuesMut<'_, K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.values_mut()
    }

    pub fn into_values<K, V, S, A>(self) -> IntoValues<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.into_proj::<K, V, S, A>();

        proj_self.into_values()
    }

    pub fn clear<K, V, S, A>(&mut self)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.clear();
    }

    pub fn truncate<K, V, S, A>(&mut self, len: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.truncate(len);
    }

    #[track_caller]
    pub fn drain<R, K, V, S, A>(&mut self, range: R) -> Drain<'_, K, V, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.drain(range)
    }

    #[track_caller]
    pub fn split_off<K, V, S, A>(&mut self, at: usize) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let proj_self =  self.as_proj_mut::<K, V, S, A>();
        let proj_split = proj_self.split_off(at);

        Self::from_proj(proj_split)
    }

    pub fn swap_remove<Q, K, V, S, A>(&mut self, key: &Q) -> Option<V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.swap_remove(key)
    }

    pub fn swap_remove_entry<Q, K, V, S, A>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.swap_remove_entry(key)
    }

    pub fn swap_remove_full<Q, K, V, S, A>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.swap_remove_full(key)
    }

    pub fn shift_remove<Q, K, V, S, A>(&mut self, key: &Q) -> Option<V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shift_remove(key)
    }

    pub fn shift_remove_entry<Q, K, V, S, A>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shift_remove_entry(key)
    }

    pub fn shift_remove_full<Q, K, V, S, A>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shift_remove_full(key)
    }

    pub fn as_slice<K, V, S, A>(&self) -> &'_ Slice<K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.as_slice()
    }

    pub fn as_mut_slice<K, V, S, A>(&mut self) -> &mut Slice<K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.as_mut_slice()
    }
}

impl OpaqueIndexMap {
    pub fn insert<K, V, S, A>(&mut self, key: K, value: V) -> Option<V>
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.insert(key, value)
    }

    pub fn insert_full<K, V, S, A>(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.insert_full(key, value)
    }

    pub fn insert_sorted<K, V, S, A>(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: any::Any + Eq + hash::Hash + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.insert_sorted(key, value)
    }

    #[track_caller]
    pub fn insert_before<K, V, S, A>(&mut self, index: usize, key: K, value: V) -> (usize, Option<V>)
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.insert_before(index, key, value)
    }

    #[track_caller]
    pub fn shift_insert<K, V, S, A>(&mut self, index: usize, key: K, value: V) -> Option<V>
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shift_insert(index, key, value)
    }

    pub fn entry<K, V, S, A>(&mut self, key: K) -> Entry<'_, K, V, A>
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.entry(key)
    }

    #[track_caller]
    pub fn splice<R, I, K, V, S, A>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, K, V, S, A>
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = (K, V)>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.splice(range, replace_with)
    }

    pub fn append<K, V, S, A>(&mut self, other: &mut OpaqueIndexMap)
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();
        let proj_other = other.as_proj_mut::<K, V, S, A>();

        proj_self.append(proj_other);
    }
}

impl OpaqueIndexMap {
    #[doc(alias = "pop_last")] // like `BTreeMap`
    pub fn pop<K, V, S, A>(&mut self) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.pop()
    }

    pub fn retain<F, K, V, S, A>(&mut self, keep: F)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&K, &mut V) -> bool,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.retain(keep)
    }

    pub fn sort_keys<K, V, S, A>(&mut self)
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.sort_keys()
    }

    pub fn sort_by<F, K, V, S, A>(&mut self, cmp: F)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.sort_by(cmp)
    }

    pub fn sorted_by<F, K, V, S, A>(self, cmp: F) -> IntoIter<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_self = self.into_proj::<K, V, S, A>();

        proj_self.sorted_by(cmp)
    }

    pub fn sort_unstable_keys<K, V, S, A>(&mut self)
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.sort_unstable_keys()
    }

    pub fn sort_unstable_by<F, K, V, S, A>(&mut self, cmp: F)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.sort_unstable_by(cmp)
    }

    #[inline]
    pub fn sorted_unstable_by<F, K, V, S, A>(self, cmp: F) -> IntoIter<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_self = self.into_proj::<K, V, S, A>();

        proj_self.sorted_unstable_by(cmp)
    }

    pub fn sort_by_cached_key<T, F, K, V, S, A>(&mut self, mut sort_key: F)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        T: Ord,
        F: FnMut(&K, &V) -> T,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.sort_by_cached_key(&mut sort_key)
    }

    pub fn binary_search_keys<K, V, S, A>(&self, key: &K) -> Result<usize, usize>
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.binary_search_keys(key)
    }

    #[inline]
    pub fn binary_search_by<F, K, V, S, A>(&self, f: F) -> Result<usize, usize>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&K, &V) -> cmp::Ordering,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.binary_search_by(f)
    }

    #[inline]
    pub fn binary_search_by_key<B, F, K, V, S, A>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&K, &V) -> B,
        B: Ord,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.binary_search_by_key(b, f)
    }

    #[must_use]
    pub fn partition_point<P, K, V, S, A>(&self, pred: P) -> usize
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        P: FnMut(&K, &V) -> bool,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.partition_point(pred)
    }

    pub fn reverse<K, V, S, A>(&mut self)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.reverse()
    }

    pub fn reserve<K, V, S, A>(&mut self, additional: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.reserve(additional)
    }

    pub fn reserve_exact<K, V, S, A>(&mut self, additional: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.reserve_exact(additional)
    }

    pub fn try_reserve<K, V, S, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.try_reserve(additional)
    }

    pub fn try_reserve_exact<K, V, S, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.try_reserve_exact(additional)
    }

    pub fn shrink_to_fit<K, V, S, A>(&mut self)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shrink_to_fit()
    }

    pub fn shrink_to<K, V, S, A>(&mut self, min_capacity: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shrink_to(min_capacity)
    }

    pub fn into_boxed_slice<K, V, S, A>(self) -> Box<Slice<K, V>, TypedProjAlloc<A>>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.into_proj::<K, V, S, A>();

        proj_self.into_boxed_slice()
    }

    pub fn get_index<K, V, S, A>(&self, index: usize) -> Option<(&K, &V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get_index(index)
    }

    pub fn get_index_mut<K, V, S, A>(&mut self, index: usize) -> Option<(&K, &mut V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.get_index_mut(index)
    }

    pub fn get_index_entry<K, V, S, A>(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.get_index_entry(index)
    }

    pub fn get_range<R, K, V, S, A>(&self, range: R) -> Option<&Slice<K, V>>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get_range(range)
    }

    pub fn get_range_mut<R, K, V, S, A>(&mut self, range: R) -> Option<&mut Slice<K, V>>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.get_range_mut(range)
    }

    #[doc(alias = "first_key_value")] // like `BTreeMap`
    pub fn first<K, V, S, A>(&self) -> Option<(&K, &V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.first()
    }

    pub fn first_mut<K, V, S, A>(&mut self) -> Option<(&K, &mut V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.first_mut()
    }

    pub fn first_entry<K, V, S, A>(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.first_entry()
    }

    #[doc(alias = "last_key_value")] // like `BTreeMap`
    pub fn last<K, V, S, A>(&self) -> Option<(&K, &V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.last()
    }

    pub fn last_mut<K, V, S, A>(&mut self) -> Option<(&K, &mut V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.last_mut()
    }

    pub fn last_entry<K, V, S, A>(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.last_entry()
    }

    pub fn swap_remove_index<K, V, S, A>(&mut self, index: usize) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.swap_remove_index(index)
    }

    pub fn shift_remove_index<K, V, S, A>(&mut self, index: usize) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shift_remove_index(index)
    }

    #[track_caller]
    pub fn move_index<K, V, S, A>(&mut self, from: usize, to: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.move_index(from, to)
    }

    #[track_caller]
    pub fn swap_indices<K, V, S, A>(&mut self, a: usize, b: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.swap_indices(a, b)
    }
}

impl fmt::Debug for OpaqueIndexMap {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpaqueIndexMap")
            .finish()
    }
}

#[cfg(feature = "std")]
impl<K, V> FromIterator<(K, V)> for OpaqueIndexMap
where
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
{
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>
    {
        let proj_map = TypedProjIndexMap::<K, V, hash::RandomState, alloc::Global>::from_iter(iterable);

        Self::from_proj(proj_map)
    }
}

#[cfg(feature = "std")]
impl<K, V, const N: usize> From<[(K, V); N]> for OpaqueIndexMap
where
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
{
    fn from(array: [(K, V); N]) -> Self {
        let proj_map = TypedProjIndexMap::<K, V, hash::RandomState, alloc::Global>::from_iter(array);

        Self::from_proj(proj_map)
    }
}

mod dummy {
    use super::*;
    use core::ptr::NonNull;
    use std::marker;

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
mod index_map_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_index_map_match_sizes<K, V, S, A>()
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::size_of::<TypedProjIndexMap<K, V, S, A>>();
        let result = mem::size_of::<OpaqueIndexMap>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_index_map_match_alignments<K, V, S, A>()
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::align_of::<TypedProjIndexMap<K, V, S, A>>();
        let result = mem::align_of::<OpaqueIndexMap>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_index_map_match_offsets<K, V, S, A>()
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::offset_of!(TypedProjIndexMap<K, V, S, A>, inner);
        let result = mem::offset_of!(OpaqueIndexMap, inner);

        assert_eq!(result, expected, "Opaque and Typed Projected data types offsets mismatch");
    }

    macro_rules! layout_tests {
        ($module_name:ident, $key_typ:ty, $value_typ:ty, $build_hasher_typ:ty, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_opaque_index_map_layout_match_sizes() {
                    run_test_opaque_index_map_match_sizes::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_index_map_layout_match_alignments() {
                    run_test_opaque_index_map_match_alignments::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_index_map_layout_match_offsets() {
                    run_test_opaque_index_map_match_offsets::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>();
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
    layout_tests!(unit_zst_strrandom_state_global, (), &'static str, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(unit_zst_tangent_spacerandom_state_global, (), layout_testing_types::TangentSpace, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(unit_zst_surface_differentialrandom_state_global, (), layout_testing_types::SurfaceDifferential, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(unit_zst_oct_tree_noderandom_state_global, (), layout_testing_types::OctTreeNode, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_unit_zstrandom_state_global, u8, (), hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_u8random_state_global, u8, u8, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_u64random_state_global, u8, u64, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_strrandom_state_global, u8, &'static str, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_tangent_spacerandom_state_global, u8, layout_testing_types::TangentSpace, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_surface_differentialrandom_state_global, u8, layout_testing_types::SurfaceDifferential, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_oct_tree_noderandom_state_global, u8, layout_testing_types::OctTreeNode, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u64_unit_zstrandom_state_global, u64, (), hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u64_u8random_state_global, u64, u8, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u64_u64random_state_global, u64, u64, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u64_strrandom_state_global, u64, &'static str, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u64_tangent_spacerandom_state_global, u64, layout_testing_types::TangentSpace, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u64_surface_differentialrandom_state_global, u64, layout_testing_types::SurfaceDifferential, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u64_oct_tree_noderandom_state_global, u64, layout_testing_types::OctTreeNode, hash::RandomState, alloc::Global);

    layout_tests!(unit_zst_unit_zst_dummy_hasher_dummy_alloc, (), (), dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(unit_zst_u8_dummy_hasher_dummy_alloc, (), u8, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(unit_zst_u64_dummy_hasher_dummy_alloc, (), u64, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(unit_zst_str_dummy_hasher_dummy_alloc, (), &'static str, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(unit_zst_tangent_space_dummy_hasher_dummy_alloc, (), layout_testing_types::TangentSpace, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(unit_zst_surface_differential_dummy_hasher_dummy_alloc, (), layout_testing_types::SurfaceDifferential, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(unit_zst_oct_tree_node_dummy_hasher_dummy_alloc, (), layout_testing_types::OctTreeNode, dummy::DummyBuildHasher, dummy::DummyAlloc);

    layout_tests!(u8_unit_zst_dummy_hasher_dummy_alloc, u8, (), dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u8_u8_dummy_hasher_dummy_alloc, u8, u8, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u8_u64_dummy_hasher_dummy_alloc, u8, u64, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u8_str_dummy_hasher_dummy_alloc, u8, &'static str, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u8_tangent_space_dummy_hasher_dummy_alloc, u8, layout_testing_types::TangentSpace, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u8_surface_differential_dummy_hasher_dummy_alloc, u8, layout_testing_types::SurfaceDifferential, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u8_oct_tree_node_dummy_hasher_dummy_alloc, u8, layout_testing_types::OctTreeNode, dummy::DummyBuildHasher, dummy::DummyAlloc);

    layout_tests!(u64_unit_zst_dummy_hasher_dummy_alloc, u64, (), dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u64_u8_dummy_hasher_dummy_alloc, u64, u8, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u64_u64_dummy_hasher_dummy_alloc, u64, u64, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u64_str_dummy_hasher_dummy_alloc, u64, &'static str, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u64_tangent_space_dummy_hasher_dummy_alloc, u64, layout_testing_types::TangentSpace, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u64_surface_differential_dummy_hasher_dummy_alloc, u64, layout_testing_types::SurfaceDifferential, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u64_oct_tree_node_dummy_hasher_dummy_alloc, u64, layout_testing_types::OctTreeNode, dummy::DummyBuildHasher, dummy::DummyAlloc);
}

#[cfg(test)]
mod index_map_assert_send_sync {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjIndexMap<i32, i32, hash::RandomState, alloc::Global>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjIndexMap<i32, i32, dummy::DummyBuildHasher, alloc::Global>>();
    }
}
