#![feature(allocator_api)]
#![feature(slice_iter_mut_as_mut_slice)]
use std::any::TypeId;
use std::iter::FusedIterator;
use std::fmt;
use std::hash;
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;
use core::cmp::Ordering;

use opaque_alloc;
use opaque_hash;
use opaque_vec::OpaqueVec;

pub use equivalent::Equivalent;

pub struct Keys<'a, K, V> {
    iter: std::slice::Iter<'a, Bucket<K, V>>,
}

impl<'a, K, V> Keys<'a, K, V> {
    fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self {
            iter: entries.iter(),
        }
    }
}

impl<'a, K, V> Clone for Keys<'a, K, V> {
    fn clone(&self) -> Self {
        Keys {
            iter: self.iter.clone(),
        }
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
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, K, V> FusedIterator for Keys<'a, K, V> {}

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

pub struct IntoKeys<K, V> {
    iter: opaque_vec::IntoIter<Bucket<K, V>, opaque_alloc::OpaqueAlloc>,
}

impl<K, V> IntoKeys<K, V>
where
    K: 'static,
    V: 'static,
{
    fn new(entries: OpaqueVec) -> Self {
        Self {
            iter: entries.into_iter(),
        }
    }
}

impl<K, V> Iterator for IntoKeys<K, V> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::key)
    }
}

impl<K, V> DoubleEndedIterator for IntoKeys<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::key)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::key)
    }
}

impl<K, V> ExactSizeIterator for IntoKeys<K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> FusedIterator for IntoKeys<K, V> {}

impl<K, V> fmt::Debug for IntoKeys<K, V>
where
    K: fmt::Debug + 'static,
    V: 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::key_ref);
        f.debug_list().entries(iter).finish()
    }
}

impl<K, V> Default for IntoKeys<K, V>
where
    K: 'static,
    V: 'static,
{
    fn default() -> Self {
        Self {
            iter: OpaqueVec::new::<Bucket<K, V>>().into_iter(),
        }
    }
}

pub struct Values<'a, K, V> {
    iter: std::slice::Iter<'a, Bucket<K, V>>,
}

impl<'a, K, V> Values<'a, K, V> {
    fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self {
            iter: entries.iter(),
        }
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

impl<'a, K, V> FusedIterator for Values<'a, K, V> {}

impl<K, V> Clone for Values<'_, K, V> {
    fn clone(&self) -> Self {
        Values {
            iter: self.iter.clone(),
        }
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

impl<'a, K, V> FusedIterator for ValuesMut<'a, K, V> {}

impl<K, V: fmt::Debug> fmt::Debug for ValuesMut<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::value_ref);
        f.debug_list().entries(iter).finish()
    }
}

impl<K, V> Default for ValuesMut<'_, K, V> {
    fn default() -> Self {
        Self {
            iter: [].iter_mut(),
        }
    }
}

pub struct IntoValues<K, V> {
    iter: opaque_vec::IntoIter<Bucket<K, V>, opaque_alloc::OpaqueAlloc>,
}

impl<K, V> IntoValues<K, V>
where
    K: 'static,
    V: 'static,
{
    fn new(entries: OpaqueVec) -> Self {
        Self {
            iter: entries.into_iter(),
        }
    }
}

impl<K, V> Iterator for IntoValues<K, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Bucket::value)
    }
}

impl<K, V> DoubleEndedIterator for IntoValues<K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(Bucket::value)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.iter.nth_back(n).map(Bucket::value)
    }
}

impl<K, V> ExactSizeIterator for IntoValues<K, V> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<K, V> FusedIterator for IntoValues<K, V> {}

impl<K, V> fmt::Debug for IntoValues<K, V>
where
    K: 'static,
    V: fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter.as_slice().iter().map(Bucket::value_ref);
        f.debug_list().entries(iter).finish()
    }
}

impl<K, V> Default for IntoValues<K, V>
where
    K: 'static,
    V: 'static,
{
    fn default() -> Self {
        Self {
            iter: OpaqueVec::new::<Bucket<K, V>>().into_iter(),
        }
    }
}

#[repr(transparent)]
pub struct Slice<'a, K, V> {
    entries: &'a [Bucket<K, V>],
}

impl<'a, K, V> Slice<'a, K, V> {
    const fn from_slice(entries: &'a [Bucket<K, V>]) -> Self {
        Self {
            entries,
        }
    }

    pub fn iter(&'a self) -> Iter<'a, K, V> {
        Iter::new(&self.entries)
    }

    pub fn values(&'a self) -> Values<'a, K, V> {
        Values::new(&self.entries)
    }

    pub fn binary_search_keys(&self, x: &K) -> Result<usize, usize>
    where
        K: Ord
    {
        self.binary_search_by(|p, _| p.cmp(x))
    }

    #[inline]
    pub fn binary_search_by<F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a K, &'a V) -> Ordering,
    {
        self.entries.binary_search_by(move |a| f(&a.key, &a.value))
    }

    #[inline]
    pub fn binary_search_by_key<B, F>(&'a self, b: &B, mut f: F) -> Result<usize, usize>
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
        self.entries
            .partition_point(move |a| pred(&a.key, &a.value))
    }
}

#[repr(transparent)]
pub struct SliceMut<'a, K, V> {
    entries: &'a mut [Bucket<K, V>],
}

impl<'a, K, V> SliceMut<'a, K, V> {
    const fn from_slice_mut(entries: &'a mut [Bucket<K, V>]) -> Self {
        Self {
            entries,
        }
    }

    pub fn iter(&'a self) -> Iter<'a, K, V> {
        Iter::new(&self.entries)
    }

    pub fn values(&'a self) -> Values<'a, K, V> {
        Values::new(&self.entries)
    }
}

pub struct Iter<'a, K, V> {
    iter: std::slice::Iter<'a, Bucket<K, V>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    #[inline]
    fn new(entries: &'a [Bucket<K, V>]) -> Self {
        Self {
            iter: entries.iter(),
        }
    }

    fn as_slice(&self) -> Slice<'a, K, V> {
        Slice::from_slice(self.iter.as_slice())
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let bucket = self.iter.next()?;

        Some((&bucket.key, &bucket.value))
    }
}

pub struct IterMut<'a, K, V> {
    iter: std::slice::IterMut<'a, Bucket<K, V>>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    #[inline]
    fn new(entries: &'a mut [Bucket<K, V>]) -> Self {
        Self {
            iter: entries.iter_mut(),
        }
    }

