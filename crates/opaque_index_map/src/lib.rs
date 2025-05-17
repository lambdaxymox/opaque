#![feature(allocator_api)]
#![feature(slice_range)]
#![feature(slice_iter_mut_as_mut_slice)]
#![feature(optimize_attribute)]
use core::cmp;
use core::ops;
use core::any;
use core::any::TypeId;
use std::alloc;
use std::fmt;
use std::hash;
use std::iter;
use std::marker::PhantomData;

use opaque_alloc;
use opaque_error::{
    TryReserveError,
    TryReserveErrorKind,
};
use opaque_hash;
use opaque_vec::{OpaqueVec, TypedProjVec};

pub use equivalent::Equivalent;
use opaque_alloc::TypedProjAlloc;
use opaque_hash::{OpaqueBuildHasher, TypedProjBuildHasher};

pub struct Drain<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    iter: opaque_vec::Drain<'a, Bucket<K, V>, A>,
}

impl<'a, K, V, A> Drain<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    const fn new(iter: opaque_vec::Drain<'a, Bucket<K, V>, A>) -> Self {
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
    A: any::Any + alloc::Allocator,
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
    A: any::Any + alloc::Allocator,
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
    A: any::Any + alloc::Allocator,
{
    fn len(&self) -> usize {
        <opaque_vec::Drain<'_, Bucket<K, V>, A> as ExactSizeIterator>::len(&self.iter)
    }
}

impl<K, V, A> iter::FusedIterator for Drain<'_, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
}

impl<K, V, A> fmt::Debug for Drain<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::refs);
        formatter.debug_list().entries(iter).finish()
    }
}

pub struct Keys<'a, K, V> {
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

impl<'a, K, V> core::ops::Index<usize> for Keys<'a, K, V> {
    type Output = K;

    fn index(&self, index: usize) -> &K {
        &self.iter.as_slice()[index].key
    }
}

pub struct IntoKeys<K, V, A> {
    iter: opaque_vec::IntoIter<Bucket<K, V>, A>,
}

impl<K, V, A> IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn new(entries: TypedProjVec<Bucket<K, V>, A>) -> Self {
        Self { iter: entries.into_iter() }
    }
}

impl<K, V, A> Iterator for IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
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
    A: any::Any + alloc::Allocator,
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
    A: any::Any + alloc::Allocator,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V, A> iter::FusedIterator for IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
}

impl<K, V, A> fmt::Debug for IntoKeys<K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::key_ref);
        f.debug_list().entries(iter).finish()
    }
}

impl<K, V, A> Default for IntoKeys<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Default,
{
    fn default() -> Self {
        Self {
            iter: TypedProjVec::new_in(Default::default()).into_iter(),
        }
    }
}

pub struct Values<'a, K, V> {
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

impl<K, V: fmt::Debug> fmt::Debug for Values<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<K, V> Default for Values<'_, K, V> {
    fn default() -> Self {
        Self { iter: [].iter() }
    }
}

pub struct ValuesMut<'a, K, V> {
    iter: core::slice::IterMut<'a, Bucket<K, V>>,
}

impl<'a, K, V> ValuesMut<'a, K, V> {
    fn new(entries: &'a mut [Bucket<K, V>]) -> Self {
        Self { iter: entries.iter_mut() }
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

impl<K, V: fmt::Debug> fmt::Debug for ValuesMut<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::value_ref);
        f.debug_list().entries(iter).finish()
    }
}

impl<K, V> Default for ValuesMut<'_, K, V> {
    fn default() -> Self {
        Self { iter: [].iter_mut() }
    }
}

pub struct IntoValues<K, V, A> {
    iter: opaque_vec::IntoIter<Bucket<K, V>, A>,
}

impl<K, V, A> IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn new(entries: TypedProjVec<Bucket<K, V>, A>) -> Self {
        Self { iter: entries.into_iter() }
    }
}

impl<K, V, A> Iterator for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
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
    A: any::Any + alloc::Allocator,
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
    A: any::Any + alloc::Allocator,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V, A> iter::FusedIterator for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
}

impl<K, V, A> fmt::Debug for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::value_ref);
        f.debug_list().entries(iter).finish()
    }
}

impl<K, V, A> Default for IntoValues<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Default,
{
    fn default() -> Self {
        Self {
            iter: TypedProjVec::new_in(Default::default()).into_iter(),
        }
    }
}

fn try_simplify_range<R>(range: R, len: usize) -> Option<ops::Range<usize>>
where
    R: ops::RangeBounds<usize>,
{
    let start = match range.start_bound() {
        ops::Bound::Unbounded => 0,
        ops::Bound::Included(&i) if i <= len => i,
        ops::Bound::Excluded(&i) if i < len => i + 1,
        _ => return None,
    };
    let end = match range.end_bound() {
        ops::Bound::Unbounded => len,
        ops::Bound::Excluded(&i) if i <= len => i,
        ops::Bound::Included(&i) if i < len => i + 1,
        _ => return None,
    };

    if start > end {
        return None;
    }

    Some(start..end)
}

#[track_caller]
fn simplify_range<R>(range: R, len: usize) -> ops::Range<usize>
where
    R: ops::RangeBounds<usize>,
{
    let start = match range.start_bound() {
        ops::Bound::Unbounded => 0,
        ops::Bound::Included(&i) if i <= len => i,
        ops::Bound::Excluded(&i) if i < len => i + 1,
        ops::Bound::Included(i) | ops::Bound::Excluded(i) => {
            panic!("range start index {i} out of range for slice of length {len}")
        }
    };
    let end = match range.end_bound() {
        ops::Bound::Unbounded => len,
        ops::Bound::Excluded(&i) if i <= len => i,
        ops::Bound::Included(&i) if i < len => i + 1,
        ops::Bound::Included(i) | ops::Bound::Excluded(i) => {
            panic!("range end index {i} out of range for slice of length {len}")
        }
    };

    if start > end {
        panic!(
            "range start index {:?} should be <= range end index {:?}",
            range.start_bound(),
            range.end_bound()
        );
    }

    start..end
}

#[repr(transparent)]
pub struct Slice<K, V> {
    entries: [Bucket<K, V>],
}

impl<K, V> Slice<K, V> {
    const fn from_slice(entries: &[Bucket<K, V>]) -> &Self {
        unsafe { &*(entries as *const [Bucket<K, V>] as *const Self) }
    }

    const fn from_slice_mut(entries: &mut [Bucket<K, V>]) -> &mut Self {
        unsafe { &mut *(entries as *mut [Bucket<K, V>] as *mut Self) }
    }

    fn from_boxed_slice<A>(entries: Box<[Bucket<K, V>], A>) -> Box<Self, A>
    where
        A: any::Any + alloc::Allocator,
    {
        unsafe {
            let (ptr, alloc) = Box::into_raw_with_allocator(entries);

            Box::from_raw_in(ptr as *mut Self, alloc)
        }
    }

    fn into_boxed_slice<A>(self: Box<Self, A>) -> Box<[Bucket<K, V>], A>
    where
        A: any::Any + alloc::Allocator,
    {
        unsafe {
            let (ptr, alloc) = Box::into_raw_with_allocator(self);

            Box::from_raw_in(ptr as *mut [Bucket<K, V>], alloc)
        }
    }

    pub(crate) fn into_entries<A>(self: Box<Self, TypedProjAlloc<A>>) -> TypedProjVec<Bucket<K, V>, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        unsafe {
            let len = self.entries.len();
            let capacity = len;
            let (ptr, alloc) = Box::into_raw_with_allocator(self.into_boxed_slice());

            TypedProjVec::from_raw_parts_proj_in(ptr as *mut Bucket<K, V>, len, capacity, alloc)
        }
    }

    pub const fn new<'a>() -> &'a Self {
        Self::from_slice(&[])
    }

    pub fn new_mut<'a>() -> &'a mut Self {
        Self::from_slice_mut(&mut [])
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
        self.entries.get(index).map(Bucket::refs)
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.entries.get_mut(index).map(Bucket::ref_mut)
    }

    pub fn get_range<R>(&self, range: R) -> Option<&Self>
    where
        R: ops::RangeBounds<usize>,
    {
        let range = try_simplify_range(range, self.entries.len())?;

        self.entries.get(range).map(Slice::from_slice)
    }

    pub fn get_range_mut<R>(&mut self, range: R) -> Option<&mut Self>
    where
        R: ops::RangeBounds<usize>,
    {
        let range = try_simplify_range(range, self.entries.len())?;

        self.entries.get_mut(range).map(Slice::from_slice_mut)
    }

    pub fn first(&self) -> Option<(&K, &V)> {
        self.entries.first().map(Bucket::refs)
    }

    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.entries.first_mut().map(Bucket::ref_mut)
    }

    pub fn last(&self) -> Option<(&K, &V)> {
        self.entries.last().map(Bucket::refs)
    }

    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        self.entries.last_mut().map(Bucket::ref_mut)
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
        if let [first, rest @ ..] = &self.entries {
            Some((first.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }

    pub fn split_first_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [first, rest @ ..] = &mut self.entries {
            Some((first.ref_mut(), Self::from_slice_mut(rest)))
        } else {
            None
        }
    }

    pub fn split_last(&self) -> Option<((&K, &V), &Self)> {
        if let [rest @ .., last] = &self.entries {
            Some((last.refs(), Self::from_slice(rest)))
        } else {
            None
        }
    }

    pub fn split_last_mut(&mut self) -> Option<((&K, &mut V), &mut Self)> {
        if let [rest @ .., last] = &mut self.entries {
            Some((last.ref_mut(), Self::from_slice_mut(rest)))
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(&self.entries)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(&mut self.entries)
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys::new(&self.entries)
    }

    pub fn into_keys<A>(self: Box<Self, TypedProjAlloc<A>>) -> IntoKeys<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        IntoKeys::new(self.into_entries())
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values::new(&self.entries)
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut::new(&mut self.entries)
    }

    pub fn into_values<A>(self: Box<Self, TypedProjAlloc<A>>) -> IntoValues<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        IntoValues::new(self.into_entries())
    }

    pub fn binary_search_keys(&self, x: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        self.binary_search_by(|p, _| p.cmp(x))
    }

    #[inline]
    pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> cmp::Ordering,
    {
        self.entries.binary_search_by(move |a| f(&a.key, &a.value))
    }

    #[inline]
    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> B,
        B: Ord,
    {
        self.binary_search_by(|k, v| f(k, v).cmp(b))
    }

    #[must_use]
    pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {
        self.entries.partition_point(move |a| pred(&a.key, &a.value))
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

impl<K, V, A> IntoIterator for Box<Slice<K, V>, opaque_alloc::TypedProjAlloc<A>>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
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

impl<K, V, A> Default for Box<Slice<K, V>, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Default,
{
    fn default() -> Self {
        Slice::from_boxed_slice(Box::new_in([], Default::default()))
    }
}

impl<K, V, A> Clone for Box<Slice<K, V>, A>
where
    K: Clone,
    V: Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    fn clone(&self) -> Self {
        let alloc = Box::<Slice<K, V>, A>::allocator(&self).clone();
        Slice::from_boxed_slice(self.entries.to_vec_in(alloc).into_boxed_slice())
    }
}
/*
impl<K, V> From<&Slice<K, V>> for Box<Slice<K, V>, opaque_alloc::OpaqueAlloc>
where
    K: Copy,
    V: Copy,
{
    fn from(slice: &Slice<K, V>) -> Self {
        Slice::from_boxed_slice(Box::from(&slice.entries))
    }
}
*/
impl<K, V> fmt::Debug for Slice<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

// Generic slice equality -- copied from the standard library but adding a custom comparator,
// allowing for our `Bucket` wrapper on either or both sides.
pub(crate) fn slice_eq<T, U>(left: &[T], right: &[U], eq: impl Fn(&T, &U) -> bool) -> bool {
    if left.len() != right.len() {
        return false;
    }

    // Implemented as explicit indexing rather
    // than zipped iterators for performance reasons.
    // See PR https://github.com/rust-lang/rust/pull/116846
    for i in 0..left.len() {
        // bound checks are optimized away
        if !eq(&left[i], &right[i]) {
            return false;
        }
    }

    true
}

impl<K, V, K2, V2> PartialEq<Slice<K2, V2>> for Slice<K, V>
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &Slice<K2, V2>) -> bool {
        slice_eq(&self.entries, &other.entries, |b1, b2| b1.key == b2.key && b1.value == b2.value)
    }
}

impl<K, V, K2, V2> PartialEq<[(K2, V2)]> for Slice<K, V>
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &[(K2, V2)]) -> bool {
        slice_eq(&self.entries, other, |b, t| b.key == t.0 && b.value == t.1)
    }
}

