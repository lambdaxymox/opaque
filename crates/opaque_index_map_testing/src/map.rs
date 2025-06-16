use core::any;
use core::fmt;
use core::hash;
use core::ops;
use alloc_crate::vec::Vec;
use alloc_crate::boxed::Box;
use alloc_crate::string::{ToString, String};

pub struct PrefixGenerator<'a, K, V> {
    current_index: usize,
    values: &'a [(K, V)],
}

impl<'a, K, V> PrefixGenerator<'a, K, V> {
    #[inline]
    pub const fn new(values: &'a [(K, V)]) -> Self {
        Self { current_index: 0, values }
    }
}

impl<'a, K, V> Iterator for PrefixGenerator<'a, K, V>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
{
    type Item = &'a [(K, V)];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.values.len() {
            return None;
        }

        let prefix = &self.values[..self.current_index];
        self.current_index += 1;

        Some(prefix)
    }
}

pub fn key_value_pairs<K, V, I, J>(keys: I, values: J) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    I: Iterator<Item = K>,
    J: Iterator<Item = V>,
{
    Vec::from_iter(keys.zip(values))
}

#[derive(Clone)]
pub struct StringRangeInclusive {
    range: ops::RangeInclusive<isize>,
}

impl StringRangeInclusive {
    #[inline]
    pub const fn new(range: ops::RangeInclusive<isize>) -> Self {
        Self { range, }
    }
}

impl Iterator for StringRangeInclusive {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|value| value.to_string())
    }
}

pub struct RangeEntriesSpec<K, V> {
    keys: Box<dyn Iterator<Item = K>>,
    values: Box<dyn Iterator<Item = V>>,
}

impl<K, V> RangeEntriesSpec<K, V> {
    #[inline]
    pub const fn new(keys: Box<dyn Iterator<Item = K>>, values: Box<dyn Iterator<Item = V>>) -> Self {
        Self { keys, values }
    }
}

pub fn range_entries<K, V>(spec: RangeEntriesSpec<K, V>) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    key_value_pairs(spec.keys, spec.values)
}

pub struct ConstantKeyEntriesSpec<K, V> {
    key: K,
    values: Box<dyn Iterator<Item = V>>,
}

impl<K, V> ConstantKeyEntriesSpec<K, V> {
    #[inline]
    pub const fn new(key: K, values: Box<dyn Iterator<Item = V>>) -> Self {
        Self { key, values }
    }
}

pub fn constant_key_entries<K, V>(spec: ConstantKeyEntriesSpec<K, V>) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
{
    let mut result = Vec::new();
    for value in spec.values {
        let key = spec.key.clone();
        result.push((key, value));
    }

    result
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

fn first_and_last_index_per_key<K, V>(entries: &[(K, V)]) -> Vec<(K, (usize, usize))>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let sorted_entries = {
        let mut _sorted_entries: Vec<(K, usize)> = entries.iter().cloned().enumerate().map(|(index, (key, _))| (key, index)).collect();
        _sorted_entries.sort();
        _sorted_entries
    };

    dedup_by_largest_index_per_key(&sorted_entries)
}

pub fn last_entry_per_key<K, V>(entries: &[(K, V)]) -> Vec<((K, V), (usize, usize))>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let indices = first_and_last_index_per_key(entries);
    let mut result = Vec::new();
    for (key, index_tuple) in indices.iter() {
        let key_value_tuple = entries[index_tuple.1].clone();
        result.push((key_value_tuple, *index_tuple));
    }

    result
}

pub fn last_entry_per_key_ordered<K, V>(entries: &[(K, V)]) -> Vec<(K, V)>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let mut filtered_entries = last_entry_per_key(entries);
    filtered_entries.sort_by(|a, b| a.1.0.cmp(&b.1.0));
    let result = filtered_entries.iter().cloned().map(|entry| entry.0).collect();

    result
}
