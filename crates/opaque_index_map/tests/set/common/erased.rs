use opaque_index_map::OpaqueIndexSet;

use core::any;
use core::fmt;
use core::ops;
use std::hash;
use std::alloc;

use proptest::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct WrappingBuildHasher1<S> {
    build_hasher: S,
}

impl<S> WrappingBuildHasher1<S> {
    #[inline]
    pub const fn new(build_hasher: S) -> Self {
        Self { build_hasher }
    }
}

impl<S> hash::BuildHasher for WrappingBuildHasher1<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
{
    type Hasher = S::Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.build_hasher.build_hasher()
    }
}

#[derive(Clone, Default, Debug)]
pub struct WrappingBuildHasher2<S> {
    build_hasher: S,
}

impl<S> WrappingBuildHasher2<S> {
    #[inline]
    pub const fn new(build_hasher: S) -> Self {
        Self { build_hasher }
    }
}

impl<S> hash::BuildHasher for WrappingBuildHasher2<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
{
    type Hasher = S::Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.build_hasher.build_hasher()
    }
}

pub trait SingleBoundedValue: Arbitrary {
    fn bounded_any() -> impl Strategy<Value = Self>;
}

impl SingleBoundedValue for () { fn bounded_any() -> impl Strategy<Value = Self> { any::<()>() } }
impl SingleBoundedValue for u8 { fn bounded_any() -> impl Strategy<Value = Self> { any::<u8>() } }
impl SingleBoundedValue for u16 { fn bounded_any() -> impl Strategy<Value = Self> { any::<u16>() } }
impl SingleBoundedValue for u32 { fn bounded_any() -> impl Strategy<Value = Self> { any::<u32>() } }
impl SingleBoundedValue for u64 { fn bounded_any() -> impl Strategy<Value = Self> { any::<u64>() } }
impl SingleBoundedValue for usize { fn bounded_any() -> impl Strategy<Value = Self> { any::<usize>() } }
impl SingleBoundedValue for i8 { fn bounded_any() -> impl Strategy<Value = Self> { any::<i8>() } }
impl SingleBoundedValue for i16 { fn bounded_any() -> impl Strategy<Value = Self> { any::<i16>() } }
impl SingleBoundedValue for i32 { fn bounded_any() -> impl Strategy<Value = Self> { any::<i32>() } }
impl SingleBoundedValue for i64 { fn bounded_any() -> impl Strategy<Value = Self> { any::<i64>() } }
impl SingleBoundedValue for isize { fn bounded_any() -> impl Strategy<Value = Self> { any::<isize>() } }
impl SingleBoundedValue for String { fn bounded_any() -> impl Strategy<Value = Self> { any::<usize>().prop_map(|value| value.to_string()) } }

pub fn strategy_bounded_value<T>() -> impl Strategy<Value = T>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
{
    <T as SingleBoundedValue>::bounded_any()
}

pub fn strategy_alloc<A>() -> impl Strategy<Value = A>
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    Just(A::default())
}

pub fn strategy_build_hasher<S>() -> impl Strategy<Value = S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    Just(S::default())
}

pub fn strategy_type_erased_index_set_len<T, S, A>(length: usize) -> impl Strategy<Value = OpaqueIndexSet>
where
    T: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (proptest::collection::vec(strategy_bounded_value::<T>(), length), strategy_build_hasher::<S>(), strategy_alloc::<A>())
        .prop_map(move |(values, build_hasher, alloc)| {
            let mut opaque_set = OpaqueIndexSet::with_hasher_in::<T, S, A>(build_hasher, alloc);
            opaque_set.extend::<_, T, S, A>(values);

            opaque_set
        })
}

pub fn strategy_type_erased_index_set_max_len<T, S, A>(max_length: usize) -> impl Strategy<Value = OpaqueIndexSet>
where
    T: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (0..=max_length).prop_flat_map(move |length| strategy_type_erased_index_set_len::<T, S, A>(length))
}

pub fn strategy_type_erased_index_set_max_len_nonempty<T, S, A>(max_length: usize) -> impl Strategy<Value = OpaqueIndexSet>
where
    T: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn clamped_interval(max_length: usize) -> ops::RangeInclusive<usize> {
        if max_length == 0 {
            1..=1
        } else {
            1..=max_length
        }
    }

    clamped_interval(max_length).prop_flat_map(move |length| strategy_type_erased_index_set_len::<T, S, A>(length))
}

fn dedup_by_largest_index_per_key<K>(sorted_entries: &[(K, usize)]) -> Vec<(K, (usize, usize))>
where
    K: any::Any + Clone + Eq + hash::Hash,
{
    let mut deduped_sorted_entries = Vec::new();
    let mut iterator = sorted_entries.iter().peekable();
    while let Some((key, index)) = iterator.next().cloned() {
        let smallest_index = index;
        let mut largest_index = index;
        while let Some((next_key, next_index)) = iterator.peek() {
            if *next_key == key {
                largest_index = *next_index;
                iterator.next();
            } else {
                break;
            }
        }

        deduped_sorted_entries.push((key.clone(), (smallest_index, largest_index)));
    }

    deduped_sorted_entries
}

fn first_and_last_index_per_key<T>(entries: &[T]) -> Vec<(T, (usize, usize))>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash,
{
    let sorted_entries = {
        let mut _sorted_entries: Vec<(T, usize)> = entries.iter().cloned().enumerate().map(|(index, key)| (key, index)).collect();
        _sorted_entries.sort();
        _sorted_entries
    };

    dedup_by_largest_index_per_key(&sorted_entries)
}

pub fn last_entry_per_key<T>(entries: &[T]) -> Vec<(T, (usize, usize))>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash,
{
    let indices = first_and_last_index_per_key(entries);
    let mut result = Vec::new();
    for (key, index_tuple) in indices.iter() {
        let key_value_tuple = entries[index_tuple.1].clone();
        result.push((key_value_tuple, *index_tuple));
    }

    result
}

pub fn last_entry_per_key_ordered<T>(entries: &[T]) -> Vec<T>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash,
{
    let mut filtered_entries = last_entry_per_key(entries);
    filtered_entries.sort_by(|a, b| a.1.0.cmp(&b.1.0));
    let result = filtered_entries.iter().cloned().map(|entry| entry.0).collect();

    result
}
