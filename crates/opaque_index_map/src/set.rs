use crate::{map_inner, OpaqueIndexMap, TypedProjIndexMap};
use crate::map_inner::{Bucket, OpaqueIndexMapInner};
use crate::range_ops;
use crate::slice_eq;
use crate::equivalent::Equivalent;

use core::any;
use core::cmp;
use core::fmt;
use core::iter;
use core::ops;
use alloc_crate::alloc;
use alloc_crate::boxed::Box;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

use opaque_alloc::TypedProjAlloc;
use opaque_hash::TypedProjBuildHasher;
use opaque_vec::TypedProjVec;
use opaque_error::TryReserveError;

#[repr(transparent)]
pub struct Slice<T> {
    entries: map_inner::Slice<T, ()>,
}

impl<T> Slice<T> {
    const fn from_slice(entries: &map_inner::Slice<T, ()>) -> &Self {
        unsafe { &*(entries as *const map_inner::Slice<T, ()> as *const Self) }
    }

    fn from_boxed_slice<A>(entries: Box<map_inner::Slice<T, ()>, TypedProjAlloc<A>>) -> Box<Self, TypedProjAlloc<A>>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let (ptr, alloc) = Box::into_raw_with_allocator(entries);
        unsafe {
            Box::from_raw_in(ptr as *const Self as *mut Self, alloc)
        }
    }

    fn into_boxed_slice<A>(self: Box<Self, TypedProjAlloc<A>>) -> Box<map_inner::Slice<T, ()>, TypedProjAlloc<A>>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let (ptr, alloc) = Box::into_raw_with_allocator(self);
        unsafe {
            Box::from_raw_in(ptr as *const map_inner::Slice<T, ()> as *mut map_inner::Slice<T, ()>, alloc)
        }
    }
}

impl<T> Slice<T> {
    pub(crate) fn into_entries<A>(self: Box<Self, TypedProjAlloc<A>>) -> TypedProjVec<Bucket<T, ()>, A>
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let boxed_entries: Box<map_inner::Slice<T, ()>, TypedProjAlloc<A>> = Self::into_boxed_slice(self);
        map_inner::Slice::into_entries(boxed_entries)

    }

    fn from_entries_in<A>(vec: TypedProjVec<Bucket<T, ()>, A>) -> Box<Self, TypedProjAlloc<A>>
    where
        T: any::Any,
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

    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        self.entries.get_index(index).map(|tuple| tuple.0)
    }

    pub fn get_range<R>(&self, range: R) -> Option<&Self>
    where
        R: ops::RangeBounds<usize>,
    {
        let range = range_ops::try_simplify_range(range, self.entries.len())?;
        self.entries.get_range(range).map(Self::from_slice)
    }

    pub fn first(&self) -> Option<&T> {
        self.entries.first().map(|tuple| tuple.0)
    }

    pub fn last(&self) -> Option<&T> {
        self.entries.last().map(|tuple| tuple.0)
    }

    pub fn split_at(&self, index: usize) -> (&Self, &Self) {
        let (first, second) = self.entries.split_at(index);
        (Self::from_slice(first), Self::from_slice(second))
    }

    pub fn split_first(&self) -> Option<(&T, &Self)> {
        self.entries.split_first().map(|((first, _), rest)| (first, Self::from_slice(rest)))
    }

    pub fn split_last(&self) -> Option<(&T, &Self)> {
        self.entries.split_last().map(|((last, _), rest)| (last, Self::from_slice(rest)))
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(&self.entries)
    }

    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.binary_search_by(|p| p.cmp(x))
    }

    #[inline]
    pub fn binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> cmp::Ordering,
    {
        self.entries.binary_search_by(move |a, b| f(a))
    }

    #[inline]
    pub fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        self.binary_search_by(|k| f(k).cmp(b))
    }

    #[must_use]
    pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        self.entries.partition_point(move |a, b| pred(a))
    }
}

impl<'a, T> IntoIterator for &'a Slice<T>
where
    T: any::Any,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, A> IntoIterator for Box<Slice<T>, TypedProjAlloc<A>>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = T;
    type IntoIter = IntoIter<T, A>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(map_inner::IntoIter::new(self.into_entries()))
    }
}

impl<T> Default for &'_ Slice<T> {
    fn default() -> Self {
        Slice::from_slice(Default::default())
    }
}

impl<T, A> Default for Box<Slice<T>, TypedProjAlloc<A>>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Slice::from_boxed_slice(Box::default())
    }
}

impl<T, A> Clone for Box<Slice<T>, TypedProjAlloc<A>>
where
    T: any::Any + Clone,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        let alloc = Box::allocator(&self).clone();
        let entries = self.entries.to_entries_in(alloc);

        Slice::from_entries_in(entries)
    }
}

impl<T> From<&Slice<T>> for Box<Slice<T>, TypedProjAlloc<alloc::Global>>
where
    T: any::Any + Copy,
{
    fn from(slice: &Slice<T>) -> Self {
        let boxed_entries: Box<map_inner::Slice<T, ()>, TypedProjAlloc<alloc::Global>> = Box::from(&slice.entries);

        Slice::from_boxed_slice(boxed_entries)
    }
}

impl<T> fmt::Debug for Slice<T>
where
    T: any::Any + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self).finish()
    }
}

impl<T, U> PartialEq<Slice<U>> for Slice<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Slice<U>) -> bool {
        slice_eq::slice_eq(self.entries.as_entries(), other.entries.as_entries(), |b1, b2| b1.key_ref() == b2.key_ref())
    }
}

impl<T, U> PartialEq<[U]> for Slice<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U]) -> bool {
        slice_eq::slice_eq(self.entries.as_entries(), other, |b, o| b.key_ref() == o)
    }
}

impl<T, U> PartialEq<Slice<U>> for [T]
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Slice<U>) -> bool {
        slice_eq::slice_eq(self, &other.entries.as_entries(), |o, b| o == b.key_ref())
    }
}

impl<T, U, const N: usize> PartialEq<[U; N]> for Slice<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U; N]) -> bool {
        <Self as PartialEq<[U]>>::eq(self, other)
    }
}

impl<T, const N: usize, U> PartialEq<Slice<U>> for [T; N]
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Slice<U>) -> bool {
        <[T] as PartialEq<Slice<U>>>::eq(self, other)
    }
}

impl<T> Eq for Slice<T>
where
    T: Eq,
{
}

impl<T> PartialOrd for Slice<T>
where
    T: any::Any + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T> Ord for Slice<T>
where
    T: any::Any + Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.iter().cmp(other)
    }
}

impl<T> hash::Hash for Slice<T>
where
    T: any::Any + hash::Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.len().hash(state);
        for value in self {
            value.hash(state);
        }
    }
}

impl<T> ops::Index<usize> for Slice<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.entries[index].key_ref()
    }
}

macro_rules! impl_index_for_index_set_slice {
    ($($range:ty),*) => {$(
        impl<T> ops::Index<$range> for Slice<T> {
            type Output = Self;

            fn index(&self, range: $range) -> &Self::Output {
                Slice::from_slice(&self.entries[range])
            }
        }
    )*}
}

impl_index_for_index_set_slice!(
    ops::Range<usize>,
    ops::RangeFrom<usize>,
    ops::RangeFull,
    ops::RangeInclusive<usize>,
    ops::RangeTo<usize>,
    ops::RangeToInclusive<usize>,
    (ops::Bound<usize>, ops::Bound<usize>)
);

pub struct Iter<'a, T> {
    iter: map_inner::Iter<'a, T, ()>,
}

impl<'a, T> Iter<'a, T> {
    fn new(entries: &'a map_inner::Slice<T, ()>) -> Self {
        Self {
            iter: entries.iter(),
        }
    }

    pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(self.iter.as_slice())
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|tuple| tuple.0)
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|tuple| tuple.0)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(|tuple| tuple.0)
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T> iter::FusedIterator for Iter<'_, T> {}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Iter {
            iter: self.iter.clone(),
        }
    }
}

impl<T> fmt::Debug for Iter<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Self { iter: map_inner::Slice::new().iter() }
    }
}

#[derive(Clone)]
pub struct IntoIter<T, A = alloc::Global>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: map_inner::IntoIter<T, (), A>,
}

impl<T, A> IntoIter<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    const fn new(iter: map_inner::IntoIter<T, (), A>) -> Self {
        Self {
            iter,
        }
    }

    pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(self.iter.as_slice())
    }
}

impl<T, A> Iterator for IntoIter<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|tuple| tuple.0)
    }
}

impl<T, A> DoubleEndedIterator for IntoIter<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|tuple| tuple.0)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(|tuple| tuple.0)
    }
}

impl<T, A> ExactSizeIterator for IntoIter<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, A> iter::FusedIterator for IntoIter<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, A> fmt::Debug for IntoIter<T, A>
where
    T: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iterator = self.iter.as_slice().iter().map(|tuple| tuple.0);
        formatter.debug_list().entries(iterator).finish()
    }
}

impl<T, A> Default for IntoIter<T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Self {
            iter: map_inner::IntoIter::default(),
        }
    }
}

pub struct Drain<'a, T, A = alloc::Global>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: map_inner::Drain<'a, T, (), A>,
}

impl<'a, T, A> Drain<'a, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn new(iter: map_inner::Drain<'a, T, (), A>) -> Self {
        Self { iter, }
    }

    pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(self.iter.as_slice())
    }
}

impl<T, A> Iterator for Drain<'_, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|tuple| tuple.0)
    }
}

impl<T, A> DoubleEndedIterator for Drain<'_, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|tuple| tuple.0)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(|tuple| tuple.0)
    }
}

impl<T, A> ExactSizeIterator for Drain<'_, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<T, A> iter::FusedIterator for Drain<'_, T, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, A> fmt::Debug for Drain<'_, T, A>
where
    T: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iterator = self.iter.as_slice().iter().map(|tuple| tuple.0);
        formatter.debug_list().entries(iterator).finish()
    }
}

pub struct Difference<'a, T, S, A = alloc::Global>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: Iter<'a, T>,
    other: &'a TypedProjIndexSet<T, S, A>,
}

impl<'a, T, S, A> Difference<'a, T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn new<S1>(set: &'a TypedProjIndexSet<T, S1, A>, other: &'a TypedProjIndexSet<T, S, A>) -> Self
    where
        S1: any::Any + hash::BuildHasher + Send + Sync,
        S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Self {
            iter: set.iter(),
            other,
        }
    }
}

impl<'a, T, S, A> Iterator for Difference<'a, T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if !self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<T, S, A> DoubleEndedIterator for Difference<'_, T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next_back() {
            if !self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }
}

impl<T, S, A> iter::FusedIterator for Difference<'_, T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, S, A> Clone for Difference<'_, T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn clone(&self) -> Self {
        Difference {
            iter: self.iter.clone(),
            other: self.other,
        }
    }
}

impl<T, S, A> fmt::Debug for Difference<'_, T, S, A>
where
    T: any::Any + hash::Hash + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

pub struct Intersection<'a, T, S, A = alloc::Global>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: Iter<'a, T>,
    other: &'a TypedProjIndexSet<T, S, A>,
}

impl<'a, T, S, A> Intersection<'a, T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn new<S1>(set: &'a TypedProjIndexSet<T, S1, A>, other: &'a TypedProjIndexSet<T, S, A>) -> Self
    where
        S1: any::Any + hash::BuildHasher + Send + Sync,
        S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Self {
            iter: set.iter(),
            other,
        }
    }
}

impl<'a, T, S, A> Iterator for Intersection<'a, T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<T, S, A> DoubleEndedIterator for Intersection<'_, T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next_back() {
            if self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }
}

impl<T, S, A> iter::FusedIterator for Intersection<'_, T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, S, A> Clone for Intersection<'_, T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn clone(&self) -> Self {
        Intersection {
            iter: self.iter.clone(),
            ..*self
        }
    }
}

impl<T, S, A> fmt::Debug for Intersection<'_, T, S, A>
where
    T: any::Any + hash::Hash + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

pub struct SymmetricDifference<'a, T, S1, S2, A = alloc::Global>
where
    T: any::Any,
    S1: any::Any + hash::BuildHasher + Send + Sync,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: iter::Chain<Difference<'a, T, S2, A>, Difference<'a, T, S1, A>>,
}

impl<'a, T, S1, S2, A> SymmetricDifference<'a, T, S1, S2, A>
where
    T: any::Any + hash::Hash + Eq,
    S1: any::Any + hash::BuildHasher + Send + Sync,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn new(set1: &'a TypedProjIndexSet<T, S1, A>, set2: &'a TypedProjIndexSet<T, S2, A>) -> Self {
        let diff1 = set1.difference(set2);
        let diff2 = set2.difference(set1);
        Self {
            iter: diff1.chain(diff2),
        }
    }
}

impl<'a, T, S1, S2, A> Iterator for SymmetricDifference<'a, T, S1, S2, A>
where
    T: any::Any + hash::Hash + Eq,
    S1: any::Any + hash::BuildHasher + Send + Sync,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, f)
    }
}

impl<T, S1, S2, A> DoubleEndedIterator for SymmetricDifference<'_, T, S1, S2, A>
where
    T: any::Any + hash::Hash + Eq,
    S1: any::Any + hash::BuildHasher + Send + Sync,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.rfold(init, f)
    }
}

impl<T, S1, S2, A> iter::FusedIterator for SymmetricDifference<'_, T, S1, S2, A>
where
    T: any::Any + hash::Hash + Eq,
    S1: any::Any + hash::BuildHasher + Send + Sync,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, S1, S2, A> Clone for SymmetricDifference<'_, T, S1, S2, A>
where
    T: any::Any,
    S1: any::Any + hash::BuildHasher + Send + Sync,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn clone(&self) -> Self {
        SymmetricDifference {
            iter: self.iter.clone(),
        }
    }
}

impl<T, S1, S2, A> fmt::Debug for SymmetricDifference<'_, T, S1, S2, A>
where
    T: any::Any + hash::Hash + Eq + fmt::Debug,
    S1: any::Any + hash::BuildHasher + Send + Sync,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

pub struct Union<'a, T, S, A = alloc::Global>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: iter::Chain<Iter<'a, T>, Difference<'a, T, S, A>>,
}

impl<'a, T, S, A> Union<'a, T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn new<S2>(set1: &'a TypedProjIndexSet<T, S, A>, set2: &'a TypedProjIndexSet<T, S2, A>) -> Self
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Self {
            iter: set1.iter().chain(set2.difference(set1)),
        }
    }
}

impl<'a, T, S, A> Iterator for Union<'a, T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, f)
    }
}

impl<T, S, A> DoubleEndedIterator for Union<'_, T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }

    fn rfold<B, F>(self, init: B, f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.rfold(init, f)
    }
}

impl<T, S, A> iter::FusedIterator for Union<'_, T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, S, A> Clone for Union<'_, T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn clone(&self) -> Self {
        Union {
            iter: self.iter.clone(),
        }
    }
}

impl<T, S, A> fmt::Debug for Union<'_, T, S, A>
where
    T: any::Any + hash::Hash + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self.clone()).finish()
    }
}

#[cfg(feature = "std")]
pub struct Splice<'a, I, T, S = hash::RandomState, A = alloc::Global>
where
    I: Iterator<Item = T>,
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: map_inner::Splice<'a, UnitValue<I>, T, (), S, A>,
}

#[cfg(not(feature = "std"))]
pub struct Splice<'a, I, T, S, A = alloc::Global>
where
    I: Iterator<Item = T>,
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    iter: map_inner::Splice<'a, UnitValue<I>, T, (), S, A>,
}

impl<'a, I, T, S, A> Splice<'a, I, T, S, A>
where
    I: Iterator<Item = T>,
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    #[track_caller]
    fn new<R>(set: &'a mut TypedProjIndexSet<T, S, A>, range: R, replace_with: I) -> Self
    where
        R: ops::RangeBounds<usize>,
    {
        Self {
            iter: set.inner.splice(range, UnitValue(replace_with)),
        }
    }
}

impl<I, T, S, A> Iterator for Splice<'_, I, T, S, A>
where
    I: Iterator<Item = T>,
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, T, S, A> DoubleEndedIterator for Splice<'_, I, T, S, A>
where
    I: Iterator<Item = T>,
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        Some(self.iter.next_back()?.0)
    }
}

impl<I, T, S, A> ExactSizeIterator for Splice<'_, I, T, S, A>
where
    I: Iterator<Item = T>,
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<I, T, S, A> iter::FusedIterator for Splice<'_, I, T, S, A>
where
    I: Iterator<Item = T>,
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

struct UnitValue<I>(I);

impl<I: Iterator> Iterator for UnitValue<I> {
    type Item = (I::Item, ());

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| (x, ()))
    }
}

impl<I, T, S, A> fmt::Debug for Splice<'_, I, T, S, A>
where
    I: fmt::Debug + Iterator<Item = T>,
    T: any::Any + hash::Hash + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.iter, formatter)
    }
}

impl<I: fmt::Debug> fmt::Debug for UnitValue<I> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, formatter)
    }
}

/// A type-projected hash set where the order of the entries inside the set is independent of the
/// hash values of the elements.
///
/// The interface to this hash set tracks closely with the standard library's [`HashSet`] interface.
/// One feature this hash set has that the standard library one does not is that it is generic over
/// the choice of memory allocator. This type supports type-erasure of generic parameters. The main
/// difference is that a `TypedProjIndexSet` can be converted to an `OpaqueIndexSet` in constant
/// **O(1)** time, hiding its value type, hash builder type, and allocator type, at runtime.
///
/// # Ordering
///
/// The values are stored in the set in their insertion order, rather than by their
/// hash value, provided no removal method have been called on an entry in the set. In particular,
/// inserting a new value into the set does not change the **storage order** of the other values in
/// the set.
///
/// # Indices
///
/// The values are stored in a packed range with no holes in the range `[0, self.len())`.
/// Thus, one can always use the [`get_index_of`] or [`get_index`] methods to interact with values
/// inside the set by their storage index instead of their value.
///
/// # Type Erasure And Type Projection
///
/// This allows for more flexible and dynamic data handling, especially when working with
/// collections of unknown or dynamic types. Type-erasable collections allow for more efficient
/// runtime dynamic typing, since one has more control over the memory layout of the collection,
/// even for erased types. Some applications of this include implementing heterogeneous data
/// structures, plugin systems, and managing foreign function interface data. There are two data
/// types that are dual to each other: [`TypedProjIndexSet`] and [`OpaqueIndexSet`].
///
/// By laying out both data types identically, we can project the underlying types in **O(1)** time,
/// and erase the underlying types in **O(1)** time, though the conversion is often zero-cost.
///
/// # See Also
///
/// - [`OpaqueIndexSet`]: the type-erased counterpart of [`TypedProjIndexSet`].
///
/// # Examples
///
/// Basic usage of a type-projected index set.
///
/// ```
/// # use opaque_index_map::TypedProjIndexSet;
/// #
/// let mut party: TypedProjIndexSet<String> = TypedProjIndexSet::from([
///     String::from("cloud"),
///     String::from("tifa"),
///     String::from("aerith"),
///     String::from("barret"),
///     String::from("cid"),
///     String::from("vincent"),
///     String::from("yuffie"),
///     String::from("red xiii"),
///     String::from("cait sith"),
/// ]);
///
/// assert_eq!(party.get("cloud"),     Some(&String::from("cloud")));
/// assert_eq!(party.get("tifa"),      Some(&String::from("tifa")));
/// assert_eq!(party.get("aerith"),    Some(&String::from("aerith")));
/// assert_eq!(party.get("barret"),    Some(&String::from("barret")));
/// assert_eq!(party.get("cid"),       Some(&String::from("cid")));
/// assert_eq!(party.get("vincent"),   Some(&String::from("vincent")));
/// assert_eq!(party.get("yuffie"),    Some(&String::from("yuffie")));
/// assert_eq!(party.get("red xiii"),  Some(&String::from("red xiii")));
/// assert_eq!(party.get("cait sith"), Some(&String::from("cait sith")));
///
/// assert!(!party.contains("sephiroth"));
/// assert!(!party.contains("jenova"));
/// assert!(!party.contains("emerald weapon"));
///
/// // Elements of an index set are stored in their insertion order, independent of their values.
/// assert_eq!(party.get_index_of("cloud"),     Some(0));
/// assert_eq!(party.get_index_of("tifa"),      Some(1));
/// assert_eq!(party.get_index_of("aerith"),    Some(2));
/// assert_eq!(party.get_index_of("barret"),    Some(3));
/// assert_eq!(party.get_index_of("cid"),       Some(4));
/// assert_eq!(party.get_index_of("vincent"),   Some(5));
/// assert_eq!(party.get_index_of("yuffie"),    Some(6));
/// assert_eq!(party.get_index_of("red xiii"),  Some(7));
/// assert_eq!(party.get_index_of("cait sith"), Some(8));
///
/// assert_eq!(party.get_index_of("sephiroth"),      None);
/// assert_eq!(party.get_index_of("jenova"),         None);
/// assert_eq!(party.get_index_of("emerald weapon"), None);
///
/// party.insert(String::from("sephiroth"));
///
/// assert!(party.contains("sephiroth"));
///
/// // Elements of an index set are stored in their insertion order, independent of their values.
/// assert_eq!(party.get_index_of("cloud"),     Some(0));
/// assert_eq!(party.get_index_of("tifa"),      Some(1));
/// assert_eq!(party.get_index_of("aerith"),    Some(2));
/// assert_eq!(party.get_index_of("barret"),    Some(3));
/// assert_eq!(party.get_index_of("cid"),       Some(4));
/// assert_eq!(party.get_index_of("vincent"),   Some(5));
/// assert_eq!(party.get_index_of("yuffie"),    Some(6));
/// assert_eq!(party.get_index_of("red xiii"),  Some(7));
/// assert_eq!(party.get_index_of("cait sith"), Some(8));
/// assert_eq!(party.get_index_of("sephiroth"), Some(9));
///
/// assert_eq!(party.get("sephiroth"), Some(&String::from("sephiroth")));
///
/// party.shift_remove("sephiroth");
///
/// assert!(!party.contains("sephiroth"));
/// ```
#[cfg(feature = "std")]
#[repr(transparent)]
pub struct TypedProjIndexSet<T, S = hash::RandomState, A = alloc::Global>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: map_inner::TypedProjIndexMapInner<T, (), S, A>,
}