impl<K, V, K2, V2> PartialEq<Slice<K2, V2>> for [(K, V)]
where
    K: PartialEq<K2>,
    V: PartialEq<V2>,
{
    fn eq(&self, other: &Slice<K2, V2>) -> bool {
        slice_eq(self, &other.entries, |t, b| t.0 == b.key && t.1 == b.value)
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

impl<K: Eq, V: Eq> Eq for Slice<K, V> {}

impl<K: PartialOrd, V: PartialOrd> PartialOrd for Slice<K, V> {
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
    type Output = V;

    fn index(&self, index: usize) -> &V {
        &self.entries[index].value
    }
}

impl<K, V> ops::IndexMut<usize> for Slice<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut V {
        &mut self.entries[index].value
    }
}

impl<K, V> ops::Index<ops::Range<usize>> for Slice<K, V> {
    type Output = Slice<K, V>;

    fn index(&self, range: ops::Range<usize>) -> &Self {
        Self::from_slice(&self.entries[range])
    }
}

impl<K, V> ops::IndexMut<ops::Range<usize>> for Slice<K, V> {
    fn index_mut(&mut self, range: ops::Range<usize>) -> &mut Self {
        Self::from_slice_mut(&mut self.entries[range])
    }
}

impl<K, V> ops::Index<ops::RangeFrom<usize>> for Slice<K, V> {
    type Output = Slice<K, V>;

    fn index(&self, range: ops::RangeFrom<usize>) -> &Self {
        Self::from_slice(&self.entries[range])
    }
}

impl<K, V> ops::IndexMut<ops::RangeFrom<usize>> for Slice<K, V> {
    fn index_mut(&mut self, range: ops::RangeFrom<usize>) -> &mut Self {
        Self::from_slice_mut(&mut self.entries[range])
    }
}

impl<K, V> ops::Index<ops::RangeFull> for Slice<K, V> {
    type Output = Slice<K, V>;

    fn index(&self, range: ops::RangeFull) -> &Self {
        Self::from_slice(&self.entries[range])
    }
}

impl<K, V> ops::IndexMut<ops::RangeFull> for Slice<K, V> {
    fn index_mut(&mut self, range: ops::RangeFull) -> &mut Self {
        Self::from_slice_mut(&mut self.entries[range])
    }
}

impl<K, V> ops::Index<ops::RangeInclusive<usize>> for Slice<K, V> {
    type Output = Slice<K, V>;

    fn index(&self, range: ops::RangeInclusive<usize>) -> &Self {
        Self::from_slice(&self.entries[range])
    }
}

impl<K, V> ops::IndexMut<ops::RangeInclusive<usize>> for Slice<K, V> {
    fn index_mut(&mut self, range: ops::RangeInclusive<usize>) -> &mut Self {
        Self::from_slice_mut(&mut self.entries[range])
    }
}

impl<K, V> ops::Index<ops::RangeTo<usize>> for Slice<K, V> {
    type Output = Slice<K, V>;

    fn index(&self, range: ops::RangeTo<usize>) -> &Self {
        Self::from_slice(&self.entries[range])
    }
}

impl<K, V> ops::IndexMut<ops::RangeTo<usize>> for Slice<K, V> {
    fn index_mut(&mut self, range: ops::RangeTo<usize>) -> &mut Self {
        Self::from_slice_mut(&mut self.entries[range])
    }
}

impl<K, V> ops::Index<ops::RangeToInclusive<usize>> for Slice<K, V> {
    type Output = Slice<K, V>;

    fn index(&self, range: ops::RangeToInclusive<usize>) -> &Self {
        Self::from_slice(&self.entries[range])
    }
}

impl<K, V> ops::IndexMut<ops::RangeToInclusive<usize>> for Slice<K, V> {
    fn index_mut(&mut self, range: ops::RangeToInclusive<usize>) -> &mut Self {
        Self::from_slice_mut(&mut self.entries[range])
    }
}

impl<K, V> ops::Index<(ops::Bound<usize>, ops::Bound<usize>)> for Slice<K, V> {
    type Output = Slice<K, V>;

    fn index(&self, range: (ops::Bound<usize>, ops::Bound<usize>)) -> &Self {
        Self::from_slice(&self.entries[range])
    }
}

impl<K, V> ops::IndexMut<(ops::Bound<usize>, ops::Bound<usize>)> for Slice<K, V> {
    fn index_mut(&mut self, range: (ops::Bound<usize>, ops::Bound<usize>)) -> &mut Self {
        Self::from_slice_mut(&mut self.entries[range])
    }
}

/*
// We can't have `impl<I: RangeBounds<usize>> Index<I>` because that conflicts
// both upstream with `Index<usize>` and downstream with `Index<&Q>`.
// Instead, we repeat the implementations for all the core range types.
macro_rules! impl_index {
    ($($range:ty),*) => {$(
        impl<K, V, S> Index<$range> for IndexMap<K, V, S> {
            type Output = Slice<K, V>;

            fn index(&self, range: $range) -> &Self::Output {
                Slice::from_slice(&self.as_entries()[range])
            }
        }

        impl<K, V, S> IndexMut<$range> for IndexMap<K, V, S> {
            fn index_mut(&mut self, range: $range) -> &mut Self::Output {
                Slice::from_mut_slice(&mut self.as_entries_mut()[range])
            }
        }
    )*}
}
impl_index!(
    ops::Range<usize>,
    ops::RangeFrom<usize>,
    ops::RangeFull,
    ops::RangeInclusive<usize>,
    ops::RangeTo<usize>,
    ops::RangeToInclusive<usize>,
    (Bound<usize>, Bound<usize>)
);
*/

pub struct Iter<'a, K, V> {
    iter: std::slice::Iter<'a, Bucket<K, V>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    #[inline]
    fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self { iter: entries.iter() }
    }

    fn as_slice(&self) -> &Slice<K, V> {
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

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for Iter<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<K, V> Default for Iter<'_, K, V> {
    fn default() -> Self {
        Self { iter: [].iter() }
    }
}

pub struct IterMut<'a, K, V> {
    iter: std::slice::IterMut<'a, Bucket<K, V>>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    #[inline]
    fn new(entries: &'a mut [Bucket<K, V>]) -> Self {
        Self { iter: entries.iter_mut() }
    }

    fn as_slice_mut(&'a mut self) -> &'a mut Slice<K, V> {
        Slice::from_slice_mut(self.iter.as_mut_slice())
    }

    pub fn into_slice_mut(self) -> &'a mut Slice<K, V> {
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

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for IterMut<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::refs);
        f.debug_list().entries(iter).finish()
    }
}

impl<K, V> Default for IterMut<'_, K, V> {
    fn default() -> Self {
        Self { iter: [].iter_mut() }
    }
}

#[derive(Clone)]
pub struct IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    iter: opaque_vec::IntoIter<Bucket<K, V>, A>,
}

impl<K, V, A> IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn new(entries: TypedProjVec<Bucket<K, V>, A>) -> Self {
        Self {
            iter: entries.into_iter(),
        }
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
    A: any::Any + alloc::Allocator,
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
    A: any::Any + alloc::Allocator,
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
    A: any::Any + alloc::Allocator,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V, A> iter::FusedIterator for IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
}

impl<K, V, A> fmt::Debug for IntoIter<K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::refs);
        formatter.debug_list().entries(iter).finish()
    }
}

impl<K, V, A> Default for IntoIter<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Default,
{
    fn default() -> Self {
        Self {
            iter: TypedProjVec::new_in(Default::default()).into_iter(),
        }
    }
}

pub struct Splice<'a, I, K, V, S, A>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    map: &'a mut TypedProjIndexMapInner<K, V, S, A>,
    tail: TypedProjIndexMapCore<K, V, A>,
    drain: opaque_vec::IntoIter<Bucket<K, V>, A>,
    replace_with: I,
    _marker: PhantomData<S>,
}

impl<'a, I, K, V, S, A> Splice<'a, I, K, V, S, A>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator + Clone,
{
    #[track_caller]
    fn new<R>(map: &'a mut TypedProjIndexMapInner<K, V, S, A>, range: R, replace_with: I) -> Self
    where
        R: ops::RangeBounds<usize>,
    {
        let (tail, drain) = map.inner.split_splice::<R>(range);
        Self {
            map,
            tail,
            drain,
            replace_with,
            _marker: PhantomData,
        }
    }
}

impl<I, K, V, S, A> Drop for Splice<'_, I, K, V, S, A>
where
    I: Iterator<Item = (K, V)>,
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
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
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
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
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
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
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
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
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
}

impl<I, K, V, S, A> fmt::Debug for Splice<'_, I, K, V, S, A>
where
    I: fmt::Debug + Iterator<Item = (K, V)>,
    K: any::Any + fmt::Debug + hash::Hash + Eq,
    V: any::Any + fmt::Debug,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Follow `vec::Splice` in only printing the drain and replacement
        f.debug_struct("Splice")
            .field("drain", &self.drain)
            .field("replace_with", &self.replace_with)
            .finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct HashValue {
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
struct Bucket<K, V> {
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
    #[inline]
    const fn new(hash: HashValue, key: K, value: V) -> Self {
        Self { hash, key, value }
    }

    fn key_ref(&self) -> &K {
        &self.key
    }

    fn value_ref(&self) -> &V {
        &self.value
    }

    fn value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    fn key(self) -> K {
        self.key
    }

    fn value(self) -> V {
        self.value
    }

    fn key_value(self) -> (K, V) {
        (self.key, self.value)
    }

    fn refs(&self) -> (&K, &V) {
        (&self.key, &self.value)
    }

    fn ref_mut(&mut self) -> (&K, &mut V) {
        (&self.key, &mut self.value)
    }

    fn muts(&mut self) -> (&mut K, &mut V) {
        (&mut self.key, &mut self.value)
    }
}

pub(crate) struct OpaqueIndexMapCoreInner {
    indices: hashbrown::HashTable<usize>,
    entries: OpaqueVec,
    key_type_id: TypeId,
    value_type_id: TypeId,
    allocator_type_id: TypeId,
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
    (isize::MAX as usize) / core::mem::size_of::<Bucket<K, V>>()
}

fn reserve_entries<K, V, A>(entries: &mut TypedProjVec<Bucket<K, V>, A>, additional: usize, try_capacity: usize)
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
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
    A: any::Any + alloc::Allocator,
{
    indices: &'a mut hashbrown::HashTable<usize>,
    entries: &'a mut TypedProjVec<Bucket<K, V>, A>,
    _marker: core::marker::PhantomData<(K, V, A)>,
}

impl<'a, K, V, A> RefMut<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    fn new(indices: &'a mut hashbrown::HashTable<usize>, entries: &'a mut TypedProjVec<Bucket<K, V>, A>) -> Self {
        Self {
            indices,
            entries,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    fn reserve_entries(&mut self, additional: usize) {
        reserve_entries::<K, V, A>(self.entries, additional, self.indices.capacity());
    }

    fn insert_unique(self, hash: HashValue, key: K, value: V) -> OccupiedEntry<'a, K, V, A> {
        let i = self.indices.len();
        debug_assert_eq!(i, self.entries.len());
        let entry = self
            .indices
            .insert_unique(hash.get(), i, get_hash(self.entries.as_slice()));
        if self.entries.len() == self.entries.capacity() {
            // We can't call `indices.capacity()` while this `entry` has borrowed it, so we'll have
            // to amortize growth on our own. It's still an improvement over the basic `Vec::push`
            // doubling though, since we also consider `MAX_ENTRIES_CAPACITY`.
            reserve_entries::<K, V, A>(self.entries, 1, 2 * self.entries.capacity());
        }
        self.entries.push(Bucket { hash, key, value });
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
        self.entries.shift_insert(index, Bucket { hash, key, value });
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
            [
                self.entries.as_slice()[a].hash.get(),
                self.entries.as_slice()[b].hash.get(),
            ],
            move |i, &x| if i == 0 { x == a } else { x == b },
        ) {
            [Some(ref_a), Some(ref_b)] => {
                core::mem::swap(ref_a, ref_b);
                self.entries.as_mut_slice().swap(a, b);
            }
            _ => panic!("indices not found"),
        }
    }
}

impl OpaqueIndexMapCoreInner {
    #[inline]
    pub(crate) const fn key_type_id(&self) -> TypeId {
        self.key_type_id
    }

    #[inline]
    pub(crate) const fn value_type_id(&self) -> TypeId {
        self.value_type_id
    }

    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> TypeId {
        self.allocator_type_id
    }
}

impl OpaqueIndexMapCoreInner {
    #[inline]
    pub(crate) fn new_proj_in<K, V, A>(alloc: TypedProjAlloc<A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let indices = hashbrown::HashTable::new();
        let entries = OpaqueVec::new_proj_in::<Bucket<K, V>, A>(alloc);
        let key_type_id = TypeId::of::<K>();
        let value_type_id = TypeId::of::<V>();
        let allocator_type_id = TypeId::of::<A>();

        Self {
            indices,
            entries,
            key_type_id,
            value_type_id,
            allocator_type_id,
        }
    }

