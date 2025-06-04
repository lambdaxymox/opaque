use core::{
    any,
    fmt,
    hash,
};
use std::ops;

pub struct PrefixGenerator<'a, T> {
    current_index: usize,
    values: &'a [T],
}

impl<'a, T> PrefixGenerator<'a, T> {
    #[inline]
    pub const fn new(values: &'a [T]) -> Self {
        Self { current_index: 0, values }
    }
}

impl<'a, T> Iterator for PrefixGenerator<'a, T>
where
    T: any::Any + Clone + Eq + hash::Hash,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.values.len() {
            return None;
        }

        let prefix = &self.values[..self.current_index];
        self.current_index += 1;

        Some(prefix)
    }
}

#[derive(Clone)]
pub struct RangeEntriesSpec<T> {
    values: ops::RangeInclusive<T>,
}

impl<T> RangeEntriesSpec<T> {
    #[inline]
    pub const fn new(values: ops::RangeInclusive<T>) -> Self {
        Self { values, }
    }
}

pub fn values<T, I>(values: I) -> Vec<T>
where
    T: any::Any + Clone + Eq + hash::Hash,
    I: Iterator<Item = T>,
{
    Vec::from_iter(values)
}

pub fn range_entries<T>(spec: RangeEntriesSpec<T>) -> Vec<T>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    ops::RangeInclusive<T>: DoubleEndedIterator<Item = T>,
{
    Vec::from_iter(spec.values)
}
