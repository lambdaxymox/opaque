extern crate core;

use core::{fmt, hash};
use core::ops;

pub struct PrefixGenerator<'a, T> {
    current_index: usize,
    values: &'a [T],
}

impl<'a, T> PrefixGenerator<'a, T> {
    #[inline]
    pub const fn new_with_start(values: &'a [T], start_len: usize) -> Self {
        Self {
            current_index: start_len,
            values,
        }
    }

    #[inline]
    pub const fn new(values: &'a [T]) -> Self {
        Self::new_with_start(values, 0)
    }

    #[inline]
    pub const fn new_only_nonempty(values: &'a [T]) -> Self {
        Self::new_with_start(values, 1)
    }
}

impl<'a, T> Iterator for PrefixGenerator<'a, T>
where
    T: Clone + PartialEq + 'static,
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
pub struct RangeValuesSpec<T> {
    start: T,
}

impl<T> RangeValuesSpec<T> {
    #[inline]
    pub const fn new(start: T) -> Self {
        Self { start }
    }
}

pub fn range_values<T, const N: usize>(spec: RangeValuesSpec<T>) -> [T; N]
where
    T: Copy + PartialEq + Clone + fmt::Debug + TryFrom<usize> + ops::Add<Output = T> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    let mut array = [spec.start; N];
    for i in 0..N {
        array[i] = spec.start + T::try_from(i).unwrap();
    }

    array
}

#[derive(Clone)]
pub struct AlternatingValuesSpec<T> {
    this: T,
    that: T,
}

impl<T> AlternatingValuesSpec<T> {
    #[inline]
    pub const fn new(this: T, that: T) -> Self {
        Self { this, that }
    }
}

pub fn alternating_values<T, const N: usize>(spec: AlternatingValuesSpec<T>) -> [T; N]
where
    T: Copy + PartialEq + Clone + fmt::Debug + TryFrom<usize> + ops::Add<Output = T> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    let mut array = [spec.this; N];
    for i in 0..N {
        let value = if i % 2 == 0 { spec.this } else { spec.that };
        array[i] = value;
    }

    array
}

#[derive(Clone)]
pub struct ConstantValuesSpec<T> {
    constant: T,
}

impl<T> ConstantValuesSpec<T> {
    #[inline]
    pub const fn new(constant: T) -> Self {
        Self { constant }
    }
}

pub fn constant_values<T, const N: usize>(spec: ConstantValuesSpec<T>) -> [T; N]
where
    T: Copy + PartialEq + Clone + fmt::Debug + TryFrom<usize> + ops::Add<Output = T> + 'static,
    <T as TryFrom<usize>>::Error: fmt::Debug,
{
    [spec.constant; N]
}
