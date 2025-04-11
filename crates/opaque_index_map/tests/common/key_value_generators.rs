use core::{fmt, hash};
use std::ops;
use opaque_vec::OpaqueVec;

pub fn key_value_pairs<K, V, I, J>(keys: I, values: J) -> OpaqueVec
where
    K: Clone + Eq + hash::Hash + 'static,
    V: Clone + Eq + 'static,
    I: Iterator<Item = K>,
    J: Iterator<Item = V>,
{
    OpaqueVec::from_iter(keys.zip(values))
}

pub fn entries<K, V>(keys: ops::RangeInclusive<K>, values: ops::RangeInclusive<V>) -> OpaqueVec
where
    K: Clone + Eq + hash::Hash + fmt::Debug  + 'static,
    V: Clone + Eq + fmt::Debug + 'static,
    ops::RangeInclusive<K>: DoubleEndedIterator<Item = K>,
    ops::RangeInclusive<V>: DoubleEndedIterator<Item = V>,
{
    key_value_pairs(keys, values)
}
