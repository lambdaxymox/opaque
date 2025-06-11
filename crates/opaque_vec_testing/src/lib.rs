#![deny(unsafe_op_in_unsafe_fn)]
#![deny(private_interfaces)]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

use core::any;
use core::fmt;
use core::mem::MaybeUninit;
use alloc_crate::boxed::Box;
use std::string::{String, ToString};

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
    T: any::Any + Clone + PartialEq,
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

pub struct RangeValuesSpec<T> {
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T> RangeValuesSpec<T> {
    #[inline]
    pub fn new(iter: Box<dyn Iterator<Item = T>>) -> Self {
        Self { iter, }
    }
}

pub fn range_values<T, const N: usize>(mut spec: RangeValuesSpec<T>) -> [T; N]
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
{
    let mut array: [MaybeUninit<T>; N] = unsafe {  MaybeUninit::uninit().assume_init() };
    for i in 0..N {
        array[i] = MaybeUninit::new(T::default().clone());
    }

    // SAFETY: Transmuting the array is safe because all the entries in the array were initialized
    // to `T::default()`.
    let mut array: [T; N] = unsafe { std::mem::transmute_copy(&array) };
    for i in 0..N {
        array[i] = spec.iter.next().unwrap();
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
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
{
    let mut array: [MaybeUninit<T>; N] = unsafe {  MaybeUninit::uninit().assume_init() };
    for i in 0..N {
        array[i] = MaybeUninit::new(T::default().clone());
    }

    // SAFETY: Transmuting the array is safe because all the entries in the array were initialized
    // to `T::default()`.
    let mut array: [T; N] = unsafe { std::mem::transmute_copy(&array) };
    for i in 0..N {
        let value = if i % 2 == 0 { spec.this.clone() } else { spec.that.clone() };
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
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
{
    let mut array: [MaybeUninit<T>; N] = unsafe {  MaybeUninit::uninit().assume_init() };
    for i in 0..N {
        array[i] = MaybeUninit::new(T::default().clone());
    }

    // SAFETY: Transmuting the array is safe because all the entries in the array were initialized
    // to `T::default()`.
    let mut array: [T; N] = unsafe { std::mem::transmute_copy(&array) };
    for i in 0..N {
        array[i] = spec.constant.clone();
    }

    array
}


#[derive(Clone)]
pub struct RepeatableValuesSpec<T, const N: usize> {
    values: [T; N],
}

impl<T, const N: usize> RepeatableValuesSpec<T, N> {
    #[inline]
    pub const fn new(values: [T; N]) -> Self {
        Self { values }
    }
}

pub fn repeat_values<T, const N: usize, const M: usize>(spec: RepeatableValuesSpec<T, N>) -> [T; M]
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
{
    let mut array: [MaybeUninit<T>; M] = unsafe {  MaybeUninit::uninit().assume_init() };
    for i in 0..M {
        array[i] = MaybeUninit::new(T::default().clone());
    }

    // SAFETY: Transmuting the array is safe because all the entries in the array were initialized
    // to `T::default()`.
    let mut array: [T; M] = unsafe { std::mem::transmute_copy(&array) };
    for i in 0..M {
        array[i] = spec.values[i % N].clone();
    }

    array
}
