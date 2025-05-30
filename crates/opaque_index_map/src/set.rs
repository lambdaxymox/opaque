use crate::map_inner;
use crate::map_inner::{Bucket, OpaqueIndexMapInner};
use crate::range_ops;
use crate::slice_eq;
use crate::equivalent::Equivalent;

use core::any;
use core::cmp;
use core::fmt;
use core::iter;
use core::ops;
use std::alloc;
use std::hash;

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
pub struct IntoIter<T, A>
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
        let iter = self.iter.as_slice().iter().map(|tuple| tuple.0);
        formatter.debug_list().entries(iter).finish()
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

pub struct Drain<'a, T, A>
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
        let iter = self.iter.as_slice().iter().map(|tuple| tuple.0);
        formatter.debug_list().entries(iter).finish()
    }
}

pub struct Difference<'a, T, S, A>
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

pub struct Intersection<'a, T, S, A>
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

pub struct SymmetricDifference<'a, T, S1, S2, A>
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

pub struct Union<'a, T, S, A>
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

pub struct Splice<'a, I, T, S, A>
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

#[repr(transparent)]
pub struct TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: map_inner::TypedProjIndexMapInner<T, (), S, A>,
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

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub fn with_hasher_proj_in(proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::<T, (), S, A>::with_hasher_proj_in(proj_build_hasher, proj_alloc);

        Self {
            inner: proj_inner,
        }
    }

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

impl<T, A> TypedProjIndexSet<T, hash::RandomState, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub fn new_proj_in(proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::<T, (), hash::RandomState, A>::new_proj_in(proj_alloc);

        Self {
            inner : proj_inner,
        }
    }

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
    pub fn with_capacity_and_hasher_in(capacity: usize, build_hasher: S, alloc: A) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::with_capacity_and_hasher_in(capacity, build_hasher, alloc);

        TypedProjIndexSet {
            inner: proj_inner,
        }
    }

    pub fn with_hasher_in(build_hasher: S, alloc: A) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::with_hasher_in(build_hasher, alloc);

        TypedProjIndexSet {
            inner: proj_inner,
        }
    }
}

impl<T, A> TypedProjIndexSet<T, hash::RandomState, A>
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub fn new_in(alloc: A) -> Self {
        let proj_inner = map_inner::TypedProjIndexMapInner::<T, (), hash::RandomState, A>::new_in(alloc);

        Self {
            inner : proj_inner,
        }
    }

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
    pub fn with_capacity_and_hasher(capacity: usize, build_hasher: S) -> Self {
        TypedProjIndexSet {
            inner: map_inner::TypedProjIndexMapInner::with_capacity_and_hasher(capacity, build_hasher),
        }
    }

    pub fn with_hasher(build_hasher: S) -> Self {
        TypedProjIndexSet {
            inner: map_inner::TypedProjIndexMapInner::with_hasher(build_hasher),
        }
    }
}

