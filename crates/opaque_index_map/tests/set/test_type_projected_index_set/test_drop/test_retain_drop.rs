use opaque_index_map::set::{Slice, TypeProjectedIndexSet};

use core::any;
use std::hash;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[derive(Clone, Debug, PartialEq, Eq)]
struct DropCounter {
    count: Rc<RefCell<usize>>,
}

impl DropCounter {
    #[inline]
    const fn new(count: Rc<RefCell<usize>>) -> Self {
        Self { count }
    }

    fn increment(&mut self) {
        *self.count.borrow_mut() += 1;
    }

    fn drop_count(&self) -> usize {
        self.count.borrow().clone()
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.increment();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct UnhashedValueWrapper<T> {
    index: usize,
    value: T,
}

impl<T> UnhashedValueWrapper<T> {
    #[inline]
    const fn new(index: usize, value: T) -> Self { Self { index, value, }}
}

impl<T> hash::Hash for UnhashedValueWrapper<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

fn create_drop_counter_index_set_in<S, A>(len: usize, build_hasher: S, alloc: A) -> (DropCounter, TypeProjectedIndexSet<UnhashedValueWrapper<DropCounter>, S, A>)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let drop_counter = DropCounter::new(Rc::new(RefCell::new(0)));
    let mut set = TypeProjectedIndexSet::with_capacity_and_hasher_in(
        len,
        build_hasher,
        alloc,
    );
    for i in 0..len {
        set.insert(UnhashedValueWrapper::new(i, drop_counter.clone()));
    }

    (drop_counter, set)
}

fn run_test_type_projected_index_set_retain_true<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut set) = create_drop_counter_index_set_in(length, build_hasher, alloc);
    let expected = 0;

    set.retain(|_| true);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_projected_index_set_retain_false<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut set) = create_drop_counter_index_set_in(length, build_hasher, alloc);
    let expected = set.len();

    set.retain(|_| false);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_projected_index_set_retain_even<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn count_even_indices(slice: &Slice<UnhashedValueWrapper<DropCounter>>) -> usize {
        slice.iter().fold(0, |acc, v| { if v.index % 2 == 0 { acc + 1 } else { acc } })
    }

    let (drop_counter, mut set) = create_drop_counter_index_set_in(length, build_hasher, alloc);
    let expected = count_even_indices(set.as_slice());

    set.retain(|v| v.index % 2 != 0);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_projected_index_set_retain_odd<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn count_odd_indices(slice: &Slice<UnhashedValueWrapper<DropCounter>>) -> usize {
        slice.iter().fold(0, |acc, v| { if v.index % 2 != 0 { acc + 1 } else { acc } })
    }

    let (drop_counter, mut set) = create_drop_counter_index_set_in(length, build_hasher, alloc);
    let expected = count_odd_indices(set.as_slice());

    set.retain(|v| v.index % 2 == 0);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_retain_true_range() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_index_set_retain_true(length, build_hasher.clone(), alloc.clone());
    }
}

#[test]
fn test_type_projected_index_set_retain_false_range() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_index_set_retain_false(length, build_hasher.clone(), alloc.clone());
    }
}

#[test]
fn test_type_projected_index_set_retain_even_range() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_index_set_retain_even(length, build_hasher.clone(), alloc.clone());
    }
}

#[test]
fn test_type_projected_index_set_retain_odd_range() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_index_set_retain_odd(length, build_hasher.clone(), alloc.clone());
    }
}