#[cfg(not(feature = "std"))]
#[repr(transparent)]
pub struct TypedProjIndexSet<T, S, A = alloc::Global>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: map_inner::TypedProjIndexMapInner<T, (), S, A>,
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Returns the [`TypeId`] of the values contained in the type-projected index set.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<isize, RandomState, Global> = TypedProjIndexSet::new();
    ///
    /// assert_eq!(proj_set.value_type_id(), TypeId::of::<isize>());
    /// ```
    #[inline]
    pub const fn value_type_id(&self) -> any::TypeId {
        self.inner.key_type_id()
    }

    /// Returns the [`TypeId`] of the hash builder for the type-projected index set.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<isize, RandomState, Global> = TypedProjIndexSet::new();
    ///
    /// assert_eq!(proj_set.build_hasher_type_id(), TypeId::of::<RandomState>());
    /// ```
    #[inline]
    pub const fn build_hasher_type_id(&self) -> any::TypeId {
        self.inner.build_hasher_type_id()
    }

    /// Returns the [`TypeId`] of the memory allocator for the type-projected index set.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<isize, RandomState, Global> = TypedProjIndexSet::new();
    ///
    /// assert_eq!(proj_set.allocator_type_id(), TypeId::of::<Global>());
    /// ```
    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    fn into_entries(self) -> TypedProjVec<Bucket<T, ()>, A> {
        self.inner.into_entries()
    }

    #[inline]
    fn as_entries(&self) -> &map_inner::Slice<T, ()> {
        self.inner.as_slice()
    }

    #[inline]
    fn as_entries_mut(&mut self) -> &mut map_inner::Slice<T, ()> {
        self.inner.as_mut_slice()
    }

    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Bucket<T, ()>]),
    {
        self.inner.with_entries(f);
    }
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Constructs a new index set with the given type-projected hash builder and type-projected
    /// memory allocator.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let proj_build_hasher = TypedProjBuildHasher::new(RandomState::new());
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_hasher_proj_in(
    ///     proj_build_hasher,
    ///     proj_alloc
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    #[inline]
    pub fn with_hasher_proj_in(proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::<T, (), S, A>::with_hasher_proj_in(proj_build_hasher, proj_alloc);

        Self {
            inner: proj_inner,
        }
    }

    /// Constructs a new index set with the given capacity, type-projected hash builder, and
    /// type-projected memory allocator.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-projected index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let proj_build_hasher = TypedProjBuildHasher::new(RandomState::new());
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_and_hasher_proj_in(
    ///     capacity,
    ///     proj_build_hasher,
    ///     proj_alloc
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert!(proj_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-projected index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let proj_build_hasher = TypedProjBuildHasher::new(RandomState::new());
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_and_hasher_proj_in(
    ///     0,
    ///     proj_build_hasher,
    ///     proj_alloc
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher_proj_in(capacity: usize, proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self {
        if capacity == 0 {
            Self::with_hasher_proj_in(proj_build_hasher, proj_alloc)
        } else {
            let proj_inner = map_inner::TypedProjIndexMapInner::<T, (), S, A>::with_capacity_and_hasher_proj_in(capacity, proj_build_hasher, proj_alloc);

            Self {
                inner: proj_inner,
            }
        }
    }
}

#[cfg(feature = "std")]
impl<T, A> TypedProjIndexSet<T, hash::RandomState, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Constructs a new index set with the given type-projected memory allocator.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::new_proj_in(proj_alloc);
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    pub fn new_proj_in(proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::<T, (), hash::RandomState, A>::new_proj_in(proj_alloc);

        Self {
            inner : proj_inner,
        }
    }

    /// Constructs a new index set with the given capacity and type-projected memory allocator.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-projected index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_proj_in(
    ///     capacity,
    ///     proj_alloc
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert!(proj_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-projected index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_proj_in(
    ///     0,
    ///     proj_alloc
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    pub fn with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::<T, (), hash::RandomState, A>::with_capacity_proj_in(capacity, proj_alloc);

        Self {
            inner: proj_inner,
        }
    }
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Constructs a new index set with the given hash builder and memory allocator.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_hasher_in(
    ///     RandomState::new(),
    ///     Global
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    pub fn with_hasher_in(build_hasher: S, alloc: A) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::with_hasher_in(build_hasher, alloc);

        TypedProjIndexSet {
            inner: proj_inner,
        }
    }

    /// Constructs a new index set with the given capacity, hash builder, and memory allocator.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-projected index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_and_hasher_in(
    ///     capacity,
    ///     RandomState::new(),
    ///     Global
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert!(proj_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-projected index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_and_hasher_in(
    ///     0,
    ///     RandomState::new(),
    ///     Global
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    pub fn with_capacity_and_hasher_in(capacity: usize, build_hasher: S, alloc: A) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::with_capacity_and_hasher_in(capacity, build_hasher, alloc);

        TypedProjIndexSet {
            inner: proj_inner,
        }
    }
}

#[cfg(feature = "std")]
impl<T, A> TypedProjIndexSet<T, hash::RandomState, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Constructs a new index set with the given memory allocator.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::new_in(Global);
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    pub fn new_in(alloc: A) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::<T, (), hash::RandomState, A>::new_in(alloc);

        Self {
            inner : proj_inner,
        }
    }

    /// Constructs a new index set with the given capacity and memory allocator.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-projected index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_in(
    ///     capacity,
    ///     Global
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert!(proj_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-projected index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_in(
    ///     0,
    ///     Global
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::<T, (), hash::RandomState, A>::with_capacity_in(capacity, alloc);

        Self {
            inner: proj_inner,
        }
    }
}

impl<T, S> TypedProjIndexSet<T, S, alloc::Global>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    /// Constructs a new index set with the given hash builder.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_hasher(RandomState::new());
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    pub fn with_hasher(build_hasher: S) -> Self {
        TypedProjIndexSet {
            inner: map_inner::TypedProjIndexMapInner::with_hasher(build_hasher),
        }
    }

    /// Constructs a new index set with the given capacity and hash builder.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-projected index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_and_hasher(
    ///     capacity,
    ///     RandomState::new(),
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert!(proj_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-projected index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_and_hasher(
    ///     0,
    ///     RandomState::new(),
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    pub fn with_capacity_and_hasher(capacity: usize, build_hasher: S) -> Self {
        TypedProjIndexSet {
            inner: map_inner::TypedProjIndexMapInner::with_capacity_and_hasher(capacity, build_hasher),
        }
    }
}

#[cfg(feature = "std")]
impl<T> TypedProjIndexSet<T, hash::RandomState, alloc::Global>
where
    T: any::Any,
{
    /// Constructs a new index set.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::new();
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    pub fn new() -> Self {
        TypedProjIndexSet {
            inner: map_inner::TypedProjIndexMapInner::new(),
        }
    }

    /// Constructs a new index set with the given capacity.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-projected index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity(
    ///     capacity,
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert!(proj_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-projected index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity(
    ///     0,
    /// );
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), 0);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        TypedProjIndexSet {
            inner: map_inner::TypedProjIndexMapInner::with_capacity(capacity),
        }
    }
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Returns the capacity of the type-projected index set.
    ///
    /// The **capacity** of a type-projected index set is the number of values the index set
    /// can hold without reallocating memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 32;
    /// let mut proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_in(
    ///     capacity,
    ///     Global,
    /// );
    ///
    /// assert_eq!(proj_set.len(), 0);
    /// assert!(proj_set.capacity() >= capacity);
    ///
    /// for i in 0..capacity {
    ///     proj_set.insert(i);
    /// }
    ///
    /// assert_eq!(proj_set.len(), capacity);
    /// assert!(proj_set.capacity() >= capacity);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns the length of the type-projected index set.
    ///
    /// The **length** of a type-projected index set is the number of values stored inside it.
    /// The length satisfies the following. Given an index set `set`
    ///
    /// ```text
    /// set.len() ≤ set.capacity().
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let len = 32;
    /// let mut proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_in(
    ///     len,
    ///     Global,
    /// );
    ///
    /// assert_eq!(proj_set.len(), 0);
    ///
    /// for i in 0..len {
    ///     proj_set.insert(i);
    /// }
    ///
    /// assert_eq!(proj_set.len(), len);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Determines whether the type-projected index set is empty.
    ///
    /// A type-projected index set is **empty** if it contains no values, i.e. its length is zero.
    /// This method satisfies the following. Given an index set `set`
    ///
    /// ```text
    /// set.is_empty() ⇔ set.len() = 0.
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_in(
    ///     1,
    ///     Global,
    /// );
    ///
    /// assert!(proj_set.is_empty());
    ///
    /// proj_set.insert(1);
    ///
    /// assert!(!proj_set.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Returns a reference to the type-projected hash builder used by the index set.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize> = TypedProjIndexSet::new();
    ///
    /// assert!(proj_set.is_empty());
    ///
    /// let build_hasher: &TypedProjBuildHasher<RandomState> = proj_set.hasher();
    /// ```
    #[inline]
    pub const fn hasher(&self) -> &TypedProjBuildHasher<S> {
        self.inner.hasher()
    }

    /// Returns a reference to the type-projected memory allocator from the index set.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize> = TypedProjIndexSet::new();
    ///
    /// assert!(proj_set.is_empty());
    ///
    /// let alloc: &TypedProjAlloc<Global> = proj_set.allocator();
    /// ```
    #[inline]
    pub fn allocator(&self) -> &TypedProjAlloc<A> {
        self.inner.allocator()
    }
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Returns an iterator over the entries in the index set.
    ///
    /// The iterator returns the entries in their storage order in the index set.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<i32> = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32]);
    /// let entries: TypedProjVec<i32> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(entries.as_slice(), &[1_i32, 2_i32, 3_i32]);
    ///
    /// // The entries come back in storage or insertion order from the index set.
    /// for i in 0..entries.len() {
    ///     let expected = i;
    ///     let result = proj_set.get_index_of(&entries[i]).unwrap();
    ///     assert_eq!(result, expected);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.as_entries())
    }

    /// Removes all the entries from the index set.
    ///
    /// After calling this method, the collection will be empty. This method does not change the
    /// allocated capacity of the type-projected index set.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let mut proj_set: TypedProjIndexSet<String> = TypedProjIndexSet::with_capacity(10);
    ///
    /// assert!(proj_set.is_empty());
    ///
    /// proj_set.extend([String::from("foo"), String::from("bar"), String::from("baz")]);
    ///
    /// assert!(!proj_set.is_empty());
    /// assert_eq!(proj_set.len(), 3);
    ///
    /// let old_capacity = proj_set.capacity();
    ///
    /// proj_set.clear();
    ///
    /// assert!(proj_set.is_empty());
    /// assert_eq!(proj_set.capacity(), old_capacity);
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Shortens an index set to the supplied length, dropping the remaining elements.
    ///
    /// This method keeps the entries of `self` in the range `[0, len)`. In particular,
    /// this method drops every entry with storage index in the range `[len, self.len())`.
    /// This method does nothing when `self.len() <= len`.
    ///
    /// # Examples
    ///
    /// Truncating a type-projected index set when `len < self.len()`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<i64> = TypedProjIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// proj_set.truncate(2);
    ///
    /// assert_eq!(proj_set.len(), 2);
    ///
    /// let expected = TypedProjVec::from([1_i64, 2_i64]);
    /// let result: TypedProjVec<i64> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// No truncation occurs when `len == self.len()`
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<i64> = TypedProjIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// proj_set.truncate(6);
    ///
    /// assert_eq!(proj_set.len(), 6);
    ///
    /// let expected = TypedProjVec::from([1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64]);
    /// let result: TypedProjVec<i64> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// No truncation occurs when `len > self.len()`
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<i64> = TypedProjIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// proj_set.truncate(7);
    ///
    /// assert_eq!(proj_set.len(), 6);
    ///
    /// let expected = TypedProjVec::from([1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64]);
    /// let result: TypedProjVec<i64> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// Truncating when `len == 0` is equivalent to calling the [`clear`] method.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<i64> = TypedProjIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// proj_set.truncate(0);
    ///
    /// assert_eq!(proj_set.len(), 0);
    ///
    /// let expected = TypedProjVec::from([]);
    /// let result: TypedProjVec<i64> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// [`clear`]: TypedProjIndexSet::clear
    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    /// Removes the subslice indicated by the given range from the index set,
    /// returning a double-ended iterator over the removed subslice.
    ///
    /// If the iterator is dropped before being fully consumed, it drops the remaining removed
    /// elements. The draining iterator shifts the remaining entries in the index set above the
    /// range down to fill in the removed entries.
    ///
    /// The returned iterator keeps a mutable borrow on the index set to optimize its
    /// implementation.
    ///
    /// # Panics
    ///
    /// This method panics if the range of the subslice falls outside the bounds of the collection.
    /// That is, if the starting point of the subslice being removed starts after the end of
    /// `self`, or if the ending point is larger than the length of the index set.
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped (due to
    /// [`mem::forget`], for example), the index set may have lost and leaked
    /// elements arbitrarily, including elements outside the range.
    ///
    /// # Examples
    ///
    /// Draining part of a type-projected index set.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<i64> = TypedProjIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// assert_eq!(proj_set.len(), 6);
    ///
    /// let drained_entries: TypedProjVec<i64> = proj_set.drain(2..).collect();
    ///
    /// assert_eq!(proj_set.len(), 2);
    /// assert_eq!(drained_entries.len(), 4);
    ///
    /// let expected_set_entries = TypedProjVec::from([1_i64, 2_i64]);
    /// let result_set_entries: TypedProjVec<i64> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result_set_entries, expected_set_entries);
    ///
    /// let expected_drained_entries: TypedProjVec<i64> = TypedProjVec::from([
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// assert_eq!(drained_entries.as_slice(), expected_drained_entries.as_slice());
    /// ```
    ///
    /// Draining an entire type-projected index set.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<i64> = TypedProjIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// assert_eq!(proj_set.len(), 6);
    ///
    /// let drained_entries: TypedProjVec<i64> = proj_set.drain(..).collect();
    ///
    /// assert_eq!(proj_set.len(), 0);
    /// assert_eq!(drained_entries.len(), 6);
    ///
    /// let expected_set_entries = TypedProjVec::from([]);
    /// let result_set_entries: TypedProjVec<i64> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result_set_entries, expected_set_entries);
    ///
    /// let expected_drained_entries: TypedProjVec<i64> = TypedProjVec::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// assert_eq!(drained_entries.as_slice(), expected_drained_entries.as_slice());
    /// ```
    ///
    /// Draining no part of a type-projected index set.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<i64> = TypedProjIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// assert_eq!(proj_set.len(), 6);
    ///
    /// let drained_entries: TypedProjVec<i64> = proj_set.drain(0..0).collect();
    ///
    /// assert_eq!(proj_set.len(), 6);
    /// assert_eq!(drained_entries.len(), 0);
    ///
    /// let expected_set_entries: TypedProjVec<i64> = TypedProjVec::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    /// let result_set_entries: TypedProjVec<i64> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result_set_entries, expected_set_entries);
    ///
    /// let expected_drained_entries: TypedProjVec<i64> = TypedProjVec::from([]);
    ///
    /// assert_eq!(drained_entries.as_slice(), expected_drained_entries.as_slice());
    /// ```
    #[track_caller]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
    where
        R: ops::RangeBounds<usize>,
    {
        Drain::new(self.inner.drain(range))
    }

    /// Splits a type-projected index set into two type-projected index sets at the given index.
    ///
    /// This method returns a newly allocated type-projected index set consisting of every entry
    /// from the original type-projected index set in the storage range `[at, len)`. The original
    /// type-projected index set will consist of the entries in the range `[0, at)` with its
    /// capacity unchanged.
    ///
    /// # Panics
    ///
    /// This method panics if `at > self.len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<i64> = TypedProjIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// assert_eq!(proj_set.len(), 6);
    ///
    /// let old_capacity = proj_set.capacity();
    /// let proj_split_set = proj_set.split_off(4);
    ///
    /// assert_eq!(proj_set.len(), 4);
    /// assert_eq!(proj_set.capacity(), old_capacity);
    ///
    /// let expected_proj_set_entries: TypedProjVec<i64> = TypedProjVec::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    /// ]);
    /// let result_proj_set_entries: TypedProjVec<i64> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result_proj_set_entries, expected_proj_set_entries);
    ///
    /// assert_eq!(proj_split_set.len(), 2);
    ///
    /// let expected_split_set_entries: TypedProjVec<i64> = TypedProjVec::from([5_i64, 6_i64]);
    /// let result_split_set_entries: TypedProjVec<i64> = proj_split_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result_split_set_entries, expected_split_set_entries);
    /// ```
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

    /// Attempts to reserve capacity for **at least** `additional` more elements to be inserted
    /// in the given index set.
    ///
    /// The collection may reserve more space to speculatively avoid frequent reallocations.
    /// After calling this method, the capacity will be greater than or equal to
    /// `self.len() + additional` if it returns. This method does nothing if the collection
    /// capacity is already sufficient. This method preserves the contents even if a panic occurs.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions occurs:
    ///
    /// * If the capacity of the index set overflows.
    /// * If the allocator reports a failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// proj_set.reserve(10);
    ///
    /// assert!(proj_set.capacity() >= proj_set.len() + 10);
    ///
    /// let old_capacity = proj_set.capacity();
    /// proj_set.extend([7_i32, 8_i32, 9_i32, 10_i32]);
    ///
    /// assert_eq!(proj_set.capacity(), old_capacity);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    /// Attempts to reserve capacity for **at least** `additional` more elements to be inserted
    /// in the given index set.
    ///
    /// Unlike [`reserve`], this will not deliberately over-allocate to speculatively avoid
    /// frequent allocations. After calling `reserve_exact`, the capacity of `self` will be greater
    /// than or equal to `self.len() + additional`. This method does nothing if the capacity is
    /// already sufficient.
    ///
    /// [`reserve`]: TypedProjIndexSet::reserve
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions occurs:
    ///
    /// * If the capacity of the index set overflows.
    /// * If the allocator reports a failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// proj_set.reserve_exact(10);
    ///
    /// assert!(proj_set.capacity() >= proj_set.len() + 10);
    ///
    /// let old_capacity = proj_set.capacity();
    /// proj_set.extend([7_i32, 8_i32, 9_i32, 10_i32]);
    ///
    /// assert_eq!(proj_set.capacity(), old_capacity);
    /// ```
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional);
    }

    /// Attempts to reserve capacity for **at least** `additional` more elements to be inserted
    /// in the given index set.
    ///
    /// The collection may reserve more space to speculatively avoid frequent reallocations.
    /// After calling this method, the capacity will be greater than or equal to
    /// `self.len() + additional` if it returns `Ok(())`. This method does nothing if the
    /// collection capacity is already sufficient. This method preserves the contents even if an
    /// error occurs.
    ///
    /// # Errors
    ///
    /// This method returns an error if the capacity overflows, or the allocator reports a failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// let result = proj_set.try_reserve(10);
    ///
    /// assert!(result.is_ok());
    /// assert!(proj_set.capacity() >= proj_set.len() + 10);
    ///
    /// let old_capacity = proj_set.capacity();
    /// proj_set.extend([7_i32, 8_i32, 9_i32, 10_i32]);
    ///
    /// assert_eq!(proj_set.capacity(), old_capacity);
    /// ```
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve(additional)
    }

    /// Attempts to reserve capacity for **at least** `additional` more elements to be inserted
    /// in the given index set.
    ///
    /// Unlike [`try_reserve`], this will not deliberately over-allocate to speculatively avoid
    /// frequent allocations. After calling `reserve_exact`, the capacity of `self` will be greater
    /// than or equal to `self.len() + additional`. This method does nothing if the capacity is
    /// already sufficient.
    ///
    /// [`try_reserve`]: TypedProjIndexSet::try_reserve
    ///
    /// # Errors
    ///
    /// This method returns an error if the capacity overflows, or the allocator reports a failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// let result = proj_set.try_reserve_exact(10);
    ///
    /// assert!(result.is_ok());
    /// assert!(proj_set.capacity() >= proj_set.len() + 10);
    ///
    /// let old_capacity = proj_set.capacity();
    /// proj_set.extend([7_i32, 8_i32, 9_i32, 10_i32]);
    ///
    /// assert_eq!(proj_set.capacity(), old_capacity);
    /// ```
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    /// Shrinks the capacity of the index set as much as possible.
    ///
    /// The behavior of this method depends on the allocator, which may either shrink the
    /// index set in place or reallocate. The resulting index set might still have some excess
    /// capacity, just as is the case for [`with_capacity`]. See [`Allocator::shrink`] for more
    /// details.
    ///
    /// [`with_capacity`]: TypedProjIndexSet::with_capacity
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set = TypedProjIndexSet::with_capacity(10);
    /// proj_set.extend([1_i32, 2_i32, 3_i32]);
    ///
    /// assert!(proj_set.capacity() >= 10);
    ///
    /// proj_set.shrink_to_fit();
    ///
    /// assert!(proj_set.capacity() >= 3);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit();
    }

    /// Shrinks the capacity of the index set to a lower bound.
    ///
    /// The behavior of this method depends on the allocator, which may either shrink the
    /// index set in place or reallocate. The resulting index set might still have some excess
    /// capacity, just as is the case for [`with_capacity`]. See [`Allocator::shrink`] for more
    /// details.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied capacity `min_capacity`. In particular, after calling this method,
    /// the capacity of `self` satisfies
    ///
    /// ```text
    /// self.capacity() >= max(self.len(), min_capacity).
    /// ```
    ///
    /// If the current capacity of the index set is less than the lower bound, the method does
    /// nothing.
    ///
    /// [`with_capacity`]: TypedProjIndexSet::with_capacity
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set = TypedProjIndexSet::with_capacity(10);
    /// proj_set.extend([1_i32, 2_i32, 3_i32]);
    ///
    /// assert!(proj_set.capacity() >= 10);
    ///
    /// proj_set.shrink_to(4);
    ///
    /// assert!(proj_set.capacity() >= 4);
    ///
    /// proj_set.shrink_to(0);
    ///
    /// assert!(proj_set.capacity() >= 3);
    /// ```
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to(min_capacity);
    }
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Inserts a new entry into the index set.
    ///
    /// This method behaves as follows:
    ///
    /// * If the equivalent value already exists in the index set, this method returns `false`. The
    ///   entry retains its position in the storage order of the index set.
    /// * If the entry with the equivalent value does not exist in the set, it is appended to the
    ///   end of the set, so the resulting entry is in last place in the storage order, and the
    ///   method returns `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<isize> = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize]);
    ///
    /// let result = proj_set.insert(isize::MAX);
    ///
    /// assert_eq!(result, true);
    ///
    /// let result = proj_set.insert(2_isize);
    ///
    /// assert_eq!(result, false);
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value, ()).is_none()
    }

    /// Inserts a new entry into the index set, returning the storage index of the old entry, if it
    /// exists.
    ///
    /// This method behaves as follows:
    ///
    /// * If the equivalent value already exists in the index set, this method returns the storage
    ///   index of the value as `(index, false)`. The entry retains its position in the storage
    ///   order of the index set.
    /// * If the entry with the equivalent value does not exist in the set, it is appended to the
    ///   end of the set, so the resulting entry is in last place in the storage order, and the
    ///   method returns `(index, true)`, where `index` is the index of the last entry in the set
    ///   in storage order.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<isize> = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize]);
    ///
    /// let result = proj_set.insert_full(isize::MAX);
    ///
    /// assert_eq!(result, (3, true));
    ///
    /// let result = proj_set.insert_full(2_isize);
    ///
    /// assert_eq!(result, (1, false));
    /// ```
    pub fn insert_full(&mut self, value: T) -> (usize, bool) {
        let (index, existing) = self.inner.insert_full(value, ());

        (index, existing.is_none())
    }

    /// Inserts a new entry in the index set at its ordered position among sorted values.
    ///
    /// An index set is in **sorted order by value** if it satisfies the following property: let
    /// `e1` and `e2` be entries in `self`. The `e1.value() <= e2.value()` if and only if
    /// `e1.index() <= e2.index()`. More precisely, given the index set `self`
    ///
    /// ```text
    /// forall e1, e2 in self. e1.index() <= e2.index() <-> e1.value() <= e2.value()
    /// ```
    ///
    /// or equivalently over values
    ///
    /// ```text
    /// forall i1, i2 in [0, self.len()). forall v1, v2 :: T.
    /// (i1, v1), (i2, v2) in self --> i1 <= i2 <-> v1 <= v2.
    /// ```
    ///
    /// Otherwise, the index set is in **unsorted order by value**, or is **unsorted** for short.
    ///
    /// This means that an index set is in sorted order if the total ordering of the values in the
    /// set matches the storage order of the entries in the set. The values are **sorted** if the
    /// index set is in sorted order, and **unsorted** otherwise.
    ///
    /// This method is equivalent to finding the position with [`binary_search_keys`], then either
    /// updating it or calling [`insert_before`] for a new value.
    ///
    /// This method behaves as follows:
    ///
    /// * If the index set is in sorted order and contains the sorted value `value`, this method
    ///   returns `(index, false)`, where `index` is the storage index of the sorted value.
    /// * If the index set is in sorted order and does not contain the sorted value `value`, this
    ///   method inserts the new entry at the sorted position, returns `(index, true)`, where
    ///   `index` is the storage index of the sorted value.
    /// * If the existing values are **not** sorted order, then the insertion index is unspecified.
    ///
    /// Instead of repeating calls to `insert_sorted`, it may be faster to call batched [`insert`]
    /// or [`extend`] and only call [`sort_keys`] or [`sort_unstable_keys`] once.
    ///
    /// [`binary_search_keys`]: TypedProjIndexSet::binary_search_keys
    /// [`insert_before`]: TypedProjIndexSet::insert_before
    /// [`insert`]: TypedProjIndexSet::insert
    /// [`extend`]: TypedProjIndexSet::extend
    /// [`sort_keys`]: TypedProjIndexSet::sort_keys
    /// [`sort_unstable_keys`]: TypedProjIndexSet::sort_unstable_keys
    ///
    /// # Examples
    ///
    /// Calling this method on an index set with a set of sorted values yields the index of the
    /// entry in the underlying storage.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<isize> = TypedProjIndexSet::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    /// ]);
    /// let result = proj_set.insert_sorted(5_isize);
    ///
    /// // The set is sorted, so the index returned is the storage index in the set.
    /// assert_eq!(result, (4, false));
    ///
    /// assert_eq!(proj_set.get(&5_isize), Some(&5_isize));
    /// ```
    ///
    /// Calling this method on an index set with a set of unsorted value yields a meaningless
    /// result for the insertion index.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<isize> = TypedProjIndexSet::from([
    ///     7_isize,
    ///     4_isize,
    ///     2_isize,
    ///     5_isize,
    ///     6_isize,
    ///     1_isize,
    ///     3_isize,
    /// ]);
    /// let result = proj_set.insert_sorted(5_isize);
    ///
    /// // The set is unsorted, so the index returned by the method is meaningless.
    /// assert_ne!(result, (4, false));
    ///
    /// assert_eq!(proj_set.get(&5_isize), Some(&5_isize));
    /// ```
    pub fn insert_sorted(&mut self, value: T) -> (usize, bool)
    where
        T: Ord,
    {
        let (index, existing) = self.inner.insert_sorted(value, ());
        (index, existing.is_none())
    }

    /// Inserts an entry into a type-projected index set before the entry at the given index, or at
    /// the end of the index set.
    ///
    /// The index `index` must be in bounds. The index `index` is **in bounds** provided that
    /// `index` is in `[0, self.len()]`. Otherwise, the index `index` is **out of bounds**.
    ///
    /// This method behaves as follows:
    ///
    /// * If an equivalent value to the value `value` exists in the index set, let `current_index`
    ///   be the storage index of the entry with the equivalent value to `value`.
    ///   - If `index > current_index`, this method moves the entry at `current_index` to
    ///     `index - 1`, shifts each entry in `(current_index, index - 1]` down one index in the
    ///     storage of the index set, then returns `(index - 1, false)`.
    ///   - If `index < current_index`, this method moves the entry at `current_index` to `index`,
    ///     shifts each entry in `[index, current_index)` up one index in the storage for the index
    ///     set, then returns `(index, false)`.
    ///   - If `index == current_index`, this method returns `(index, false)`. No entries are moved
    ///     around in this case.
    /// * If an equivalent value to the value `value` does not exist in the index set, the new entry
    ///   is inserted exactly at the index `index`, every element in `[index, self.len())` is
    ///   shifted up one index, and the method returns `(index, true)`. When `index == self.len()`,
    ///   the interval `[index, self.len()] == [self.len(), self.len())` is empty, so no shifting
    ///   occurs.
    ///
    /// # Panics
    ///
    /// This method panics if the index `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// Inserting an existing value `value` where `index > self.get_index_of(value)`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<char> = TypedProjIndexSet::from([
    ///     'a',
    ///     '*',
    ///     'c',
    ///     'd',
    ///     'e',
    ///     'f',
    ///     'g',
    /// ]);
    /// let removed = proj_set.insert_before(5, '*');
    /// let expected: TypedProjVec<char> = TypedProjVec::from([
    ///     'a',
    ///     'c',
    ///     'd',
    ///     'e',
    ///     '*',
    ///     'f',
    ///     'g',
    /// ]);
    /// let result: TypedProjVec<char> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, (4, false));
    /// ```
    ///
    /// Inserting an existing value `value` where `index < self.get_index_of(value)`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<char> = TypedProjIndexSet::from([
    ///     'a',
    ///     'b',
    ///     'c',
    ///     'd',
    ///     'e',
    ///     '*',
    ///     'g',
    /// ]);
    /// let removed = proj_set.insert_before(2, '*');
    /// let expected: TypedProjVec<char> = TypedProjVec::from([
    ///     'a',
    ///     'b',
    ///     '*',
    ///     'c',
    ///     'd',
    ///     'e',
    ///     'g',
    /// ]);
    /// let result: TypedProjVec<char> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, (2, false));
    /// ```
    ///
    /// Inserting an existing value `value` where `index == self.get_index_of(value)`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<char> = TypedProjIndexSet::from([
    ///     'a',
    ///     'b',
    ///     'c',
    ///     '*',
    ///     'e',
    ///     'f',
    ///     'g',
    /// ]);
    /// let removed = proj_set.insert_before(3, '*');
    /// let expected: TypedProjVec<char> = TypedProjVec::from([
    ///     'a',
    ///     'b',
    ///     'c',
    ///     '*',
    ///     'e',
    ///     'f',
    ///     'g',
    /// ]);
    /// let result: TypedProjVec<char> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, (3, false));
    /// ```
    ///
    /// Inserting a value `value` that does not exist in the index set at an index `index`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<char> = TypedProjIndexSet::from([
    ///     'a',
    ///     'b',
    ///     'c',
    ///     'd',
    ///     'e',
    ///     'f',
    ///     'g',
    /// ]);
    /// let removed = proj_set.insert_before(3, '*');
    /// let expected: TypedProjVec<char> = TypedProjVec::from([
    ///     'a',
    ///     'b',
    ///     'c',
    ///     '*',
    ///     'd',
    ///     'e',
    ///     'f',
    ///     'g',
    /// ]);
    /// let result: TypedProjVec<char> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, (3, true));
    /// ```
    #[track_caller]
    pub fn insert_before(&mut self, index: usize, value: T) -> (usize, bool) {
        let (index, existing) = self.inner.insert_before(index, value, ());
        (index, existing.is_none())
    }

    /// Inserts an entry into a type-projected index set at the given storage index.
    ///
    /// The index `index` must be in bounds. The index `index` is **in bounds** provided that one
    /// of the following conditions holds:
    ///
    /// * If an entry with a value equivalent to the value `value` exists in the index set, and
    ///   `index` is in `[0, self.len())`.
    /// * If an entry with a value equivalent to the value `value` does not exist in the index set,
    ///   and index is in `[0, self.len()]`.
    ///
    /// Otherwise, the index `index` is **out of bounds**.
    ///
    /// This method behaves as follows:
    ///
    /// * If an equivalent value already exists in the set, let `current_index` be the storage
    ///   index of the entry with value equivalent to `value`.
    ///   - If `index < current_index`, every entry in range `[index, current_index)` is shifted up
    ///     one entry in the storage order, the current entry is moved from `current_index` to
    ///     `index`, and the method returns `(index, false)`.
    ///   - If `index > current_index`, every entry in range `(current_index, index]` is shifted
    ///     down one entry in the storage order, the current entry is moved from `current_index` to
    ///     `index`, and the method returns `(index, false)`.
    ///   - If `index == current_index`, no shifting occurs, and the method returns
    ///     `(index, false)`.
    /// * If an equivalent value does not exist in the index set, the new entry is inserted at the
    ///   storage index `index`, and each entry in the range `[index, self.len())` is shifted
    ///   up one index, and the method returns `(index, true)`.
    ///
    /// Note that an existing entry **cannot** be moved to the index `self.len()`.
    ///
    /// # Panics
    ///
    /// This method panics if the index `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// Shift inserting an entry that **does not** exist with index `index < self.len()`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<isize> = TypedProjIndexSet::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    /// ]);
    /// let inserted = proj_set.shift_insert(3, isize::MAX);
    /// let expected: TypedProjVec<isize> = TypedProjVec::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     isize::MAX,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    /// ]);
    /// let result: TypedProjVec<isize> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert!(inserted);
    /// ```
    ///
    /// Shift inserting an entry that **does not** exist with index `index == self.len()`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<isize> = TypedProjIndexSet::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    /// ]);
    /// let inserted = proj_set.shift_insert(proj_set.len(), isize::MAX);
    /// let expected: TypedProjVec<isize> = TypedProjVec::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    ///     isize::MAX,
    /// ]);
    /// let result: TypedProjVec<isize> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert!(inserted);
    /// ```
    ///
    /// Shift inserting an entry that **does** exist with index `index < self.len()`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<isize> = TypedProjIndexSet::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    /// ]);
    /// let inserted = proj_set.shift_insert(3, 6_isize);
    /// let expected: TypedProjVec<isize> = TypedProjVec::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     6_isize,
    ///     4_isize,
    ///     5_isize,
    ///     7_isize,
    /// ]);
    /// let result: TypedProjVec<isize> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert!(!inserted);
    /// ```
    #[track_caller]
    pub fn shift_insert(&mut self, index: usize, value: T) -> bool {
        self.inner.shift_insert(index, value, ()).is_none()
    }

    /// Adds a new value to the index set, and replaces the existing value equal to the given one,
    /// if it exists, and returns the value of the existing one.
    ///
    /// This method does not change the storage order of the other elements in the set.
    ///
    /// # Examples
    ///
    /// Replacing a value where two different string values are equal up to letter case.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// struct CaseInsensitiveString(String);
    ///
    /// impl PartialEq for CaseInsensitiveString {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         self.0.eq_ignore_ascii_case(&other.0)
    ///     }
    /// }
    /// #
    /// # impl Eq for CaseInsensitiveString {}
    /// #
    /// # impl Hash for CaseInsensitiveString {
    /// #     fn hash<H: Hasher>(&self, state: &mut H) {
    /// #        for byte in self.0.bytes() {
    /// #            state.write_u8(byte.to_ascii_lowercase());
    /// #        }
    /// #    }
    /// # }
    /// #
    ///
    /// let mut proj_set = TypedProjIndexSet::from([
    ///     CaseInsensitiveString(String::from("foo")),
    ///     CaseInsensitiveString(String::from("bar")),
    ///     CaseInsensitiveString(String::from("baz")),
    /// ]);
    ///
    /// let expected = Some(String::from("bar"));
    /// let result: Option<String> = {
    ///     let _result = proj_set.replace(CaseInsensitiveString(String::from("BAR")));
    ///     _result.map(|s| s.0)
    /// };
    ///
    /// assert_eq!(result, expected);
    ///
    /// let expected_entries = TypedProjVec::from([
    ///     String::from("foo"),
    ///     String::from("BAR"),
    ///     String::from("baz"),
    /// ]);
    /// let result_entries: TypedProjVec<String> = proj_set
    ///     .iter()
    ///     .map(|s| s.0.clone())
    ///     .collect();
    ///
    /// assert_eq!(result_entries, expected_entries);
    /// ```
    pub fn replace(&mut self, value: T) -> Option<T> {
        self.replace_full(value).1
    }

    /// Adds a new value to the index set, and replaces the existing value equal to the given one,
    /// if it exists, and returns the storage index and value of the existing one.
    ///
    /// This method does not change the storage order of the other elements in the set.
    ///
    /// # Examples
    ///
    /// Replacing a value where two different string values are equal up to letter case.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// struct CaseInsensitiveString(String);
    ///
    /// impl PartialEq for CaseInsensitiveString {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         self.0.eq_ignore_ascii_case(&other.0)
    ///     }
    /// }
    /// #
    /// # impl Eq for CaseInsensitiveString {}
    /// #
    /// # impl Hash for CaseInsensitiveString {
    /// #     fn hash<H: Hasher>(&self, state: &mut H) {
    /// #        for byte in self.0.bytes() {
    /// #            state.write_u8(byte.to_ascii_lowercase());
    /// #        }
    /// #    }
    /// # }
    /// #
    ///
    /// let mut proj_set = TypedProjIndexSet::from([
    ///     CaseInsensitiveString(String::from("foo")),
    ///     CaseInsensitiveString(String::from("bar")),
    ///     CaseInsensitiveString(String::from("baz")),
    /// ]);
    ///
    /// let expected = (1, Some(String::from("bar")));
    /// let result: (usize, Option<String>) = {
    ///     let (i, _result) = proj_set.replace_full(CaseInsensitiveString(String::from("BAR")));
    ///     (i, _result.map(|s| s.0))
    /// };
    ///
    /// assert_eq!(result, expected);
    ///
    /// let expected_entries = TypedProjVec::from([
    ///     String::from("foo"),
    ///     String::from("BAR"),
    ///     String::from("baz"),
    /// ]);
    /// let result_entries: TypedProjVec<String> = proj_set
    ///     .iter()
    ///     .map(|s| s.0.clone())
    ///     .collect();
    ///
    /// assert_eq!(result_entries, expected_entries);
    /// ```
    pub fn replace_full(&mut self, value: T) -> (usize, Option<T>) {
        match self.inner.replace_full(value, ()) {
            (i, Some((replaced, ()))) => (i, Some(replaced)),
            (i, None) => (i, None),
        }
    }

    /// Return an iterator over the values in the set-theoretic difference of two index sets.
    ///
    /// This iterator behaves as follows. Let `self` and `other` be index sets. Let `v` be a value
    /// produced by the iterator. Then `v` satisfies `(v in self) && (not (v in other))`. More
    /// informally, this iterator produces values that are in `self`, but not in `other`.
    ///
    /// This iterator produces values in the same order that they appear in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set1 = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// let proj_set2 = TypedProjIndexSet::from([2_i32, 4_i32, 6_i32, 7_i32, 8_i32]);
    ///
    /// let expected = TypedProjIndexSet::from([1_i32, 3_i32, 5_i32]);
    /// let result: TypedProjIndexSet<i32> = proj_set1
    ///     .difference(&proj_set2)
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.as_slice(), expected.as_slice());
    /// ```
    pub fn difference<'a, S2>(&'a self, other: &'a TypedProjIndexSet<T, S2, A>) -> Difference<'a, T, S2, A>
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Difference::new(self, other)
    }

    /// Return an iterator over the values in the set-theoretic symmetric difference of two index
    /// sets.
    ///
    /// This iterator behaves as follows. Let `self` and `other` be index sets. Let `v` be a value
    /// produced by the iterator. Then `v` satisfies
    ///
    /// ```text
    /// (v in self) && (not (v in other)) || (not (v in self)) && (v in other).
    /// ```
    ///
    /// More informally, this iterator produces those elements that are in one set or the other
    /// set, but not both sets.
    ///
    /// The iterator produces the values from `self` storage order, followed by the values from
    /// `other` in their storage order.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set1 = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// let proj_set2 = TypedProjIndexSet::from([2_i32, 4_i32, 6_i32, 7_i32, 8_i32]);
    ///
    /// let expected = TypedProjIndexSet::from([1_i32, 3_i32, 5_i32, 7_i32, 8_i32]);
    /// let result: TypedProjIndexSet<i32> = proj_set1
    ///     .symmetric_difference(&proj_set2)
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.as_slice(), expected.as_slice());
    /// ```
    pub fn symmetric_difference<'a, S2>(
        &'a self,
        other: &'a TypedProjIndexSet<T, S2, A>,
    ) -> SymmetricDifference<'a, T, S, S2, A>
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        SymmetricDifference::new(self, other)
    }

    /// Return an iterator over the values in the set-theoretic intersection of two index sets.
    ///
    /// This iterator behaves as follows. Let `self` and `other` be index sets. Let `v` be a value
    /// produced by the iterator. Then `v` satisfies `(v in self) && (v in other)`. More
    /// informally, this iterator produces those elements that are in both sets, and none of the
    /// elements that are only in one set.
    ///
    /// This iterator produces values in the order that they appear in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set1 = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// let proj_set2 = TypedProjIndexSet::from([2_i32, 4_i32, 6_i32, 7_i32, 8_i32]);
    ///
    /// let expected = TypedProjIndexSet::from([2_i32, 4_i32, 6_i32]);
    /// let result: TypedProjIndexSet<i32> = proj_set1
    ///     .intersection(&proj_set2)
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.as_slice(), expected.as_slice());
    /// ```
    pub fn intersection<'a, S2>(&'a self, other: &'a TypedProjIndexSet<T, S2, A>) -> Intersection<'a, T, S2, A>
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Intersection::new(self, other)
    }

    /// Return an iterator over the values in the set-theoretic union of two index sets.
    ///
    /// This iterator behaves as follows. Let `self` and `other` be index sets. Let `v` be a value
    /// produced by the iterator. Then `v` satisfies `(v in self) || (v in other)`. More
    /// informally, this iterator produces every value in `self` and `other` exactly once.
    ///
    /// This iterator produces values in the same order as their storage order in `self`, followed
    /// by the storage order of the values unique to `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set1 = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// let proj_set2 = TypedProjIndexSet::from([2_i32, 4_i32, 6_i32, 7_i32, 8_i32]);
    ///
    /// let expected = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32, 8_i32]);
    /// let result: TypedProjIndexSet<i32> = proj_set1
    ///     .union(&proj_set2)
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.as_slice(), expected.as_slice());
    /// ```
    pub fn union<'a, S2>(&'a self, other: &'a TypedProjIndexSet<T, S2, A>) -> Union<'a, T, S, A>
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Union::new(self, other)
    }

    /// Creates a splicing iterator that replaces the specified storage range in the type-projected
    /// index set with the given `replace_with` iterator and yields the removed items. The argument
    /// `replace_with` does not need to be the same length as `range`.
    ///
    /// The `range` argument is removed even if the `Splice` iterator is not consumed before it is
    /// dropped.
    ///
    /// It is unspecified how many elements are removed from the type-projected index set
    /// if the `Splice` value is leaked.
    ///
    /// The input iterator `replace_with` is only consumed when the `Splice` value is dropped.
    /// If a key from the iterator matches an existing entry in the set (i.e. outside the range
    /// `range`), then the value will be updated in that position. Otherwise, the new entry will be
    /// inserted in the replaced `range`.
    ///
    /// # Panics
    ///
    /// This method panics if the starting point is greater than the end point or if the end point
    /// is greater than the length of the index set.
    ///
    /// # Examples
    ///
    /// Splicing entries into an index set.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set = TypedProjIndexSet::from(["foo", "bar", "baz", "quux"]);
    /// let new = ["garply", "corge", "grault"];
    /// let expected = TypedProjVec::from(["foo", "garply", "corge", "grault", "quux"]);
    /// let expected_removed = TypedProjVec::from(["bar", "baz"]);
    /// let removed: TypedProjVec<&str> = proj_set.splice(1..3, new).collect();
    /// let result: TypedProjVec<&str> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, expected_removed);
    /// ```
    ///
    /// Using `splice` to insert new items into an index set efficiently at a specific position
    /// indicated by an empty range.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<&str> = TypedProjIndexSet::from(["foo", "grault"]);
    /// let new = ["bar", "baz", "quux"];
    /// let expected = TypedProjVec::from(["foo", "bar", "baz", "quux", "grault"]);
    /// let expected_removed = TypedProjVec::from([]);
    /// let removed: TypedProjVec<&str> = proj_set.splice(1..1, new).collect();
    /// let result: TypedProjVec<&str> = proj_set
    ///     .iter()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, expected_removed);
    /// ```
    #[track_caller]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, T, S, A>
    where
        R: ops::RangeBounds<usize>,
        A: any::Any + alloc::Allocator + Clone,
        I: IntoIterator<Item = T>,
    {
        Splice::new(self, range, replace_with.into_iter())
    }

    /// Moves all entries from `other` into `self`, leaving `other` empty.
    ///
    /// This is equivalent to calling [`insert`] for each entry from `other` in order, which means
    /// that for keys that already exist in `self`, their value is updated in the current position.
    ///
    /// [`insert`]: TypedProjIndexSet::insert
    ///
    /// # Formal Properties
    ///
    /// Let `set1` and `set2` be index sets, `set1_before` be the state of `set1` before this
    /// method is called, `set2_before` be the state of `set2` before this method is called,
    /// `set1_after` be the state of `set1` after this method completes, and `set2_after` be the
    /// state of `set2` after this method completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { true }
    /// set1.append(set2)
    /// {
    ///     set1_after.len() ≤ set1_before.len() + set2_before.len()
    ///     ∧ set2_after.len() = 0
    ///     ∧ (∀ v ∈ set2_before. v ∈ set1_before ⇒ v ∈ set1_after)
    ///     ∧ (∀ v ∈ set2_before. v ∉ set1_before ⇒ v ∈ set1_after)
    ///     ∧ (∀ v ∈ set2_before. v ∉ set2_after)
    ///     ∧ (∀ i ∈ [0, set1_before.len()). set1_after[i] = set1_before[i])
    ///     ∧ (∀ j1, j2 ∈ [0, set2_before.len()).
    ///          ((set2_before[j1] ∉ set1_before) ∧ (set2_before[j2] ∉ set1_before) ∧ (j1 < j2))
    ///          ⇒ (∃ i1, i2 ∈ [set1_before.len(), set1_after.len()).
    ///               i1 < i2
    ///               ∧ set1_after[i1] = set2_before[j1]
    ///               ∧ set1_after[i2] = set2_before[j2]
    ///          )
    ///     )
    /// }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Examples
    ///
    /// Appending one index set to another when they have no overlapping values.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set1 = TypedProjIndexSet::from(["foo", "bar", "baz", "quux"]);
    /// let mut proj_set2 = TypedProjIndexSet::from(["garply", "corge", "grault"]);
    ///
    /// assert_eq!(proj_set1.len(), 4);
    /// assert_eq!(proj_set2.len(), 3);
    ///
    /// proj_set1.append(&mut proj_set2);
    ///
    /// assert_eq!(proj_set1.len(), 7);
    /// assert_eq!(proj_set2.len(), 0);
    ///
    /// let expected = ["foo", "bar", "baz", "quux", "garply", "corge", "grault"];
    /// let result = TypedProjVec::from_iter(proj_set1.iter().cloned());
    ///
    /// assert_eq!(result.as_slice(), expected.as_slice());
    /// ```
    ///
    /// Appending one index set to another when they have overlapping values.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set1 = TypedProjIndexSet::from(["foo", "bar", "baz", "quux"]);
    /// let mut proj_set2 = TypedProjIndexSet::from(["garply", "corge", "grault", "baz"]);
    ///
    /// assert_eq!(proj_set1.len(), 4);
    /// assert_eq!(proj_set2.len(), 4);
    ///
    /// proj_set1.append(&mut proj_set2);
    ///
    /// assert_eq!(proj_set1.len(), 7);
    /// assert_eq!(proj_set2.len(), 0);
    ///
    /// let expected = ["foo", "bar", "baz", "quux", "garply", "corge", "grault"];
    /// let result = TypedProjVec::from_iter(proj_set1.iter().cloned());
    ///
    /// assert_eq!(result.as_slice(), expected.as_slice());
    /// ```
    pub fn append<S2, A2>(&mut self, other: &mut TypedProjIndexSet<T, S2, A2>)
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
        A2: any::Any + alloc::Allocator + Send + Sync,
    {
        self.inner.append(&mut other.inner);
    }
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Determines whether a given lookup value exists in the index set.
    ///
    /// This method returns `true` if the equivalent value to `value` exists in `self`. This method
    /// returns `false` if the equivalent value to `value` does not exist in `self`.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with values of type `T`. Let `v :: T` be an value of type `T`. We
    /// say that `set` **contains** a value `v :: T`, or that `v` is an **entry of** `set` if the
    /// following holds:
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// This method satisfies the following:
    ///
    /// ```text
    /// ∀ v :: V. set.contains(v) ⇔ (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v.
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set = TypedProjIndexSet::from([1_usize, 2_usize, 3_usize]);
    ///
    /// assert!(proj_set.contains(&1_usize));
    /// assert!(proj_set.contains(&2_usize));
    /// assert!(proj_set.contains(&3_usize));
    /// assert!(!proj_set.contains(&4_usize));
    /// assert!(!proj_set.contains(&usize::MAX));
    /// ```
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.contains_key(value)
    }

    /// Returns a reference to the value corresponding equivalent to the given lookup value, if it
    /// exists in the index set.
    ///
    /// This method returns `Some(&eq_value)` where `eq_value` is the value stored in `self`
    /// equivalent to the value `value`, if such a value exists in `self`. This method returns
    /// `None` if a value equivalent to `value` does not exist in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set = TypedProjIndexSet::from([1_usize, 2_usize, 3_usize]);
    ///
    /// assert_eq!(proj_set.get(&1_usize), Some(&1_usize));
    /// assert_eq!(proj_set.get(&2_usize), Some(&2_usize));
    /// assert_eq!(proj_set.get(&3_usize), Some(&3_usize));
    /// assert_eq!(proj_set.get(&4_usize), None);
    /// assert_eq!(proj_set.get(&usize::MAX), None);
    /// ```
    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.get_key_value(value).map(|(x, &())| x)
    }

    /// Returns the storage index and a reference to the value of the entry with the equivalent
    /// value to the lookup value, if it exists in the index set.
    ///
    /// This method returns `Some((index, &eq_value))` where `index` is the storage index of the
    /// entry, `eq_value` is the equivalent value to the lookup value `value` stored in the set, if
    /// the entry exists in `self`. This method returns `None` if the equivalent value to `value`
    /// does not exist in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set = TypedProjIndexSet::from([1_usize, 2_usize, 3_usize]);
    ///
    /// assert_eq!(proj_set.get_full(&1_usize), Some((0, &1_usize)));
    /// assert_eq!(proj_set.get_full(&2_usize), Some((1, &2_usize)));
    /// assert_eq!(proj_set.get_full(&3_usize), Some((2, &3_usize)));
    /// assert_eq!(proj_set.get_full(&4_usize), None);
    /// assert_eq!(proj_set.get_full(&usize::MAX), None);
    /// ```
    pub fn get_full<Q>(&self, value: &Q) -> Option<(usize, &T)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.get_full(value).map(|(i, x, &())| (i, x))
    }

    /// Returns the storage index of the equivalent value to the given lookup value, if it exists
    /// in the index set.
    ///
    /// This method returns `Some(index)`, where `index` is the storage index of the equivalent
    /// value to `value`, if the equivalent value exists in `self`. This method returns `None` if
    /// the equivalent value to `value` does not exist in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set = TypedProjIndexSet::from([1_usize, 2_usize, 3_usize]);
    ///
    /// assert_eq!(proj_set.get_index_of(&1_usize), Some(0));
    /// assert_eq!(proj_set.get_index_of(&2_usize), Some(1));
    /// assert_eq!(proj_set.get_index_of(&3_usize), Some(2));
    /// assert_eq!(proj_set.get_index_of(&4_usize), None);
    /// assert_eq!(proj_set.get_index_of(&usize::MAX), None);
    /// ```
    pub fn get_index_of<Q>(&self, value: &Q) -> Option<usize>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.get_index_of(value)
    }

    /// Removes an entry from a type-projected index set, moving the last entry in storage order in
    /// the collection to the index where the removed entry occupies the collection.
    ///
    /// This method behaves with respect to the lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves the last entry in the collection to the slot
    ///   at `index`, leaving the rest of the entries in place. If `index == self.len() - 1`, it
    ///   removes the entry from the end of the collection with no reordering of the remaining
    ///   entries in the collection. The method then returns `true`, indicating that it removed the
    ///   equivalent value to `value` from the collection.
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `false`.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, map.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// The **last entry** in the set `set` when `set` is non-empty is defined by
    ///
    /// ```text
    /// last(set) := set[set.len() - 1].
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.swap_remove(value)
    /// {
    ///     result = true
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (set_after[index(set_before, value)] = last(set_before)
    ///        ∧ (∀ v ∈ set_after. v ≠ last(set_before) ∧ (v ≠ value ⇒ set_after[v] = set_before[v])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// set.swap_remove(value)
    /// { result = false ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Examples
    ///
    /// Showing how swap removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_remove(&isize::MAX);
    ///     assert_eq!(result, expected);
    ///     assert!(removed);
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_remove(&3_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, true);
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, isize::MAX, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_remove(&2_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, true);
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([isize::MAX, 2_isize, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_remove(&1_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, true);
    /// }
    /// ```
    pub fn swap_remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.swap_remove(value).is_some()
    }

    /// Removes an entry from a type-projected index set, shifting every successive entry in the
    /// collection in storage order down one index to fill where the removed entry occupies the
    /// collection.
    ///
    /// This method behaves with respect to the lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves every successive entry in the collection to
    ///   the entry at storage index `index` down one unit. Every entry preceding the entry at
    ///   index `index` remains in the same location.  The method returns `true`, which indicates
    ///   that the entry with equivalent value to `value` was removed from the index set.
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `false`.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.shift_remove(value)
    /// {
    ///     result = true
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (let i = index(set_before, value);
    ///        (∀ j ∈ [0, i). set_after[j] = set_before[j])
    ///        ∧ (∀ j ∈ [i, set_after.len()). set_after[j] = set_before[j + 1])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// map.shift_remove(value)
    /// { result = false ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Examples
    ///
    /// Showing how shift removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_remove(&isize::MAX);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, true);
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_remove(&3_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, true);
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 3_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_remove(&2_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, true);
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([2_isize, 3_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_remove(&1_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, true);
    /// }
    /// ```
    pub fn shift_remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.shift_remove(value).is_some()
    }

    /// Removes an entry from a type-projected index set, moving the last entry in storage order
    /// in the collection to the index where the removed entry occupies the collection.
    ///
    /// This method behaves with respect to the lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves the last entry in the collection to the slot
    ///   at `index`, leaving the rest of the entries in place. If `index == self.len() - 1`, it
    ///   removes the entry from end of the collection with no reordering of the remaining entries
    ///   in the collection. The method then returns `Some(eq_value)`, where `eq_value` is the
    ///   equivalent value to the value `value` stored in the index set.
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `None`.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, map.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// The **last entry** in the set `set` when `set` is non-empty is defined by
    ///
    /// ```text
    /// last(set) := set[set.len() - 1].
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.swap_take(value)
    /// {
    ///     result = Some(set_before[value])
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (set_after[index(set_before, value)] = last(set_before)
    ///        ∧ (∀ v ∈ set_after. (v ≠ last(set_before) ∧ v ≠ value) ⇒ set_after[v] = set_before[v])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// set.swap_take(value)
    /// { result = None ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Examples
    ///
    /// Showing how swap removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set = TypedProjIndexSet::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     isize::MAX,
    /// ]);
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_take(&isize::MAX);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some(isize::MAX));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_take(&3_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some(3_isize));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, isize::MAX, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_take(&2_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some(2_isize));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([isize::MAX, 2_isize, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_take(&1_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some(1_isize));
    /// }
    /// ```
    pub fn swap_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.swap_remove_entry(value).map(|(x, ())| x)
    }

    /// Removes an entry from a type-projected index set, shifting every successive entry in the
    /// collection in storage order down one index to fill where the removed entry occupies the
    /// collection.
    ///
    /// This method behaves with respect to the lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves every successive entry in the collection to
    ///   the entry at storage index `index` down one unit. Every entry preceding the entry at
    ///   index `index` remains in the same location. The method returns `Some(eq_value)`, where
    ///   `eq_value` is the equivalent value to the value `value` stored in the index set.
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `None`.
    ///
    /// In particular, the method acts like a [`pop`] when the last value in the collection is
    /// shift-removed, because the sub-collection of successor entries in the entry storage is
    /// empty.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.shift_take(value)
    /// {
    ///     result = Some(set_before[value])
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (let i = index(set_before, value);
    ///        (∀ j ∈ [0, i). set_after[j] = set_before[j])
    ///        ∧ (∀ j ∈ [i, set_after.len()). set_after[j] = set_before[j + 1])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// set.shift_take(value)
    /// { result = None ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Examples
    ///
    /// Showing how shift removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_take(&isize::MAX);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some(isize::MAX));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_take(&3_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some(3_isize));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 3_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_take(&2_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some(2_isize));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([2_isize, 3_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_take(&1_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some(1_isize));
    /// }
    /// ```
    ///
    /// [`pop`]: TypedProjIndexSet::pop
    pub fn shift_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.shift_remove_entry(value).map(|(x, ())| x)
    }

    /// Removes an entry from a type-projected index set, moving the last entry in storage order in
    /// the collection to the index where the removed entry occupies the collection.
    ///
    /// This method behaves with respect to lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves the last entry in the collection to the slot
    ///   at `index`, leaving the rest of the entries in place. If `index == self.len() - 1`, it
    ///   removes the entry from end of the collection with no reordering of the remaining entries
    ///   in the collection. The method then returns `Some((index, eq_value))`, where `eq_value` is
    ///   the equivalent value to `value` stored in the index set..
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `None`.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.swap_remove_full(value)
    /// {
    ///     result = Some((index(set_before, value), set_before[index(set_before, value)]))
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (set_after[index(set_before, value)] = last(set_before)
    ///        ∧ (∀ v ∈ set_after. v ≠ last(set_before) ∧ v ≠ value ⇒ set_after[v] = set_before[v])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// set.swap_remove_full(value)
    /// { result = None ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Examples
    ///
    /// Showing how swap removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_remove_full(&isize::MAX);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some((3, isize::MAX)));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_remove_full(&3_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some((2, 3_isize)));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, isize::MAX, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_remove_full(&2_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some((1, 2_isize)));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([isize::MAX, 2_isize, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.swap_remove_full(&1_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some((0, 1_isize)));
    /// }
    /// ```
    pub fn swap_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.swap_remove_full(value).map(|(i, x, ())| (i, x))
    }

    /// Removes an entry from a type-projected index set, shifting every successive entry in the
    /// collection in storage order down one index to fill where the removed entry occupies the
    /// collection.
    ///
    /// This method behaves with respect to the lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves every successive entry in the collection to
    ///   the entry at storage index `index` down one unit. Every entry preceding the entry at
    ///   index `index` remains in the same location. The method returns `Some((index, eq_value))`,
    ///   where `eq_value` is the equivalent value to the value `value` stored in the index set.
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `None`.
    ///
    /// In particular, the method acts like a [`pop`] when the last value in the collection is
    /// shift-removed, because the sub-collection of successor entries in the entry storage is
    /// empty.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.shift_remove_full(value)
    /// {
    ///     result = Some((index(set_before, value), set_before[index(set_before, value)]))
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (let i = index(set_before, value);
    ///        (∀ j ∈ [0, i). set_after[j] = set_before[j])
    ///        ∧ (∀ j ∈ [i, set_after.len()). set_after[j] = set_before[j + 1])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// set.shift_remove_full(value)
    /// { result = None ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Examples
    ///
    /// Showing how shift removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, 3_isize]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_remove_full(&isize::MAX);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some((3, isize::MAX)));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 2_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_remove_full(&3_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some((2, 3_isize)));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([1_isize, 3_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_remove_full(&2_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some((1, 2_isize)));
    /// }
    /// {
    ///     let expected = TypedProjIndexSet::from([2_isize, 3_isize, isize::MAX]);
    ///     let mut result = proj_set.clone();
    ///     let removed = result.shift_remove_full(&1_isize);
    ///     assert_eq!(result, expected);
    ///     assert_eq!(removed, Some((0, 1_isize)));
    /// }
    /// ```
    ///
    /// [`pop`]: TypedProjIndexSet::pop
    pub fn shift_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.shift_remove_full(value).map(|(i, x, ())| (i, x))
    }
}

impl<T, S, A> Clone for TypedProjIndexSet<T, S, A>
where
    T: any::Any + Clone,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        TypedProjIndexSet {
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, other: &Self) {
        self.inner.clone_from(&other.inner);
    }
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[doc(alias = "pop_last")]
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop().map(|(x, ())| x)
    }

    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.inner.retain(move |x, &mut ()| keep(x))
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.inner.sort_keys()
    }

    pub fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&T, &T) -> cmp::Ordering,
    {
        self.inner.sort_by(move |a, _, b, _| cmp(a, b));
    }

    pub fn sorted_by<F>(self, mut cmp: F) -> IntoIter<T, A>
    where
        F: FnMut(&T, &T) -> cmp::Ordering,
    {
        let mut entries = self.into_entries();
        entries.sort_by(move |a, b| cmp(a.key_ref(), b.key_ref()));

        IntoIter::new(map_inner::IntoIter::new(entries))
    }

    pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.inner.sort_unstable_keys()
    }

    pub fn sort_unstable_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&T, &T) -> cmp::Ordering,
    {
        self.inner.sort_unstable_by(move |a, _, b, _| cmp(a, b))
    }

    pub fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<T, A>
    where
        F: FnMut(&T, &T) -> cmp::Ordering,
    {
        let mut entries = self.inner.into_entries();
        entries.sort_unstable_by(move |a, b| cmp(a.key_ref(), b.key_ref()));

        IntoIter::new(map_inner::IntoIter::new(entries))
    }

    pub fn sort_by_cached_key<K, F>(&mut self, mut sort_key: F)
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        self.with_entries(move |entries| {
            entries.sort_by_cached_key(move |a| sort_key(a.key_ref()));
        });
    }

    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.as_slice().binary_search(x)
    }

    #[inline]
    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> cmp::Ordering,
    {
        self.as_slice().binary_search_by(f)
    }

    #[inline]
    pub fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        self.as_slice().binary_search_by_key(b, f)
    }

    #[must_use]
    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        self.as_slice().partition_point(pred)
    }

    pub fn reverse(&mut self) {
        self.inner.reverse()
    }

    pub fn as_slice(&self) -> &Slice<T> {
        Slice::from_slice(self.as_entries())
    }

    pub fn into_boxed_slice(self) -> Box<Slice<T>, TypedProjAlloc<A>> {
        let boxed_map = self.inner.into_boxed_slice();

        Slice::from_boxed_slice(boxed_map)
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        self.as_entries().get_index(index).map(|tuple| tuple.0)
    }

    pub fn get_range<R>(&self, range: R) -> Option<&Slice<T>>
    where
        R: ops::RangeBounds<usize>,
    {
        let entries = self.as_entries();
        let range = range_ops::try_simplify_range(range, entries.len())?;
        entries.get_range(range).map(Slice::from_slice)
    }

    pub fn first(&self) -> Option<&T> {
        self.as_entries().first().map(|tuple| tuple.0)
    }

    pub fn last(&self) -> Option<&T> {
        self.as_entries().last().map(|tuple| tuple.0)
    }

    pub fn swap_remove_index(&mut self, index: usize) -> Option<T> {
        self.inner.swap_remove_index(index).map(|(x, ())| x)
    }

    pub fn shift_remove_index(&mut self, index: usize) -> Option<T> {
        self.inner.shift_remove_index(index).map(|(x, ())| x)
    }

    #[track_caller]
    pub fn move_index(&mut self, from: usize, to: usize) {
        self.inner.move_index(from, to)
    }

    #[track_caller]
    pub fn swap_indices(&mut self, a: usize, b: usize) {
        self.inner.swap_indices(a, b)
    }
}

