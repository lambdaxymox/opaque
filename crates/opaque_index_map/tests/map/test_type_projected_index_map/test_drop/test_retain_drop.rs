use opaque_index_map::map::{
    Slice,
    TypeProjectedIndexMap,
};

use core::any;
use std::cell::RefCell;
use std::hash;
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

fn create_drop_counter_index_map_in<S, A>(
    len: usize,
    build_hasher: S,
    alloc: A,
) -> (DropCounter, TypeProjectedIndexMap<usize, DropCounter, S, A>)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let drop_counter = DropCounter::new(Rc::new(RefCell::new(0)));
    let mut map = TypeProjectedIndexMap::with_capacity_and_hasher_in(len, build_hasher, alloc);
    for i in 0..len {
        map.insert(i, drop_counter.clone());
    }

    (drop_counter, map)
}

fn run_test_type_projected_index_map_retain_true<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut map) = create_drop_counter_index_map_in(length, build_hasher, alloc);
    let expected = 0;

    map.retain(|_, _| true);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_projected_index_map_retain_false<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut map) = create_drop_counter_index_map_in(length, build_hasher, alloc);
    let expected = map.len();

    map.retain(|_, _| false);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_projected_index_map_retain_even<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn count_even_indices(slice: &Slice<usize, DropCounter>) -> usize {
        slice.iter().fold(0, |acc, (i, _)| if i % 2 == 0 { acc + 1 } else { acc })
    }

    let (drop_counter, mut map) = create_drop_counter_index_map_in(length, build_hasher, alloc);
    let expected = count_even_indices(map.as_slice());

    map.retain(|k, _| k % 2 != 0);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_projected_index_map_retain_odd<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn count_odd_indices(slice: &Slice<usize, DropCounter>) -> usize {
        slice.iter().fold(0, |acc, (i, _)| if i % 2 != 0 { acc + 1 } else { acc })
    }

    let (drop_counter, mut map) = create_drop_counter_index_map_in(length, build_hasher, alloc);
    let expected = count_odd_indices(map.as_slice());

    map.retain(|i, _| i % 2 == 0);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_retain_true_range() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_index_map_retain_true(length, build_hasher.clone(), alloc.clone());
    }
}

#[test]
fn test_type_projected_index_map_retain_false_range() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_index_map_retain_false(length, build_hasher.clone(), alloc.clone());
    }
}

#[test]
fn test_type_projected_index_map_retain_even_range() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_index_map_retain_even(length, build_hasher.clone(), alloc.clone());
    }
}

#[test]
fn test_type_projected_index_map_retain_odd_range() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_index_map_retain_odd(length, build_hasher.clone(), alloc.clone());
    }
}