    fn as_slice_mut(&'a mut self) -> SliceMut<'a, K, V> {
        SliceMut::from_slice_mut(self.iter.as_mut_slice())
    }

    pub fn into_slice_mut(self) -> SliceMut<'a, K, V> {
        SliceMut::from_slice_mut(self.iter.into_slice())
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        let bucket = self.iter.next()?;

        Some((&bucket.key, &mut bucket.value))
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct HashValue {
    value: usize,
}

impl HashValue {
    #[inline]
    const fn new(value: usize) -> Self {
        Self {
            value,
        }
    }

    #[inline(always)]
    const fn get(self) -> u64 {
        self.value as u64
    }
}

#[derive(Debug)]
struct Bucket<K, V> {
    hash: HashValue,
    key: K,
    value: V,
}

impl<K, V> Bucket<K, V> {
    #[inline]
    const fn new(hash: HashValue, key: K, value: V) -> Self {
        Self {
            hash,
            key,
            value,
        }
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct OpaqueBucketSize {
    hash_size: usize,
    key_size: usize,
    value_size: usize,
}

impl OpaqueBucketSize {
    #[inline]
    const fn new<K, V>() -> Self
    where
        K: Sized,
        V: Sized,
    {
        Self {
            hash_size: std::mem::size_of::<HashValue>(),
            key_size: std::mem::size_of::<K>(),
            value_size: std::mem::size_of::<V>(),
        }
    }

    #[inline]
    const fn hash_size(&self) -> usize {
        self.hash_size
    }

    #[inline]
    const fn key_size(&self) -> usize {
        self.key_size
    }

    #[inline]
    const fn value_size(&self) -> usize {
        self.value_size
    }

    #[inline]
    const fn bucket_size(&self) -> usize {
        self.hash_size + self.key_size + self.value_size
    }
}

pub(crate) struct OpaqueMapInner {
    indices: hashbrown::HashTable<usize>,
    entries: OpaqueVec,
    bucket_size: OpaqueBucketSize,
}

impl Clone for OpaqueMapInner {
    fn clone(&self) -> Self {
        Self {
            indices: self.indices.clone(),
            entries: self.entries.clone(),
            bucket_size: self.bucket_size,
        }
    }
}

#[inline(always)]
fn get_hash<K, V>(entries: &[Bucket<K, V>]) -> impl Fn(&usize) -> u64 + '_ {
    move |&i| entries[i].hash.get()
}

#[inline]
fn equivalent<'a, K, V, Q: ?Sized + Equivalent<K>>(
    key: &'a Q,
    entries: &'a [Bucket<K, V>],
) -> impl Fn(&usize) -> bool + 'a {
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
    let index = table
        .find_mut(hash.get(), move |&i| i == old)
        .expect("index not found");
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

fn reserve_entries<K, V>(entries: &mut OpaqueVec, additional: usize, try_capacity: usize) {
    // Use a soft-limit on the maximum capacity, but if the caller explicitly
    // requested more, do it and let them have the resulting panic.
    let try_capacity = try_capacity.min(max_entries_capacity::<K, V>());
    let try_add = try_capacity - entries.len();
    if try_add > additional && entries.try_reserve_exact(try_add).is_ok() {
        return;
    }
    entries.reserve_exact(additional);
}

struct RefMut<'a, K, V> {
    indices: &'a mut hashbrown::HashTable<usize>,
    entries: &'a mut OpaqueVec,
    _marker: core::marker::PhantomData<(K, V)>,
}

impl<'a, K, V> RefMut<'a, K, V>
where
    K: 'static,
    V: 'static,
{
    #[inline]
    fn new(indices: &'a mut hashbrown::HashTable<usize>, entries: &'a mut OpaqueVec) -> Self {
        Self {
            indices,
            entries,
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    fn reserve_entries(&mut self, additional: usize) {
        reserve_entries::<K, V>(self.entries, additional, self.indices.capacity());
    }

    fn insert_unique(self, hash: HashValue, key: K, value: V) -> OccupiedEntry<'a, K, V> {
        let i = self.indices.len();
        debug_assert_eq!(i, self.entries.len());
        let entry = self
            .indices
            .insert_unique(hash.get(), i, get_hash(self.entries.as_slice::<Bucket<K, V>>()));
        if self.entries.len() == self.entries.capacity() {
            // We can't call `indices.capacity()` while this `entry` has borrowed it, so we'll have
            // to amortize growth on our own. It's still an improvement over the basic `Vec::push`
            // doubling though, since we also consider `MAX_ENTRIES_CAPACITY`.
            reserve_entries::<K, V>(self.entries, 1, 2 * self.entries.capacity());
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
            entries.as_slice::<Bucket<K, V>>()[i].hash.get()
        });
        if self.entries.len() == self.entries.capacity() {
            // Reserve our own capacity synced to the indices,
            // rather than letting `Vec::insert` just double it.
            self.reserve_entries(1);
        }
        self.entries.shift_insert::<Bucket<K, V>>(index, Bucket { hash, key, value });
    }

    fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        match self.entries.get::<Bucket<K, V>>(index) {
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
        let entry = self.entries.shift_remove::<Bucket<K, V>>(index);

        (entry.key, entry.value)
    }

    fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        match self.entries.get::<Bucket<K, V>>(index) {
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
        let entry = self.entries.swap_remove::<Bucket<K, V>>(index);

        // correct index that points to the entry that had to swap places
        if let Some(entry) = self.entries.get::<Bucket<K, V>>(index) {
            // was not last element
            // examine new element in `index` and find it in indices
            let last = self.entries.len();
            update_index(self.indices, entry.hash, last, index);
        }

        (entry.key, entry.value)
    }

    fn decrement_indices(&mut self, start: usize, end: usize) {
        // Use a heuristic between a full sweep vs. a `find()` for every shifted item.
        let shifted_entries = &self.entries.as_slice::<Bucket<K, V>>()[start..end];
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
        let shifted_entries = &self.entries.as_slice::<Bucket<K, V>>()[start..end];
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
        let from_hash = self.entries.as_slice::<Bucket<K, V>>()[from].hash;
        let _ = self.entries.as_slice::<Bucket<K, V>>()[to]; // explicit bounds check
        if from != to {
            // Use a sentinel index so other indices don't collide.
            update_index(self.indices, from_hash, from, usize::MAX);

            // Update all other indices and rotate the entry positions.
            if from < to {
                self.decrement_indices(from + 1, to + 1);
                self.entries.as_mut_slice::<Bucket<K, V>>()[from..=to].rotate_left(1);
            } else if to < from {
                self.increment_indices(to, from);
                self.entries.as_mut_slice::<Bucket<K, V>>()[to..=from].rotate_right(1);
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
            [self.entries.as_slice::<Bucket<K, V>>()[a].hash.get(), self.entries.as_slice::<Bucket<K, V>>()[b].hash.get()],
            move |i, &x| if i == 0 { x == a } else { x == b },
        ) {
            [Some(ref_a), Some(ref_b)] => {
                core::mem::swap(ref_a, ref_b);
                self.entries.as_mut_slice::<Bucket<K, V>>().swap(a, b);
            }
            _ => panic!("indices not found"),
        }
    }
}

impl OpaqueMapInner {
    #[inline]
    pub(crate) fn new<K, V>() -> Self
    where
        K: 'static,
        V: 'static,
    {
        let indices = hashbrown::HashTable::new();
        let entries = OpaqueVec::new::<Bucket<K, V>>();
        let bucket_size = OpaqueBucketSize::new::<K, V>();

        Self {
            indices,
            entries,
            bucket_size,
        }
    }

    #[inline]
    pub(crate) fn with_capacity<K, V>(capacity: usize) -> Self
    where
        K: 'static,
        V: 'static,
    {
        let indices = hashbrown::HashTable::with_capacity(capacity);
        let entries = OpaqueVec::with_capacity::<Bucket<K, V>>(capacity);
        let bucket_size = OpaqueBucketSize::new::<K, V>();

        Self {
            indices,
            entries,
            bucket_size,
        }
    }

    #[inline]
    fn borrow_mut<K, V>(&mut self) -> RefMut<'_, K, V>
    where
        K: 'static,
        V: 'static,
    {
        RefMut::new(&mut self.indices, &mut self.entries)
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.indices.len()
    }

    #[inline]
    pub(crate) fn capacity(&self) -> usize {
        Ord::min(self.indices.capacity(), self.entries.capacity())
    }

    pub(crate) fn clear(&mut self) {
        self.indices.clear();
        self.entries.clear();
    }

    pub(crate) fn truncate<K, V>(&mut self, len: usize)
    where
        K: 'static,
        V: 'static,
    {
        if len < self.len() {
            self.erase_indices::<K, V>(len, self.entries.len());
            self.entries.truncate(len);
        }
    }

    /*
    #[track_caller]
    pub(crate) fn drain<R>(&mut self, range: R) -> vec::Drain<'_, Bucket<K, V>>
    where
        R: RangeBounds<usize>,
    {
        let range = simplify_range(range, self.entries.len());
        self.erase_indices(range.start, range.end);
        self.entries.drain(range)
    }

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

    #[track_caller]
    pub(crate) fn split_off(&mut self, at: usize) -> Self {
        let len = self.entries.len();
        assert!(
            at <= len,
            "index out of bounds: the len is {len} but the index is {at}. Expected index <= len"
        );

        self.erase_indices(at, self.entries.len());
        let entries = self.entries.split_off(at);

        let mut indices = Indices::with_capacity(entries.len());
        insert_bulk_no_grow(&mut indices, &entries);
        Self { indices, entries }
    }

    #[track_caller]
    pub(crate) fn split_splice<R>(&mut self, range: R) -> (Self, vec::IntoIter<Bucket<K, V>>)
    where
        R: RangeBounds<usize>,
    {
        let range = simplify_range(range, self.len());
        self.erase_indices(range.start, self.entries.len());
        let entries = self.entries.split_off(range.end);
        let drained = self.entries.split_off(range.start);

        let mut indices = Indices::with_capacity(entries.len());
        insert_bulk_no_grow(&mut indices, &entries);
        (Self { indices, entries }, drained.into_iter())
    }

    pub(crate) fn append_unchecked(&mut self, other: &mut Self) {
        self.reserve(other.len());
        insert_bulk_no_grow(&mut self.indices, &other.entries);
        self.entries.append(&mut other.entries);
        other.indices.clear();
    }

    pub(crate) fn reserve(&mut self, additional: usize) {
        self.indices.reserve(additional, get_hash(&self.entries));
        // Only grow entries if necessary, since we also round up capacity.
        if additional > self.entries.capacity() - self.entries.len() {
            self.borrow_mut().reserve_entries(additional);
        }
    }

    pub(crate) fn reserve_exact(&mut self, additional: usize) {
        self.indices.reserve(additional, get_hash(&self.entries));
        self.entries.reserve_exact(additional);
    }

    pub(crate) fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.indices
            .try_reserve(additional, get_hash(&self.entries))
            .map_err(TryReserveError::from_hashbrown)?;
        // Only grow entries if necessary, since we also round up capacity.
        if additional > self.entries.capacity() - self.entries.len() {
            self.try_reserve_entries(additional)
        } else {
            Ok(())
        }
    }

    fn try_reserve_entries(&mut self, additional: usize) -> Result<(), TryReserveError> {
        // Use a soft-limit on the maximum capacity, but if the caller explicitly
        // requested more, do it and let them have the resulting error.
        let new_capacity = Ord::min(self.indices.capacity(), Self::MAX_ENTRIES_CAPACITY);
        let try_add = new_capacity - self.entries.len();
        if try_add > additional && self.entries.try_reserve_exact(try_add).is_ok() {
            return Ok(());
        }
        self.entries
            .try_reserve_exact(additional)
            .map_err(TryReserveError::from_alloc)
    }

    pub(crate) fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.indices
            .try_reserve(additional, get_hash(&self.entries))
            .map_err(TryReserveError::from_hashbrown)?;
        self.entries
            .try_reserve_exact(additional)
            .map_err(TryReserveError::from_alloc)
    }

    pub(crate) fn shrink_to(&mut self, min_capacity: usize) {
        self.indices
            .shrink_to(min_capacity, get_hash(&self.entries));
        self.entries.shrink_to(min_capacity);
    }

     */

    pub(crate) fn pop<K, V>(&mut self) -> Option<(K, V)>
    where
        K: 'static,
        V: 'static,
    {
        if let Some(entry) = self.entries.pop::<Bucket<K, V>>() {
            let last = self.entries.len();
            erase_index(&mut self.indices, entry.hash, last);
            Some((entry.key, entry.value))
        } else {
            None
        }
    }

    pub(crate) fn get_index_of<Q, K, V>(&self, hash: HashValue, key: &Q) -> Option<usize>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + Equivalent<K>,
    {
        let eq = equivalent(key, self.entries.as_slice::<Bucket<K, V>>());

        self.indices.find(hash.get(), eq).copied()
    }

    fn push_entry<K, V>(&mut self, hash: HashValue, key: K, value: V)
    where
        K: 'static,
        V: 'static,
    {
        if self.entries.len() == self.entries.capacity() {
            // Reserve our own capacity synced to the indices,
            // rather than letting `Vec::push` just double it.
            self.borrow_mut::<K, V>().reserve_entries(1);
        }

        self.entries.push(Bucket { hash, key, value });
    }

    pub(crate) fn insert_full<K, V>(&mut self, hash: HashValue, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + 'static,
        V: 'static,
    {
        let eq = equivalent(&key, self.entries.as_slice::<Bucket<K, V>>());
        let hasher = get_hash(self.entries.as_slice::<Bucket<K, V>>());
        match self.indices.entry(hash.get(), eq, hasher) {
            hashbrown::hash_table::Entry::Occupied(entry) => {
                let i = *entry.get();

                (i, Some(core::mem::replace(&mut self.as_entries_mut::<K, V>()[i].value, value)))
            }
            hashbrown::hash_table::Entry::Vacant(entry) => {
                let i = self.entries.len();
                entry.insert(i);
                self.push_entry(hash, key, value);

                debug_assert_eq!(self.indices.len(), self.entries.len());

                (i, None)
            }
        }
    }

    pub(crate) fn shift_remove_full<Q, K, V>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, K, V)>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + Equivalent<K>,
    {
        let eq = equivalent(key, self.entries.as_slice::<Bucket<K, V>>());
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
    pub(crate) fn shift_remove_index<K, V>(&mut self, index: usize) -> Option<(K, V)>
    where
        K: 'static,
        V: 'static,
    {
        self.borrow_mut::<K, V>().shift_remove_index(index)
    }

    #[inline]
    #[track_caller]
    pub(crate) fn move_index<K, V>(&mut self, from: usize, to: usize)
    where
        K: 'static,
        V: 'static,
    {
        self.borrow_mut::<K, V>().move_index(from, to);
    }

    #[inline]
    #[track_caller]
    pub(crate) fn swap_indices<K, V>(&mut self, a: usize, b: usize)
    where
        K: 'static,
        V: 'static,
    {
        self.borrow_mut::<K, V>().swap_indices(a, b);
    }

    pub(crate) fn swap_remove_full<Q, K, V>(&mut self, hash: HashValue, key: &Q) -> Option<(usize, K, V)>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + Equivalent<K>,
    {
        let eq = equivalent(key, self.entries.as_slice::<Bucket<K, V>>());
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
    pub(crate) fn swap_remove_index<K, V>(&mut self, index: usize) -> Option<(K, V)>
    where
        K: 'static,
        V: 'static,
    {
        self.borrow_mut::<K, V>().swap_remove_index(index)
    }

    fn erase_indices<K, V>(&mut self, start: usize, end: usize)
    where
        K: 'static,
        V: 'static,
    {
        let (init, shifted_entries) = self.entries.as_slice::<Bucket<K, V>>().split_at(end);
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

    pub(crate) fn retain_in_order<F, K, V>(&mut self, mut keep: F)
    where
        K: 'static,
        V: 'static,
        F: FnMut(&mut K, &mut V) -> bool,
    {
        self.entries
            .retain_mut::<_, Bucket<K, V>>(|entry: &mut Bucket<K, V>| keep(&mut entry.key, &mut entry.value));
        if self.entries.len() < self.indices.len() {
            self.rebuild_hash_table::<K, V>();
        }
    }

    fn rebuild_hash_table<K, V>(&mut self)
    where
        K: 'static,
        V: 'static,
    {
        self.indices.clear();
        insert_bulk_no_grow(&mut self.indices, self.entries.as_slice::<Bucket<K, V>>());
    }

    pub(crate) fn reverse<V>(&mut self)
    where
        V: 'static,
    {
        self.entries.reverse::<V>();

        // No need to save hash indices, can easily calculate what they should
        // be, given that this is an in-place reversal.
        let len = self.entries.len();
        for i in &mut self.indices {
            *i = len - *i - 1;
        }
    }
}

impl OpaqueMapInner {
    #[inline]
    fn into_entries(self) -> OpaqueVec {
        self.entries
    }

    #[inline]
    fn as_entries<K, V>(&self) -> &[Bucket<K, V>]
    where
        K: 'static,
        V: 'static,
    {
        self.entries.as_slice::<Bucket<K, V>>()
    }

    #[inline]
    fn as_entries_mut<K, V>(&mut self) -> &mut [Bucket<K, V>]
    where
        K: 'static,
        V: 'static,
    {
        self.entries.as_mut_slice::<Bucket<K, V>>()
    }

    fn with_entries<F, K, V>(&mut self, f: F)
    where
        K: 'static,
        V: 'static,
        F: FnOnce(&mut [Bucket<K, V>]),
    {
        f(self.entries.as_mut_slice::<Bucket<K, V>>());

        self.rebuild_hash_table::<K, V>();
    }
}

impl OpaqueMapInner {
    pub(crate) fn entry<K, V>(&mut self, hash: HashValue, key: K) -> Entry<'_, K, V>
    where
        K: Eq + 'static,
        V: 'static,
    {
        let entries = &mut self.entries;
        let eq = equivalent(&key, entries.as_slice::<Bucket<K, V>>());
        match self.indices.find_entry(hash.get(), eq) {
            Ok(index) => Entry::Occupied(OccupiedEntry { entries, index, _marker: PhantomData, }),
            Err(absent) => Entry::Vacant(VacantEntry {
                map: RefMut::new(absent.into_table(), entries),
                hash,
                key,
            }),
        }
    }
}

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: 'static,
    V: 'static,
{
    pub fn index(&self) -> usize {
        match *self {
            Entry::Occupied(ref entry) => entry.index(),
            Entry::Vacant(ref entry) => entry.index(),
        }
    }

    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {
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

impl<K, V> fmt::Debug for Entry<'_, K, V>
where
    K: fmt::Debug + 'static,
    V: fmt::Debug + 'static,
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

pub struct OccupiedEntry<'a, K, V> {
    entries: &'a mut OpaqueVec,
    index: hashbrown::hash_table::OccupiedEntry<'a, usize>,
    _marker: PhantomData<(K, V)>,
}

impl<'a, K, V> OccupiedEntry<'a, K, V>
where
    K: 'static,
    V: 'static,
{
    pub(crate) fn new(
        entries: &'a mut OpaqueVec,
        index: hashbrown::hash_table::OccupiedEntry<'a, usize>,
    ) -> Self {
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
    fn into_ref_mut(self) -> RefMut<'a, K, V> {
        RefMut::new(self.index.into_table(), self.entries)
    }

    pub fn key(&self) -> &K {
        &self.entries.as_slice::<Bucket<K, V>>()[self.index()].key
    }

    pub(crate) fn key_mut(&mut self) -> &mut K {
        let index = self.index();

        &mut self.entries.as_mut_slice::<Bucket<K, V>>()[index].key
    }

    pub fn get(&self) -> &V {
        &self.entries.as_slice::<Bucket<K, V>>()[self.index()].value
    }

    pub fn get_mut(&mut self) -> &mut V {
        let index = self.index();

        &mut self.entries.as_mut_slice::<Bucket<K, V>>()[index].value
    }

    pub fn into_mut(self) -> &'a mut V {
        let index = self.index();

        &mut self.entries.as_mut_slice::<Bucket<K, V>>()[index].value
    }

    fn into_muts(self) -> (&'a mut K, &'a mut V) {
        let index = self.index();

        self.entries.as_mut_slice::<Bucket<K, V>>()[index].muts()
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
        RefMut::new(entry.into_table(), self.entries).swap_remove_finish(index)
    }

    pub fn shift_remove_entry(self) -> (K, V) {
        let (index, entry) = self.index.remove();
        RefMut::new(entry.into_table(), self.entries).shift_remove_finish(index)
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

impl<K, V> fmt::Debug for OccupiedEntry<'_, K, V>
where
    K: fmt::Debug + 'static,
    V: fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

impl<'a, K, V> From<IndexedEntry<'a, K, V>> for OccupiedEntry<'a, K, V>
where
    K: 'static,
    V: 'static,
{
    fn from(other: IndexedEntry<'a, K, V>) -> Self {
        let IndexedEntry {
            map: RefMut { indices, entries, _marker },
            index,
        } = other;
        let hash = entries.as_slice::<Bucket<K, V>>()[index].hash;
        let index = indices
            .find_entry(hash.get(), move |&i| i == index)
            .expect("index not found");

        Self {
            entries,
            index,
            _marker: PhantomData,
        }
    }
}

pub struct VacantEntry<'a, K, V> {
    map: RefMut<'a, K, V>,
    hash: HashValue,
    key: K,
}

impl<'a, K, V> VacantEntry<'a, K, V>
where
    K: 'static,
    V: 'static,
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

    pub fn insert_entry(self, value: V) -> OccupiedEntry<'a, K, V> {
        let Self { map, hash, key } = self;

        map.insert_unique(hash, key, value)
    }

    pub fn insert_sorted(self, value: V) -> (usize, &'a mut V)
    where
        K: Ord,
    {
        let slice = Slice::from_slice(self.map.entries.as_slice::<Bucket<K, V>>());
        let i = slice.binary_search_keys(&self.key).unwrap_err();

        (i, self.shift_insert(i, value))
    }

    pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
        self.map
            .shift_insert_unique(index, self.hash, self.key, value);

        &mut self.map.entries.as_mut_slice::<Bucket<K, V>>()[index].value
    }
}

impl<K, V> fmt::Debug for VacantEntry<'_, K, V>
where
    K: fmt::Debug + 'static,
    V: 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("VacantEntry").field(self.key()).finish()
    }
}

pub struct IndexedEntry<'a, K, V> {
    map: RefMut<'a, K, V>,
    index: usize,
}

impl<'a, K, V> IndexedEntry<'a, K, V>
where
    K: 'static,
    V: 'static,
{
    pub(crate) fn new(map: &'a mut OpaqueMap, index: usize) -> Self
    where
        K: Ord + 'static,
        V: 'static,
    {
        Self {
            map: map.inner.borrow_mut::<K, V>(),
            index,
        }
    }

    #[inline]
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn key(&self) -> &K {
        &self.map.entries.as_slice::<Bucket<K, V>>()[self.index].key
    }

    pub(crate) fn key_mut(&mut self) -> &mut K {
        &mut self.map.entries.as_mut_slice::<Bucket<K, V>>()[self.index].key
    }

    pub fn get(&self) -> &V {
        &self.map.entries.as_slice::<Bucket<K, V>>()[self.index].value
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.map.entries.as_mut_slice::<Bucket<K, V>>()[self.index].value
    }

    pub fn insert(&mut self, value: V) -> V {
        core::mem::replace(self.get_mut(), value)
    }

    pub fn into_mut(self) -> &'a mut V {
        &mut self.map.entries.as_mut_slice::<Bucket<K, V>>()[self.index].value
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

impl<K, V> fmt::Debug for IndexedEntry<'_, K, V>
where
    K: fmt::Debug + 'static,
    V: fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IndexedEntry")
            .field("index", &self.index)
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

impl<'a, K, V> From<OccupiedEntry<'a, K, V>> for IndexedEntry<'a, K, V>
where
    K: 'static,
    V: 'static,
{
    fn from(other: OccupiedEntry<'a, K, V>) -> Self {
        Self {
            index: other.index(),
            map: other.into_ref_mut(),
        }
    }
}


#[derive(Clone)]
pub struct OpaqueMap {
    inner: OpaqueMapInner,
    hash_builder: opaque_hash::OpaqueBuildHasher,
}

impl OpaqueMap {
    #[inline]
    fn into_entries(self) -> OpaqueVec {
        self.inner.into_entries()
    }

    #[inline]
    fn as_entries<K, V>(&self) -> &[Bucket<K, V>]
    where
        K: 'static,
        V: 'static,
    {
        self.inner.as_entries()
    }

    #[inline]
    fn as_entries_mut<K, V>(&mut self) -> &mut [Bucket<K, V>]
    where
        K: 'static,
        V: 'static,
    {
        self.inner.as_entries_mut()
    }

    fn with_entries<F, K, V>(&mut self, f: F)
    where
        K: 'static,
        V: 'static,
        F: FnOnce(&mut [Bucket<K, V>]),
    {
        self.inner.with_entries(f);
    }
}

impl OpaqueMap {
    pub fn new<K, V>() -> Self
    where
        K: 'static,
        V: 'static,
    {
        let inner = OpaqueMapInner::new::<K, V>();
        let opaque_hash_builder = opaque_hash::OpaqueBuildHasher::new::<hash::RandomState>(Box::new(hash::RandomState::default()));

        Self {
            inner,
            hash_builder: opaque_hash_builder,
        }
    }

    pub fn with_hasher<K, V, S>(hash_builder: S) -> Self
    where
        K: 'static,
        V: 'static,
        S: hash::BuildHasher + Clone + 'static,
    {
        let opaque_hash_builder = opaque_hash::OpaqueBuildHasher::new::<S>(Box::new(hash_builder));

        Self {
            inner: OpaqueMapInner::new::<K, V>(),
            hash_builder: opaque_hash_builder,
        }
    }

    #[inline]
    pub fn with_capacity_and_hasher<K, V, S>(capacity: usize, hash_builder: S) -> Self
    where
        K: 'static,
        V: 'static,
        S: hash::BuildHasher + Clone + 'static,
    {
        if capacity == 0 {
            Self::with_hasher::<K, V, S>(hash_builder)
        } else {
            let opaque_hash_builder = opaque_hash::OpaqueBuildHasher::new::<S>(Box::new(hash_builder));

            OpaqueMap {
                inner: OpaqueMapInner::with_capacity::<K, V>(capacity),
                hash_builder: opaque_hash_builder,
            }
        }
    }

    pub fn with_capacity<K, V>(capacity: usize) -> Self
    where
        K: 'static,
        V: 'static,
    {
        let inner = OpaqueMapInner::with_capacity::<K, V>(capacity);
        let opaque_hash_builder = opaque_hash::OpaqueBuildHasher::new(Box::new(hash::RandomState::default()));

        Self {
            inner,
            hash_builder: opaque_hash_builder,
        }
    }

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
        self.len() == 0
    }

    #[inline]
    pub const fn hasher(&self) -> &opaque_hash::OpaqueBuildHasher {
        &self.hash_builder
    }

    pub(crate) fn hash<Q: ?Sized + hash::Hash>(&self, key: &Q) -> HashValue {
        let mut hasher = self.hash_builder.build_hasher();
        key.hash(&mut hasher);

        HashValue::new(hasher.finish() as usize)
    }

    pub fn get_index_of<Q, K, V>(&self, key: &Q) -> Option<usize>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + hash::Hash + Equivalent<K> + 'static,
    {
        match self.as_entries::<K, V>() {
            [] => None,
            [x] => key.equivalent(&x.key).then_some(0),
            _ => {
                let hash = self.hash(key);
                self.inner.get_index_of::<Q, K, V>(hash, key)
            }
        }
    }

    pub fn contains_key<Q, K, V>(&self, key: &Q) -> bool
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + hash::Hash + Equivalent<K> + 'static,
    {
        self.get_index_of::<Q, K, V>(key).is_some()
    }

    pub fn get<Q, K, V>(&self, key: &Q) -> Option<&V>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + hash::Hash + Equivalent<K> + 'static,
    {
        if let Some(index) = self.get_index_of::<Q, K, V>(key) {
            let entry = &self.as_entries::<K, V>()[index];
            Some(&entry.value)
        } else {
            None
        }
    }

    pub fn get_key_value<Q, K, V>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + Hash + Equivalent<K> + 'static,
    {
        if let Some(i) = self.get_index_of::<Q, K, V>(key) {
            let entry = &self.as_entries::<K, V>()[i];
            Some((&entry.key, &entry.value))
        } else {
            None
        }
    }

    pub fn get_full<Q, K, V>(&self, key: &Q) -> Option<(usize, &K, &V)>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + Hash + Equivalent<K> + 'static,
    {
        if let Some(i) = self.get_index_of::<Q, K, V>(key) {
            let entry = &self.as_entries::<K, V>()[i];
            Some((i, &entry.key, &entry.value))
        } else {
            None
        }
    }

    pub fn get_mut<Q, K, V>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + hash::Hash + Equivalent<K> + 'static,
    {
        if let Some(i) = self.get_index_of::<Q, K, V>(key) {
            let entry = &mut self.as_entries_mut::<K, V>()[i];
            Some(&mut entry.value)
        } else {
            None
        }
    }

    pub fn get_full_mut<Q, K, V>(&mut self, key: &Q) -> Option<(usize, &K, &mut V)>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + Hash + Equivalent<K> + 'static,
    {
        if let Some(i) = self.get_index_of::<Q, K, V>(key) {
            let entry = &mut self.as_entries_mut::<K, V>()[i];

            Some((i, &entry.key, &mut entry.value))
        } else {
            None
        }
    }

    pub fn keys<K, V>(&self) -> Keys<'_, K, V>
    where
        K: 'static,
        V: 'static,
    {
        Keys::new(self.as_entries::<K, V>())
    }

    pub fn into_keys<K, V>(self) -> IntoKeys<K, V>
    where
        K: 'static,
        V: 'static,
    {
        IntoKeys::new(self.into_entries())
    }

    pub fn iter<K, V>(&self) -> Iter<'_, K, V>
    where
        K: 'static,
        V: 'static,
    {
        Iter::new(self.as_entries::<K, V>())
    }

    pub fn iter_mut<K, V>(&mut self) -> IterMut<'_, K, V>
    where
        K: 'static,
        V: 'static,
    {
        IterMut::new(self.as_entries_mut::<K, V>())
    }

    pub fn values<K, V>(&self) -> Values<'_, K, V>
    where
        K: 'static,
        V: 'static,
    {
        Values::new(self.as_entries())
    }

    pub fn values_mut<K, V>(&mut self) -> ValuesMut<'_, K, V>
    where
        K: 'static,
        V: 'static,
    {
        ValuesMut::new(self.as_entries_mut())
    }

    /*
    pub fn into_values(self) -> IntoValues<K, V> {
        IntoValues::new(self.into_entries())
    }
    */

    /// Remove all key-value pairs in the map, while preserving its capacity.
    ///
    /// Computes in **O(n)** time.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Shortens the map, keeping the first `len` elements and dropping the rest.
    ///
    /// If `len` is greater than the map's current length, this has no effect.
    pub fn truncate<K, V>(&mut self, len: usize)
    where
        K: 'static,
        V: 'static,
    {
        self.inner.truncate::<K, V>(len);
    }

    pub fn swap_remove<Q, K, V>(&mut self, key: &Q) -> Option<V>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + hash::Hash + Equivalent<K>,
    {
        fn third<A, B, C>(triple: (A, B, C)) -> C {
            triple.2
        }

        self.swap_remove_full(key).map(third)
    }

    pub fn swap_remove_entry<Q, K, V>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + hash::Hash + Equivalent<K>,
    {
        match self.swap_remove_full(key) {
            Some((_, key, value)) => Some((key, value)),
            None => None,
        }
    }

    pub fn swap_remove_full<Q, K, V>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + hash::Hash + Equivalent<K>,
    {
        match self.as_entries::<K, V>() {
            [x] if key.equivalent(&x.key) => {
                let (k, v) = self.inner.pop()?;
                Some((0, k, v))
            }
            [_] | [] => None,
            _ => {
                let hash = self.hash(key);
                self.inner.swap_remove_full::<Q, K, V>(hash, key)
            }
        }
    }

    pub fn shift_remove<Q, K, V>(&mut self, key: &Q) -> Option<V>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + Hash + Equivalent<K> + 'static,
    {
        fn third<A, B, C>(triple: (A, B, C)) -> C {
            triple.2
        }

        self.shift_remove_full::<Q, K, V>(key).map(third)
    }

    pub fn shift_remove_entry<Q, K, V>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + Hash + Equivalent<K> + 'static,
    {
        match self.shift_remove_full(key) {
            Some((_, key, value)) => Some((key, value)),
            None => None,
        }
    }

    pub fn shift_remove_full<Q, K, V>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        K: 'static,
        V: 'static,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        match self.as_entries::<K, V>() {
            [x] if key.equivalent(&x.key) => {
                let (k, v) = self.inner.pop()?;
                Some((0, k, v))
            }
            [_] | [] => None,
            _ => {
                let hash = self.hash(key);

                self.inner.shift_remove_full(hash, key)
            }
        }
    }