impl<T> TypedProjIndexSet<T, hash::RandomState, alloc::Global>
where
    T: any::Any,
{
    pub fn new() -> Self {
        TypedProjIndexSet {
            inner: map_inner::TypedProjIndexMapInner::new(),
        }
    }

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
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

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
    #[inline]
    pub const fn hasher(&self) -> &TypedProjBuildHasher<S> {
        self.inner.hasher()
    }

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
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.as_entries())
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    #[track_caller]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, A>
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
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub fn insert(&mut self, value: T) -> bool {
        self.inner.insert(value, ()).is_none()
    }

    pub fn insert_full(&mut self, value: T) -> (usize, bool) {
        let (index, existing) = self.inner.insert_full(value, ());

        (index, existing.is_none())
    }

    pub fn insert_sorted(&mut self, value: T) -> (usize, bool)
    where
        T: Ord,
    {
        let (index, existing) = self.inner.insert_sorted(value, ());
        (index, existing.is_none())
    }

    #[track_caller]
    pub fn insert_before(&mut self, index: usize, value: T) -> (usize, bool) {
        let (index, existing) = self.inner.insert_before(index, value, ());
        (index, existing.is_none())
    }

    #[track_caller]
    pub fn shift_insert(&mut self, index: usize, value: T) -> bool {
        self.inner.shift_insert(index, value, ()).is_none()
    }

    pub fn replace(&mut self, value: T) -> Option<T> {
        self.replace_full(value).1
    }

    pub fn replace_full(&mut self, value: T) -> (usize, Option<T>) {
        match self.inner.replace_full(value, ()) {
            (i, Some((replaced, ()))) => (i, Some(replaced)),
            (i, None) => (i, None),
        }
    }

    pub fn difference<'a, S2>(&'a self, other: &'a TypedProjIndexSet<T, S2, A>) -> Difference<'a, T, S2, A>
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Difference::new(self, other)
    }

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

    pub fn intersection<'a, S2>(&'a self, other: &'a TypedProjIndexSet<T, S2, A>) -> Intersection<'a, T, S2, A>
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Intersection::new(self, other)
    }

    pub fn union<'a, S2>(&'a self, other: &'a TypedProjIndexSet<T, S2, A>) -> Union<'a, T, S, A>
    where
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Union::new(self, other)
    }

    #[track_caller]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, T, S, A>
    where
        R: ops::RangeBounds<usize>,
        A: any::Any + alloc::Allocator + Clone,
        I: IntoIterator<Item = T>,
    {
        Splice::new(self, range, replace_with.into_iter())
    }

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
    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.contains_key(value)
    }

    pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.get_key_value(value).map(|(x, &())| x)
    }

    pub fn get_full<Q>(&self, value: &Q) -> Option<(usize, &T)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.get_full(value).map(|(i, x, &())| (i, x))
    }

    pub fn get_index_of<Q>(&self, value: &Q) -> Option<usize>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.get_index_of(value)
    }

    pub fn swap_remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.swap_remove(value).is_some()
    }

    pub fn shift_remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.shift_remove(value).is_some()
    }

    pub fn swap_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.swap_remove_entry(value).map(|(x, ())| x)
    }

    pub fn shift_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.shift_remove_entry(value).map(|(x, ())| x)
    }

    pub fn swap_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.swap_remove_full(value).map(|(i, x, ())| (i, x))
    }

    pub fn shift_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<T>,
    {
        self.inner.shift_remove_full(value).map(|(i, x, ())| (i, x))
    }
}

impl<T, S, A> TypedProjIndexSet<T, S, A>
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[doc(alias = "pop_last")] // like `BTreeSet`
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
        formatter.debug_map().entries(self.iter()).finish()
    }
}

impl<T, S> FromIterator<T> for TypedProjIndexSet<T, S, alloc::Global>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn from_iter<I: IntoIterator<Item = T>>(iterable: I) -> Self {
        let iter = iterable.into_iter();
        let (low, _) = iter.size_hint();
        let mut set = Self::with_capacity_and_hasher_in(low, S::default(), alloc::Global::default());
        set.extend(iter);

        set
    }
}

impl<T, S, const N: usize> From<[T; N]> for TypedProjIndexSet<T, S, alloc::Global>
where
    T: any::Any + hash::Hash + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
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
        let iter = iterable.into_iter().map(|x| (x, ()));
        self.inner.extend(iter);
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
        let iter = iterable.into_iter().copied();
        self.extend(iter);
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
        let iter = self.intersection(other).cloned();
        let capacity = Ord::max(self.len(), other.len());
        let mut set = TypedProjIndexSet::with_capacity_and_hasher_in(capacity, S1::default(), A::default());
        set.extend(iter);

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
        let iter = self.union(other).cloned();
        let capacity = Ord::max(self.len(), other.len());
        let mut set = TypedProjIndexSet::with_capacity_and_hasher_in(capacity, S1::default(), A::default());
        set.extend(iter);

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
        let iter = self.symmetric_difference(other).cloned();
        let capacity = Ord::max(self.len(), other.len());
        let mut set = TypedProjIndexSet::with_capacity_and_hasher_in(capacity, S1::default(), A::default());
        set.extend(iter);

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
        let iter = self.difference(other).cloned();
        let capacity = Ord::max(self.len(), other.len());
        let mut set = TypedProjIndexSet::with_capacity_and_hasher_in(capacity, S1::default(), A::default());
        set.extend(iter);

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

#[repr(transparent)]
pub struct OpaqueIndexSet {
    inner: map_inner::OpaqueIndexMapInner,
}