    #[inline]
    pub(crate) fn with_capacity_proj_in<K, V, A>(capacity: usize, alloc: TypedProjAlloc<A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let indices = hashbrown::HashTable::with_capacity(capacity);
        let entries = OpaqueVec::with_capacity_proj_in::<Bucket<K, V>, A>(capacity, alloc);
        let key_type_id = TypeId::of::<K>();
        let value_type_id = TypeId::of::<V>();
        let allocator_type_id = TypeId::of::<A>();

        Self {
            indices,
            entries,
            key_type_id,
            value_type_id,
            allocator_type_id,
        }
    }
}

impl OpaqueIndexMapCoreInner {
    #[inline]
    pub(crate) fn new_in<K, V, A>(alloc: A) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let indices = hashbrown::HashTable::new();
        let entries = OpaqueVec::new_in::<Bucket<K, V>, A>(alloc);
        let key_type_id = TypeId::of::<K>();
        let value_type_id = TypeId::of::<V>();
        let allocator_type_id = TypeId::of::<A>();

        Self {
            indices,
            entries,
            key_type_id,
            value_type_id,
            allocator_type_id,
        }
    }

    #[inline]
    pub(crate) fn with_capacity_in<K, V, A>(capacity: usize, alloc: A) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let indices = hashbrown::HashTable::with_capacity(capacity);
        let entries = OpaqueVec::with_capacity_in::<Bucket<K, V>, A>(capacity, alloc);
        let key_type_id = TypeId::of::<K>();
        let value_type_id = TypeId::of::<V>();
        let allocator_type_id = TypeId::of::<A>();

        Self {
            indices,
            entries,
            key_type_id,
            value_type_id,
            allocator_type_id,
        }
    }
}

impl OpaqueIndexMapCoreInner {
    #[inline]
    pub(crate) fn new<K, V>() -> Self
    where
        K: any::Any,
        V: any::Any,
    {
        let indices = hashbrown::HashTable::new();
        let entries = OpaqueVec::new::<Bucket<K, V>>();
        let key_type_id = TypeId::of::<K>();
        let value_type_id = TypeId::of::<V>();
        let allocator_type_id = TypeId::of::<alloc::Global>();

        Self {
            indices,
            entries,
            key_type_id,
            value_type_id,
            allocator_type_id,
        }
    }

    #[inline]
    pub(crate) fn with_capacity<K, V>(capacity: usize) -> Self
    where
        K: any::Any,
        V: any::Any,
    {
        let indices = hashbrown::HashTable::with_capacity(capacity);
        let entries = OpaqueVec::with_capacity::<Bucket<K, V>>(capacity);
        let key_type_id = TypeId::of::<K>();
        let value_type_id = TypeId::of::<V>();
        let allocator_type_id = TypeId::of::<alloc::Global>();

        Self {
            indices,
            entries,
            key_type_id,
            value_type_id,
            allocator_type_id,
        }
    }
}

impl OpaqueIndexMapCoreInner {
    #[inline]
    fn borrow_mut<K, V, A>(&mut self) -> RefMut<'_, K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        RefMut::new(&mut self.indices, self.entries.as_proj_mut::<Bucket<K, V>, A>())
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.indices.len()
    }

    #[inline]
    pub(crate) fn capacity<K, V, A>(&self) -> usize
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        Ord::min(self.indices.capacity(), self.entries.capacity())
    }

    pub(crate) fn clear<K, V, A>(&mut self)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.indices.clear();
        self.entries.clear::<Bucket<K, V>, A>();
    }

    pub(crate) fn truncate<K, V, A>(&mut self, len: usize)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        if len < self.len() {
            self.erase_indices::<K, V, A>(len, self.entries.len());
            self.entries.truncate::<Bucket<K, V>, A>(len);
        }
    }

    #[track_caller]
    pub(crate) fn drain<R, K, V, A>(&mut self, range: R) -> opaque_vec::Drain<'_, Bucket<K, V>, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
        R: ops::RangeBounds<usize>,
    {
        let range = simplify_range(range, self.entries.len());
        self.erase_indices::<K, V, A>(range.start, range.end);

        self.entries.drain::<_, Bucket<K, V>, A>(range)
    }

    /*
    #[cfg(feature = "rayon")]
    pub(crate) fn par_drain<R>(&mut self, range: R) -> rayon::vec::Drain<'_, Bucket<K, V>>
    where
        K: Send,
        V: Send,
        R: RangeBounds<usize>,
    {
        use rayon::iter::ParallelDrainRange;
        let range = simplify_range(range, self.entries.len());
        self.erase_indices(range.start, range.end);
        self.entries.par_drain(range)
    }
    */

    #[track_caller]
    pub(crate) fn split_off<K, V, A>(&mut self, at: usize) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Clone,
    {
        let len = self.entries.len();
        assert!(
            at <= len,
            "index out of bounds: the len is {len} but the index is {at}. Expected index <= len"
        );

        self.erase_indices::<K, V, A>(at, self.entries.len());
        let entries = self.entries.split_off::<Bucket<K, V>, A>(at);

        // let mut indices = Indices::with_capacity(entries.len());
        let mut indices = hashbrown::HashTable::with_capacity(entries.len());
        insert_bulk_no_grow(&mut indices, entries.as_slice::<Bucket<K, V>, A>());

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
    pub(crate) fn split_splice<R, K, V, A>(&mut self, range: R) -> (Self, opaque_vec::IntoIter<Bucket<K, V>, A>)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator + Clone,
        R: ops::RangeBounds<usize>,
    {
        let range = simplify_range(range, self.len());
        self.erase_indices::<K, V, A>(range.start, self.entries.len());
        let entries = self.entries.split_off::<Bucket<K, V>, A>(range.end);
        let drained = self.entries.split_off::<Bucket<K, V>, A>(range.start);

        // let mut indices = Indices::with_capacity(entries.len());
        let mut indices = hashbrown::HashTable::with_capacity(entries.len());
        insert_bulk_no_grow(&mut indices, entries.as_slice::<Bucket<K, V>, A>());

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

    pub(crate) fn append_unchecked<K, V, A>(&mut self, other: &mut Self)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.reserve::<K, V, A>(other.len());
        insert_bulk_no_grow(&mut self.indices, other.entries.as_slice::<Bucket<K, V>, A>());
        self.entries.append::<Bucket<K, V>, A>(&mut other.entries);
        other.indices.clear();
    }

    pub(crate) fn reserve<K, V, A>(&mut self, additional: usize)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.indices.reserve(additional, get_hash(self.entries.as_slice::<Bucket<K, V>, A>()));
        // Only grow entries if necessary, since we also round up capacity.
        if additional > self.entries.capacity() - self.entries.len() {
            self.borrow_mut::<K, V, A>().reserve_entries(additional);
        }
    }

    pub(crate) fn reserve_exact<K, V, A>(&mut self, additional: usize)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.indices.reserve(additional, get_hash(self.entries.as_slice::<Bucket<K, V>, A>()));
        self.entries.reserve_exact::<Bucket<K, V>, A>(additional);
    }

    pub(crate) fn try_reserve<K, V, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        fn from_hashbrown(error: hashbrown::TryReserveError) -> TryReserveError {
            let kind = match error {
                hashbrown::TryReserveError::CapacityOverflow => TryReserveErrorKind::CapacityOverflow,
                hashbrown::TryReserveError::AllocError { layout } => TryReserveErrorKind::AllocError { layout },
            };

            TryReserveError::from(kind)
        }

        self.indices
            .try_reserve(additional, get_hash::<K, V>(self.entries.as_slice::<Bucket<K, V>, A>()))
            .map_err(from_hashbrown)?;
        // Only grow entries if necessary, since we also round up capacity.
        if additional > self.entries.capacity() - self.entries.len() {
            self.try_reserve_entries::<K, V, A>(additional)
        } else {
            Ok(())
        }
    }

    /// The maximum capacity before the `entries` allocation would exceed `isize::MAX`.
    /// TODO: Use the stored Layout information to calculate `core::mem::size_of::<Bucket<K, V>>()`.
    #[inline]
    const fn max_entries_capacity<K, V>() -> usize {
        (isize::MAX as usize) / core::mem::size_of::<Bucket<K, V>>()
    }

    fn try_reserve_entries<K, V, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        // Use a soft-limit on the maximum capacity, but if the caller explicitly
        // requested more, do it and let them have the resulting error.
        let new_capacity = Ord::min(self.indices.capacity(), Self::max_entries_capacity::<K, V>());
        let try_add = new_capacity - self.entries.len();
        if try_add > additional && self.entries.try_reserve_exact::<Bucket<K, V>, A>(try_add).is_ok() {
            return Ok(());
        }

        self.entries.try_reserve_exact::<Bucket<K, V>, A>(additional)
    }

    pub(crate) fn try_reserve_exact<K, V, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        fn from_hashbrown(error: hashbrown::TryReserveError) -> TryReserveError {
            let kind = match error {
                hashbrown::TryReserveError::CapacityOverflow => TryReserveErrorKind::CapacityOverflow,
                hashbrown::TryReserveError::AllocError { layout } => TryReserveErrorKind::AllocError { layout },
            };

            TryReserveError::from(kind)
        }

        self.indices
            .try_reserve(additional, get_hash(self.entries.as_slice::<Bucket<K, V>, A>()))
            .map_err(from_hashbrown)?;
        self.entries.try_reserve_exact::<Bucket<K, V>, A>(additional)
    }

    pub(crate) fn shrink_to_fit<K, V, A>(&mut self)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.shrink_to::<K, V, A>(0)
    }

    pub(crate) fn shrink_to<K, V, A>(&mut self, min_capacity: usize)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.indices
            .shrink_to(min_capacity, get_hash(self.entries.as_slice::<Bucket<K, V>, A>()));
        self.entries.shrink_to::<Bucket<K, V>, A>(min_capacity);
    }

    pub(crate) fn pop<K, V, A>(&mut self) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        if let Some(entry) = self.entries.pop::<Bucket<K, V>, A>() {
            let last = self.entries.len();
            erase_index(&mut self.indices, entry.hash, last);
            Some((entry.key, entry.value))
        } else {
            None
        }
    }

    pub(crate) fn get_index_of<Q, K, V, A>(&self, hash: HashValue, key: &Q) -> Option<usize>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
        Q: ?Sized + Equivalent<K>,
    {
        let eq = equivalent(key, self.entries.as_slice::<Bucket<K, V>, A>());

        self.indices.find(hash.get(), eq).copied()
    }

    fn push_entry<K, V, A>(&mut self, hash: HashValue, key: K, value: V)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        if self.entries.len() == self.entries.capacity() {
            // Reserve our own capacity synced to the indices,
            // rather than letting `Vec::push` just double it.
            self.borrow_mut::<K, V, A>().reserve_entries(1);
        }

        self.entries.push::<Bucket<K, V>, A>(Bucket { hash, key, value });
    }

    pub(crate) fn insert_full<K, V, A>(&mut self, hash: HashValue, key: K, value: V) -> (usize, Option<V>)
    where
        K: any::Any + Eq,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let eq = equivalent(&key, self.entries.as_slice::<Bucket<K, V>, A>());
        let hasher = get_hash(self.entries.as_slice::<Bucket<K, V>, A>());
        match self.indices.entry(hash.get(), eq, hasher) {
            hashbrown::hash_table::Entry::Occupied(entry) => {
                let i = *entry.get();

                (i, Some(core::mem::replace(&mut self.as_entries_mut::<K, V, A>()[i].value, value)))
            }
            hashbrown::hash_table::Entry::Vacant(entry) => {
                let i = self.entries.len();
                entry.insert(i);
                self.push_entry::<K, V, A>(hash, key, value);

                debug_assert_eq!(self.indices.len(), self.entries.len());

                (i, None)
            }
        }
    }

    pub(crate) fn shift_remove_full<Q, K, V, A>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, K, V)>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
        Q: ?Sized + Equivalent<K>,
    {
        let eq = equivalent(key, self.entries.as_slice::<Bucket<K, V>, A>());
        match self.indices.find_entry(hash.get(), eq) {
            Ok(entry) => {
                let (index, _) = entry.remove();
                let (key, value) = self.borrow_mut::<K, V, A>().shift_remove_finish(index);
                Some((index, key, value))
            }
            Err(_) => None,
        }
    }

    #[inline]
    pub(crate) fn shift_remove_index<K, V, A>(&mut self, index: usize) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.borrow_mut::<K, V, A>().shift_remove_index(index)
    }

    #[inline]
    #[track_caller]
    pub(crate) fn move_index<K, V, A>(&mut self, from: usize, to: usize)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.borrow_mut::<K, V, A>().move_index(from, to);
    }

    #[inline]
    #[track_caller]
    pub(crate) fn swap_indices<K, V, A>(&mut self, a: usize, b: usize)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.borrow_mut::<K, V, A>().swap_indices(a, b);
    }

    pub(crate) fn swap_remove_full<Q, K, V, A>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, K, V)>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
        Q: ?Sized + Equivalent<K>,
    {
        let eq = equivalent(key, self.entries.as_slice::<Bucket<K, V>, A>());
        match self.indices.find_entry(hash.get(), eq) {
            Ok(entry) => {
                let (index, _) = entry.remove();
                let (key, value) = self.borrow_mut::<K, V, A>().swap_remove_finish(index);
                Some((index, key, value))
            }
            Err(_) => None,
        }
    }

    #[inline]
    pub(crate) fn swap_remove_index<K, V, A>(&mut self, index: usize) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.borrow_mut::<K, V, A>().swap_remove_index(index)
    }

    fn erase_indices<K, V, A>(&mut self, start: usize, end: usize)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let (init, shifted_entries) = self.entries.as_slice::<Bucket<K, V>, A>().split_at(end);
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

    pub(crate) fn retain_in_order<F, K, V, A>(&mut self, mut keep: F)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
        F: FnMut(&mut K, &mut V) -> bool,
    {
        self.entries
            .retain_mut::<_, Bucket<K, V>, A>(|entry: &mut Bucket<K, V>| keep(&mut entry.key, &mut entry.value));
        if self.entries.len() < self.indices.len() {
            self.rebuild_hash_table::<K, V, A>();
        }
    }

    fn rebuild_hash_table<K, V, A>(&mut self)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.indices.clear();
        insert_bulk_no_grow(&mut self.indices, self.entries.as_slice::<Bucket<K, V>, A>());
    }

    pub(crate) fn reverse<K, V, A>(&mut self)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.entries.reverse::<Bucket<K, V>, A>();

        // No need to save hash indices, can easily calculate what they should
        // be, given that this is an in-place reversal.
        let len = self.entries.len();
        for i in &mut self.indices {
            *i = len - *i - 1;
        }
    }
}

