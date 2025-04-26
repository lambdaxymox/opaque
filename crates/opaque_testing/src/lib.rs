use core::fmt;
use core::ops;
use core::hash;

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