impl<T, S, A> ops::Index<usize> for TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_index(index).unwrap_or_else(|| {
            panic!(
                "index out of bounds: the len is {len} but the index is {index}",
                len = self.len()
            );
        })
    }
}

macro_rules! impl_index_for_index_set {
    ($($range:ty),*) => {$(
        impl<T, S, A> ops::Index<$range> for TypedProjIndexSet<T, S, A>
        where
            T: any::Any,
            S: any::Any + hash::BuildHasher + Send + Sync,
            S::Hasher: any::Any + hash::Hasher + Send + Sync,
            A: any::Any + alloc::Allocator + Send + Sync,
        {
            type Output = Slice<T>;

            fn index(&self, range: $range) -> &Self::Output {
                Slice::from_slice(&self.as_entries()[range])
            }
        }
    )*}
}

impl_index_for_index_set!(
    ops::Range<usize>,
    ops::RangeFrom<usize>,
    ops::RangeFull,
    ops::RangeInclusive<usize>,
    ops::RangeTo<usize>,
    ops::RangeToInclusive<usize>,
    (ops::Bound<usize>, ops::Bound<usize>)
);

impl<T, S, A> fmt::Debug for TypedProjIndexSet<T, S, A>
where
    T: any::Any + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_set().entries(self.iter()).finish()
    }
}