impl OpaqueIndexMapCoreInner {
    #[inline]
    fn into_entries(self) -> OpaqueVec {
        self.entries
    }

    #[inline]
    fn as_entries<K, V, A>(&self) -> &[Bucket<K, V>]
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.entries.as_slice::<Bucket<K, V>, A>()
    }

    #[inline]
    fn as_entries_mut<K, V, A>(&mut self) -> &mut [Bucket<K, V>]
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.entries.as_mut_slice::<Bucket<K, V>, A>()
    }

    #[inline]
    fn with_entries<F, K, V, A>(&mut self, f: F)
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
        F: FnOnce(&mut [Bucket<K, V>]),
    {
        f(self.entries.as_mut_slice::<Bucket<K, V>, A>());

        self.rebuild_hash_table::<K, V, A>();
    }
}

impl OpaqueIndexMapCoreInner {
    pub(crate) fn entry<K, V, A>(&mut self, hash: HashValue, key: K) -> Entry<'_, K, V, A>
    where
        K: any::Any + Eq,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let entries = self.entries.as_proj_mut::<Bucket<K, V>, A>();
        let eq = equivalent(&key, entries.as_slice());
        match self.indices.find_entry(hash.get(), eq) {
            Ok(index) => Entry::Occupied(OccupiedEntry {
                entries,
                index,
                _marker: PhantomData,
            }),
            Err(absent) => Entry::Vacant(VacantEntry {
                map: RefMut::new(absent.into_table(), entries),
                hash,
                key,
            }),
        }
    }
}

impl OpaqueIndexMapCoreInner {
    pub(crate) fn clone<K, V, A>(&self) -> Self
    where
        K: any::Any + Clone,
        V: any::Any + Clone,
        A: any::Any + alloc::Allocator + Clone,
    {
        let cloned_indices = self.indices.clone();
        let cloned_entries = self.entries.clone::<Bucket<K, V>, A>();
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
}

#[repr(transparent)]
struct TypedProjIndexMapCore<K, V, A> {
    inner: OpaqueIndexMapCoreInner,
    _marker: PhantomData<(K, V, A)>,
}

impl<K, V, A> TypedProjIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub(crate) fn new_proj_in(alloc: TypedProjAlloc<A>) -> Self {
        let inner = OpaqueIndexMapCoreInner::new_proj_in::<K, V, A>(alloc);

        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub(crate) fn with_capacity_proj_in(capacity: usize, alloc: TypedProjAlloc<A>) -> Self {
        let inner = OpaqueIndexMapCoreInner::with_capacity_proj_in::<K, V, A>(capacity, alloc);

        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<K, V, A> TypedProjIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub(crate) fn new_in(alloc: A) -> Self {
        let inner = OpaqueIndexMapCoreInner::new_in::<K, V, A>(alloc);

        Self {
            inner,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let inner = OpaqueIndexMapCoreInner::with_capacity_in::<K, V, A>(capacity, alloc);

        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<K, V> TypedProjIndexMapCore<K, V, alloc::Global>
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

impl<K, V, A> TypedProjIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    fn into_entries(self) -> TypedProjVec<Bucket<K, V>, A> {
        self.inner.into_entries().into_proj::<Bucket<K, V>, A>()
    }

    #[inline]
    fn as_entries(&self) -> &[Bucket<K, V>] {
        self.inner.as_entries::<K, V, A>()
    }

    #[inline]
    fn as_entries_mut(&mut self) -> &mut [Bucket<K, V>] {
        self.inner.as_entries_mut::<K, V, A>()
    }

    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Bucket<K, V>]),
    {
        self.inner.with_entries::<F, K, V, A>(f)
    }
}

impl<K, V, A> TypedProjIndexMapCore<K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    fn borrow_mut(&mut self) -> RefMut<'_, K, V, A> {
        self.inner.borrow_mut()
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        self.inner.capacity::<K, V, A>()
    }

    pub(crate) fn clear(&mut self) {
        self.inner.clear::<K, V, A>();
    }

    pub(crate) fn truncate(&mut self, len: usize) {
        self.inner.truncate::<K, V, A>(len);
    }

    #[track_caller]
    pub(crate) fn drain<R>(&mut self, range: R) -> opaque_vec::Drain<'_, Bucket<K, V>, A>
    where
        R: ops::RangeBounds<usize>,
    {
        self.inner.drain::<R, K, V, A>(range)
    }

    /*
    #[cfg(feature = "rayon")]
    pub(crate) fn par_drain<R>(&mut self, range: R) -> rayon::vec::Drain<'_, Bucket<K, V>, A>
    where
        K: any::Any + Send,
        V: any::Any + Send,
        A: any::Any + alloc::Allocator,
        R: ops::RangeBounds<usize>,
    {
        self.inner.par_drain::<R, K, V, A>(range)
    }
    */

    #[track_caller]
    pub(crate) fn split_off(&mut self, at: usize) -> Self
    where
        A: Clone,
    {
        let inner = self.inner.split_off::<K, V, A>(at);

        Self {
            inner,
            _marker: PhantomData,
        }
    }

    #[track_caller]
    pub(crate) fn split_splice<R>(&mut self, range: R) -> (Self, opaque_vec::IntoIter<Bucket<K, V>, A>)
    where
        A: Clone,
        R: ops::RangeBounds<usize>,
    {
        let (split_inner, splice_iter) = self.inner.split_splice::<R, K, V, A>(range);
        let proj_split_inner = Self {
            inner: split_inner,
            _marker: PhantomData,
        };

        (proj_split_inner, splice_iter)
    }

    pub(crate) fn append_unchecked(&mut self, other: &mut Self) {
        self.inner.append_unchecked::<K, V, A>(&mut other.inner);
    }

    pub(crate) fn reserve(&mut self, additional: usize) {
        self.inner.reserve::<K, V, A>(additional);
    }

    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact::<K, V, A>(additional);
    }

    pub(crate) fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve::<K, V, A>(additional)
    }

    fn try_reserve_entries(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_entries::<K, V, A>(self.capacity())
    }

    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact::<K, V, A>(self.capacity())
    }

    pub(crate) fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit::<K, V, A>();
    }

    pub(crate) fn shrink_to(&mut self, min_capacity: usize) {
        self.inner.shrink_to::<K, V, A>(min_capacity);
    }

    pub(crate) fn pop(&mut self) -> Option<(K, V)> {
        self.inner.pop::<K, V, A>()
    }

    pub(crate) fn get_index_of<Q>(&self, hash: HashValue, key: &Q) -> Option<usize>
    where
        Q: ?Sized + Equivalent<K>,
    {
        self.inner.get_index_of::<Q, K, V, A>(hash, key)
    }

    fn push_entry(&mut self, hash: HashValue, key: K, value: V) {
        self.inner.push_entry::<K, V, A>(hash, key, value);
    }

    pub(crate) fn insert_full(&mut self, hash: HashValue, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq,
    {
        self.inner.insert_full::<K, V, A>(hash, key, value)
    }

    pub(crate) fn shift_remove_full<Q>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Equivalent<K>,
    {
        self.inner.shift_remove_full::<Q, K, V, A>(hash, key)
    }

    #[inline]
    pub(crate) fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.inner.shift_remove_index::<K, V, A>(index)
    }

    #[inline]
    #[track_caller]
    pub(crate) fn move_index(&mut self, from: usize, to: usize) {
        self.inner.move_index::<K, V, A>(from, to);
    }

    #[inline]
    #[track_caller]
    pub(crate) fn swap_indices(&mut self, a: usize, b: usize) {
        self.inner.swap_indices::<K, V, A>(a, b);
    }

    pub(crate) fn swap_remove_full<Q>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Equivalent<K>,
    {
        self.inner.swap_remove_full::<Q, K, V, A>(hash, key)
    }

    #[inline]
    pub(crate) fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.inner.swap_remove_index::<K, V, A>(index)
    }

    fn erase_indices(&mut self, start: usize, end: usize) {
        self.inner.erase_indices::<K, V, A>(start, end);
    }

    pub(crate) fn retain_in_order<F>(&mut self, mut keep: F)
    where
        F: FnMut(&mut K, &mut V) -> bool,
    {
        self.inner.retain_in_order::<F, K, V, A>(keep)
    }

    pub(crate) fn reverse(&mut self) {
        self.inner.reverse::<K, V, A>();
    }
}

impl<K, V, A> TypedProjIndexMapCore<K, V, A> {
    pub(crate) fn entry(&mut self, hash: HashValue, key: K) -> Entry<'_, K, V, A>
    where
        K: any::Any + Eq,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        self.inner.entry::<K, V, A>(hash, key)
    }
}

impl<K, V, A> Clone for TypedProjIndexMapCore<K, V, A>
where
    K: any::Any + Clone,
    V: any::Any + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    fn clone(&self) -> Self {
        let cloned_inner = self.inner.clone::<K, V, A>();

        Self {
            inner: cloned_inner,
            _marker: PhantomData,
        }
    }

    fn clone_from(&mut self, other: &Self) {
        todo!()
    }
}

#[repr(transparent)]
struct OpaqueIndexMapCore {
    inner: OpaqueIndexMapCoreInner,
}

impl OpaqueIndexMapCore {
    #[inline]
    pub(crate) const fn key_type_id(&self) -> TypeId {
        self.inner.key_type_id()
    }

    #[inline]
    pub(crate) const fn value_type_id(&self) -> TypeId {
        self.inner.value_type_id()
    }

    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> TypeId {
        self.inner.allocator_type_id()
    }
}

impl OpaqueIndexMapCore {
    #[inline]
    const fn as_proj_assuming_type<K, V, A>(&self) -> &TypedProjIndexMapCore<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        unsafe { &*(self as *const OpaqueIndexMapCore as *const TypedProjIndexMapCore<K, V, A>) }
    }

    #[inline]
    const fn as_proj_mut_assuming_type<K, V, A>(&mut self) -> &mut TypedProjIndexMapCore<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        unsafe { &mut *(self as *mut OpaqueIndexMapCore as *mut TypedProjIndexMapCore<K, V, A>) }
    }

    #[inline]
    fn into_proj_assuming_type<K, V, A>(self) -> TypedProjIndexMapCore<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        TypedProjIndexMapCore {
            inner: self.inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    fn from_proj_assuming_type<K, V, A>(proj_self: TypedProjIndexMapCore<K, V, A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        Self {
            inner: proj_self.inner,
        }
    }
}

pub enum Entry<'a, K, V, A>
where
    A: any::Any + alloc::Allocator,
{
    Occupied(OccupiedEntry<'a, K, V, A>),
    Vacant(VacantEntry<'a, K, V, A>),
}

impl<'a, K, V, A> Entry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
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
                let value = call(&entry.key);
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
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = f.debug_tuple("Entry");
        match self {
            Entry::Vacant(v) => tuple.field(v),
            Entry::Occupied(o) => tuple.field(o),
        };
        tuple.finish()
    }
}

