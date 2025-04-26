use core::{fmt, hash};
use std::{marker, ops};
use opaque_vec::OpaqueVec;

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