impl<T, S> FromIterator<T> for TypedProjIndexSet<T, S, alloc::Global>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self {
        let iterator = iterable.into_iter();
        let (low, _) = iterator.size_hint();
        let mut set = Self::with_capacity_and_hasher_in(low, S::default(), alloc::Global::default());
        set.extend(iterator);

        set
    }
}

impl<T, const N: usize> From<[T; N]> for TypedProjIndexSet<T, hash::RandomState, alloc::Global>
where
    T: any::Any + hash::Hash + Eq,
{
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
    }
}

impl<T, S, A> Extend<T> for TypedProjIndexSet<T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iterable: I) {
        let iterator = iterable.into_iter().map(|x| (x, ()));
        self.inner.extend(iterator);
    }
}

impl<'a, T, S, A> Extend<&'a T> for TypedProjIndexSet<T, S, A>
where
    T: any::Any + hash::Hash + Eq + Copy + 'a,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iterable: I) {
        let iterator = iterable.into_iter().copied();
        self.extend(iterator);
    }
}

impl<T, S, A> Default for TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Self { inner: map_inner::TypedProjIndexMapInner::default(), }
    }
}

impl<T, S1, S2, A1, A2> PartialEq<TypedProjIndexSet<T, S2, A2>> for TypedProjIndexSet<T, S1, A1>
where
    T: any::Any + hash::Hash + Eq,
    S1: any::Any + hash::BuildHasher + Send + Sync,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A1: any::Any + alloc::Allocator + Send + Sync,
    A2: any::Any + alloc::Allocator + Send + Sync,
{
    fn eq(&self, other: &TypedProjIndexSet<T, S2, A2>) -> bool {
        self.len() == other.len() && self.is_subset(other)
    }
}

impl<T, S, A> Eq for TypedProjIndexSet<T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub fn is_disjoint<S2, A2>(&self, other: &TypedProjIndexSet<T, S2, A2>) -> bool
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
        A2: any::Any + alloc::Allocator + Send + Sync,
    {
        if self.len() <= other.len() {
            self.iter().all(move |value| !other.contains(value))
        } else {
            other.iter().all(move |value| !self.contains(value))
        }
    }

    pub fn is_subset<S2, A2>(&self, other: &TypedProjIndexSet<T, S2, A2>) -> bool
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
        A2: any::Any + alloc::Allocator + Send + Sync,
    {
        self.len() <= other.len() && self.iter().all(move |value| other.contains(value))
    }

    pub fn is_superset<S2, A2>(&self, other: &TypedProjIndexSet<T, S2, A2>) -> bool
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
        A2: any::Any + alloc::Allocator + Send + Sync,
    {
        other.is_subset(self)
    }
}

impl<T, S1, S2, A> ops::BitAnd<&TypedProjIndexSet<T, S2, A>> for &TypedProjIndexSet<T, S1, A>
where
    T: any::Any + hash::Hash + Eq + Clone,
    S1: any::Any + hash::BuildHasher + Send + Sync + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    type Output = TypedProjIndexSet<T, S1, A>;

    fn bitand(self, other: &TypedProjIndexSet<T, S2, A>) -> Self::Output {
        let iterator = self.intersection(other).cloned();
        let capacity = Ord::max(self.len(), other.len());
        let mut set = TypedProjIndexSet::with_capacity_and_hasher_in(capacity, S1::default(), A::default());
        set.extend(iterator);

        set
    }
}

impl<T, S1, S2, A> ops::BitOr<&TypedProjIndexSet<T, S2, A>> for &TypedProjIndexSet<T, S1, A>
where
    T: any::Any + hash::Hash + Eq + Clone,
    S1: any::Any + hash::BuildHasher + Send + Sync + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    type Output = TypedProjIndexSet<T, S1, A>;

    fn bitor(self, other: &TypedProjIndexSet<T, S2, A>) -> Self::Output {
        let iterator = self.union(other).cloned();
        let capacity = Ord::max(self.len(), other.len());
        let mut set = TypedProjIndexSet::with_capacity_and_hasher_in(capacity, S1::default(), A::default());
        set.extend(iterator);

        set
    }
}

impl<T, S1, S2, A> ops::BitXor<&TypedProjIndexSet<T, S2, A>> for &TypedProjIndexSet<T, S1, A>
where
    T: any::Any + hash::Hash + Eq + Clone,
    S1: any::Any + hash::BuildHasher + Send + Sync + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    type Output = TypedProjIndexSet<T, S1, A>;

    fn bitxor(self, other: &TypedProjIndexSet<T, S2, A>) -> Self::Output {
        let iterator = self.symmetric_difference(other).cloned();
        let capacity = Ord::max(self.len(), other.len());
        let mut set = TypedProjIndexSet::with_capacity_and_hasher_in(capacity, S1::default(), A::default());
        set.extend(iterator);

        set
    }
}

impl<T, S1, S2, A> ops::Sub<&TypedProjIndexSet<T, S2, A>> for &TypedProjIndexSet<T, S1, A>
where
    T: any::Any + hash::Hash + Eq + Clone,
    S1: any::Any + hash::BuildHasher + Send + Sync + Default,
    S1::Hasher: any::Any + hash::Hasher + Send + Sync,
    S2: any::Any + hash::BuildHasher + Send + Sync,
    S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    type Output = TypedProjIndexSet<T, S1, A>;

    fn sub(self, other: &TypedProjIndexSet<T, S2, A>) -> Self::Output {
        let iterator = self.difference(other).cloned();
        let capacity = Ord::max(self.len(), other.len());
        let mut set = TypedProjIndexSet::with_capacity_and_hasher_in(capacity, S1::default(), A::default());
        set.extend(iterator);

        set
    }
}

impl<'a, T, S, A> IntoIterator for &'a TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T, S, A> IntoIterator for TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Item = T;
    type IntoIter = IntoIter<T, A>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(map_inner::IntoIter::new(self.inner.into_entries()))
    }
}

/// A type-erased hash set where the order of the entries inside the set is independent of the
/// hash values of the elements.
///
/// The interface to this hash set tracks closely with the standard library's [`HashSet`] interface.
/// One feature this hash set has that the standard library one does not is that it is generic over
/// the choice of memory allocator. This type supports type-erasure of generic parameters. The main
/// difference is that a `TypedProjIndexSet` can be converted to an `OpaqueIndexSet` in constant
/// **O(1)** time, hiding its value type, hash builder type, and allocator type, at runtime.
///
/// # Ordering
///
/// The values are stored in the set in their insertion order, rather than by their
/// hash value, provided no removal method have been called on an entry in the set. In particular,
/// inserting a new value into the set does not change the **storage order** of the other values in
/// the set.
///
/// # Indices
///
/// The values are stored in a packed range with no holes in the range `[0, self.len())`.
/// Thus, one can always use the [`get_index_of`] or [`get_index`] methods to interact with values
/// inside the set by their storage index instead of their value.
///
/// # Type Erasure And Type Projection
///
/// This allows for more flexible and dynamic data handling, especially when working with
/// collections of unknown or dynamic types. Type-erasable collections allow for more efficient
/// runtime dynamic typing, since one has more control over the memory layout of the collection,
/// even for erased types. Some applications of this include implementing heterogeneous data
/// structures, plugin systems, and managing foreign function interface data. There are two data
/// types that are dual to each other: [`TypedProjIndexSet`] and [`OpaqueIndexSet`].
///
/// By laying out both data types identically, we can project the underlying types in **O(1)** time,
/// and erase the underlying types in **O(1)** time, though the conversion is often zero-cost.
///
/// # See Also
///
/// - [`TypedProjIndexSet`]: the type-projected counterpart of [`OpaqueIndexSet`].
///
/// # Examples
///
/// Basic usage of a type-erased index set.
///
/// ```
/// # #![feature(allocator_api)]
/// # use opaque_index_map::OpaqueIndexSet;
/// # use std::hash::RandomState;
/// # use std::alloc::Global;
/// #
/// let mut party: OpaqueIndexSet = OpaqueIndexSet::from([
///     String::from("cloud"),
///     String::from("tifa"),
///     String::from("aerith"),
///     String::from("barret"),
///     String::from("cid"),
///     String::from("vincent"),
///     String::from("yuffie"),
///     String::from("red xiii"),
///     String::from("cait sith"),
/// ]);
///
/// assert!(party.has_value_type::<String>());
/// assert!(party.has_build_hasher_type::<RandomState>());
/// assert!(party.has_allocator_type::<Global>());
///
/// assert_eq!(party.get::<_, String, RandomState, Global>("cloud"),     Some(&String::from("cloud")));
/// assert_eq!(party.get::<_, String, RandomState, Global>("tifa"),      Some(&String::from("tifa")));
/// assert_eq!(party.get::<_, String, RandomState, Global>("aerith"),    Some(&String::from("aerith")));
/// assert_eq!(party.get::<_, String, RandomState, Global>("barret"),    Some(&String::from("barret")));
/// assert_eq!(party.get::<_, String, RandomState, Global>("cid"),       Some(&String::from("cid")));
/// assert_eq!(party.get::<_, String, RandomState, Global>("vincent"),   Some(&String::from("vincent")));
/// assert_eq!(party.get::<_, String, RandomState, Global>("yuffie"),    Some(&String::from("yuffie")));
/// assert_eq!(party.get::<_, String, RandomState, Global>("red xiii"),  Some(&String::from("red xiii")));
/// assert_eq!(party.get::<_ ,String, RandomState, Global>("cait sith"), Some(&String::from("cait sith")));
///
/// assert!(!party.contains::<_, String, RandomState, Global>("sephiroth"));
/// assert!(!party.contains::<_, String, RandomState, Global>("jenova"));
/// assert!(!party.contains::<_, String, RandomState, Global>("emerald weapon"));
///
/// // Elements of an index set are stored in their insertion order, independent of their values.
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("cloud"),     Some(0));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("tifa"),      Some(1));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("aerith"),    Some(2));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("barret"),    Some(3));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("cid"),       Some(4));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("vincent"),   Some(5));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("yuffie"),    Some(6));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("red xiii"),  Some(7));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("cait sith"), Some(8));
///
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("sephiroth"),      None);
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("jenova"),         None);
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("emerald weapon"), None);
///
/// party.insert::<String, RandomState, Global>(String::from("sephiroth"));
///
/// assert!(party.contains::<_, String, RandomState, Global>("sephiroth"));
///
/// // Elements of an index set are stored in their insertion order, independent of their values.
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("cloud"),     Some(0));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("tifa"),      Some(1));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("aerith"),    Some(2));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("barret"),    Some(3));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("cid"),       Some(4));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("vincent"),   Some(5));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("yuffie"),    Some(6));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("red xiii"),  Some(7));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("cait sith"), Some(8));
/// assert_eq!(party.get_index_of::<_, String, RandomState, Global>("sephiroth"), Some(9));
///
/// assert_eq!(party.get::<_, String, RandomState, Global>("sephiroth"), Some(&String::from("sephiroth")));
///
/// party.shift_remove::<_, String, RandomState, Global>("sephiroth");
///
/// assert!(!party.contains::<_, String, RandomState, Global>("sephiroth"));
/// ```
#[repr(transparent)]
pub struct OpaqueIndexSet {
    inner: map_inner::OpaqueIndexMapInner,
}

