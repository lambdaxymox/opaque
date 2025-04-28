use core::{fmt, hash};
use std::ops;

pub struct PrefixGenerator<K, V> {
    current_index: usize,
    values: Vec<(K, V)>,
}

impl<K, V> PrefixGenerator<K, V> {
    #[inline]
    const fn new(values: Vec<(K, V)>) -> Self {
        Self {
            current_index: 0,
            values,
        }
    }
}

impl<K, V> Iterator for PrefixGenerator<K, V>
where
    K: Clone + Eq + hash::Hash + 'static,
    V: Clone + Eq + 'static,
{
    type Item = Vec<(K, V)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.values.len() {
            return None;
        }

        let prefix = Vec::from(&self.values[..self.current_index]);
        self.current_index += 1;

        Some(prefix)
    }
}

pub fn key_value_pairs<K, V, I, J>(keys: I, values: J) -> PrefixGenerator<K, V>
where
    K: Clone + Eq + hash::Hash + 'static,
    V: Clone + Eq + 'static,
    I: Iterator<Item = K>,
    J: Iterator<Item = V>,
{
    let values = Vec::from_iter(keys.zip(values));
    PrefixGenerator::new(values)
}

pub fn range_entries<K, V>(keys: ops::RangeInclusive<K>, values: ops::RangeInclusive<V>) -> PrefixGenerator<K, V>
where
    K: Clone + Eq + hash::Hash + fmt::Debug  + 'static,
    V: Clone + Eq + fmt::Debug + 'static,
    ops::RangeInclusive<K>: DoubleEndedIterator<Item = K>,
    ops::RangeInclusive<V>: DoubleEndedIterator<Item = V>,
{
    key_value_pairs(keys, values)
}

pub fn constant_key_entries<K, V>(key: K, values: ops::RangeInclusive<V>) -> PrefixGenerator<K, V>
where
    K: Clone + Eq + hash::Hash + fmt::Debug  + 'static,
    V: Clone + Eq + fmt::Debug + 'static,
    ops::RangeInclusive<V>: DoubleEndedIterator<Item = V>,
{
    let keys = core::iter::repeat(key).take(values.clone().count());
    key_value_pairs(keys, values)
}

fn dedup_by_largest_index_per_key<K>(sorted_entries: &[(K, usize)]) -> Vec<(K, usize)>
where
    K: Clone + Eq + hash::Hash + 'static,
{
    let mut deduped_sorted_entries = Vec::new();
    let mut iter = sorted_entries.iter().peekable();
    while let Some((key, index)) = iter.next() {
        let mut largest_idx = *index;
        while let Some((next_key, next_index)) = iter.peek() {
            if *next_key == *key {
                largest_idx = *next_index;
                iter.next();
            } else {
                break;
            }
        }
        deduped_sorted_entries.push((key.clone(), largest_idx));
    }

    deduped_sorted_entries
}

fn last_index_per_key<K, V>(entries: &[(K, V)]) -> Vec<(K, usize)>
where
    K: Clone + Eq + Ord + hash::Hash + 'static,
    V: Clone + Eq + 'static,
{
    let sorted_entries = {
        let mut _sorted_entries: Vec<(K, usize)> = entries
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, (key, _))| (key, index))
            .collect();
        _sorted_entries.sort();
        _sorted_entries
    };

    dedup_by_largest_index_per_key(&sorted_entries)
}

pub fn last_entry_per_key<K, V>(entries: &[(K, V)]) -> Vec<(K, V)>
where
    K: Clone + Eq + Ord + hash::Hash + 'static,
    V: Clone + Eq + 'static,
{
    let indices = crate::last_index_per_key(entries);
    let mut result = Vec::new();
    for (key, index) in indices.iter() {
        result.push(entries[*index].clone());
    }

    result
}
