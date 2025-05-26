use crate::index_map_inner::{OpaqueIndexMapInner, TypedProjIndexMapInner, Drain, Iter, IterMut, IntoIter, Keys, IntoKeys, Values, ValuesMut, IntoValues, Entry, IndexedEntry, Slice, Splice};

pub use crate::index_map_inner::*;

use core::any;
use core::cmp;
use core::mem;
use core::ops;
use std::alloc;
use std::hash;

use opaque_alloc;
use opaque_error::{
    TryReserveError,
};
use opaque_hash;

pub use equivalent::Equivalent;
use opaque_alloc::TypedProjAlloc;
use opaque_hash::{TypedProjBuildHasher};

#[repr(transparent)]
pub struct TypedProjIndexMap<K, V, S = hash::RandomState, A = alloc::Global>
where
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
    pub fn hasher(&self) -> &TypedProjBuildHasher<S> {
        self.inner.hasher()
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
        self.inner.keys()
    }

    pub fn into_keys(self) -> IntoKeys<K, V, A> {
        self.inner.into_keys()
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.inner.iter_mut()
    }

    pub fn values(&self) -> Values<'_, K, V> {
        self.inner.values()
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        self.inner.values_mut()
    }

    pub fn into_values(self) -> IntoValues<K, V, A> {
        self.inner.into_values()
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
        self.inner.drain(range)
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
        self.inner.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut Slice<K, V> {
        self.inner.as_mut_slice()
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
        self.inner.entry(key)
    }

    #[track_caller]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, K, V, S, A>
    where
        K: Eq + hash::Hash,
        A: any::Any + alloc::Allocator + Clone,
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = (K, V)>,
    {
        self.inner.splice(range, replace_with)
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
        self.inner.sorted_by(cmp)
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
        self.inner.sorted_unstable_by(cmp)
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
        self.inner.into_boxed_slice()
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
        self.inner.get_index_entry(index)
    }

    pub fn get_range<R>(&self, range: R) -> Option<&Slice<K, V>>
    where
        R: ops::RangeBounds<usize>,
    {
        self.inner.get_range(range)
    }

    pub fn get_range_mut<R>(&mut self, range: R) -> Option<&mut Slice<K, V>>
    where
        R: ops::RangeBounds<usize>,
    {
        self.inner.get_range_mut(range)
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
        self.inner.first_entry()
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
        self.inner.last_entry()
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

    fn index(&self, key: &Q) -> &V {
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
    fn index_mut(&mut self, key: &Q) -> &mut V {
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
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
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

impl<K, V, S, A> ops::Index<ops::Range<usize>> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = Slice<K, V>;

    fn index(&self, range: ops::Range<usize>) -> &Self::Output {
        Slice::from_slice(&self.inner.as_entries()[range])
    }
}

impl<K, V, S, A> ops::IndexMut<ops::Range<usize>> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn index_mut(&mut self, range: ops::Range<usize>) -> &mut Self::Output {
        Slice::from_slice_mut(&mut self.inner.as_entries_mut()[range])
    }
}

impl<K, V, S, A> ops::Index<ops::RangeFrom<usize>> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = Slice<K, V>;

    fn index(&self, range: ops::RangeFrom<usize>) -> &Self::Output {
        Slice::from_slice(&self.inner.as_entries()[range])
    }
}

impl<K, V, S, A> ops::IndexMut<ops::RangeFrom<usize>> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn index_mut(&mut self, range: ops::RangeFrom<usize>) -> &mut Self::Output {
        Slice::from_slice_mut(&mut self.inner.as_entries_mut()[range])
    }
}

impl<K, V, S, A> ops::Index<ops::RangeFull> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = Slice<K, V>;

    fn index(&self, range: ops::RangeFull) -> &Self::Output {
        Slice::from_slice(&self.inner.as_entries()[range])
    }
}

impl<K, V, S, A> ops::IndexMut<ops::RangeFull> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn index_mut(&mut self, range: ops::RangeFull) -> &mut Self::Output {
        Slice::from_slice_mut(&mut self.inner.as_entries_mut()[range])
    }
}

impl<K, V, S, A> ops::Index<ops::RangeInclusive<usize>> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = Slice<K, V>;

    fn index(&self, range: ops::RangeInclusive<usize>) -> &Self::Output {
        Slice::from_slice(&self.inner.as_entries()[range])
    }
}

impl<K, V, S, A> ops::IndexMut<ops::RangeInclusive<usize>> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn index_mut(&mut self, range: ops::RangeInclusive<usize>) -> &mut Self::Output {
        Slice::from_slice_mut(&mut self.inner.as_entries_mut()[range])
    }
}

impl<K, V, S, A> ops::Index<ops::RangeTo<usize>> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = Slice<K, V>;

    fn index(&self, range: ops::RangeTo<usize>) -> &Self::Output {
        Slice::from_slice(&self.inner.as_entries()[range])
    }
}

impl<K, V, S, A> ops::IndexMut<ops::RangeTo<usize>> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn index_mut(&mut self, range: ops::RangeTo<usize>) -> &mut Self::Output {
        Slice::from_slice_mut(&mut self.inner.as_entries_mut()[range])
    }
}

impl<K, V, S, A> ops::Index<ops::RangeToInclusive<usize>> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = Slice<K, V>;

    fn index(&self, range: ops::RangeToInclusive<usize>) -> &Self::Output {
        Slice::from_slice(&self.inner.as_entries()[range])
    }
}

impl<K, V, S, A> ops::IndexMut<ops::RangeToInclusive<usize>> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn index_mut(&mut self, range: ops::RangeToInclusive<usize>) -> &mut Self::Output {
        Slice::from_slice_mut(&mut self.inner.as_entries_mut()[range])
    }
}

impl<K, V, S, A> ops::Index<(ops::Bound<usize>, ops::Bound<usize>)> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    type Output = Slice<K, V>;

    fn index(&self, range: (ops::Bound<usize>, ops::Bound<usize>)) -> &Self::Output {
        Slice::from_slice(&self.inner.as_entries()[range])
    }
}

impl<K, V, S, A> ops::IndexMut<(ops::Bound<usize>, ops::Bound<usize>)> for TypedProjIndexMap<K, V, S, A>
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn index_mut(&mut self, range: (ops::Bound<usize>, ops::Bound<usize>)) -> &mut Self::Output {
        Slice::from_slice_mut(&mut self.inner.as_entries_mut()[range])
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
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
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
        Self::with_capacity_and_hasher_in(0, S::default(), A::default())
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

#[cfg(test)]
mod index_map_layout_tests {
    use super::*;
    use core::mem;
    use core::ptr::NonNull;

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

    struct Pair(u8, u64);

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

    layout_tests!(u8_u8_random_state_global, u8, u8, hash::RandomState, alloc::Global);
    layout_tests!(u64_pair_dummy_hasher_dummy_alloc, u64, Pair, DummyBuildHasher, DummyAlloc);
    layout_tests!(unit_str_zst_hasher_dummy_alloc, (), &'static str, DummyBuildHasher, DummyAlloc);
}

#[cfg(test)]
mod index_map_assert_send_sync {
    use super::*;

    #[test]
    fn test_assert_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjIndexMap<i32, i32, hash::RandomState, alloc::Global>>();
    }
}