impl OpaqueIndexSet {
    /// Returns the [`TypeId`] of the values contained in the type-erased index set.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set: OpaqueIndexSet = OpaqueIndexSet::new::<isize>();
    ///
    /// assert_eq!(opaque_set.value_type_id(), TypeId::of::<isize>());
    /// ```
    #[inline]
    pub const fn value_type_id(&self) -> any::TypeId {
        self.inner.key_type_id()
    }

    /// Returns the [`TypeId`] of the hash builder for the type-erased index set.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set: OpaqueIndexSet = OpaqueIndexSet::new::<isize>();
    ///
    /// assert_eq!(opaque_set.build_hasher_type_id(), TypeId::of::<RandomState>());
    /// ```
    #[inline]
    pub const fn build_hasher_type_id(&self) -> any::TypeId {
        self.inner.build_hasher_type_id()
    }

    /// Returns the [`TypeId`] of the memory allocator for the type-erased index set.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set: OpaqueIndexSet = OpaqueIndexSet::new::<isize>();
    ///
    /// assert_eq!(opaque_set.allocator_type_id(), TypeId::of::<Global>());
    /// ```
    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }
}

impl OpaqueIndexSet {
    /// Determines whether the type-erased index set has the given value type.
    ///
    /// Returns `true` if `self` has the specified value type. Returns `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::new::<isize>();
    ///
    /// assert!(opaque_set.has_value_type::<isize>());
    /// ```
    #[inline]
    pub fn has_value_type<T>(&self) -> bool
    where
        T: any::Any,
    {
        self.inner.key_type_id() == any::TypeId::of::<T>()
    }

    /// Determines whether the type-erased index set has the given hash builder type.
    ///
    /// Returns `true` if `self` has the specified hash builder type. Returns `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::new::<isize>();
    ///
    /// assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// ```
    #[inline]
    pub fn has_build_hasher_type<S>(&self) -> bool
    where
        S: any::Any,
    {
        self.inner.build_hasher_type_id() == any::TypeId::of::<S>()
    }

    /// Determines whether the type-erased index set has the given memory allocator type.
    ///
    /// Returns `true` if `self` has the specified memory allocator type. Returns `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::new::<isize>();
    ///
    /// assert!(opaque_set.has_allocator_type::<Global>());
    /// ```
    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.inner.allocator_type_id() == any::TypeId::of::<A>()
    }

    /// Assert the concrete types underlying a type-erased data type.
    ///
    /// This method's main use case is ensuring the type safety of an operation before projecting
    /// into the type-projected counterpart of the type-erased index set.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    #[inline]
    #[track_caller]
    fn assert_type_safety<T, S, A>(&self)
    where
        T: any::Any,
        S: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(st: &str, type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("{:?} type mismatch. Need `{:?}`, got `{:?}`", st, type_id_self, type_id_other);
        }

        if !self.has_value_type::<T>() {
            type_check_failed("Value", self.inner.key_type_id(), any::TypeId::of::<T>());
        }

        if !self.has_build_hasher_type::<S>() {
            type_check_failed("BuildHasher", self.inner.build_hasher_type_id(), any::TypeId::of::<S>());
        }

        if !self.has_allocator_type::<A>() {
            type_check_failed("Allocator", self.inner.allocator_type_id(), any::TypeId::of::<A>());
        }
    }
}

impl OpaqueIndexSet {
    /// Projects the type-erased index set reference into a type-projected index set reference.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::{OpaqueIndexSet, TypedProjIndexSet};
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::with_hasher_in::<usize, RandomState, Global>(
    ///     RandomState::new(),
    ///     Global
    /// );
    /// #
    /// # assert!(opaque_set.has_value_type::<usize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let proj_set: &TypedProjIndexSet<usize, RandomState, Global> = opaque_set.as_proj::<usize, RandomState, Global>();
    /// ```
    #[inline]
    pub fn as_proj<T, S, A>(&self) -> &TypedProjIndexSet<T, S, A>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<T, S, A>();

        unsafe { &*(self as *const OpaqueIndexSet as *const TypedProjIndexSet<T, S, A>) }
    }

    /// Projects the mutable type-erased index set reference into a mutable type-projected
    /// index set reference.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::{OpaqueIndexSet, TypedProjIndexSet};
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::with_hasher_in::<usize, RandomState, Global>(
    ///     RandomState::new(),
    ///     Global
    /// );
    /// #
    /// # assert!(opaque_set.has_value_type::<usize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let proj_set: &mut TypedProjIndexSet<usize, RandomState, Global> = opaque_set.as_proj_mut::<usize, RandomState, Global>();
    /// ```
    #[inline]
    pub fn as_proj_mut<T, S, A>(&mut self) -> &mut TypedProjIndexSet<T, S, A>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<T, S, A>();

        unsafe { &mut *(self as *mut OpaqueIndexSet as *mut TypedProjIndexSet<T, S, A>) }
    }

    /// Projects the type-erased index set value into a type-projected index set value.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::{OpaqueIndexSet, TypedProjIndexSet};
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::with_hasher_in::<usize, RandomState, Global>(
    ///     RandomState::new(),
    ///     Global
    /// );
    /// #
    /// # assert!(opaque_set.has_value_type::<usize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = opaque_set.into_proj::<usize, RandomState, Global>();
    /// ```
    #[inline]
    pub fn into_proj<T, S, A>(self) -> TypedProjIndexSet<T, S, A>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<T, S, A>();

        TypedProjIndexSet {
            inner: self.inner.into_proj::<T, (), S, A>(),
        }
    }

    /// Erases the type-projected index set value into a type-erased index set value.
    ///
    /// Unlike the type projection methods [`as_proj`], [`as_proj_mut`], and [`into_proj`], this
    /// method never panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::{OpaqueIndexSet, TypedProjIndexSet};
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_hasher_in(
    ///     RandomState::new(),
    ///     Global
    /// );
    /// let opaque_set: OpaqueIndexSet = OpaqueIndexSet::from_proj(proj_set);
    /// #
    /// # assert!(opaque_set.has_value_type::<usize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// ```
    ///
    /// [`as_proj`]: OpaqueIndexSet::as_proj,
    /// [`as_proj_mut`]: OpaqueIndexSet::as_proj_mut
    /// [`into_proj`]: OpaqueIndexSet::into_proj
    #[inline]
    pub fn from_proj<T, S, A>(proj_self: TypedProjIndexSet<T, S, A>) -> Self
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        Self {
            inner: OpaqueIndexMapInner::from_proj(proj_self.inner),
        }
    }
}

impl OpaqueIndexSet {
    /// Constructs a new index set with the given type-projected hash builder and type-projected
    /// memory allocator.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let proj_build_hasher = TypedProjBuildHasher::new(RandomState::new());
    /// let opaque_set = OpaqueIndexSet::with_hasher_proj_in::<usize, RandomState, Global>(
    ///     proj_build_hasher,
    ///     proj_alloc
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    #[inline]
    pub fn with_hasher_proj_in<T, S, A>(proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, S, A>::with_hasher_proj_in(proj_build_hasher, proj_alloc);

        Self::from_proj(proj_index_set)
    }

    /// Constructs a new index set with the given capacity, type-projected hash builder, and
    /// type-projected memory allocator.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-erased index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let proj_build_hasher = TypedProjBuildHasher::new(RandomState::new());
    /// let opaque_set = OpaqueIndexSet::with_capacity_and_hasher_proj_in::<usize, RandomState, Global>(
    ///     capacity,
    ///     proj_build_hasher,
    ///     proj_alloc
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert!(opaque_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-erased index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let proj_build_hasher = TypedProjBuildHasher::new(RandomState::new());
    /// let opaque_set = OpaqueIndexSet::with_capacity_and_hasher_proj_in::<usize, RandomState, Global>(
    ///     0,
    ///     proj_build_hasher,
    ///     proj_alloc
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher_proj_in<T, S, A>(capacity: usize, proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, S, A>::with_capacity_and_hasher_proj_in(capacity, proj_build_hasher, proj_alloc);

        Self::from_proj(proj_index_set)
    }
}

#[cfg(feature = "std")]
impl OpaqueIndexSet {
    /// Constructs a new index set with the given type-projected memory allocator.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let opaque_set = OpaqueIndexSet::new_proj_in::<usize, Global>(proj_alloc);
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    pub fn new_proj_in<T, A>(proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, hash::RandomState, A>::new_proj_in(proj_alloc);

        Self::from_proj(proj_index_set)
    }

    /// Constructs a new index set with the given capacity and type-projected memory allocator.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-erased index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let opaque_set = OpaqueIndexSet::with_capacity_proj_in::<usize, Global>(
    ///     capacity,
    ///     proj_alloc
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert!(opaque_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-erased index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let opaque_set = OpaqueIndexSet::with_capacity_proj_in::<usize, Global>(
    ///     0,
    ///     proj_alloc
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    pub fn with_capacity_proj_in<T, A>(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, hash::RandomState, A>::with_capacity_proj_in(capacity, proj_alloc);

        Self::from_proj(proj_index_set)
    }
}

impl OpaqueIndexSet {
    /// Constructs a new index set with the given hash builder and memory allocator.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::with_hasher_in::<usize, RandomState, Global>(
    ///     RandomState::new(),
    ///     Global
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    #[inline]
    pub fn with_hasher_in<T, S, A>(build_hasher: S, alloc: A) -> Self
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, S, A>::with_hasher_in(build_hasher, alloc);

        Self::from_proj(proj_index_set)
    }

    /// Constructs a new index set with the given capacity, hash builder, and memory allocator.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-erased index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let opaque_set = OpaqueIndexSet::with_capacity_and_hasher_in::<usize, RandomState, Global>(
    ///     capacity,
    ///     RandomState::new(),
    ///     Global
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert!(opaque_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-erased index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::with_capacity_and_hasher_in::<usize, RandomState, Global>(
    ///     0,
    ///     RandomState::new(),
    ///     Global
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher_in<T, S, A>(capacity: usize, build_hasher: S, alloc: A) -> Self
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, S, A>::with_capacity_and_hasher_in(capacity, build_hasher, alloc);

        Self::from_proj(proj_index_set)
    }
}

impl OpaqueIndexSet {
    /// Constructs a new index set with the given memory allocator.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::new_in::<usize, Global>(Global);
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    pub fn new_in<T, A>(alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, _, A>::new_in(alloc);

        Self::from_proj(proj_index_set)
    }

    /// Constructs a new index set with the given capacity and memory allocator.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-erased index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let opaque_set = OpaqueIndexSet::with_capacity_in::<usize, Global>(
    ///     capacity,
    ///     Global
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert!(opaque_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-erased index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::with_capacity_in::<usize, Global>(
    ///     0,
    ///     Global
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    pub fn with_capacity_in<T, A>(capacity: usize, alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, _, A>::with_capacity_in(capacity, alloc);

        Self::from_proj(proj_index_set)
    }
}

impl OpaqueIndexSet {
    /// Constructs a new index set with the given hash builder.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::with_hasher::<usize, RandomState>(RandomState::new());
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    #[inline]
    pub fn with_hasher<T, S>(build_hasher: S) -> Self
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, S, _>::with_hasher(build_hasher);

        Self::from_proj(proj_index_set)
    }

    /// Constructs a new index set with the given capacity and hash builder.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-erased index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let opaque_set = OpaqueIndexSet::with_capacity_and_hasher::<usize, RandomState>(
    ///     capacity,
    ///     RandomState::new(),
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert!(opaque_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-erased index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::with_capacity_and_hasher::<usize, RandomState>(
    ///     0,
    ///     RandomState::new(),
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    #[inline]
    pub fn with_capacity_and_hasher<T, S>(capacity: usize, build_hasher: S) -> Self
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, S, _>::with_capacity_and_hasher(capacity, build_hasher);

        Self::from_proj(proj_index_set)
    }
}

impl OpaqueIndexSet {
    /// Constructs a new index set.
    ///
    /// This method **does not** allocate memory. In particular, the index set has zero capacity
    /// and will not allocate memory until values are inserted into it. The index set will have
    /// length zero until values are inserted into it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::new::<usize>();
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    #[inline]
    pub fn new<T>() -> Self
    where
        T: any::Any,
    {
        Self::new_in::<T, alloc::Global>(alloc::Global)
    }

    /// Constructs a new index set with the given capacity.
    ///
    /// This method **does** allocate memory if the capacity `capacity` is non-zero. In particular,
    /// the index set has capacity at least `capacity`, and will allocate enough memory to store at
    /// least `capacity` values. The index set will have length zero until values are inserted into
    /// it.
    ///
    /// # Examples
    ///
    /// Creating a type-erased index set with capacity `capacity > 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let opaque_set = OpaqueIndexSet::with_capacity::<usize>(
    ///     capacity,
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert!(opaque_set.capacity() >= capacity);
    /// ```
    ///
    /// Creating a type-erased index set with capacity `capacity == 0`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::with_capacity::<usize>(
    ///     0,
    /// );
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), 0);
    /// ```
    #[inline]
    pub fn with_capacity<T>(capacity: usize) -> Self
    where
        T: any::Any,
    {
        Self::with_capacity_in::<T, alloc::Global>(capacity, alloc::Global)
    }
}

impl OpaqueIndexSet {
    /// Returns the capacity of the type-erased index set.
    ///
    /// The **capacity** of a type-erased index set is the number of values the index set
    /// can hold without reallocating memory.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 32;
    /// let mut opaque_set = OpaqueIndexSet::with_capacity_in::<usize, Global>(
    ///     capacity,
    ///     Global,
    /// );
    ///
    /// assert_eq!(opaque_set.len(), 0);
    /// assert!(opaque_set.capacity() >= capacity);
    ///
    /// for i in 0..capacity {
    ///     opaque_set.insert::<usize, RandomState, Global>(i);
    /// }
    ///
    /// assert_eq!(opaque_set.len(), capacity);
    /// assert!(opaque_set.capacity() >= capacity);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns the length of the type-erased index set.
    ///
    /// The **length** of a type-erased index set is the number of values stored inside it.
    /// The length satisfies the following. Given an index set `set`
    ///
    /// ```text
    /// set.len() ≤ set.capacity().
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let len = 32;
    /// let mut opaque_set = OpaqueIndexSet::with_capacity_in::<usize, Global>(
    ///     len,
    ///     Global,
    /// );
    ///
    /// assert_eq!(opaque_set.len(), 0);
    ///
    /// for i in 0..len {
    ///     opaque_set.insert::<usize, RandomState, Global>(i);
    /// }
    ///
    /// assert_eq!(opaque_set.len(), len);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Determines whether the type-erased index set is empty.
    ///
    /// A type-erased index set is **empty** if it contains no values, i.e. its length is zero.
    /// This method satisfies the following. Given an index set `set`
    ///
    /// ```text
    /// set.is_empty() ⇔ set.len() = 0.
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::TypedProjIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut proj_set: TypedProjIndexSet<usize, RandomState, Global> = TypedProjIndexSet::with_capacity_in(
    ///     1,
    ///     Global,
    /// );
    ///
    /// assert!(proj_set.is_empty());
    ///
    /// proj_set.insert(1);
    ///
    /// assert!(!proj_set.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl OpaqueIndexSet {
    /// Returns a reference to the type-projected hash builder used by the index set.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::new::<usize>();
    /// #
    /// # assert!(opaque_set.has_value_type::<usize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert!(opaque_set.is_empty());
    ///
    /// let build_hasher: &TypedProjBuildHasher<RandomState> = opaque_set.hasher::<usize, RandomState, Global>();
    /// ```
    #[inline]
    pub fn hasher<T, S, A>(&self) -> &TypedProjBuildHasher<S>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.hasher()
    }

    /// Returns a reference to the type-projected memory allocator from the index set.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::new::<usize>();
    /// #
    /// # assert!(opaque_set.has_value_type::<usize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert!(opaque_set.is_empty());
    ///
    /// let alloc: &TypedProjAlloc<Global> = opaque_set.allocator::<usize, RandomState, Global>();
    /// ```
    #[inline]
    pub fn allocator<T, S, A>(&self) -> &TypedProjAlloc<A>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.allocator()
    }
}

impl OpaqueIndexSet {
    /// Returns an iterator over the entries in the index set.
    ///
    /// The iterator returns the entries in their storage order in the index set.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_i32, 2_i32, 3_i32]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i32>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let entries: TypedProjVec<i32> = opaque_set
    ///     .iter::<i32, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(entries.as_slice(), &[1_i32, 2_i32, 3_i32]);
    ///
    /// // The entries come back in storage or insertion order from the index set.
    /// for i in 0..entries.len() {
    ///     let expected = i;
    ///     let result = opaque_set.get_index_of::<_, i32, RandomState, Global>(&entries[i]).unwrap();
    ///     assert_eq!(result, expected);
    /// }
    /// ```
    pub fn iter<T, S, A>(&self) -> Iter<'_, T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.iter()
    }

    /// Removes all the entries from the index set.
    ///
    /// After calling this method, the collection will be empty. This method does not change the
    /// allocated capacity of the type-erased index set.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let capacity = 10;
    /// let mut opaque_set = OpaqueIndexSet::with_capacity::<String>(10);
    /// #
    /// # assert!(opaque_set.has_value_type::<String>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert!(opaque_set.is_empty());
    ///
    /// opaque_set.extend::<_, String, RandomState, Global>([String::from("foo"), String::from("bar"), String::from("baz")]);
    ///
    /// assert!(!opaque_set.is_empty());
    /// assert_eq!(opaque_set.len(), 3);
    ///
    /// let old_capacity = opaque_set.capacity();
    ///
    /// opaque_set.clear::<String, RandomState, Global>();
    ///
    /// assert!(opaque_set.is_empty());
    /// assert_eq!(opaque_set.capacity(), old_capacity);
    /// ```
    pub fn clear<T, S, A>(&mut self)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.clear()
    }