pub struct OccupiedEntry<'a, K, V, A>
where
    A: any::Any + alloc::Allocator,
{
    entries: &'a mut TypedProjVec<Bucket<K, V>, A>,
    index: hashbrown::hash_table::OccupiedEntry<'a, usize>,
    _marker: PhantomData<(K, V, A)>,
}

impl<'a, K, V, A> OccupiedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub(crate) fn new(entries: &'a mut TypedProjVec<Bucket<K, V>, A>, index: hashbrown::hash_table::OccupiedEntry<'a, usize>) -> Self {
        Self {
            entries,
            index,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn index(&self) -> usize {
        *self.index.get()
    }

    #[inline]
    fn into_ref_mut(self) -> RefMut<'a, K, V, A> {
        RefMut::new(self.index.into_table(), self.entries)
    }

    pub fn key(&self) -> &K {
        &self.entries.as_slice()[self.index()].key
    }

    pub(crate) fn key_mut(&mut self) -> &mut K {
        let index = self.index();

        &mut self.entries.as_mut_slice()[index].key
    }

    pub fn get(&self) -> &V {
        &self.entries.as_slice()[self.index()].value
    }

    pub fn get_mut(&mut self) -> &mut V {
        let index = self.index();

        &mut self.entries.as_mut_slice()[index].value
    }

    pub fn into_mut(self) -> &'a mut V {
        let index = self.index();

        &mut self.entries.as_mut_slice()[index].value
    }

    fn into_muts(self) -> (&'a mut K, &'a mut V) {
        let index = self.index();

        self.entries.as_mut_slice()[index].muts()
    }

    pub fn insert(&mut self, value: V) -> V {
        core::mem::replace(self.get_mut(), value)
    }

    /*
    /// Remove the key, value pair stored in the map for this entry, and return the value.
    ///
    /// **NOTE:** This is equivalent to [`.swap_remove()`][Self::swap_remove], replacing this
    /// entry's position with the last element, and it is deprecated in favor of calling that
    /// explicitly. If you need to preserve the relative order of the keys in the map, use
    /// [`.shift_remove()`][Self::shift_remove] instead.
    #[deprecated(note = "`remove` disrupts the map order -- \
        use `swap_remove` or `shift_remove` for explicit behavior.")]
    pub fn remove(self) -> V {
        self.swap_remove()
    }
    */

    pub fn swap_remove(self) -> V {
        self.swap_remove_entry().1
    }

    pub fn shift_remove(self) -> V {
        self.shift_remove_entry().1
    }

    /*
    /// Remove and return the key, value pair stored in the map for this entry
    ///
    /// **NOTE:** This is equivalent to [`.swap_remove_entry()`][Self::swap_remove_entry],
    /// replacing this entry's position with the last element, and it is deprecated in favor of
    /// calling that explicitly. If you need to preserve the relative order of the keys in the map,
    /// use [`.shift_remove_entry()`][Self::shift_remove_entry] instead.
    #[deprecated(note = "`remove_entry` disrupts the map order -- \
        use `swap_remove_entry` or `shift_remove_entry` for explicit behavior.")]
    pub fn remove_entry(self) -> (K, V) {
        self.swap_remove_entry()
    }
    */

    pub fn swap_remove_entry(self) -> (K, V) {
        let (index, entry) = self.index.remove();
        RefMut::<'_, K, V, A>::new(entry.into_table(), self.entries).swap_remove_finish(index)
    }

    pub fn shift_remove_entry(self) -> (K, V) {
        let (index, entry) = self.index.remove();
        RefMut::<'_, K, V, A>::new(entry.into_table(), self.entries).shift_remove_finish(index)
    }

    #[track_caller]
    pub fn move_index(self, to: usize) {
        let index = self.index();
        self.into_ref_mut().move_index(index, to);
    }

    pub fn swap_indices(self, other: usize) {
        let index = self.index();
        self.into_ref_mut().swap_indices(index, other);
    }
}

impl<K, V, A> fmt::Debug for OccupiedEntry<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

impl<'a, K, V, A> From<IndexedEntry<'a, K, V, A>> for OccupiedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn from(other: IndexedEntry<'a, K, V, A>) -> Self {
        let IndexedEntry {
            map: RefMut { indices, entries, _marker },
            index,
        } = other;
        let hash = entries.as_slice()[index].hash;
        let index = indices.find_entry(hash.get(), move |&i| i == index).expect("index not found");

        Self {
            entries,
            index,
            _marker: PhantomData,
        }
    }
}

pub struct VacantEntry<'a, K, V, A>
where
    A: any::Any + alloc::Allocator,
{
    map: RefMut<'a, K, V, A>,
    hash: HashValue,
    key: K,
}

impl<'a, K, V, A> VacantEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub fn index(&self) -> usize {
        self.map.indices.len()
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub(crate) fn key_mut(&mut self) -> &mut K {
        &mut self.key
    }

    pub fn into_key(self) -> K {
        self.key
    }

    pub fn insert(self, value: V) -> &'a mut V {
        self.insert_entry(value).into_mut()
    }

    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V, A> {
        let Self { map, hash, key } = self;

        map.insert_unique(hash, key, value)
    }

    pub fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {
        let slice = Slice::from_slice(self.map.entries.as_slice());
        let i = slice.binary_search_keys(&self.key).unwrap_err();

        (i, self.shift_insert(i, value))
    }

    pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
        self.map.shift_insert_unique(index, self.hash, self.key, value);

        &mut self.map.entries.as_mut_slice()[index].value
    }
}

impl<K, V, A> fmt::Debug for VacantEntry<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}

pub struct IndexedEntry<'a, K, V, A>
where
    A: any::Any + alloc::Allocator,
{
    map: RefMut<'a, K, V, A>,
    index: usize,
}

impl<'a, K, V, A> IndexedEntry<'a, K, V, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub(crate) fn new(map: &'a mut TypedProjIndexMapCore<K, V, A>, index: usize) -> Self
    where
        K: Ord,
    {
        Self {
            map: map.borrow_mut(),
            index,
        }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn key(&self) -> &K {
        &self.map.entries.as_slice()[self.index].key
    }

    pub(crate) fn key_mut(&mut self) -> &mut K {
        &mut self.map.entries.as_mut_slice()[self.index].key
    }

    pub fn get(&self) -> &V {
        &self.map.entries.as_slice()[self.index].value
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.map.entries.as_mut_slice()[self.index].value
    }

    pub fn insert(&mut self, value: V) -> V {
        core::mem::replace(self.get_mut(), value)
    }

    pub fn into_mut(self) -> &'a mut V {
        &mut self.map.entries.as_mut_slice()[self.index].value
    }

    pub fn swap_remove_entry(mut self) -> (K, V) {
        self.map.swap_remove_index(self.index).unwrap()
    }

    pub fn shift_remove_entry(mut self) -> (K, V) {
        self.map.shift_remove_index(self.index).unwrap()
    }

    pub fn swap_remove(self) -> V {
        self.swap_remove_entry().1
    }

    pub fn shift_remove(self) -> V {
        self.shift_remove_entry().1
    }

    #[track_caller]
    pub fn move_index(mut self, to: usize) {
        self.map.move_index(self.index, to);
    }

    pub fn swap_indices(mut self, other: usize) {
        self.map.swap_indices(self.index, other);
    }
}

impl<K, V, A> fmt::Debug for IndexedEntry<'_, K, V, A>
where
    K: any::Any + fmt::Debug,
    V: any::Any + fmt::Debug,
    A: any::Any + alloc::Allocator,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IndexedEntry")
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
    A: any::Any + alloc::Allocator,
{
    fn from(other: OccupiedEntry<'a, K, V, A>) -> Self {
        Self {
            index: other.index(),
            map: other.into_ref_mut(),
        }
    }
}

#[repr(C)]
struct TypedProjIndexMapInner<K, V, S, A>
where
    A: any::Any + alloc::Allocator,
{
    inner: TypedProjIndexMapCore<K, V, A>,
    build_hasher: TypedProjBuildHasher<S>,
}

impl<K, V, S, A> TypedProjIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    fn into_entries(self) -> TypedProjVec<Bucket<K, V>, A> {
        self.inner.into_entries()
    }

    #[inline]
    fn as_entries(&self) -> &[Bucket<K, V>] {
        self.inner.as_entries()
    }

    #[inline]
    fn as_entries_mut(&mut self) -> &mut [Bucket<K, V>] {
        self.inner.as_entries_mut()
    }

    fn with_entries<F>(&mut self, f: F)
    where
        F: FnOnce(&mut [Bucket<K, V>]),
    {
        self.inner.with_entries(f);
    }
}

impl<K, V, S, A> TypedProjIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn with_hasher_proj_in(proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = TypedProjIndexMapCore::<K, V, A>::new_proj_in(proj_alloc);

        Self {
            inner: proj_inner,
            build_hasher: proj_build_hasher,
        }
    }

    #[inline]
    pub fn with_capacity_and_hasher_proj_in(capacity: usize, proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self {
        if capacity == 0 {
            Self::with_hasher_proj_in(proj_build_hasher, proj_alloc)
        } else {
            let proj_inner = TypedProjIndexMapCore::<K, V, A>::with_capacity_proj_in(capacity, proj_alloc);

            Self {
                inner: proj_inner,
                build_hasher: proj_build_hasher,
            }
        }
    }
}

impl<K, V, A> TypedProjIndexMapInner<K, V, hash::RandomState, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub fn new_proj_in(proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = TypedProjIndexMapCore::<K, V, A>::new_proj_in(proj_alloc);
        let proj_build_hasher = TypedProjBuildHasher::new(hash::RandomState::new());

        Self {
            inner : proj_inner,
            build_hasher: proj_build_hasher,
        }
    }

    pub fn with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = TypedProjIndexMapCore::<K, V, A>::with_capacity_proj_in(capacity, proj_alloc);
        let proj_build_hasher = TypedProjBuildHasher::new(hash::RandomState::new());

        Self {
            inner : proj_inner,
            build_hasher: proj_build_hasher,
        }
    }
}

impl<K, V, S, A> TypedProjIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn with_hasher_in(build_hasher: S, alloc: A) -> Self {
        let proj_inner = TypedProjIndexMapCore::<K, V, A>::new_in(alloc);
        let proj_build_hasher = TypedProjBuildHasher::new(build_hasher);

        Self {
            inner : proj_inner,
            build_hasher: proj_build_hasher,
        }
    }

    #[inline]
    pub fn with_capacity_and_hasher_in(capacity: usize, build_hasher: S, alloc: A) -> Self {
        if capacity == 0 {
            Self::with_hasher_in(build_hasher, alloc)
        } else {
            let proj_inner = TypedProjIndexMapCore::<K, V, A>::with_capacity_in(capacity, alloc);
            let proj_build_hasher = TypedProjBuildHasher::new(build_hasher);

            Self {
                inner: proj_inner,
                build_hasher: proj_build_hasher,
            }
        }
    }
}

impl<K, V, A> TypedProjIndexMapInner<K, V, hash::RandomState, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub fn new_in(alloc: A) -> Self {
        let proj_inner = TypedProjIndexMapCore::<K, V, A>::new_in(alloc);
        let proj_build_hasher = TypedProjBuildHasher::<hash::RandomState>::new(hash::RandomState::default());

        Self {
            inner : proj_inner,
            build_hasher: proj_build_hasher,
        }
    }

    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let proj_inner = TypedProjIndexMapCore::<K, V, A>::with_capacity_in(capacity, alloc);
        let proj_build_hasher = TypedProjBuildHasher::<hash::RandomState>::new(hash::RandomState::default());

        Self {
            inner: proj_inner,
            build_hasher: proj_build_hasher,
        }
    }
}

impl<K, V, S> TypedProjIndexMapInner<K, V, S, alloc::Global>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
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

impl<K, V> TypedProjIndexMapInner<K, V, hash::RandomState, alloc::Global>
where
    K: any::Any,
    V: any::Any,
{
    #[inline]
    pub fn new() -> Self {
        Self::new_in(alloc::Global)
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, alloc::Global)
    }
}

impl<K, V, S, A> TypedProjIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub const fn hasher(&self) -> &TypedProjBuildHasher<S> {
        &self.build_hasher
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn hash<Q>(&self, key: &Q) -> HashValue
    where
        Q: ?Sized + hash::Hash,
    {
        let mut hasher = hash::BuildHasher::build_hasher(&self.build_hasher);
        key.hash(&mut hasher);

        HashValue::new(hash::Hasher::finish(&mut hasher) as usize)
    }

