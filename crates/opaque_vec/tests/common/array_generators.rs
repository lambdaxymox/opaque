use core::fmt;
use core::ops;

#[derive(Clone)]
pub struct RangeValuesSpec<T> {
    start: T,
}

impl<T> RangeValuesSpec<T> {
    #[inline]
    pub const fn new(start: T) -> Self {
        Self { start, }
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
        Self {
            this,
            that,
        }
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