    /// Shortens an index set to the supplied length, dropping the remaining elements.
    ///
    /// This method keeps the entries of `self` in the range `[0, len)`. In particular,
    /// this method drops every entry with storage index in the range `[len, self.len())`.
    /// This method does nothing when `self.len() <= len`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Truncating a type-erased index set when `len < self.len()`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i64>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// opaque_set.truncate::<i64, RandomState, Global>(2);
    ///
    /// assert_eq!(opaque_set.len(), 2);
    ///
    /// let expected = TypedProjVec::from([1_i64, 2_i64]);
    /// let result: TypedProjVec<i64> = opaque_set
    ///     .iter::<i64, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// No truncation occurs when `len == self.len()`
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i64>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// opaque_set.truncate::<i64, RandomState, Global>(6);
    ///
    /// assert_eq!(opaque_set.len(), 6);
    ///
    /// let expected = TypedProjVec::from([1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64]);
    /// let result: TypedProjVec<i64> = opaque_set
    ///     .iter::<i64, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// No truncation occurs when `len > self.len()`
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i64>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// opaque_set.truncate::<i64, RandomState, Global>(7);
    ///
    /// assert_eq!(opaque_set.len(), 6);
    ///
    /// let expected = TypedProjVec::from([1_i64, 2_i64, 3_i64, 4_i64, 5_i64, 6_i64]);
    /// let result: TypedProjVec<i64> = opaque_set
    ///     .iter::<i64, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// Truncating when `len == 0` is equivalent to calling the [`clear`] method.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// opaque_set.truncate::<i64, RandomState, Global>(0);
    ///
    /// assert_eq!(opaque_set.len(), 0);
    ///
    /// let expected = TypedProjVec::from([]);
    /// let result: TypedProjVec<i64> = opaque_set
    ///     .iter::<i64, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// [`clear`]: OpaqueIndexSet::clear
    pub fn truncate<T, S, A>(&mut self, len: usize)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.truncate(len)
    }

    /// Removes the subslice indicated by the given range from the index set,
    /// returning a double-ended iterator over the removed subslice.
    ///
    /// If the iterator is dropped before being fully consumed, it drops the remaining removed
    /// elements. The draining iterator shifts the remaining entries in the index set above the
    /// range down to fill in the removed entries.
    ///
    /// The returned iterator keeps a mutable borrow on the index set to optimize its
    /// implementation.
    ///
    /// # Panics
    ///
    /// This method panics under one of the following conditions:
    ///
    /// * If the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    ///   builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    ///   value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    /// * If the range of the subslice falls outside the bounds of the collection.
    ///   That is, if the starting point of the subslice being removed starts after the end of
    ///   `self`, or if the ending point is larger than the length of the index set.
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped (due to
    /// [`mem::forget`], for example), the index set may have lost and leaked
    /// elements arbitrarily, including elements outside the range.
    ///
    /// # Examples
    ///
    /// Draining part of a type-erased index set.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i64>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert_eq!(opaque_set.len(), 6);
    ///
    /// let drained_entries: TypedProjVec<i64> = opaque_set.drain::<_, i64, RandomState, Global>(2..).collect();
    ///
    /// assert_eq!(opaque_set.len(), 2);
    /// assert_eq!(drained_entries.len(), 4);
    ///
    /// let expected_set_entries = TypedProjVec::from([1_i64, 2_i64]);
    /// let result_set_entries: TypedProjVec<i64> = opaque_set
    ///     .iter::<i64, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result_set_entries, expected_set_entries);
    ///
    /// let expected_drained_entries: TypedProjVec<i64> = TypedProjVec::from([
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// assert_eq!(drained_entries.as_slice(), expected_drained_entries.as_slice());
    /// ```
    ///
    /// Draining an entire type-erased index set.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i64>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert_eq!(opaque_set.len(), 6);
    ///
    /// let drained_entries: TypedProjVec<i64> = opaque_set.drain::<_, i64, RandomState, Global>(..).collect();
    ///
    /// assert_eq!(opaque_set.len(), 0);
    /// assert_eq!(drained_entries.len(), 6);
    ///
    /// let expected_set_entries = TypedProjVec::from([]);
    /// let result_set_entries: TypedProjVec<i64> = opaque_set
    ///     .iter::<i64, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result_set_entries, expected_set_entries);
    ///
    /// let expected_drained_entries: TypedProjVec<i64> = TypedProjVec::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    ///
    /// assert_eq!(drained_entries.as_slice(), expected_drained_entries.as_slice());
    /// ```
    ///
    /// Draining no part of a type-erased index set.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i64>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert_eq!(opaque_set.len(), 6);
    ///
    /// let drained_entries: TypedProjVec<i64> = opaque_set.drain::<_, i64, RandomState, Global>(0..0).collect();
    ///
    /// assert_eq!(opaque_set.len(), 6);
    /// assert_eq!(drained_entries.len(), 0);
    ///
    /// let expected_set_entries: TypedProjVec<i64> = TypedProjVec::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    /// let result_set_entries: TypedProjVec<i64> = opaque_set
    ///     .iter::<i64, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result_set_entries, expected_set_entries);
    ///
    /// let expected_drained_entries: TypedProjVec<i64> = TypedProjVec::from([]);
    ///
    /// assert_eq!(drained_entries.as_slice(), expected_drained_entries.as_slice());
    /// ```
    #[track_caller]
    pub fn drain<R, T, S, A>(&mut self, range: R) -> Drain<'_, T, A>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.drain(range)
    }

    /// Splits a type-erased index set into two type-erased index sets at the given index.
    ///
    /// This method returns a newly allocated type-erased index set consisting of every entry
    /// from the original type-erased index set in the storage range `[at, len)`. The original
    /// type-erased index set will consist of the entries in the range `[0, at)` with its
    /// capacity unchanged.
    ///
    /// # Panics
    ///
    /// This method panics under one of the following conditions:
    ///
    /// * If the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash builder of `self`,
    ///   and the [`TypeId`] of the memory allocator of `self` do not match the value type `T`,
    ///   hash builder type `S`, and allocator type `A`, respectively.
    /// * If `at > self.len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    ///     5_i64,
    ///     6_i64,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i64>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert_eq!(opaque_set.len(), 6);
    ///
    /// let old_capacity = opaque_set.capacity();
    /// let opaque_split_set = opaque_set.split_off::<i64, RandomState, Global>(4);
    ///
    /// assert_eq!(opaque_set.len(), 4);
    /// assert_eq!(opaque_set.capacity(), old_capacity);
    ///
    /// let expected_proj_set_entries: TypedProjVec<i64> = TypedProjVec::from([
    ///     1_i64,
    ///     2_i64,
    ///     3_i64,
    ///     4_i64,
    /// ]);
    /// let result_proj_set_entries: TypedProjVec<i64> = opaque_set
    ///     .iter::<i64, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result_proj_set_entries, expected_proj_set_entries);
    ///
    /// assert_eq!(opaque_split_set.len(), 2);
    ///
    /// let expected_split_set_entries: TypedProjVec<i64> = TypedProjVec::from([5_i64, 6_i64]);
    /// let result_split_set_entries: TypedProjVec<i64> = opaque_split_set
    ///     .iter::<i64, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result_split_set_entries, expected_split_set_entries);
    /// ```
    #[track_caller]
    pub fn split_off<T, S, A>(&mut self, at: usize) -> Self
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();
        let proj_split = proj_self.split_off(at);

        Self::from_proj(proj_split)
    }

    /// Attempts to reserve capacity for **at least** `additional` more elements to be inserted
    /// in the given index set.
    ///
    /// The collection may reserve more space to speculatively avoid frequent reallocations.
    /// After calling this method, the capacity will be greater than or equal to
    /// `self.len() + additional` if it returns. This method does nothing if the collection
    /// capacity is already sufficient. This method preserves the contents even if a panic occurs.
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions occurs:
    ///
    /// * If the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash builder of `self`,
    ///   and the [`TypeId`] of the memory allocator of `self` do not match the value type `T`,
    ///   hash builder type `S`, and allocator type `A`, respectively.
    /// * If the capacity of the index set overflows.
    /// * If the allocator reports a failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i32>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// opaque_set.reserve::<i32, RandomState, Global>(10);
    ///
    /// assert!(opaque_set.capacity() >= opaque_set.len() + 10);
    ///
    /// let old_capacity = opaque_set.capacity();
    /// opaque_set.extend::<_, i32, RandomState, Global>([7_i32, 8_i32, 9_i32, 10_i32]);
    ///
    /// assert_eq!(opaque_set.capacity(), old_capacity);
    /// ```
    pub fn reserve<T, S, A>(&mut self, additional: usize)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.reserve(additional)
    }

    /// Attempts to reserve capacity for **at least** `additional` more elements to be inserted
    /// in the given index set.
    ///
    /// Unlike [`reserve`], this will not deliberately over-allocate to speculatively avoid
    /// frequent allocations. After calling `reserve_exact`, the capacity of `self` will be greater
    /// than or equal to `self.len() + additional`. This method does nothing if the capacity is
    /// already sufficient.
    ///
    /// [`reserve`]: OpaqueIndexSet::reserve
    ///
    /// # Panics
    ///
    /// This method panics if one of the following conditions occurs:
    ///
    /// * If the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash builder of `self`,
    ///   and the [`TypeId`] of the memory allocator of `self` do not match the value type `T`,
    ///   hash builder type `S`, and allocator type `A`, respectively.
    /// * If the capacity of the index set overflows.
    /// * If the allocator reports a failure.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i32>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// opaque_set.reserve_exact::<i32, RandomState, Global>(10);
    ///
    /// assert!(opaque_set.capacity() >= opaque_set.len() + 10);
    ///
    /// let old_capacity = opaque_set.capacity();
    /// opaque_set.extend::<_, i32, RandomState, Global>([7_i32, 8_i32, 9_i32, 10_i32]);
    ///
    /// assert_eq!(opaque_set.capacity(), old_capacity);
    /// ```
    pub fn reserve_exact<T, S, A>(&mut self, additional: usize)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.reserve_exact(additional)
    }

    /// Attempts to reserve capacity for **at least** `additional` more elements to be inserted
    /// in the given index set.
    ///
    /// The collection may reserve more space to speculatively avoid frequent reallocations.
    /// After calling this method, the capacity will be greater than or equal to
    /// `self.len() + additional` if it returns `Ok(())`. This method does nothing if the
    /// collection capacity is already sufficient. This method preserves the contents even if an
    /// error occurs.
    ///
    /// # Errors
    ///
    /// This method returns an error if the capacity overflows, or the allocator reports a failure.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i32>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let result = opaque_set.try_reserve::<i32, RandomState, Global>(10);
    ///
    /// assert!(result.is_ok());
    /// assert!(opaque_set.capacity() >= opaque_set.len() + 10);
    ///
    /// let old_capacity = opaque_set.capacity();
    /// opaque_set.extend::<_, i32, RandomState, Global>([7_i32, 8_i32, 9_i32, 10_i32]);
    ///
    /// assert_eq!(opaque_set.capacity(), old_capacity);
    /// ```
    pub fn try_reserve<T, S, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.try_reserve(additional)
    }

    /// Attempts to reserve capacity for **at least** `additional` more elements to be inserted
    /// in the given index set.
    ///
    /// Unlike [`try_reserve`], this will not deliberately over-allocate to speculatively avoid
    /// frequent allocations. After calling `reserve_exact`, the capacity of `self` will be greater
    /// than or equal to `self.len() + additional`. This method does nothing if the capacity is
    /// already sufficient.
    ///
    /// [`try_reserve`]: OpaqueIndexSet::try_reserve
    ///
    /// # Errors
    ///
    /// This method returns an error if the capacity overflows, or the allocator reports a failure.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// #
    /// # assert!(opaque_set.has_value_type::<i32>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let result = opaque_set.try_reserve_exact::<i32, RandomState, Global>(10);
    ///
    /// assert!(result.is_ok());
    /// assert!(opaque_set.capacity() >= opaque_set.len() + 10);
    ///
    /// let old_capacity = opaque_set.capacity();
    /// opaque_set.extend::<_, i32, RandomState, Global>([7_i32, 8_i32, 9_i32, 10_i32]);
    ///
    /// assert_eq!(opaque_set.capacity(), old_capacity);
    /// ```
    pub fn try_reserve_exact<T, S, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.try_reserve_exact(additional)
    }

    /// Shrinks the capacity of the index set as much as possible.
    ///
    /// The behavior of this method depends on the allocator, which may either shrink the
    /// index set in place or reallocate. The resulting index set might still have some excess
    /// capacity, just as is the case for [`with_capacity`]. See [`Allocator::shrink`] for more
    /// details.
    ///
    /// [`with_capacity`]: OpaqueIndexSet::with_capacity
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::with_capacity::<i32>(10);
    /// #
    /// # assert!(opaque_set.has_value_type::<i32>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// opaque_set.extend::<_, i32, RandomState, Global>([1_i32, 2_i32, 3_i32]);
    ///
    /// assert!(opaque_set.capacity() >= 10);
    ///
    /// opaque_set.shrink_to_fit::<i32, RandomState, Global>();
    ///
    /// assert!(opaque_set.capacity() >= 3);
    /// ```
    pub fn shrink_to_fit<T, S, A>(&mut self)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.shrink_to_fit()
    }

    /// Shrinks the capacity of the index set to a lower bound.
    ///
    /// The behavior of this method depends on the allocator, which may either shrink the
    /// index set in place or reallocate. The resulting index set might still have some excess
    /// capacity, just as is the case for [`with_capacity`]. See [`Allocator::shrink`] for more
    /// details.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied capacity `min_capacity`. In particular, after calling this method,
    /// the capacity of `self` satisfies
    ///
    /// ```text
    /// self.capacity() >= max(self.len(), min_capacity).
    /// ```
    ///
    /// If the current capacity of the index set is less than the lower bound, the method does
    /// nothing.
    ///
    /// [`with_capacity`]: OpaqueIndexSet::with_capacity
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::cmp::Ordering;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::with_capacity::<i32>(10);
    /// #
    /// # assert!(opaque_set.has_value_type::<i32>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// opaque_set.extend::<_, i32, RandomState, Global>([1_i32, 2_i32, 3_i32]);
    ///
    /// assert!(opaque_set.capacity() >= 10);
    ///
    /// opaque_set.shrink_to::<i32, RandomState, Global>(4);
    ///
    /// assert!(opaque_set.capacity() >= 4);
    ///
    /// opaque_set.shrink_to::<i32, RandomState, Global>(0);
    ///
    /// assert!(opaque_set.capacity() >= 3);
    /// ```
    pub fn shrink_to<T, S, A>(&mut self, min_capacity: usize)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.shrink_to(min_capacity)
    }
}