impl OpaqueIndexSet {
    #[inline]
    pub const fn value_type_id(&self) -> any::TypeId {
        self.inner.key_type_id()
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
    pub fn has_value_type<T>(&self) -> bool
    where
        T: any::Any,
    {
        self.inner.key_type_id() == any::TypeId::of::<T>()
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

impl OpaqueIndexSet {
    pub fn new_proj_in<T, A>(proj_alloc: TypedProjAlloc<A>) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, hash::RandomState, A>::new_proj_in(proj_alloc);

        Self::from_proj(proj_index_set)
    }

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
    pub fn new_in<T, A>(alloc: A) -> Self
    where
        T: any::Any,
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_index_set = TypedProjIndexSet::<T, _, A>::new_in(alloc);

        Self::from_proj(proj_index_set)
    }

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
    #[inline]
    pub fn new<T>() -> Self
    where
        T: any::Any,
    {
        Self::new_in::<T, alloc::Global>(alloc::Global)
    }

    #[inline]
    pub fn with_capacity<T>(capacity: usize) -> Self
    where
        T: any::Any,
    {
        Self::with_capacity_in::<T, alloc::Global>(capacity, alloc::Global)
    }
}

impl OpaqueIndexSet {
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

impl OpaqueIndexSet {
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

    pub fn difference<'a, S2, T, S, A>(&'a self, other: &'a TypedProjIndexSet<T, S2, A>) -> Difference<'a, T, S2, A>
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.difference(other)
    }

    pub fn symmetric_difference<'a, S2, T, S, A>(&'a self, other: &'a TypedProjIndexSet<T, S2, A>) -> SymmetricDifference<'a, T, S, S2, A>
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.symmetric_difference(other)
    }

    pub fn intersection<'a, S2, T, S, A>(&'a self, other: &'a TypedProjIndexSet<T, S2, A>) -> Intersection<'a, T, S2, A>
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.intersection(other)
    }

    pub fn union<'a, S2, T, S, A>(&'a self, other: &'a TypedProjIndexSet<T, S2, A>) -> Union<'a, T, S, A>
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<T, S, A>();

        proj_self.union(other)
    }

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

    pub fn append<S2, A2, T, S, A>(&mut self, other: &mut TypedProjIndexSet<T, S2, A2>)
    where
        T: any::Any + hash::Hash + Eq,
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync,
        S2: any::Any + hash::BuildHasher + Send + Sync,
        S2::Hasher: any::Any + hash::Hasher + Send + Sync,
        A2: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_self = self.as_proj_mut::<T, S, A2>();

        proj_self.append(other)
    }
}

impl OpaqueIndexSet {
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
    #[doc(alias = "pop_last")] // like `BTreeSet`
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

impl fmt::Debug for OpaqueIndexSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpaqueIndexSet")
            .finish()
    }
}

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

impl<T, const N: usize> From<[T; N]> for OpaqueIndexSet
where
    T: any::Any + hash::Hash + Eq,
{
    fn from(arr: [T; N]) -> Self {
        let proj_set = TypedProjIndexSet::<T, hash::RandomState, alloc::Global>::from(arr);

        Self::from_proj(proj_set)
    }
}

#[cfg(test)]
mod index_set_layout_tests {
    use super::*;
    use core::mem;
    use core::ptr::NonNull;

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

    struct DummyBuildHasher {}

    impl hash::BuildHasher for DummyBuildHasher {
        type Hasher = hash::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            Default::default()
        }
    }

    struct DummyAlloc {}

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            alloc::Global.allocate(layout)
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
            unsafe {
                alloc::Global.deallocate(ptr, layout)
            }
        }
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

    layout_tests!(u8_u8_random_state_global, u8, hash::RandomState, alloc::Global);
    layout_tests!(u64_pair_dummy_hasher_dummy_alloc, u64, DummyBuildHasher, DummyAlloc);
    layout_tests!(unit_str_zst_hasher_dummy_alloc, (), DummyBuildHasher, DummyAlloc);
}

#[cfg(test)]
mod index_set_assert_send_sync {
    use super::*;

    #[test]
    fn test_assert_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjIndexSet<i32, hash::RandomState, alloc::Global>>();
    }
}
