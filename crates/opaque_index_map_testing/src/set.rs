use core::any;
use core::fmt;
use core::hash;
use core::ops;
use alloc_crate::vec::Vec;
use alloc_crate::boxed::Box;
use alloc_crate::string::{ToString, String};

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
pub struct StringRangeFrom {
    start: isize,
    current: isize,
}

impl StringRangeFrom {
    #[inline]
    pub const fn new(start: isize) -> Self {
        Self { start, current: start, }
    }
}

impl Iterator for StringRangeFrom {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current.to_string();
        self.current += 1;

        Some(result)
    }
}

pub struct RangeEntriesSpec<T> {
    values: Box<dyn Iterator<Item = T>>,
}

impl<T> RangeEntriesSpec<T> {
    #[inline]
    pub const fn new(values: Box<dyn Iterator<Item = T>>) -> Self {
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