impl OpaqueIndexSet {
    /// Inserts a new entry into the index set.
    ///
    /// This method behaves as follows:
    ///
    /// * If the equivalent value already exists in the index set, this method returns `false`. The
    ///   entry retains its position in the storage order of the index set.
    /// * If the entry with the equivalent value does not exist in the set, it is appended to the
    ///   end of the set, so the resulting entry is in last place in the storage order, and the
    ///   method returns `true`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize]);
    /// #
    /// # assert!(opaque_set.has_value_type::<isize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// let result = opaque_set.insert::<isize, RandomState, Global>(isize::MAX);
    ///
    /// assert_eq!(result, true);
    ///
    /// let result = opaque_set.insert::<isize, RandomState, Global>(2_isize);
    ///
    /// assert_eq!(result, false);
    /// ```
    pub fn insert<T, S, A>(&mut self, value: T) -> bool
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.insert(value)
    }

    /// Inserts a new entry into the index set, returning the storage index of the old entry, if it
    /// exists.
    ///
    /// This method behaves as follows:
    ///
    /// * If the equivalent value already exists in the index set, this method returns the storage
    ///   index of the value as `(index, false)`. The entry retains its position in the storage
    ///   order of the index set.
    /// * If the entry with the equivalent value does not exist in the set, it is appended to the
    ///   end of the set, so the resulting entry is in last place in the storage order, and the
    ///   method returns `(index, true)`, where `index` is the index of the last entry in the set
    ///   in storage order.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize]);
    /// #
    /// # assert!(opaque_set.has_value_type::<isize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// let result = opaque_set.insert_full::<isize, RandomState, Global>(isize::MAX);
    ///
    /// assert_eq!(result, (3, true));
    ///
    /// let result = opaque_set.insert_full::<isize, RandomState, Global>(2_isize);
    ///
    /// assert_eq!(result, (1, false));
    /// ```
    pub fn insert_full<T, S, A>(&mut self, value: T) -> (usize, bool)
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self =  self.as_proj_mut::<T, S, A>();

        proj_self.insert_full(value)
    }

    /// Inserts a new entry in the index set at its ordered position among sorted values.
    ///
    /// An index set is in **sorted order by value** if it satisfies the following property: let
    /// `e1` and `e2` be entries in `self`. Then `e1.value() <= e2.value()` if and only if
    /// `e1.index() <= e2.index()`. More precisely, given the index set `self`
    ///
    /// ```text
    /// forall e1, e2 in self. e1.index() <= e2.index() <-> e1.value() <= e2.value()
    /// ```
    ///
    /// or equivalently over values
    ///
    /// ```text
    /// forall i1, i2 in [0, self.len()). forall v1, v2 :: T.
    /// (i1, v1), (i2, v2) in self --> i1 <= i2 <-> v1 <= v2.
    /// ```
    ///
    /// Otherwise, the index set is in **unsorted order by value**, or is **unsorted** for short.
    ///
    /// This means that an index set is in sorted order if the total ordering of the values in the
    /// set matches the storage order of the entries in the set. The values are **sorted** if the
    /// index set is in sorted order, and **unsorted** otherwise.
    ///
    /// This method is equivalent to finding the position with [`binary_search_keys`], then either
    /// updating it or calling [`insert_before`] for a new value.
    ///
    /// This method behaves as follows:
    ///
    /// * If the index set is in sorted order and contains the sorted value `value`, this method
    ///   returns `(index, false)`, where `index` is the storage index of the sorted value.
    /// * If the index set is in sorted order and does not contain the sorted value `value`, this
    ///   method inserts the new entry at the sorted position, returns `(index, true)`, where
    ///   `index` is the storage index of the sorted value.
    /// * If the existing values are **not** sorted order, then the insertion index is unspecified.
    ///
    /// Instead of repeating calls to `insert_sorted`, it may be faster to call batched [`insert`]
    /// or [`extend`] and only call [`sort_keys`] or [`sort_unstable_keys`] once.
    ///
    /// [`binary_search_keys`]: TypedProjIndexSet::binary_search_keys
    /// [`insert_before`]: TypedProjIndexSet::insert_before
    /// [`insert`]: TypedProjIndexSet::insert
    /// [`extend`]: TypedProjIndexSet::extend
    /// [`sort_keys`]: TypedProjIndexSet::sort_keys
    /// [`sort_unstable_keys`]: TypedProjIndexSet::sort_unstable_keys
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Calling this method on an index set with a set of sorted values yields the index of the
    /// entry in the underlying storage.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<isize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let result = opaque_set.insert_sorted::<isize, RandomState, Global>(5_isize);
    ///
    /// // The set is sorted, so the index returned is the storage index in the set.
    /// assert_eq!(result, (4, false));
    ///
    /// assert_eq!(opaque_set.get::<_, isize, RandomState, Global>(&5_isize), Some(&5_isize));
    /// ```
    ///
    /// Calling this method on an index set with a set of unsorted value yields a meaningless
    /// result for the insertion index.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     7_isize,
    ///     4_isize,
    ///     2_isize,
    ///     5_isize,
    ///     6_isize,
    ///     1_isize,
    ///     3_isize,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<isize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let result = opaque_set.insert_sorted::<isize, RandomState, Global>(5_isize);
    ///
    /// // The set is unsorted, so the index returned by the method is meaningless.
    /// assert_ne!(result, (4, false));
    ///
    /// assert_eq!(opaque_set.get::<_, isize, RandomState, Global>(&5_isize), Some(&5_isize));
    /// ```
    pub fn insert_sorted<T, S, A>(&mut self, value: T) -> (usize, bool)
    where
        T: any::Any + hash::Hash + Eq + Ord,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.insert_sorted(value)
    }

    /// Inserts an entry into a type-erased index set before the entry at the given index, or at
    /// the end of the index set.
    ///
    /// The index `index` must be in bounds. The index `index` is **in bounds** provided that
    /// `index` is in `[0, self.len()]`. Otherwise, the index `index` is **out of bounds**.
    ///
    /// This method behaves as follows:
    ///
    /// * If an equivalent value to the value `value` exists in the index set, let `current_index`
    ///   be the storage index of the entry with the equivalent value to `value`.
    ///   - If `index > current_index`, this method moves the entry at `current_index` to
    ///     `index - 1`, shifts each entry in `(current_index, index - 1]` down one index in the
    ///     storage of the index set, then returns `(index - 1, false)`.
    ///   - If `index < current_index`, this method moves the entry at `current_index` to `index`,
    ///     shifts each entry in `[index, current_index)` up one index in the storage for the index
    ///     set, then returns `(index, false)`.
    ///   - If `index == current_index`, this method returns `(index, false)`. No entries are moved
    ///     around in this case.
    /// * If an equivalent value to the value `value` does not exist in the index set, the new entry
    ///   is inserted exactly at the index `index`, every element in `[index, self.len())` is
    ///   shifted up one index, and the method returns `(index, true)`. When `index == self.len()`,
    ///   the interval `[index, self.len()] == [self.len(), self.len())` is empty, so no shifting
    ///   occurs.
    ///
    /// # Panics
    ///
    /// This method panics under one of the following conditions:
    ///
    /// * If the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash builder of `self`,
    ///   and the [`TypeId`] of the memory allocator of `self` do not match the value type `T`,
    ///   hash builder type `S`, and allocator type `A`, respectively.
    /// * If the index `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// Inserting an existing value `value` where `index > self.get_index_of(value)`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     'a',
    ///     '*',
    ///     'c',
    ///     'd',
    ///     'e',
    ///     'f',
    ///     'g',
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<char>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let removed = opaque_set.insert_before::<char, RandomState, Global>(5, '*');
    /// let expected: TypedProjVec<char> = TypedProjVec::from([
    ///     'a',
    ///     'c',
    ///     'd',
    ///     'e',
    ///     '*',
    ///     'f',
    ///     'g',
    /// ]);
    /// let result: TypedProjVec<char> = opaque_set
    ///     .iter::<char, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, (4, false));
    /// ```
    ///
    /// Inserting an existing value `value` where `index < self.get_index_of(value)`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     'a',
    ///     'b',
    ///     'c',
    ///     'd',
    ///     'e',
    ///     '*',
    ///     'g',
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<char>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let removed = opaque_set.insert_before::<char, RandomState, Global>(2, '*');
    /// let expected: TypedProjVec<char> = TypedProjVec::from([
    ///     'a',
    ///     'b',
    ///     '*',
    ///     'c',
    ///     'd',
    ///     'e',
    ///     'g',
    /// ]);
    /// let result: TypedProjVec<char> = opaque_set
    ///     .iter::<char, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, (2, false));
    /// ```
    ///
    /// Inserting an existing value `value` where `index == self.get_index_of(value)`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     'a',
    ///     'b',
    ///     'c',
    ///     '*',
    ///     'e',
    ///     'f',
    ///     'g',
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<char>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let removed = opaque_set.insert_before::<char, RandomState, Global>(3, '*');
    /// let expected: TypedProjVec<char> = TypedProjVec::from([
    ///     'a',
    ///     'b',
    ///     'c',
    ///     '*',
    ///     'e',
    ///     'f',
    ///     'g',
    /// ]);
    /// let result: TypedProjVec<char> = opaque_set
    ///     .iter::<char, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, (3, false));
    /// ```
    ///
    /// Inserting a value `value` that does not exist in the index set at an index `index`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     'a',
    ///     'b',
    ///     'c',
    ///     'd',
    ///     'e',
    ///     'f',
    ///     'g',
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<char>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let removed = opaque_set.insert_before::<char, RandomState, Global>(3, '*');
    /// let expected: TypedProjVec<char> = TypedProjVec::from([
    ///     'a',
    ///     'b',
    ///     'c',
    ///     '*',
    ///     'd',
    ///     'e',
    ///     'f',
    ///     'g',
    /// ]);
    /// let result: TypedProjVec<char> = opaque_set
    ///     .iter::<char, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, (3, true));
    /// ```
    #[track_caller]
    pub fn insert_before<T, S, A>(&mut self, index: usize, value: T) -> (usize, bool)
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.insert_before(index, value)
    }

    /// Inserts an entry into a type-erased index set at the given storage index.
    ///
    /// The index `index` must be in bounds. The index `index` is **in bounds** provided that one
    /// of the following conditions holds:
    ///
    /// * If an entry with a value equivalent to the value `value` exists in the index set, and
    ///   `index` is in `[0, self.len())`.
    /// * If an entry with a value equivalent to the value `value` does not exist in the index set,
    ///   and index is in `[0, self.len()]`.
    ///
    /// Otherwise, the index `index` is **out of bounds**.
    ///
    /// This method behaves as follows:
    ///
    /// * If an equivalent value already exists in the set, let `current_index` be the storage
    ///   index of the entry with value equivalent to `value`.
    ///   - If `index < current_index`, every entry in range `[index, current_index)` is shifted up
    ///     one entry in the storage order, the current entry is moved from `current_index` to
    ///     `index`, and the method returns `(index, false)`.
    ///   - If `index > current_index`, every entry in range `(current_index, index]` is shifted
    ///     down one entry in the storage order, the current entry is moved from `current_index` to
    ///     `index`, and the method returns `(index, false)`.
    ///   - If `index == current_index`, no shifting occurs, and the method returns
    ///     `(index, false)`.
    /// * If an equivalent value does not exist in the index set, the new entry is inserted at the
    ///   storage index `index`, and each entry in the range `[index, self.len())` is shifted
    ///   up one index, and the method returns `(index, true)`.
    ///
    /// Note that an existing entry **cannot** be moved to the index `self.len()`.
    ///
    /// # Panics
    ///
    /// This method panics under one of the following conditions:
    ///
    /// * This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    ///   builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    ///   value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    /// * If the index `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// Shift inserting an entry that **does not** exist with index `index < self.len()`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<isize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let inserted = opaque_set.shift_insert::<isize, RandomState, Global>(3, isize::MAX);
    /// let expected: TypedProjVec<isize> = TypedProjVec::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     isize::MAX,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    /// ]);
    /// let result: TypedProjVec<isize> = opaque_set
    ///     .iter::<isize, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert!(inserted);
    /// ```
    ///
    /// Shift inserting an entry that **does not** exist with index `index == self.len()`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<isize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let inserted = opaque_set.shift_insert::<isize, RandomState, Global>(opaque_set.len(), isize::MAX);
    /// let expected: TypedProjVec<isize> = TypedProjVec::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    ///     isize::MAX,
    /// ]);
    /// let result: TypedProjVec<isize> = opaque_set
    ///     .iter::<isize, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert!(inserted);
    /// ```
    ///
    /// Shift inserting an entry that **does** exist with index `index < self.len()`.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     4_isize,
    ///     5_isize,
    ///     6_isize,
    ///     7_isize,
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<isize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let inserted = opaque_set.shift_insert::<isize, RandomState, Global>(3, 6_isize);
    /// let expected: TypedProjVec<isize> = TypedProjVec::from([
    ///     1_isize,
    ///     2_isize,
    ///     3_isize,
    ///     6_isize,
    ///     4_isize,
    ///     5_isize,
    ///     7_isize,
    /// ]);
    /// let result: TypedProjVec<isize> = opaque_set
    ///     .iter::<isize, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert!(!inserted);
    /// ```
    #[track_caller]
    pub fn shift_insert<T, S, A>(&mut self, index: usize, value: T) -> bool
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.shift_insert(index, value)
    }

    /// Adds a new value to the index set, and replaces the existing value equal to the given one,
    /// if it exists, and returns the value of the existing one.
    ///
    /// This method does not change the storage order of the other elements in the set.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Replacing a value where two different string values are equal up to letter case.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// struct CaseInsensitiveString(String);
    ///
    /// impl PartialEq for CaseInsensitiveString {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         self.0.eq_ignore_ascii_case(&other.0)
    ///     }
    /// }
    /// #
    /// # impl Eq for CaseInsensitiveString {}
    /// #
    /// # impl Hash for CaseInsensitiveString {
    /// #     fn hash<H: Hasher>(&self, state: &mut H) {
    /// #        for byte in self.0.bytes() {
    /// #            state.write_u8(byte.to_ascii_lowercase());
    /// #        }
    /// #    }
    /// # }
    /// #
    ///
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     CaseInsensitiveString(String::from("foo")),
    ///     CaseInsensitiveString(String::from("bar")),
    ///     CaseInsensitiveString(String::from("baz")),
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<CaseInsensitiveString>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// let expected = Some(String::from("bar"));
    /// let result: Option<String> = {
    ///     let _result = opaque_set.replace::<CaseInsensitiveString, RandomState, Global>(
    ///         CaseInsensitiveString(String::from("BAR")),
    ///     );
    ///     _result.map(|s| s.0)
    /// };
    ///
    /// assert_eq!(result, expected);
    ///
    /// let expected_entries = TypedProjVec::from([
    ///     String::from("foo"),
    ///     String::from("BAR"),
    ///     String::from("baz"),
    /// ]);
    /// let result_entries: TypedProjVec<String> = opaque_set
    ///     .iter::<CaseInsensitiveString, RandomState, Global>()
    ///     .map(|s| s.0.clone())
    ///     .collect();
    ///
    /// assert_eq!(result_entries, expected_entries);
    /// ```
    pub fn replace<T, S, A>(&mut self, value: T) -> Option<T>
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.replace(value)
    }

    /// Adds a new value to the index set, and replaces the existing value equal to the given one,
    /// if it exists, and returns the storage index and value of the existing one.
    ///
    /// This method does not change the storage order of the other elements in the set.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Replacing a value where two different string values are equal up to letter case.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// struct CaseInsensitiveString(String);
    ///
    /// impl PartialEq for CaseInsensitiveString {
    ///     fn eq(&self, other: &Self) -> bool {
    ///         self.0.eq_ignore_ascii_case(&other.0)
    ///     }
    /// }
    /// #
    /// # impl Eq for CaseInsensitiveString {}
    /// #
    /// # impl Hash for CaseInsensitiveString {
    /// #     fn hash<H: Hasher>(&self, state: &mut H) {
    /// #        for byte in self.0.bytes() {
    /// #            state.write_u8(byte.to_ascii_lowercase());
    /// #        }
    /// #    }
    /// # }
    /// #
    ///
    /// let mut opaque_set = OpaqueIndexSet::from([
    ///     CaseInsensitiveString(String::from("foo")),
    ///     CaseInsensitiveString(String::from("bar")),
    ///     CaseInsensitiveString(String::from("baz")),
    /// ]);
    /// #
    /// # assert!(opaque_set.has_value_type::<CaseInsensitiveString>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// let expected = (1, Some(String::from("bar")));
    /// let result: (usize, Option<String>) = {
    ///     let (i, _result) = opaque_set.replace_full::<CaseInsensitiveString, RandomState, Global>(
    ///         CaseInsensitiveString(String::from("BAR")),
    ///     );
    ///     (i, _result.map(|s| s.0))
    /// };
    ///
    /// assert_eq!(result, expected);
    ///
    /// let expected_entries = TypedProjVec::from([
    ///     String::from("foo"),
    ///     String::from("BAR"),
    ///     String::from("baz"),
    /// ]);
    /// let result_entries: TypedProjVec<String> = opaque_set
    ///     .iter::<CaseInsensitiveString, RandomState, Global>()
    ///     .map(|s| s.0.clone())
    ///     .collect();
    ///
    /// assert_eq!(result_entries, expected_entries);
    /// ```
    pub fn replace_full<T, S, A>(&mut self, value: T) -> (usize, Option<T>)
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let  proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.replace_full(value)
    }

    /// Return an iterator over the values in the set-theoretic difference of two index sets.
    ///
    /// This iterator behaves as follows. Let `self` and `other` be index sets. Let `v` be a value
    /// produced by the iterator. Then `v` satisfies `(v in self) && (not (v in other))`. More
    /// informally, this iterator produces values that are in `self`, but not in `other`.
    ///
    /// This iterator produces values in the same order that they appear in `self`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::{OpaqueIndexSet, TypedProjIndexSet};
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set1 = OpaqueIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// #
    /// # assert!(opaque_set1.has_value_type::<i32>());
    /// # assert!(opaque_set1.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set1.has_allocator_type::<Global>());
    /// #
    /// let opaque_set2 = OpaqueIndexSet::from([2_i32, 4_i32, 6_i32, 7_i32, 8_i32]);
    /// #
    /// # assert!(opaque_set2.has_value_type::<i32>());
    /// # assert!(opaque_set2.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set2.has_allocator_type::<Global>());
    /// #
    ///
    /// let expected = TypedProjIndexSet::from([1_i32, 3_i32, 5_i32]);
    /// let result: TypedProjIndexSet<i32> = opaque_set1
    ///     .difference::<RandomState, i32, RandomState, Global>(&opaque_set2)
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.as_slice(), expected.as_slice());
    /// ```
    pub fn difference<'a, S2, T, S, A>(&'a self, other: &'a OpaqueIndexSet) -> Difference<'a, T, S2, A>
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();
        let proj_other = other.as_proj::<T, S2, A>();

        proj_self.difference(proj_other)
    }

    /// Return an iterator over the values in the set-theoretic symmetric difference of two index
    /// sets.
    ///
    /// This iterator behaves as follows. Let `self` and `other` be index sets. Let `v` be a value
    /// produced by the iterator. Then `v` satisfies
    ///
    /// ```text
    /// (v in self) && (not (v in other)) || (not (v in self)) && (v in other).
    /// ```
    ///
    /// More informally, this iterator produces those elements that are in one set or the other
    /// set, but not both sets.
    ///
    /// The iterator produces the values from `self` storage order, followed by the values from
    /// `other` in their storage order.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::{OpaqueIndexSet, TypedProjIndexSet};
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set1 = OpaqueIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// #
    /// # assert!(opaque_set1.has_value_type::<i32>());
    /// # assert!(opaque_set1.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set1.has_allocator_type::<Global>());
    /// #
    /// let opaque_set2 = OpaqueIndexSet::from([2_i32, 4_i32, 6_i32, 7_i32, 8_i32]);
    /// #
    /// # assert!(opaque_set2.has_value_type::<i32>());
    /// # assert!(opaque_set2.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set2.has_allocator_type::<Global>());
    /// #
    ///
    /// let expected = TypedProjIndexSet::from([1_i32, 3_i32, 5_i32, 7_i32, 8_i32]);
    /// let result: TypedProjIndexSet<i32> = opaque_set1
    ///     .symmetric_difference::<RandomState, i32, RandomState, Global>(&opaque_set2)
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.as_slice(), expected.as_slice());
    /// ```
    pub fn symmetric_difference<'a, S2, T, S, A>(&'a self, other: &'a OpaqueIndexSet) -> SymmetricDifference<'a, T, S, S2, A>
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();
        let proj_other = other.as_proj::<T, S2, A>();

        proj_self.symmetric_difference(proj_other)
    }

    /// Return an iterator over the values in the set-theoretic intersection of two index sets.
    ///
    /// This iterator behaves as follows. Let `self` and `other` be index sets. Let `v` be a value
    /// produced by the iterator. Then `v` satisfies `(v in self) && (v in other)`. More
    /// informally, this iterator produces those elements that are in both sets, and none of the
    /// elements that are only in one set.
    ///
    /// This iterator produces values in the order that they appear in `self`.
    ///
    /// # Panics
    ///
    /// This method panics under the following conditions:
    ///
    /// * If the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash builder of `self`,
    ///   and the [`TypeId`] of the memory allocator of `self` do not match the value type `T`,
    ///   hash builder type `S`, and allocator type `A`, respectively.
    /// * If the [`TypeId`] of the values of `other`, the [`TypeId`] for the hash builder of
    ///   `other`, and the [`TypeId`] of the memory allocator of `self` do not match the value type
    ///   `T`, hash builder type `S2`, and allocator type `A`, respectively.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::{OpaqueIndexSet, TypedProjIndexSet};
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set1 = OpaqueIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// #
    /// # assert!(opaque_set1.has_value_type::<i32>());
    /// # assert!(opaque_set1.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set1.has_allocator_type::<Global>());
    /// #
    /// let opaque_set2 = OpaqueIndexSet::from([2_i32, 4_i32, 6_i32, 7_i32, 8_i32]);
    /// #
    /// # assert!(opaque_set2.has_value_type::<i32>());
    /// # assert!(opaque_set2.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set2.has_allocator_type::<Global>());
    /// #
    ///
    /// let expected = TypedProjIndexSet::from([2_i32, 4_i32, 6_i32]);
    /// let result: TypedProjIndexSet<i32> = opaque_set1
    ///     .intersection::<RandomState, i32, RandomState, Global>(&opaque_set2)
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.as_slice(), expected.as_slice());
    /// ```
    pub fn intersection<'a, S2, T, S, A>(&'a self, other: &'a OpaqueIndexSet) -> Intersection<'a, T, S2, A>
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();
        let proj_other = other.as_proj::<T, S2, A>();

        proj_self.intersection(proj_other)
    }

    /// Return an iterator over the values in the set-theoretic union of two index sets.
    ///
    /// This iterator behaves as follows. Let `self` and `other` be index sets. Let `v` be a value
    /// produced by the iterator. Then `v` satisfies `(v in self) || (v in other)`. More
    /// informally, this iterator produces every value in `self` and `other` exactly once.
    ///
    /// This iterator produces values in the same order as their storage order in `self`, followed
    /// by the storage order of the values unique to `other`.
    ///
    /// # Panics
    ///
    /// This method panics under the following conditions:
    ///
    /// * If the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash builder of `self`,
    ///   and the [`TypeId`] of the memory allocator of `self` do not match the value type `T`,
    ///   hash builder type `S`, and allocator type `A`, respectively.
    /// * If the [`TypeId`] of the values of `other`, the [`TypeId`] for the hash builder of
    ///   `other`, and the [`TypeId`] of the memory allocator of `self` do not match the value type
    ///   `T`, hash builder type `S2`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::{OpaqueIndexSet, TypedProjIndexSet};
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::{Hash, Hasher, RandomState};
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set1 = OpaqueIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    /// #
    /// # assert!(opaque_set1.has_value_type::<i32>());
    /// # assert!(opaque_set1.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set1.has_allocator_type::<Global>());
    /// #
    /// let opaque_set2 = OpaqueIndexSet::from([2_i32, 4_i32, 6_i32, 7_i32, 8_i32]);
    /// #
    /// # assert!(opaque_set2.has_value_type::<i32>());
    /// # assert!(opaque_set2.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set2.has_allocator_type::<Global>());
    /// #
    ///
    /// let expected = TypedProjIndexSet::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32, 8_i32]);
    /// let result: TypedProjIndexSet<i32> = opaque_set1
    ///     .union::<RandomState, i32, RandomState, Global>(&opaque_set2)
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(result.as_slice(), expected.as_slice());
    /// ```
    pub fn union<'a, S2, T, S, A>(&'a self, other: &'a OpaqueIndexSet) -> Union<'a, T, S, A>
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();
        let proj_other = other.as_proj::<T, S2, A>();

        proj_self.union(proj_other)
    }

    /// Creates a splicing iterator that replaces the specified storage range in the type-erased
    /// index set with the given `replace_with` iterator and yields the removed items. The argument
    /// `replace_with` does not need to be the same length as `range`.
    ///
    /// The `range` argument is removed even if the `Splice` iterator is not consumed before it is
    /// dropped.
    ///
    /// It is unspecified how many elements are removed from the type-erased index set
    /// if the `Splice` value is leaked.
    ///
    /// The input iterator `replace_with` is only consumed when the `Splice` value is dropped.
    /// If a key from the iterator matches an existing entry in the set (i.e. outside the range
    /// `range`), then the value will be updated in that position. Otherwise, the new entry will be
    /// inserted in the replaced `range`.
    ///
    /// # Panics
    ///
    /// This method panics if the starting point is greater than the end point or if the end point
    /// is greater than the length of the index set.
    ///
    /// # Examples
    ///
    /// Splicing entries into an index set.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from(["foo", "bar", "baz", "quux"]);
    /// #
    /// # assert!(opaque_set.has_value_type::<&str>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let new = ["garply", "corge", "grault"];
    /// let expected = TypedProjVec::from(["foo", "garply", "corge", "grault", "quux"]);
    /// let expected_removed = TypedProjVec::from(["bar", "baz"]);
    /// let removed: TypedProjVec<&str> = opaque_set
    ///     .splice::<_, _, &str, RandomState, Global>(1..3, new)
    ///     .collect();
    /// let result: TypedProjVec<&str> = opaque_set
    ///     .iter::<&str, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, expected_removed);
    /// ```
    ///
    /// Using `splice` to insert new items into an index set efficiently at a specific position
    /// indicated by an empty range.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set = OpaqueIndexSet::from(["foo", "grault"]);
    /// #
    /// # assert!(opaque_set.has_value_type::<&str>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// let new = ["bar", "baz", "quux"];
    /// let expected = TypedProjVec::from(["foo", "bar", "baz", "quux", "grault"]);
    /// let expected_removed = TypedProjVec::from([]);
    /// let removed: TypedProjVec<&str> = opaque_set
    ///     .splice::<_, _, &str, RandomState, Global>(1..1, new)
    ///     .collect();
    /// let result: TypedProjVec<&str> = opaque_set
    ///     .iter::<&str, RandomState, Global>()
    ///     .cloned()
    ///     .collect();
    ///
    /// assert_eq!(result, expected);
    /// assert_eq!(removed, expected_removed);
    /// ```
    #[track_caller]
    pub fn splice<R, I, T, S, A>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, T, S, A>
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.splice(range, replace_with)
    }

    /// Moves all entries from `other` into `self`, leaving `other` empty.
    ///
    /// This is equivalent to calling [`insert`] for each entry from `other` in order, which means
    /// that for keys that already exist in `self`, their value is updated in the current position.
    ///
    /// [`insert`]: TypedProjIndexSet::insert
    ///
    /// # Formal Properties
    ///
    /// Let `set1` and `set2` be index sets, `set1_before` be the state of `set1` before this
    /// method is called, `set2_before` be the state of `set2` before this method is called,
    /// `set1_after` be the state of `set1` after this method completes, and `set2_after` be the
    /// state of `set2` after this method completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { true }
    /// set1.append(set2)
    /// {
    ///     set1_after.len() ≤ set1_before.len() + set2_before.len()
    ///     ∧ set2_after.len() = 0
    ///     ∧ (∀ v ∈ set2_before. v ∈ set1_before ⇒ v ∈ set1_after)
    ///     ∧ (∀ v ∈ set2_before. v ∉ set1_before ⇒ v ∈ set1_after)
    ///     ∧ (∀ v ∈ set2_before. v ∉ set2_after)
    ///     ∧ (∀ i ∈ [0, set1_before.len()). set1_after[i] = set1_before[i])
    ///     ∧ (∀ j1, j2 ∈ [0, set2_before.len()).
    ///          ((set2_before[j1] ∉ set1_before) ∧ (set2_before[j2] ∉ set1_before) ∧ (j1 < j2))
    ///          ⇒ (∃ i1, i2 ∈ [set1_before.len(), set1_after.len()).
    ///               i1 < i2
    ///               ∧ set1_after[i1] = set2_before[j1]
    ///               ∧ set1_after[i2] = set2_before[j2]
    ///          )
    ///     )
    /// }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Panics
    ///
    /// This method panics under the following conditions:
    ///
    /// * If the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash builder of `self`,
    ///   and the [`TypeId`] of the memory allocator of `self` do not match the value type `T`, hash
    ///   builder type `S`, and allocator type `A`, respectively.
    /// * If the [`TypeId`] of the values of `other`, the [`TypeId`] for the hash builder of
    ///   `other`, and the [`TypeId`] of the memory allocator of `other` do not match the value type
    ///   `T`, hash builder type `S2`, and allocator type `A2`, respectively.
    ///
    /// # Examples
    ///
    /// Appending one index set to another when they have no overlapping values.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set1 = OpaqueIndexSet::from(["foo", "bar", "baz", "quux"]);
    /// #
    /// # assert!(opaque_set1.has_value_type::<&str>());
    /// # assert!(opaque_set1.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set1.has_allocator_type::<Global>());
    /// #
    /// let mut opaque_set2 = OpaqueIndexSet::from(["garply", "corge", "grault"]);
    /// #
    /// # assert!(opaque_set2.has_value_type::<&str>());
    /// # assert!(opaque_set2.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set2.has_allocator_type::<Global>());
    /// #
    ///
    /// assert_eq!(opaque_set1.len(), 4);
    /// assert_eq!(opaque_set2.len(), 3);
    ///
    /// opaque_set1.append::<&str, RandomState, Global, RandomState, Global>(&mut opaque_set2);
    ///
    /// assert_eq!(opaque_set1.len(), 7);
    /// assert_eq!(opaque_set2.len(), 0);
    ///
    /// let expected = &["foo", "bar", "baz", "quux", "garply", "corge", "grault"];
    /// let result = opaque_set1.as_slice::<&str, RandomState, Global>();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// Appending one index set to another when they have overlapping values.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_set1 = OpaqueIndexSet::from(["foo", "bar", "baz", "quux"]);
    /// #
    /// # assert!(opaque_set1.has_value_type::<&str>());
    /// # assert!(opaque_set1.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set1.has_allocator_type::<Global>());
    /// #
    /// let mut opaque_set2 = OpaqueIndexSet::from(["garply", "corge", "grault", "baz"]);
    /// #
    /// # assert!(opaque_set2.has_value_type::<&str>());
    /// # assert!(opaque_set2.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set2.has_allocator_type::<Global>());
    /// #
    ///
    /// assert_eq!(opaque_set1.len(), 4);
    /// assert_eq!(opaque_set2.len(), 4);
    ///
    /// opaque_set1.append::<&str, RandomState, Global, RandomState, Global>(&mut opaque_set2);
    ///
    /// assert_eq!(opaque_set1.len(), 7);
    /// assert_eq!(opaque_set2.len(), 0);
    ///
    /// let expected =  &["foo", "bar", "baz", "quux", "garply", "corge", "grault"];
    /// let result = opaque_set1.as_slice::<&str, RandomState, Global>();
    ///
    /// assert_eq!(result, expected);
    /// ```
    pub fn append<T, S1, A1, S2, A2>(&mut self, other: &mut OpaqueIndexSet)
    where
        T: any::Any + hash::Hash + Eq,
        S1: any::Any + hash::BuildHasher + Send + Sync,
        S1::Hasher: any::Any + hash::Hasher + Send + Sync,
        A1: any::Any + alloc::Allocator + Send + Sync,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
        A2: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S1, A1>();
        let proj_other = other.as_proj_mut::<T, S2, A2>();

        proj_self.append(proj_other)
    }
}