    pub fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        match self.as_entries() {
            [] => None,
            [x] => key.equivalent(&x.key).then_some(0),
            _ => {
                let hash = self.hash(key);
                self.inner.get_index_of(hash, key)
            }
        }
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        self.get_index_of::<Q>(key).is_some()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        if let Some(index) = self.get_index_of::<Q>(key) {
            let entry = &self.as_entries()[index];
            Some(&entry.value)
        } else {
            None
        }
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        if let Some(i) = self.get_index_of::<Q>(key) {
            let entry = &self.as_entries()[i];
            Some((&entry.key, &entry.value))
        } else {
            None
        }
    }

    pub fn get_full<Q>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        if let Some(i) = self.get_index_of::<Q>(key) {
            let entry = &self.as_entries()[i];
            Some((i, &entry.key, &entry.value))
        } else {
            None
        }
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        if let Some(i) = self.get_index_of::<Q>(key) {
            let entry = &mut self.as_entries_mut()[i];
            Some(&mut entry.value)
        } else {
            None
        }
    }

    pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        if let Some(i) = self.get_index_of::<Q>(key) {
            let entry = &mut self.as_entries_mut()[i];

            Some((i, &entry.key, &mut entry.value))
        } else {
            None
        }
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys::new(self.as_entries())
    }

    pub fn into_keys(self) -> IntoKeys<K, V, A> {
        IntoKeys::new(self.into_entries())
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter::new(self.as_entries())
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut::new(self.as_entries_mut())
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values::new(self.as_entries())
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut::new(self.as_entries_mut())
    }

    pub fn into_values(self) -> IntoValues<K, V, A> {
        IntoValues::new(self.into_entries())
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
        Drain::new(self.inner.drain::<R>(range))
    }

    pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        fn third<A, B, C>(triple: (A, B, C)) -> C {
            triple.2
        }

        self.swap_remove_full::<Q>(key).map(third)
    }

    pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        match self.swap_remove_full::<Q>(key) {
            Some((_, key, value)) => Some((key, value)),
            None => None,
        }
    }

    pub fn swap_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
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

    pub fn shift_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        fn third<A, B, C>(triple: (A, B, C)) -> C {
            triple.2
        }

        self.shift_remove_full::<Q>(key).map(third)
    }

    pub fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        match self.shift_remove_full::<Q>(key) {
            Some((_, key, value)) => Some((key, value)),
            None => None,
        }
    }

    pub fn shift_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
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

    pub fn as_slice(&self) -> &'_ Slice<K, V> {
        Slice::from_slice(self.as_entries())
    }

    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
        Slice::from_slice_mut(self.as_entries_mut())
    }
}

impl<K, V, S, A> TypedProjIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Eq + hash::Hash,
    {
        self.insert_full(key, value).1
    }

    pub fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash,
    {
        let hash = self.hash(&key);

        self.inner.insert_full(hash, key, value)
    }

    pub fn insert_sorted(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash + Ord,
    {
        match self.binary_search_keys(&key) {
            Ok(i) => {
                let destination = self.get_index_mut(i).unwrap().1;
                let old_value = core::mem::replace(destination, value);

                (i, Some(old_value))
            }
            Err(i) => self.insert_before(i, key, value),
        }
    }

    #[track_caller]
    pub fn insert_before(&mut self, mut index: usize, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash,
    {
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
                let old = core::mem::replace(entry.get_mut(), value);
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
    pub fn shift_insert(&mut self, index: usize, key: K, value: V) -> Option<V>
    where
        K: Eq + hash::Hash,
    {
        let len = self.len();
        match self.entry(key) {
            Entry::Occupied(mut entry) => {
                assert!(index < len, "index out of bounds: the len is {len} but the index is {index}");

                let old = core::mem::replace(entry.get_mut(), value);
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

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, A>
    where
        K: Eq + hash::Hash,
    {
        let hash = self.hash(&key);

        self.inner.entry(hash, key)
    }

    #[track_caller]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, K, V, S, A>
    where
        K: Eq + hash::Hash,
        A: Clone,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item=(K, V)>,
    {
        Splice::new(self, range, replace_with.into_iter())
    }
}

impl<K, V, S, A> TypedProjIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    pub fn append<S2, A2>(&mut self, other: &mut TypedProjIndexMapInner<K, V, S2, A2>)
    where
        K: Eq + hash::Hash,
        S2: any::Any + hash::BuildHasher,
        A2: any::Any + alloc::Allocator,
    {
        self.extend(other.drain::<_>(..));
    }
}

impl<K, V, S, A> Clone for TypedProjIndexMapInner<K, V, S, A>
where
    K: any::Any + Clone,
    V: any::Any + Clone,
    S: any::Any + hash::BuildHasher + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    fn clone(&self) -> Self {
        let cloned_inner = self.inner.clone();
        let cloned_builder_hasher = self.build_hasher.clone();

        Self {
            inner: cloned_inner,
            build_hasher: cloned_builder_hasher,
        }
    }

    fn clone_from(&mut self, other: &Self) {
        todo!()
    }
}

impl<K, V, S, A> Extend<(K, V)> for TypedProjIndexMapInner<K, V, S, A>
where
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        // (Note: this is a copy of `std`/`hashbrown`'s reservation logic.)
        // Keys may be already present or show multiple times in the iterator.
        // Reserve the entire hint lower bound if the map is empty.
        // Otherwise reserve half the hint (rounded up), so the map
        // will only resize twice in the worst case.
        let iter = iterable.into_iter();
        let reserve_count = if self.is_empty() {
            iter.size_hint().0
        } else {
            (iter.size_hint().0 + 1) / 2
        };
        self.reserve(reserve_count);
        iter.for_each(move |(k, v)| {
            self.insert(k, v);
        });
    }
}

impl<'a, K, V, S, A> Extend<(&'a K, &'a V)> for TypedProjIndexMapInner<K, V, S, A>
where
    K: any::Any + hash::Hash + Eq + Copy,
    V: any::Any + Copy,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (&'a K, &'a V)>,
    {
        self.extend(iterable.into_iter().map(|(&key, &value)| (key, value)));
    }
}

impl<K, V, S, A> TypedProjIndexMapInner<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    #[doc(alias = "pop_last")] // like `BTreeMap`
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.inner.pop()
    }

    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.inner.retain_in_order::<_>(move |k, v| keep(k, v));
    }

    pub fn sort_keys(&mut self)
    where
        K: Ord,
    {
        self.with_entries::<_>(move |entries| {
            entries.sort_by(move |a, b| K::cmp(&a.key, &b.key));
        });
    }

    pub fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        self.with_entries::<_>(move |entries| {
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
        });
    }

    pub fn sorted_by<F>(self, mut cmp: F) -> IntoIter<K, V, A>
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let mut entries = self.into_entries();
        entries
            .as_mut_slice()
            .sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));

        IntoIter::new(entries)
    }

    pub fn sort_unstable_keys(&mut self)
    where
        K: Ord,
    {
        self.with_entries::<_>(move |entries| {
            entries.sort_unstable_by(move |a, b| K::cmp(&a.key, &b.key));
        });
    }

    pub fn sort_unstable_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        self.with_entries::<_>(move |entries| {
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
        });
    }

    #[inline]
    pub fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<K, V, A>
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let mut entries = self.into_entries();
        entries
            .as_mut_slice()
            .sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));

        IntoIter::new(entries)
    }

    pub fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
    where
        T: Ord,
        F: FnMut(&K, &V) -> T,
    {
        self.with_entries::<_>(move |entries| {
            entries.sort_by_cached_key(move |a| sort_key(&a.key, &a.value));
        });
    }

    pub fn binary_search_keys(&self, key: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        self.as_slice().binary_search_keys(key)
    }

    #[inline]
    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> cmp::Ordering,
    {
        self.as_slice().binary_search_by(f)
    }

    #[inline]
    pub fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> B,
        B: Ord,
    {
        self.as_slice().binary_search_by_key(b, f)
    }

    #[must_use]
    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {
        self.as_slice().partition_point(pred)
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

    pub fn into_boxed_slice(self) -> Box<Slice<K, V>, opaque_alloc::TypedProjAlloc<A>> {
        Slice::from_boxed_slice(self.into_entries().into_boxed_slice())
    }

    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.as_entries().get(index).map(Bucket::refs)
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.as_entries_mut().get_mut(index).map(Bucket::ref_mut)
    }

    pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        if index >= self.len() {
            return None;
        }

        Some(IndexedEntry::new(&mut self.inner, index))
    }

    pub fn get_range<R>(&self, range: R) -> Option<&Slice<K, V>>
    where
        R: ops::RangeBounds<usize>,
    {
        let entries = self.as_entries();
        let range = try_simplify_range(range, entries.len())?;
        entries.get(range).map(Slice::from_slice)
    }

    pub fn get_range_mut<R>(&mut self, range: R) -> Option<&mut Slice<K, V>>
    where
        R: ops::RangeBounds<usize>,
    {
        let entries = self.as_entries_mut();
        let range = try_simplify_range(range, entries.len())?;
        entries.get_mut(range).map(Slice::from_slice_mut)
    }

    #[doc(alias = "first_key_value")] // like `BTreeMap`
    pub fn first(&self) -> Option<(&K, &V)> {
        self.as_entries().first().map(Bucket::refs)
    }

    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.as_entries_mut().first_mut().map(Bucket::ref_mut)
    }

    pub fn first_entry(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        self.get_index_entry(0)
    }

    #[doc(alias = "last_key_value")] // like `BTreeMap`
    pub fn last(&self) -> Option<(&K, &V)> {
        self.as_entries().last().map(Bucket::refs)
    }

    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        self.as_entries_mut().last_mut().map(Bucket::ref_mut)
    }

    pub fn last_entry(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        self.get_index_entry(self.len().checked_sub(1)?)
    }

    pub fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.inner.swap_remove_index(index)
    }

    pub fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.inner.shift_remove_index(index)
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

#[repr(C)]
struct OpaqueIndexMapInner {
    inner: OpaqueIndexMapCore,
    build_hasher: OpaqueBuildHasher,
}

impl OpaqueIndexMapInner {
    #[inline]
    pub fn key_type_id(&self) -> TypeId {
        self.inner.key_type_id()
    }

    #[inline]
    pub fn value_type_id(&self) -> TypeId {
        self.inner.value_type_id()
    }

    #[inline]
    pub fn build_hasher_type_id(&self) -> TypeId {
        self.build_hasher.build_hasher_type_id()
    }

    #[inline]
    pub fn allocator_type_id(&self) -> TypeId {
        self.inner.allocator_type_id()
    }
}

impl OpaqueIndexMapInner {
    #[inline]
    pub fn as_proj<K, V, S, A>(&self) -> &TypedProjIndexMapInner<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        unsafe { &*(self as *const OpaqueIndexMapInner as *const TypedProjIndexMapInner<K, V, S, A>) }
    }

    #[inline]
    pub fn as_proj_mut<K, V, S, A>(&mut self) -> &mut TypedProjIndexMapInner<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        unsafe { &mut *(self as *mut OpaqueIndexMapInner as *mut TypedProjIndexMapInner<K, V, S, A>) }
    }

    #[inline]
    pub fn into_proj<K, V, S, A>(self) -> TypedProjIndexMapInner<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_inner = self.inner.into_proj_assuming_type::<K, V, A>();
        let proj_build_hasher = self.build_hasher.into_proj::<S>();

        TypedProjIndexMapInner {
            inner: proj_inner,
            build_hasher: proj_build_hasher,
        }
    }

    #[inline]
    pub fn from_proj<K, V, S, A>(proj_self: TypedProjIndexMapInner<K, V, S, A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let opaque_inner = OpaqueIndexMapCore::from_proj_assuming_type::<K, V, A>(proj_self.inner);
        let opaque_build_hasher = OpaqueBuildHasher::from_proj::<S>(proj_self.build_hasher);

        Self {
            inner: opaque_inner,
            build_hasher: opaque_build_hasher,
        }
    }
}