    pub fn as_slice<K, V>(&self) -> Slice<'_, K, V>
    where
        K: 'static,
        V: 'static,
    {
        Slice::from_slice(self.as_entries::<K, V>())
    }

    pub fn as_mut_slice<K, V>(&mut self) -> SliceMut<'_, K, V>
    where
        K: 'static,
        V: 'static,
    {
        SliceMut::from_slice_mut(self.as_entries_mut::<K, V>())
    }
}

impl OpaqueMap {
    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<V>
    where
        K: Eq + hash::Hash + 'static,
        V: 'static,
    {
        self.insert_full(key, value).1
    }

    pub fn insert_full<K, V>(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash + 'static,
        V: 'static,
    {
        let hash = self.hash(&key);

        self.inner.insert_full(hash, key, value)
    }

    pub fn insert_sorted<K, V>(&mut self, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash + Ord + 'static,
        V: 'static,
    {
        match self.binary_search_keys::<K, V>(&key) {
            Ok(i) => {
                let destination = self.get_index_mut::<K, V>(i).unwrap().1;
                let old_value = core::mem::replace(destination, value);

                (i, Some(old_value))
            },
            Err(i) => self.insert_before::<K, V>(i, key, value),
        }
    }

    #[track_caller]
    pub fn insert_before<K, V>(&mut self, mut index: usize, key: K, value: V) -> (usize, Option<V>)
    where
        K: Eq + hash::Hash + 'static,
        V: 'static,
    {
        let len = self.len();

        assert!(
            index <= len,
            "index out of bounds: the len is {len} but the index is {index}. Expected index <= len"
        );

        match self.entry::<K, V>(key) {
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
    pub fn shift_insert<K, V>(&mut self, index: usize, key: K, value: V) -> Option<V>
    where
        K: Eq + hash::Hash + 'static,
        V: 'static,
    {
        let len = self.len();
        match self.entry(key) {
            Entry::Occupied(mut entry) => {
                assert!(
                    index < len,
                    "index out of bounds: the len is {len} but the index is {index}"
                );

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

    pub fn entry<K, V>(&mut self, key: K) -> Entry<'_, K, V>
    where
        K: Eq + hash::Hash + 'static,
        V: 'static,
    {
        let hash = self.hash(&key);

        self.inner.entry(hash, key)
    }

    /*
    #[track_caller]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, K, V, S>
    where
        R: RangeBounds<usize>,
        I: IntoIterator<Item = (K, V)>,
    {
        Splice::new(self, range, replace_with.into_iter())
    }

    pub fn append<S2>(&mut self, other: &mut IndexMap<K, V, S2>) {
        self.extend(other.drain(..));
    }
     */
}

impl OpaqueMap {
    /*
    #[doc(alias = "pop_last")] // like `BTreeMap`
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.core.pop()
    }

    pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.core.retain_in_order(move |k, v| keep(k, v));
    }

    pub fn sort_keys(&mut self)
    where
        K: Ord,
    {
        self.with_entries(move |entries| {
            entries.sort_by(move |a, b| K::cmp(&a.key, &b.key));
        });
    }

    pub fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        self.with_entries(move |entries| {
            entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
        });
    }

    pub fn sorted_by<F>(self, mut cmp: F) -> IntoIter<K, V>
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        let mut entries = self.into_entries();
        entries.sort_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
        IntoIter::new(entries)
    }

    pub fn sort_unstable_keys(&mut self)
    where
        K: Ord,
    {
        self.with_entries(move |entries| {
            entries.sort_unstable_by(move |a, b| K::cmp(&a.key, &b.key));
        });
    }

    pub fn sort_unstable_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        self.with_entries(move |entries| {
            entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
        });
    }

    #[inline]
    pub fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<K, V>
    where
        F: FnMut(&K, &V, &K, &V) -> Ordering,
    {
        let mut entries = self.into_entries();
        entries.sort_unstable_by(move |a, b| cmp(&a.key, &a.value, &b.key, &b.value));
        IntoIter::new(entries)
    }

    pub fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
    where
        T: Ord,
        F: FnMut(&K, &V) -> T,
    {
        self.with_entries(move |entries| {
            entries.sort_by_cached_key(move |a| sort_key(&a.key, &a.value));
        });
    }
     */

    pub fn binary_search_keys<K, V>(&self, key: &K) -> Result<usize, usize>
    where
        K: Ord + 'static,
        V: 'static,
    {
        self.as_slice::<K, V>().binary_search_keys(key)
    }

    #[inline]
    pub fn binary_search_by<F, K, V>(&self, f: F) -> Result<usize, usize>
    where
        K: 'static,
        V: 'static,
        F: FnMut(&K, &V) -> Ordering,
    {
        self.as_slice::<K, V>().binary_search_by(f)
    }

    #[inline]
    pub fn binary_search_by_key<B, F, K, V>(&self, b: &B, f: F) -> Result<usize, usize>
    where
        K: 'static,
        V: 'static,
        F: FnMut(&K, &V) -> B,
        B: Ord,
    {
        self.as_slice::<K, V>().binary_search_by_key(b, f)
    }

    #[must_use]
    pub fn partition_point<P, K, V>(&self, pred: P) -> usize
    where
        K: 'static,
        V: 'static,
        P: FnMut(&K, &V) -> bool,
    {
        self.as_slice().partition_point(pred)
    }

    pub fn reverse<V>(&mut self)
    where
        V: 'static,
    {
        self.inner.reverse::<V>();
    }

    /*
    pub fn as_slice(&self) -> &Slice<K, V> {
        Slice::from_slice(self.as_entries())
    }

    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
        Slice::from_mut_slice(self.as_entries_mut())
    }

    pub fn into_boxed_slice(self) -> Box<Slice<K, V>> {
        Slice::from_boxed(self.into_entries().into_boxed_slice())
    }
    */

    pub fn get_index<K, V>(&self, index: usize) -> Option<(&K, &V)>
    where
        K: 'static,
        V: 'static,
    {
        self.as_entries::<K, V>().get(index).map(Bucket::refs)
    }

    pub fn get_index_mut<K, V>(&mut self, index: usize) -> Option<(&K, &mut V)>
    where
        K: 'static,
        V: 'static,
    {
        self.as_entries_mut::<K, V>().get_mut(index).map(Bucket::ref_mut)
    }

    /*
    pub fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V>> {
        if index >= self.len() {
            return None;
        }
        Some(IndexedEntry::new(&mut self.core, index))
    }

    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Slice<K, V>> {
        let entries = self.as_entries();
        let range = try_simplify_range(range, entries.len())?;
        entries.get(range).map(Slice::from_slice)
    }

    pub fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<K, V>> {
        let entries = self.as_entries_mut();
        let range = try_simplify_range(range, entries.len())?;
        entries.get_mut(range).map(Slice::from_mut_slice)
    }

    #[doc(alias = "first_key_value")] // like `BTreeMap`
    pub fn first(&self) -> Option<(&K, &V)> {
        self.as_entries().first().map(Bucket::refs)
    }

    pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.as_entries_mut().first_mut().map(Bucket::ref_mut)
    }

    pub fn first_entry(&mut self) -> Option<IndexedEntry<'_, K, V>> {
        self.get_index_entry(0)
    }

    #[doc(alias = "last_key_value")] // like `BTreeMap`
    pub fn last(&self) -> Option<(&K, &V)> {
        self.as_entries().last().map(Bucket::refs)
    }

    pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        self.as_entries_mut().last_mut().map(Bucket::ref_mut)
    }

    pub fn last_entry(&mut self) -> Option<IndexedEntry<'_, K, V>> {
        self.get_index_entry(self.len().checked_sub(1)?)
    }

    pub fn swap_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.core.swap_remove_index(index)
    }

    pub fn shift_remove_index(&mut self, index: usize) -> Option<(K, V)> {
        self.core.shift_remove_index(index)
    }

    #[track_caller]
    pub fn move_index(&mut self, from: usize, to: usize) {
        self.core.move_index(from, to)
    }

    #[track_caller]
    pub fn swap_indices(&mut self, a: usize, b: usize) {
        self.core.swap_indices(a, b)
    }
     */
}

pub struct Map<'a, K, V> {
    opaque_map: &'a OpaqueMap,
    _marker: std::marker::PhantomData<(K, V)>,
}

impl<'a, K, V> Map<'a, K, V>
where
    K: 'static,
    V: 'static,
{
    #[inline]
    const fn new(opaque_map: &'a OpaqueMap) -> Self {
        Self {
            opaque_map,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn capacity(&self) -> usize {
        self.opaque_map.capacity()
    }

    pub fn len(&self) -> usize {
        self.opaque_map.len()
    }

    pub fn get<Q>(&self, index: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Equivalent<K> + 'static,
    {
        self.opaque_map.get::<Q, K, V>(index)
    }

    pub fn keys(&self) -> Keys<'a, K, V> {
        self.opaque_map.keys::<K, V>()
    }

    pub fn as_slice(&self) -> Slice<'a, K, V> {
        self.opaque_map.as_slice::<K, V>()
    }
}

pub struct MapMut<'a, K, V> {
    opaque_map: &'a mut OpaqueMap,
    _marker: std::marker::PhantomData<(K, V)>,
}

impl<'a, K, V> MapMut<'a, K, V>
where
    K: 'static,
    V: 'static,
{
    #[inline]
    const fn new(opaque_map: &'a mut OpaqueMap) -> Self {
        Self {
            opaque_map,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn capacity(&self) -> usize {
        self.opaque_map.capacity()
    }

    pub fn len(&self) -> usize {
        self.opaque_map.len()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Equivalent<K> + 'static,
    {
        self.opaque_map.get::<Q, K, V>(key)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: ?Sized + Hash + Equivalent<K> + 'static,
    {
        self.opaque_map.get_mut::<Q, K, V>(key)
    }

    pub fn insert(&mut self, key: K, item: V) -> Option<V>
    where
        K: Hash + Eq,
    {
        self.opaque_map.insert::<K, V>(key, item)
    }

    pub fn keys(&'a self) -> Keys<'a, K, V> {
        self.opaque_map.keys::<K, V>()
    }

    pub fn as_slice(&'a self) -> Slice<'a, K, V> {
        self.opaque_map.as_slice::<K, V>()
    }
}

impl OpaqueMap {
    pub fn as_map<K, V>(&self) -> Map<'_, K, V>
    where
        K: 'static,
        V: 'static,
    {
        Map::<K, V>::new(self)
    }

    pub fn as_map_mut<K, V>(&mut self) -> MapMut<'_, K, V>
    where
        K: 'static,
        V: 'static,
    {
        MapMut::<K, V>::new(self)
    }
}