impl OpaqueIndexSet {
    /// Determines whether a given lookup value exists in the index set.
    ///
    /// This method returns `true` if the equivalent value to `value` exists in `self`. This method
    /// returns `false` if the equivalent value to `value` does not exist in `self`.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with values of type `T`. Let `v :: T` be an value of type `T`. We
    /// say that `set` **contains** a value `v :: T`, or that `v` is an **entry of** `set` if the
    /// following holds:
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// This method satisfies the following:
    ///
    /// ```text
    /// ∀ v :: V. set.contains(v) ⇔ (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v.
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_usize, 2_usize, 3_usize]);
    /// #
    /// # assert!(opaque_set.has_value_type::<usize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert!(opaque_set.contains::<_, usize, RandomState, Global>(&1_usize));
    /// assert!(opaque_set.contains::<_, usize, RandomState, Global>(&2_usize));
    /// assert!(opaque_set.contains::<_, usize, RandomState, Global>(&3_usize));
    /// assert!(!opaque_set.contains::<_, usize, RandomState, Global>(&4_usize));
    /// assert!(!opaque_set.contains::<_, usize, RandomState, Global>(&usize::MAX));
    /// ```
    pub fn contains<Q, T, S, A>(&self, value: &Q) -> bool
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        let proj_self =  self.as_proj::<T, S, A>();

        proj_self.contains(value)
    }

    /// Returns a reference to the value corresponding equivalent to the given lookup value, if it
    /// exists in the index set.
    ///
    /// This method returns `Some(&eq_value)` where `eq_value` is the value stored in `self`
    /// equivalent to the value `value`, if such a value exists in `self`. This method returns
    /// `None` if a value equivalent to `value` does not exist in `self`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_usize, 2_usize, 3_usize]);
    /// #
    /// # assert!(opaque_set.has_value_type::<usize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert_eq!(opaque_set.get::<_, usize, RandomState, Global>(&1_usize), Some(&1_usize));
    /// assert_eq!(opaque_set.get::<_, usize, RandomState, Global>(&2_usize), Some(&2_usize));
    /// assert_eq!(opaque_set.get::<_, usize, RandomState, Global>(&3_usize), Some(&3_usize));
    /// assert_eq!(opaque_set.get::<_, usize, RandomState, Global>(&4_usize), None);
    /// assert_eq!(opaque_set.get::<_, usize, RandomState, Global>(&usize::MAX), None);
    /// ```
    pub fn get<Q, T, S, A>(&self, value: &Q) -> Option<&T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        let proj_self =  self.as_proj::<T, S, A>();

        proj_self.get(value)
    }

    /// Returns the storage index and a reference to the value of the entry with the equivalent
    /// value to the lookup value, if it exists in the index set.
    ///
    /// This method returns `Some((index, &eq_value))` where `index` is the storage index of the
    /// entry, `eq_value` is the equivalent value to the lookup value `value` stored in the set, if
    /// the entry exists in `self`. This method returns `None` if the equivalent value to `value`
    /// does not exist in `self`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_usize, 2_usize, 3_usize]);
    /// #
    /// # assert!(opaque_set.has_value_type::<usize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert_eq!(opaque_set.get_full::<_, usize, RandomState, Global>(&1_usize), Some((0, &1_usize)));
    /// assert_eq!(opaque_set.get_full::<_, usize, RandomState, Global>(&2_usize), Some((1, &2_usize)));
    /// assert_eq!(opaque_set.get_full::<_, usize, RandomState, Global>(&3_usize), Some((2, &3_usize)));
    /// assert_eq!(opaque_set.get_full::<_, usize, RandomState, Global>(&4_usize), None);
    /// assert_eq!(opaque_set.get_full::<_, usize, RandomState, Global>(&usize::MAX), None);
    /// ```
    pub fn get_full<Q, T, S, A>(&self, value: &Q) -> Option<(usize, &T)>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        let proj_self =  self.as_proj::<T, S, A>();

        proj_self.get_full(value)
    }

    /// Returns the storage index of the equivalent value to the given lookup value, if it exists
    /// in the index set.
    ///
    /// This method returns `Some(index)`, where `index` is the storage index of the equivalent
    /// value to `value`, if the equivalent value exists in `self`. This method returns `None` if
    /// the equivalent value to `value` does not exist in `self`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_usize, 2_usize, 3_usize]);
    /// #
    /// # assert!(opaque_set.has_value_type::<usize>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    ///
    /// assert_eq!(opaque_set.get_index_of::<_, usize, RandomState, Global>(&1_usize), Some(0));
    /// assert_eq!(opaque_set.get_index_of::<_, usize, RandomState, Global>(&2_usize), Some(1));
    /// assert_eq!(opaque_set.get_index_of::<_, usize, RandomState, Global>(&3_usize), Some(2));
    /// assert_eq!(opaque_set.get_index_of::<_, usize, RandomState, Global>(&4_usize), None);
    /// assert_eq!(opaque_set.get_index_of::<_, usize, RandomState, Global>(&usize::MAX), None);
    /// ```
    pub fn get_index_of<Q, T, S, A>(&self, value: &Q) -> Option<usize>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        let proj_self =  self.as_proj::<T, S, A>();

        proj_self.get_index_of(value)
    }

    /// Removes an entry from a type-erased index set, moving the last entry in storage order in
    /// the collection to the index where the removed entry occupies the collection.
    ///
    /// This method behaves with respect to the lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves the last entry in the collection to the slot
    ///   at `index`, leaving the rest of the entries in place. If `index == self.len() - 1`, it
    ///   removes the entry from the end of the collection with no reordering of the remaining
    ///   entries in the collection. The method then returns `true`, indicating that it removed the
    ///   equivalent value to `value` from the collection.
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `false`.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, map.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// The **last entry** in the set `set` when `set` is non-empty is defined by
    ///
    /// ```text
    /// last(set) := set[set.len() - 1].
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.swap_remove(value)
    /// {
    ///     result = true
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (set_after[index(set_before, value)] = last(set_before)
    ///        ∧ (∀ v ∈ set_after. v ≠ last(set_before) ∧ (v ≠ value ⇒ set_after[v] = set_before[v])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// set.swap_remove(value)
    /// { result = false ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Showing how swap removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(opaque_set.has_value_type::<isize>());
    /// #   assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// #   assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_remove::<_, isize, RandomState, Global>(&isize::MAX);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>()
    ///     );
    ///     assert!(removed);
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_remove::<_, isize, RandomState, Global>(&3_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, true);
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, isize::MAX, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_remove::<_, isize, RandomState, Global>(&2_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, true);
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([isize::MAX, 2_isize, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_remove::<_, isize, RandomState, Global>(&1_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, true);
    /// }
    /// ```
    pub fn swap_remove<Q, T, S, A>(&mut self, value: &Q) -> bool
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        let proj_self =  self.as_proj_mut::<T, S, A>();

        proj_self.swap_remove(value)
    }

    /// Removes an entry from a type-erased index set, shifting every successive entry in the
    /// collection in storage order down one index to fill where the removed entry occupies the
    /// collection.
    ///
    /// This method behaves with respect to the lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves every successive entry in the collection to
    ///   the entry at storage index `index` down one unit. Every entry preceding the entry at
    ///   index `index` remains in the same location.  The method returns `true`, which indicates
    ///   that the entry with equivalent value to `value` was removed from the index set.
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `false`.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.shift_remove(value)
    /// {
    ///     result = true
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (let i = index(set_before, value);
    ///        (∀ j ∈ [0, i). set_after[j] = set_before[j])
    ///        ∧ (∀ j ∈ [i, set_after.len()). set_after[j] = set_before[j + 1])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// map.shift_remove(value)
    /// { result = false ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Showing how shift removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(opaque_set.has_value_type::<isize>());
    /// #   assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// #   assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_remove::<_, isize, RandomState, Global>(&isize::MAX);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, true);
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_remove::<_, isize, RandomState, Global>(&3_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, true);
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_remove::<_, isize, RandomState, Global>(&2_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, true);
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([2_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_remove::<_, isize, RandomState, Global>(&1_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, true);
    /// }
    /// ```
    pub fn shift_remove<Q, T, S, A>(&mut self, value: &Q) -> bool
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        let proj_self =  self.as_proj_mut::<T, S, A>();

        proj_self.shift_remove(value)
    }

    /// Removes an entry from a type-erased index set, moving the last entry in storage order
    /// in the collection to the index where the removed entry occupies the collection.
    ///
    /// This method behaves with respect to the lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves the last entry in the collection to the slot
    ///   at `index`, leaving the rest of the entries in place. If `index == self.len() - 1`, it
    ///   removes the entry from end of the collection with no reordering of the remaining entries
    ///   in the collection. The method then returns `Some(eq_value)`, where `eq_value` is the
    ///   equivalent value to the value `value` stored in the index set.
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `None`.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, map.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// The **last entry** in the set `set` when `set` is non-empty is defined by
    ///
    /// ```text
    /// last(set) := set[set.len() - 1].
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.swap_take(value)
    /// {
    ///     result = Some(set_before[value])
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (set_after[index(set_before, value)] = last(set_before)
    ///        ∧ (∀ v ∈ set_after. (v ≠ last(set_before) ∧ v ≠ value) ⇒ set_after[v] = set_before[v])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// set.swap_take(value)
    /// { result = None ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Showing how swap removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(opaque_set.has_value_type::<isize>());
    /// #   assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// #   assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_take::<_, isize, RandomState, Global>(&isize::MAX);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some(isize::MAX));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_take::<_, isize, RandomState, Global>(&3_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some(3_isize));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, isize::MAX, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_take::<_, isize, RandomState, Global>(&2_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some(2_isize));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([isize::MAX, 2_isize, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_take::<_, isize, RandomState, Global>(&1_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some(1_isize));
    /// }
    /// ```
    pub fn swap_take<Q, T, S, A>(&mut self, value: &Q) -> Option<T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.swap_take(value)
    }

    /// Removes an entry from a type-erased index set, shifting every successive entry in the
    /// collection in storage order down one index to fill where the removed entry occupies the
    /// collection.
    ///
    /// This method behaves with respect to the lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves every successive entry in the collection to
    ///   the entry at storage index `index` down one unit. Every entry preceding the entry at
    ///   index `index` remains in the same location. The method returns `Some(eq_value)`, where
    ///   `eq_value` is the equivalent value to the value `value` stored in the index set.
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `None`.
    ///
    /// In particular, the method acts like a [`pop`] when the last value in the collection is
    /// shift-removed, because the sub-collection of successor entries in the entry storage is
    /// empty.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.shift_take(value)
    /// {
    ///     result = Some(set_before[value])
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (let i = index(set_before, value);
    ///        (∀ j ∈ [0, i). set_after[j] = set_before[j])
    ///        ∧ (∀ j ∈ [i, set_after.len()). set_after[j] = set_before[j + 1])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// set.shift_take(value)
    /// { result = None ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Showing how shift removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(opaque_set.has_value_type::<isize>());
    /// #   assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// #   assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_take::<_, isize, RandomState, Global>(&isize::MAX);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some(isize::MAX));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_take::<_, isize, RandomState, Global>(&3_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some(3_isize));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_take::<_, isize, RandomState, Global>(&2_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some(2_isize));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([2_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_take::<_, isize, RandomState, Global>(&1_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some(1_isize));
    /// }
    /// ```
    ///
    /// [`pop`]: OpaqueIndexSet::pop
    pub fn shift_take<Q, T, S, A>(&mut self, value: &Q) -> Option<T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.shift_take(value)
    }

    /// Removes an entry from a type-projected index set, moving the last entry in storage order in
    /// the collection to the index where the removed entry occupies the collection.
    ///
    /// This method behaves with respect to lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves the last entry in the collection to the slot
    ///   at `index`, leaving the rest of the entries in place. If `index == self.len() - 1`, it
    ///   removes the entry from end of the collection with no reordering of the remaining entries
    ///   in the collection. The method then returns `Some((index, eq_value))`, where `eq_value` is
    ///   the equivalent value to `value` stored in the index set..
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `None`.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.swap_remove_full(value)
    /// {
    ///     result = Some((index(set_before, value), set_before[index(set_before, value)]))
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (set_after[index(set_before, value)] = last(set_before)
    ///        ∧ (∀ v ∈ set_after. v ≠ last(set_before) ∧ v ≠ value ⇒ set_after[v] = set_before[v])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// set.swap_remove_full(value)
    /// { result = None ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Showing how swap removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(opaque_set.has_value_type::<isize>());
    /// #   assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// #   assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_remove_full::<_, isize, RandomState, Global>(&isize::MAX);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some((3, isize::MAX)));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_remove_full::<_, isize, RandomState, Global>(&3_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some((2, 3_isize)));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, isize::MAX, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_remove_full::<_, isize, RandomState, Global>(&2_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some((1, 2_isize)));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([isize::MAX, 2_isize, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.swap_remove_full::<_, isize, RandomState, Global>(&1_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some((0, 1_isize)));
    /// }
    /// ```
    pub fn swap_remove_full<Q, T, S, A>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.swap_remove_full(value)
    }

    /// Removes an entry from a type-erased index set, shifting every successive entry in the
    /// collection in storage order down one index to fill where the removed entry occupies the
    /// collection.
    ///
    /// This method behaves with respect to the lookup value `value` as follows:
    ///
    /// * If the equivalent value to `value` exists in the index set, let `index` be its storage
    ///   index. If `index < self.len() - 1`, it moves every successive entry in the collection to
    ///   the entry at storage index `index` down one unit. Every entry preceding the entry at
    ///   index `index` remains in the same location. The method returns `Some((index, eq_value))`,
    ///   where `eq_value` is the equivalent value to the value `value` stored in the index set.
    /// * If the equivalent value to `value` does not exist in the index set, the method returns
    ///   `None`.
    ///
    /// In particular, the method acts like a [`pop`] when the last value in the collection is
    /// shift-removed, because the sub-collection of successor entries in the entry storage is
    /// empty.
    ///
    /// # Formal Properties
    ///
    /// Let `set` be an index set with value type `T`. Let `set_before` be the state of `set`
    /// before this method is called, `set_after` be the state of `set` after this method
    /// completes.
    ///
    /// We say that a value `v` is in the set `set` provided that
    ///
    /// ```text
    /// ∀ v :: T. (v ∈ set) ⇔ (∃ i ∈ [0, set.len()). set[i] = v).
    /// ```
    ///
    /// The **index** of a value `v` in `set` is defined by
    ///
    /// ```text
    /// index(set, v) := i such that set[i] = v ∧ (∀ j ∈ [0, set.len()). j ≠ i ⇒ set[j] ≠ v).
    /// ```
    ///
    /// We say that two sets `set1` and `set2` are **equal** if and only if
    ///
    /// ```text
    /// set1 = set2 ⇔ (set1.len() = set2.len()) ∧ (∀ i ∈ [0, set1.len()). set1[i] = set2[i]).
    /// ```
    ///
    /// This method satisfies:
    ///
    /// ```text
    /// { value ∈ set_before }
    /// set.shift_remove_full(value)
    /// {
    ///     result = Some((index(set_before, value), set_before[index(set_before, value)]))
    ///     ∧ set_after.len() = set_before.len() - 1
    ///     ∧ value ∉ set_after
    ///     ∧ (let i = index(set_before, value);
    ///        (∀ j ∈ [0, i). set_after[j] = set_before[j])
    ///        ∧ (∀ j ∈ [i, set_after.len()). set_after[j] = set_before[j + 1])
    ///     )
    /// }
    ///
    /// { value ∉ set_before }
    /// set.shift_remove_full(value)
    /// { result = None ∧ set_after = set_before }
    /// ```
    ///
    /// where `{P} S {Q}` is the Hoare triple indicating how this method acts on `set`.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Showing how shift removal happens.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(opaque_set.has_value_type::<isize>());
    /// #   assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// #   assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, 3_isize]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_remove_full::<_, isize, RandomState, Global>(&isize::MAX);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some((3, isize::MAX)));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 2_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_remove_full::<_, isize, RandomState, Global>(&3_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some((2, 3_isize)));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([1_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_remove_full::<_, isize, RandomState, Global>(&2_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some((1, 2_isize)));
    /// }
    /// {
    ///     let expected = OpaqueIndexSet::from([2_isize, 3_isize, isize::MAX]);
    /// #
    /// #   assert!(expected.has_value_type::<isize>());
    /// #   assert!(expected.has_build_hasher_type::<RandomState>());
    /// #   assert!(expected.has_allocator_type::<Global>());
    /// #
    ///     let mut result = opaque_set.clone::<isize, RandomState, Global>();
    /// #
    /// #   assert!(result.has_value_type::<isize>());
    /// #   assert!(result.has_build_hasher_type::<RandomState>());
    /// #   assert!(result.has_allocator_type::<Global>());
    /// #
    ///     let removed = result.shift_remove_full::<_, isize, RandomState, Global>(&1_isize);
    ///     assert_eq!(
    ///         result.as_slice::<isize, RandomState, Global>(),
    ///         expected.as_slice::<isize, RandomState, Global>(),
    ///     );
    ///     assert_eq!(removed, Some((0, 1_isize)));
    /// }
    /// ```
    ///
    /// [`pop`]: OpaqueIndexSet::pop
    pub fn shift_remove_full<Q, T, S, A>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.shift_remove_full(value)
    }
}

impl OpaqueIndexSet {
    #[doc(alias = "pop_last")]
    pub fn pop<T, S, A>(&mut self) -> Option<T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.pop()
    }

    pub fn retain<F, T, S, A>(&mut self, mut keep: F)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&T) -> bool,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.retain(&mut keep);
    }

    pub fn sort<T, S, A>(&mut self)
    where
        T: any::Any + Ord,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.sort();
    }

    pub fn sort_by<F, T, S, A>(&mut self, mut cmp: F)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&T, &T) -> cmp::Ordering,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.sort_by(&mut cmp);
    }

    pub fn sorted_by<F, T, S, A>(self, mut cmp: F) -> IntoIter<T, A>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&T, &T) -> cmp::Ordering,
    {
        let proj_self = self.into_proj::<T, S, A>();

        proj_self.sorted_by(&mut cmp)
    }

    pub fn sort_unstable<T, S, A>(&mut self)
    where
        T: any::Any + Ord,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.sort_unstable()
    }

    pub fn sort_unstable_by<F, T, S, A>(&mut self, mut cmp: F)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&T, &T) -> cmp::Ordering,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.sort_unstable_by(&mut cmp)
    }

    pub fn sorted_unstable_by<F, T, S, A>(self, mut cmp: F) -> IntoIter<T, A>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&T, &T) -> cmp::Ordering,
    {
        let proj_self = self.into_proj::<T, S, A>();

        proj_self.sorted_unstable_by(&mut cmp)
    }

    pub fn sort_by_cached_key<K, F, T, S, A>(&mut self, mut sort_key: F)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        K: Ord,
        F: FnMut(&T) -> K,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.sort_by_cached_key(&mut sort_key)
    }

    pub fn binary_search<T, S, A>(&self, x: &T) -> Result<usize, usize>
    where
        T: any::Any + Ord,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.binary_search(x)
    }

    #[inline]
    pub fn binary_search_by<F, T, S, A>(&self, f: F) -> Result<usize, usize>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&T) -> cmp::Ordering,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.binary_search_by(f)
    }

    #[inline]
    pub fn binary_search_by_key<B, F, T, S, A>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        F: FnMut(&T) -> B,
        B: Ord,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.binary_search_by_key(b, f)
    }

    #[must_use]
    pub fn partition_point<P, T, S, A>(&self, pred: P) -> usize
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        P: FnMut(&T) -> bool,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.partition_point(pred)
    }

    pub fn reverse<T, S, A>(&mut self)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.reverse()
    }

    pub fn as_slice<T, S, A>(&self) -> &Slice<T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.as_slice()
    }

    pub fn into_boxed_slice<T, S, A>(self) -> Box<Slice<T>, TypedProjAlloc<A>>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.into_proj::<T, S, A>();

        proj_self.into_boxed_slice()
    }

    pub fn get_index<T, S, A>(&self, index: usize) -> Option<&T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.get_index(index)
    }

    pub fn get_range<R, T, S, A>(&self, range: R) -> Option<&Slice<T>>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.get_range(range)
    }

    pub fn first<T, S, A>(&self) -> Option<&T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.first()
    }

    pub fn last<T, S, A>(&self) -> Option<&T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.last()
    }

    pub fn swap_remove_index<T, S, A>(&mut self, index: usize) -> Option<T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.swap_remove_index(index)
    }

    pub fn shift_remove_index<T, S, A>(&mut self, index: usize) -> Option<T>
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.shift_remove_index(index)
    }

    #[track_caller]
    pub fn move_index<T, S, A>(&mut self, from: usize, to: usize)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.move_index(from, to)
    }

    #[track_caller]
    pub fn swap_indices<T, S, A>(&mut self, a: usize, b: usize)
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.swap_indices(a, b)
    }
}

impl OpaqueIndexSet {
    /// Clones a type-erased index set.
    ///
    /// This method acts identically to an implementation of the [`Clone`] trait on a
    /// type-projected index set [`TypedProjIndexSet`], or a generic [`HashSet`].
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Cloning an empty type-erased index set.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_set = OpaqueIndexSet::new::<i32>();
    /// #
    /// # assert!(opaque_set.has_value_type::<i32>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// assert!(opaque_set.is_empty());
    ///
    /// let cloned_opaque_set = opaque_set.clone::<i32, RandomState, Global>();
    /// #
    /// # assert!(cloned_opaque_set.has_value_type::<i32>());
    /// # assert!(cloned_opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(cloned_opaque_set.has_allocator_type::<Global>());
    /// #
    /// assert!(cloned_opaque_set.is_empty());
    ///
    /// let expected = cloned_opaque_set.as_slice::<i32, RandomState, Global>();
    /// let result = opaque_set.as_slice::<i32, RandomState, Global>();
    ///
    /// assert_eq!(result, expected);
    /// ```
    ///
    /// Cloning a non-empty type-erased index set.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 6] = [1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32];
    /// let opaque_set = OpaqueIndexSet::from(array);
    /// #
    /// # assert!(opaque_set.has_value_type::<i32>());
    /// # assert!(opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(opaque_set.has_allocator_type::<Global>());
    /// #
    /// assert!(!opaque_set.is_empty());
    ///
    /// let cloned_opaque_set = opaque_set.clone::<i32, RandomState, Global>();
    /// #
    /// # assert!(cloned_opaque_set.has_value_type::<i32>());
    /// # assert!(cloned_opaque_set.has_build_hasher_type::<RandomState>());
    /// # assert!(cloned_opaque_set.has_allocator_type::<Global>());
    /// #
    /// assert!(!cloned_opaque_set.is_empty());
    ///
    /// assert_eq!(opaque_set.len(), cloned_opaque_set.len());
    ///
    /// let expected = cloned_opaque_set.as_slice::<i32, RandomState, Global>();
    /// let result = opaque_set.as_slice::<i32, RandomState, Global>();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub fn clone<T, S, A>(&self) -> Self
    where
        T: any::Any + Clone,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let proj_self = self.as_proj::<T, S, A>();
        let proj_cloned_self = Clone::clone(proj_self);
        let cloned_self = OpaqueIndexSet::from_proj(proj_cloned_self);

        cloned_self
    }
}

impl OpaqueIndexSet {
    /// Extends a type-erased index set.
    ///
    /// This method acts identically to an implementation of the [`Extend`] trait on a
    /// type-projected index set [`TypedProjIndexSet`], or a generic [`HashSet`].
    ///
    /// If any entry from the iterable has an equivalent value in `self`, the value of the entry
    /// will not be included from the iterator.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the values of `self`, the [`TypeId`] for the hash
    /// builder of `self`, and the [`TypeId`] of the memory allocator of `self` do not match the
    /// value type `T`, hash builder type `S`, and allocator type `A`, respectively.
    ///
    /// # Examples
    ///
    /// Extending a type-erased index set without overlapping values.
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_index_map::OpaqueIndexSet;
    /// # use opaque_hash::TypedProjBuildHasher;
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use opaque_vec::TypedProjVec;
    /// # use std::any::TypeId;
    /// # use std::hash::RandomState;
    /// # use std::alloc::Global;
    /// #
    /// let array: [i32; 6] = [1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32];
    /// let extension: [i32; 4] = [7_i32, 8_i32, 9_i32, 10_i32];
    /// let combined: [i32; 10] = [
    ///     1_i32,
    ///     2_i32,
    ///     3_i32,
    ///     4_i32,
    ///     5_i32,
    ///     6_i32,
    ///     7_i32,
    ///     8_i32,
    ///     9_i32,
    ///     10_i32,
    /// ];
    /// let expected = OpaqueIndexSet::from(combined);
    /// #
    /// # assert!(expected.has_value_type::<i32>());
    /// # assert!(expected.has_build_hasher_type::<RandomState>());
    /// # assert!(expected.has_allocator_type::<Global>());
    /// #
    /// let mut result = OpaqueIndexSet::from(array);
    /// #
    /// # assert!(result.has_value_type::<i32>());
    /// # assert!(result.has_build_hasher_type::<RandomState>());
    /// # assert!(result.has_allocator_type::<Global>());
    /// #
    /// result.extend::<_, i32, RandomState, Global>(extension.iter().cloned());
    ///
    /// assert_eq!(result.len(), expected.len());
    /// assert_eq!(result.as_slice::<i32, RandomState, Global>(), expected.as_slice::<i32, RandomState, Global>());
    /// ```
    #[inline]
    pub fn extend<I, T, S, A>(&mut self, iterable: I)
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        I: IntoIterator<Item = T>,
    {
        let proj_self = self.as_proj_mut::<T, S, A>();

        proj_self.extend(iterable)
    }
}

impl fmt::Debug for OpaqueIndexSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpaqueIndexSet")
            .finish()
    }
}

#[cfg(feature = "std")]
impl<T> FromIterator<T> for OpaqueIndexSet
where
    T: any::Any + hash::Hash + Eq,
{
    fn from_iter<I>(iterable: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let proj_set = TypedProjIndexSet::<T, hash::RandomState, alloc::Global>::from_iter(iterable);

        Self::from_proj(proj_set)
    }
}

#[cfg(feature = "std")]
impl<T, const N: usize> From<[T; N]> for OpaqueIndexSet
where
    T: any::Any + hash::Hash + Eq,
{
    fn from(array: [T; N]) -> Self {
        let proj_set = TypedProjIndexSet::<T, hash::RandomState, alloc::Global>::from(array);

        Self::from_proj(proj_set)
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
        fn write(&mut self, _bytes: &[u8]) {
            panic!("[`DummyHasher::write`] should never actually be called. Its purpose is to test struct layouts.");
        }

        #[inline]
        fn finish(&self) -> u64 {
            panic!("[`DummyHasher::finish`] should never actually be called. Its purpose is to test struct layouts.");
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
mod index_set_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_index_set_match_sizes<T, S, A>()
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::size_of::<TypedProjIndexSet<T, S, A>>();
        let result = mem::size_of::<OpaqueIndexSet>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_index_set_match_alignments<T, S, A>()
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::align_of::<TypedProjIndexSet<T, S, A>>();
        let result = mem::align_of::<OpaqueIndexSet>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_index_set_match_offsets<T, S, A>()
    where
        T: any::Any,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::offset_of!(TypedProjIndexSet<T, S, A>, inner);
        let result = mem::offset_of!(OpaqueIndexSet, inner);

        assert_eq!(result, expected, "Opaque and Typed Projected data types offsets mismatch");
    }

    macro_rules! layout_tests {
        ($module_name:ident, $value_typ:ty, $build_hasher_typ:ty, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_opaque_index_set_layout_match_sizes() {
                    run_test_opaque_index_set_match_sizes::<$value_typ, $build_hasher_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_index_set_layout_match_alignments() {
                    run_test_opaque_index_set_match_alignments::<$value_typ, $build_hasher_typ, $alloc_typ>();
                }

                #[test]
                fn test_opaque_index_set_layout_match_offsets() {
                    run_test_opaque_index_set_match_offsets::<$value_typ, $build_hasher_typ, $alloc_typ>();
                }
            }
        };
    }

    #[cfg(feature = "std")]
    layout_tests!(unit_zst_random_state_global, (), hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u8_random_state_global, u8, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(u64_random_state_global, u64, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(str_random_state_global, &'static str, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(tangent_space_random_state_global, layout_testing_types::TangentSpace, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(surface_differential_random_state_global, layout_testing_types::SurfaceDifferential, hash::RandomState, alloc::Global);

    #[cfg(feature = "std")]
    layout_tests!(oct_tree_node_random_state_global, layout_testing_types::OctTreeNode, hash::RandomState, alloc::Global);

    layout_tests!(unit_zst_dummy_hasher_dummy_alloc, (), dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u8_dummy_hasher_dummy_alloc, u8, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(u64_dummy_hasher_dummy_alloc, u64, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(str_dummy_hasher_dummy_alloc, &'static str, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(tangent_space_dummy_hasher_dummy_alloc, layout_testing_types::TangentSpace, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(surface_differential_dummy_hasher_dummy_alloc, layout_testing_types::SurfaceDifferential, dummy::DummyBuildHasher, dummy::DummyAlloc);
    layout_tests!(oct_tree_node_dummy_hasher_dummy_alloc, layout_testing_types::OctTreeNode, dummy::DummyBuildHasher, dummy::DummyAlloc);
}

#[cfg(test)]
mod index_set_assert_send_sync {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjIndexSet<i32, hash::RandomState, alloc::Global>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjIndexSet<i32, dummy::DummyBuildHasher, alloc::Global>>();
    }
}