#[repr(transparent)]
pub struct TypedProjIndexMap<K, V, S, A>
where
    A: any::Any + alloc::Allocator,
{
    inner: OpaqueIndexMapInner,
    _marker: core::marker::PhantomData<(K, V, S, A)>,
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn with_hasher_proj_in(proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, S, A>::with_hasher_proj_in(proj_build_hasher, proj_alloc);
        let opaque_inner = OpaqueIndexMapInner::from_proj(proj_inner);

        Self {
            inner: opaque_inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn with_capacity_and_hasher_proj_in(capacity: usize, proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self {
        if capacity == 0 {
            Self::with_hasher_proj_in(proj_build_hasher, proj_alloc)
        } else {
            let proj_inner = TypedProjIndexMapInner::<K, V, S, A>::with_capacity_and_hasher_proj_in(capacity, proj_build_hasher, proj_alloc);
            let opaque_inner = OpaqueIndexMapInner::from_proj(proj_inner);

            Self {
                inner: opaque_inner,
                _marker: core::marker::PhantomData,
            }
        }
    }
}

impl<K, V, A> TypedProjIndexMap<K, V, hash::RandomState, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub fn new_proj_in(proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, hash::RandomState, A>::new_proj_in(proj_alloc);
        let opaque_inner = OpaqueIndexMapInner::from_proj(proj_inner);

        Self {
            inner : opaque_inner,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn with_capacity_proj_in(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, hash::RandomState, A>::with_capacity_proj_in(capacity, proj_alloc);
        let opaque_inner = OpaqueIndexMapInner::from_proj(proj_inner);

        Self {
            inner: opaque_inner,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn with_hasher_in(build_hasher: S, alloc: A) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, S, A>::with_hasher_in(build_hasher, alloc);
        let opaque_inner = OpaqueIndexMapInner::from_proj(proj_inner);

        Self {
            inner: opaque_inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn with_capacity_and_hasher_in(capacity: usize, build_hasher: S, alloc: A) -> Self {
        if capacity == 0 {
            Self::with_hasher_in(build_hasher, alloc)
        } else {
            let proj_inner = TypedProjIndexMapInner::<K, V, S, A>::with_capacity_and_hasher_in(capacity, build_hasher, alloc);
            let opaque_inner = OpaqueIndexMapInner::from_proj(proj_inner);

            Self {
                inner: opaque_inner,
                _marker: core::marker::PhantomData,
            }
        }
    }
}

impl<K, V, A> TypedProjIndexMap<K, V, hash::RandomState, A>
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator,
{
    pub fn new_in(alloc: A) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, hash::RandomState, A>::new_in(alloc);
        let opaque_inner = OpaqueIndexMapInner::from_proj(proj_inner);

        Self {
            inner : opaque_inner,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        let proj_inner = TypedProjIndexMapInner::<K, V, hash::RandomState, A>::with_capacity_in(capacity, alloc);
        let opaque_inner = OpaqueIndexMapInner::from_proj(proj_inner);

        Self {
            inner: opaque_inner,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<K, V, S> TypedProjIndexMap<K, V, S, alloc::Global>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
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

impl<K, V> TypedProjIndexMap<K, V, hash::RandomState, alloc::Global>
where
    K: any::Any,
    V: any::Any,
{
    #[inline]
    pub fn new() -> Self {
        Self::new_in(alloc::Global)
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, alloc::Global)
    }
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn capacity(&self) -> usize {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.capacity()
    }

    #[inline]
    pub fn hasher(&self) -> &TypedProjBuildHasher<S> {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.hasher()
    }

    #[inline]
    pub fn len(&self) -> usize {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.is_empty()
    }

    /*
    fn hash<Q>(&self, key: &Q) -> HashValue
    where
        Q: ?Sized + hash::Hash,
    {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);

        HashValue::new(hasher.finish() as usize)
    }
    */

    pub fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.get_index_of(key)
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.contains_key(key)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.get(key)
    }

    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.get_key_value(key)
    }

    pub fn get_full<Q>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.get_full(key)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.get_mut(key)
    }

    pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.get_full_mut(key)
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.keys()
    }

    pub fn into_keys(self) -> IntoKeys<K, V, A> {
        let proj_inner = self.inner.into_proj::<K, V, S, A>();

        proj_inner.into_keys()
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.iter_mut()
    }

    pub fn values(&self) -> Values<'_, K, V> {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.values()
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.values_mut()
    }

    pub fn into_values(self) -> IntoValues<K, V, A> {
        let proj_inner = self.inner.into_proj::<K, V, S, A>();

        proj_inner.into_values()
    }

    pub fn clear(&mut self) {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.clear();
    }

    pub fn truncate(&mut self, len: usize) {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.truncate(len);
    }

    #[track_caller]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, K, V, A>
    where
        R: ops::RangeBounds<usize>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.drain(range)
    }

    pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.swap_remove(key)
    }

    pub fn swap_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.swap_remove_entry(key)
    }

    pub fn swap_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.swap_remove_full(key)
    }

    pub fn shift_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.shift_remove(key)
    }

    pub fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.shift_remove_entry(key)
    }

    pub fn shift_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.shift_remove_full(key)
    }

    pub fn as_slice(&self) -> &'_ Slice<K, V> {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.as_mut_slice()
    }
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Eq + hash::Hash,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.insert(key, value)
    }

    pub fn insert_full(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.insert_full(key, value)
    }

    pub fn insert_sorted(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash + Ord,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.insert_sorted(key, value)
    }

    #[track_caller]
    pub fn insert_before(&mut self, mut index: usize, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.insert_before(index, key, value)
    }

    #[track_caller]
    pub fn shift_insert(&mut self, index: usize, key: K, value: V) -> Option<V>
    where
        K: Eq + hash::Hash,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.shift_insert(index, key, value)
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V, A>
    where
        K: Eq + hash::Hash,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.entry(key)
    }

    #[track_caller]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, K, V, S, A>
    where
        K: Eq + hash::Hash,
        A: any::Any + alloc::Allocator + Clone,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = (K, V)>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.splice(range, replace_with)
    }

    pub fn append<S2, A2>(&mut self, other: &mut TypedProjIndexMap<K, V, S2, A2>)
    where
        K: Eq + hash::Hash,
        S2: any::Any + hash::BuildHasher,
        A2: any::Any + alloc::Allocator,
    {
        let proj_self_inner = self.inner.as_proj_mut::<K, V, S, A>();
        let proj_other_inner = other.inner.as_proj_mut::<K, V, S, A>();

        proj_self_inner.append(proj_other_inner);
    }
}

impl<K, V, S, A> TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    #[doc(alias = "pop_last")] // like `BTreeMap`
    pub fn pop(&mut self) -> Option<(K, V)> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.pop()
    }

    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.retain(keep);
    }

    pub fn sort_keys(&mut self)
    where
        K: Ord,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.sort_keys();
    }

    pub fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.sort_by(cmp);
    }

    pub fn sorted_by<F>(self, mut cmp: F) -> IntoIter<K, V, A>
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_inner = self.inner.into_proj::<K, V, S, A>();

        proj_inner.sorted_by(cmp)
    }

    pub fn sort_unstable_keys(&mut self)
    where
        K: Ord,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.sort_unstable_keys();
    }

    pub fn sort_unstable_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.sort_unstable_by(cmp);
    }

    #[inline]
    pub fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<K, V, A>
    where
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_inner = self.inner.into_proj::<K, V, S, A>();

        proj_inner.sorted_unstable_by(cmp)
    }

    pub fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
    where
        T: Ord,
        F: FnMut(&K, &V) -> T,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.sort_by_cached_key(&mut sort_key);
    }

    pub fn binary_search_keys(&self, key: &K) -> Result<usize, usize>
    where
        K: Ord,
    {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.binary_search_keys(key)
    }

    #[inline]
    pub fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> cmp::Ordering,
    {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.binary_search_by(f)
    }

    #[inline]
    pub fn binary_search_by_key<B, F>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        F: FnMut(&K, &V) -> B,
        B: Ord,
    {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.binary_search_by_key(b, f)
    }

    #[must_use]
    pub fn partition_point<P>(&self, pred: P) -> usize
    where
        P: FnMut(&K, &V) -> bool,
    {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.partition_point(pred)
    }

    pub fn reverse(&mut self) {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.reverse();
    }

    pub fn reserve(&mut self, additional: usize) {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.reserve(additional);
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.reserve_exact(additional);
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.try_reserve(additional)
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.try_reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.shrink_to_fit();
    }

    pub fn shrink_to(&mut self, min_capacity: usize) {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.shrink_to(min_capacity);
    }

    pub fn into_boxed_slice(self) -> Box<Slice<K, V>, TypedProjAlloc<A>> {
        let proj_inner = self.inner.into_proj::<K, V, S, A>();

        proj_inner.into_boxed_slice()
    }

    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.get_index(index)
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.get_index_mut(index)
    }

    pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.get_index_entry(index)
    }

    pub fn get_range<R>(&self, range: R) -> Option<&Slice<K, V>>
    where
        R: ops::RangeBounds<usize>,
    {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.get_range(range)
    }

    pub fn get_range_mut<R>(&mut self, range: R) -> Option<&mut Slice<K, V>>
    where
        R: ops::RangeBounds<usize>,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.get_range_mut(range)
    }

    #[doc(alias = "first_key_value")] // like `BTreeMap`
    pub fn first(&self) -> Option<(&K, &V)> {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.first()
    }

    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.first_mut()
    }

    pub fn first_entry(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.first_entry()
    }

    #[doc(alias = "last_key_value")] // like `BTreeMap`
    pub fn last(&self) -> Option<(&K, &V)> {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();

        proj_inner.last()
    }

    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.last_mut()
    }

    pub fn last_entry(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: Ord,
    {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.last_entry()
    }

    pub fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.swap_remove_index(index)
    }

    pub fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.shift_remove_index(index)
    }

    #[track_caller]
    pub fn move_index(&mut self, from: usize, to: usize) {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.move_index(from, to);
    }

    #[track_caller]
    pub fn swap_indices(&mut self, a: usize, b: usize) {
        let proj_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_inner.swap_indices(a, b)
    }
}

impl<Q, K, V, S, A> ops::Index<&Q> for TypedProjIndexMap<K, V, S, A>
where
    Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("Entry not found for key")
    }
}


impl<Q, K, V, S, A> ops::IndexMut<&Q> for TypedProjIndexMap<K, V, S, A>
where
    Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    fn index_mut(&mut self, key: &Q) -> &mut V {
        self.get_mut(key).expect("Entry not found for key")
    }
}

impl<K, V, S, A> ops::Index<usize> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    type Output = V;

    fn index(&self, index: usize) -> &V {
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
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    fn index_mut(&mut self, index: usize) -> &mut V {
        let len: usize = self.len();

        self.get_index_mut(index)
            .unwrap_or_else(|| {
                panic!("index out of bounds: the len is `{len}` but the index is `{index}`");
            })
            .1
    }
}

impl<K, V, S, A> Clone for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any + Clone,
    V: any::Any + Clone,
    S: any::Any + hash::BuildHasher + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    fn clone(&self) -> Self {
        let proj_inner = self.inner.as_proj::<K, V, S, A>();
        let cloned_proj_inner = Clone::clone(proj_inner);
        let cloned_opaque_inner = OpaqueIndexMapInner::from_proj(cloned_proj_inner);

        Self {
            inner: cloned_opaque_inner,
            _marker: PhantomData,
        }
    }

    fn clone_from(&mut self, other: &Self) {
        let proj_self_inner = self.inner.as_proj_mut::<K, V, S, A>();
        let proj_other_inner = other.inner.as_proj::<K, V, S, A>();

        Clone::clone_from(proj_self_inner, proj_other_inner);
    }
}

impl<K, V, S, A> Extend<(K, V)> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let proj_self_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_self_inner.extend(iterable);
    }
}

impl<'a, K, V, S, A> Extend<(&'a K, &'a V)> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any + hash::Hash + Eq + Copy,
    V: any::Any + Copy,
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
    fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (&'a K, &'a V)>,
    {
        let proj_self_inner = self.inner.as_proj_mut::<K, V, S, A>();

        proj_self_inner.extend(iterable);
    }
}

impl<K, V, S> FromIterator<(K, V)> for TypedProjIndexMap<K, V, S, alloc::Global>
where
    K: any::Any + hash::Hash + Eq,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Default,
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
    S: any::Any + hash::BuildHasher + Default,
{
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
    }
}

impl<K, V, S, A> Default for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Default,
    A: any::Any + alloc::Allocator + Default,
{
    fn default() -> Self {
        Self::with_capacity_and_hasher_in(0, S::default(), A::default())
    }
}

impl<K, V1, S1, V2, S2, A1, A2> PartialEq<TypedProjIndexMap<K, V2, S2, A2>> for TypedProjIndexMap<K, V1, S1, A1>
where
    K: any::Any + hash::Hash + Eq,
    V1: any::Any + PartialEq<V2>,
    V2: any::Any,
    S1: any::Any + hash::BuildHasher,
    S2: any::Any + hash::BuildHasher,
    A1: any::Any + alloc::Allocator,
    A2: any::Any + alloc::Allocator,
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
    S: any::Any + hash::BuildHasher,
    A: any::Any + alloc::Allocator,
{
}

#[repr(transparent)]
pub struct OpaqueIndexMap {
    inner: OpaqueIndexMapInner,
}

impl OpaqueIndexMap {
    #[inline]
    pub fn has_key_type<K>(&self) -> bool
    where
        K: any::Any,
    {
        self.inner.key_type_id() == TypeId::of::<K>()
    }

    #[inline]
    pub fn has_value_type<V>(&self) -> bool
    where
        V: any::Any,
    {
        self.inner.value_type_id() == TypeId::of::<V>()
    }

    #[inline]
    pub fn has_build_hasher_type<S>(&self) -> bool
    where
        S: any::Any,
    {
        self.inner.build_hasher_type_id() == TypeId::of::<S>()
    }

    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator,
    {
        self.inner.allocator_type_id() == TypeId::of::<A>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<K, V, S, A>(&self)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any,
        A: any::Any + alloc::Allocator,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(st: &str, type_id_self: TypeId, type_id_other: TypeId) -> ! {
            panic!("{:?} type mismatch. Need `{:?}`, got `{:?}`", st, type_id_self, type_id_other);
        }

        if !self.has_key_type::<K>() {
            type_check_failed("Key", self.inner.key_type_id(), TypeId::of::<K>());
        }

        if !self.has_value_type::<V>() {
            type_check_failed("Value", self.inner.value_type_id(), TypeId::of::<V>());
        }

        if !self.has_build_hasher_type::<S>() {
            type_check_failed("BuildHasher", self.inner.build_hasher_type_id(), TypeId::of::<S>());
        }

        if !self.has_allocator_type::<A>() {
            type_check_failed("Allocator", self.inner.allocator_type_id(), TypeId::of::<A>());
        }
    }
}

impl OpaqueIndexMap {
    #[inline]
    pub fn as_proj<K, V, S, A>(&self) -> &TypedProjIndexMap<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<K, V, S, A>();

        unsafe { &*(self as *const OpaqueIndexMap as *const TypedProjIndexMap<K, V, S, A>) }
    }

    #[inline]
    pub fn as_proj_mut<K, V, S, A>(&mut self) -> &mut TypedProjIndexMap<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<K, V, S, A>();

        unsafe { &mut *(self as *mut OpaqueIndexMap as *mut TypedProjIndexMap<K, V, S, A>) }
    }

    #[inline]
    pub fn into_proj<K, V, S, A>(self) -> TypedProjIndexMap<K, V, S, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<K, V, S, A>();

        TypedProjIndexMap {
            inner: self.inner,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_proj<K, V, S, A>(proj_self: TypedProjIndexMap<K, V, S, A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        Self {
            inner: proj_self.inner,
        }
    }
}

impl OpaqueIndexMap {
    #[inline]
    pub fn with_hasher_proj_in<K, V, S, A>(proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, S, A>::with_hasher_proj_in(proj_build_hasher, proj_alloc);

        Self::from_proj(proj_index_map)
    }

    #[inline]
    pub fn with_capacity_and_hasher_proj_in<K, V, S, A>(capacity: usize, proj_build_hasher: TypedProjBuildHasher<S>, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, S, A>::with_capacity_and_hasher_proj_in(capacity, proj_build_hasher, proj_alloc);

        Self::from_proj(proj_index_map)
    }
}

impl OpaqueIndexMap {
    pub fn new_proj_in<K, V, A>(proj_alloc: TypedProjAlloc<A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, hash::RandomState, A>::new_proj_in(proj_alloc);

        Self::from_proj(proj_index_map)
    }

    pub fn with_capacity_proj_in<K, V, A>(capacity: usize, proj_alloc: TypedProjAlloc<A>) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
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
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, S, A>::with_hasher_in(build_hasher, alloc);

        Self::from_proj(proj_index_map)
    }

    #[inline]
    pub fn with_capacity_and_hasher_in<K, V, S, A>(capacity: usize, build_hasher: S, alloc: A) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
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
        A: any::Any + alloc::Allocator,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, _, A>::new_in(alloc);

        Self::from_proj(proj_index_map)
    }

    pub fn with_capacity_in<K, V, A>(capacity: usize, alloc: A) -> Self
    where
        K: any::Any,
        V: any::Any,
        A: any::Any + alloc::Allocator,
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
        S: any::Any + hash::BuildHasher,
    {
        let proj_index_map = TypedProjIndexMap::<K, V, S, _>::with_hasher(build_hasher);

        Self::from_proj(proj_index_map)
    }

    #[inline]
    pub fn with_capacity_and_hasher<K, V, S>(capacity: usize, build_hasher: S) -> Self
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
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
    pub fn capacity<K, V, S, A>(&self) -> usize
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.capacity()
    }

    #[inline]
    pub fn hasher<K, V, S, A>(&self) -> &TypedProjBuildHasher<S>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.hasher()
    }

    #[inline]
    pub fn len<K, V, S, A>(&self) -> usize
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.len()
    }

    #[inline]
    pub fn is_empty<K, V, S, A>(&self) -> bool
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.is_empty()
    }

    pub fn get_index_of<Q, K, V, S, A>(&self, key: &Q) -> Option<usize>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get_index_of(key)
    }

    pub fn contains_key<Q, K, V, S, A>(&self, key: &Q) -> bool
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.contains_key(key)
    }

    pub fn get<Q, K, V, S, A>(&self, key: &Q) -> Option<&V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get(key)
    }

    pub fn get_key_value<Q, K, V, S, A>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get_key_value(key)
    }

    pub fn get_full<Q, K, V, S, A>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get_full(key)
    }

    pub fn get_mut<Q, K, V, S, A>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.get_mut(key)
    }

    pub fn get_full_mut<Q, K, V, S, A>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.get_full_mut(key)
    }

    pub fn keys<K, V, S, A>(&self) -> Keys<'_, K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.keys()
    }

    pub fn into_keys<K, V, S, A>(self) -> IntoKeys<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.into_proj::<K, V, S, A>();

        proj_self.into_keys()
    }

    pub fn iter<K, V, S, A>(&self) -> Iter<'_, K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.iter()
    }

    pub fn iter_mut<K, V, S, A>(&mut self) -> IterMut<'_, K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.iter_mut()
    }

    pub fn values<K, V, S, A>(&self) -> Values<'_, K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.values()
    }

    pub fn values_mut<K, V, S, A>(&mut self) -> ValuesMut<'_, K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.values_mut()
    }

    pub fn into_values<K, V, S, A>(self) -> IntoValues<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.into_proj::<K, V, S, A>();

        proj_self.into_values()
    }

    pub fn clear<K, V, S, A>(&mut self)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.clear();
    }

    pub fn truncate<K, V, S, A>(&mut self, len: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.truncate(len);
    }

    #[track_caller]
    pub fn drain<R, K, V, S, A>(&mut self, range: R) -> Drain<'_, K, V, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.drain(range)
    }

    pub fn swap_remove<Q, K, V, S, A>(&mut self, key: &Q) -> Option<V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.swap_remove(key)
    }

    pub fn swap_remove_entry<Q, K, V, S, A>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.swap_remove_entry(key)
    }

    pub fn swap_remove_full<Q, K, V, S, A>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.swap_remove_full(key)
    }

    pub fn shift_remove<Q, K, V, S, A>(&mut self, key: &Q) -> Option<V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shift_remove(key)
    }

    pub fn shift_remove_entry<Q, K, V, S, A>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shift_remove_entry(key)
    }

    pub fn shift_remove_full<Q, K, V, S, A>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        Q: any::Any + ?Sized + hash::Hash + Equivalent<K>,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shift_remove_full(key)
    }

    pub fn as_slice<K, V, S, A>(&self) -> &'_ Slice<K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.as_slice()
    }

    pub fn as_mut_slice<K, V, S, A>(&mut self) -> &mut Slice<K, V>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
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
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.insert(key, value)
    }

    pub fn insert_full<K, V, S, A>(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.insert_full(key, value)
    }

    pub fn insert_sorted<K, V, S, A>(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: any::Any + Eq + hash::Hash + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.insert_sorted(key, value)
    }

    #[track_caller]
    pub fn insert_before<K, V, S, A>(&mut self, mut index: usize, key: K, value: V) -> (usize, Option<V>)
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.insert_before(index, key, value)
    }

    #[track_caller]
    pub fn shift_insert<K, V, S, A>(&mut self, index: usize, key: K, value: V) -> Option<V>
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shift_insert(index, key, value)
    }

    pub fn entry<K, V, S, A>(&mut self, key: K) -> Entry<'_, K, V, A>
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.entry(key)
    }

    #[track_caller]
    pub fn splice<R, I, K, V, S, A>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, K, V, S, A>
    where
        K: any::Any + Eq + hash::Hash,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator + Clone,
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
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
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
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.pop()
    }

    pub fn retain<F, K, V, S, A>(&mut self, mut keep: F)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        F: FnMut(&K, &mut V) -> bool,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.retain(keep)
    }

    pub fn sort_keys<K, V, S, A>(&mut self)
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.sort_keys()
    }

    pub fn sort_by<F, K, V, S, A>(&mut self, mut cmp: F)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.sort_by(cmp)
    }

    pub fn sorted_by<F, K, V, S, A>(self, mut cmp: F) -> IntoIter<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_self = self.into_proj::<K, V, S, A>();

        proj_self.sorted_by(cmp)
    }

    pub fn sort_unstable_keys<K, V, S, A>(&mut self)
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.sort_unstable_keys()
    }

    pub fn sort_unstable_by<F, K, V, S, A>(&mut self, mut cmp: F)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.sort_unstable_by(cmp)
    }

    #[inline]
    pub fn sorted_unstable_by<F, K, V, S, A>(self, mut cmp: F) -> IntoIter<K, V, A>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
    {
        let proj_self = self.into_proj::<K, V, S, A>();

        proj_self.sorted_unstable_by(cmp)
    }

    pub fn sort_by_cached_key<T, F, K, V, S, A>(&mut self, mut sort_key: F)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
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
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.binary_search_keys(key)
    }

    #[inline]
    pub fn binary_search_by<F, K, V, S, A>(&self, f: F) -> Result<usize, usize>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
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
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
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
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        P: FnMut(&K, &V) -> bool,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.partition_point(pred)
    }

    pub fn reverse<K, V, S, A>(&mut self)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.reverse()
    }

    pub fn reserve<K, V, S, A>(&mut self, additional: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.reserve(additional)
    }

    pub fn reserve_exact<K, V, S, A>(&mut self, additional: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.reserve_exact(additional)
    }

    pub fn try_reserve<K, V, S, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.try_reserve(additional)
    }

    pub fn try_reserve_exact<K, V, S, A>(&mut self, additional: usize) -> Result<(), TryReserveError>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.try_reserve_exact(additional)
    }

    pub fn shrink_to_fit<K, V, S, A>(&mut self)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shrink_to_fit()
    }

    pub fn shrink_to<K, V, S, A>(&mut self, min_capacity: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shrink_to(min_capacity)
    }

    pub fn into_boxed_slice<K, V, S, A>(self) -> Box<Slice<K, V>, TypedProjAlloc<A>>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator
    {
        let proj_self = self.into_proj::<K, V, S, A>();

        proj_self.into_boxed_slice()
    }

    pub fn get_index<K, V, S, A>(&self, index: usize) -> Option<(&K, &V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get_index(index)
    }

    pub fn get_index_mut<K, V, S, A>(&mut self, index: usize) -> Option<(&K, &mut V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.get_index_mut(index)
    }

    pub fn get_index_entry<K, V, S, A>(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.get_index_entry(index)
    }

    pub fn get_range<R, K, V, S, A>(&self, range: R) -> Option<&Slice<K, V>>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
        R: ops::RangeBounds<usize>,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.get_range(range)
    }

    pub fn get_range_mut<R, K, V, S, A>(&mut self, range: R) -> Option<&mut Slice<K, V>>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
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
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.first()
    }

    pub fn first_mut<K, V, S, A>(&mut self) -> Option<(&K, &mut V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.first_mut()
    }

    pub fn first_entry<K, V, S, A>(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.first_entry()
    }

    #[doc(alias = "last_key_value")] // like `BTreeMap`
    pub fn last<K, V, S, A>(&self) -> Option<(&K, &V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj::<K, V, S, A>();

        proj_self.last()
    }

    pub fn last_mut<K, V, S, A>(&mut self) -> Option<(&K, &mut V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.last_mut()
    }

    pub fn last_entry<K, V, S, A>(&mut self) -> Option<IndexedEntry<'_, K, V, A>>
    where
        K: any::Any + Ord,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.last_entry()
    }

    pub fn swap_remove_index<K, V, S, A>(&mut self, index: usize) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.swap_remove_index(index)
    }

    pub fn shift_remove_index<K, V, S, A>(&mut self, index: usize) -> Option<(K, V)>
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.shift_remove_index(index)
    }

    #[track_caller]
    pub fn move_index<K, V, S, A>(&mut self, from: usize, to: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.move_index(from, to)
    }

    #[track_caller]
    pub fn swap_indices<K, V, S, A>(&mut self, a: usize, b: usize)
    where
        K: any::Any,
        V: any::Any,
        S: any::Any + hash::BuildHasher,
        A: any::Any + alloc::Allocator,
    {
        let proj_self = self.as_proj_mut::<K, V, S, A>();

        proj_self.swap_indices(a, b)
    }
}

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
