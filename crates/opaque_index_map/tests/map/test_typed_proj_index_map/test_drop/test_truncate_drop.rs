use opaque_index_map::map::TypedProjIndexMap;

use core::any;
use std::hash;
use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[derive(Clone, Debug)]
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

fn create_drop_counter_index_map_in<S, A>(len: usize, build_hasher: S, alloc: A) -> (DropCounter, TypedProjIndexMap<usize, DropCounter, S, A>)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let drop_counter = DropCounter::new(Rc::new(RefCell::new(0)));
    let mut map = TypedProjIndexMap::with_capacity_and_hasher_in(len, build_hasher, alloc);
    for i in 0..len {
        map.insert(i, drop_counter.clone());
    }

    (drop_counter, map)
}

fn run_test_typed_proj_index_map_truncate_drop_to_zero_direct<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut map) = create_drop_counter_index_map_in(length, build_hasher, alloc);
    let expected = map.len();
    map.truncate(0);

    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_typed_proj_index_map_truncate_drop_to_zero_steps<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut map) = create_drop_counter_index_map_in(length, build_hasher, alloc);
    for i in 0..(length + 1) {
        map.truncate(length - i);
        let expected = i;
        let result = drop_counter.drop_count();

        assert_eq!(result, expected);
    }
}

fn run_test_typed_proj_index_map_truncate_drop_to_length<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut map) = create_drop_counter_index_map_in(length, build_hasher, alloc);
    let expected = 0;
    map.truncate(length);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_typed_proj_index_map_truncate_drop_to_above_length<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut map) = create_drop_counter_index_map_in(length, build_hasher, alloc);
    let expected = 0;
    for i in length..(3 * length) {
        map.truncate(i);
        let result = drop_counter.drop_count();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_typed_proj_index_map_truncate_drop_to_zero_direct_range() {
    let max_length = 128;
    let build_hasher = hash::RandomState::new();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_typed_proj_index_map_truncate_drop_to_zero_direct(length, build_hasher.clone(), alloc.clone());
    }
}

#[test]
fn test_typed_proj_index_map_truncate_drop_to_zero_steps_range() {
    let max_length = 128;
    let build_hasher = hash::RandomState::new();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_typed_proj_index_map_truncate_drop_to_zero_steps(length, build_hasher.clone(), alloc.clone());
    }
}

#[test]
fn test_typed_proj_index_map_truncate_drop_to_length() {
    let max_length = 128;
    let build_hasher = hash::RandomState::new();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_typed_proj_index_map_truncate_drop_to_length(length, build_hasher.clone(), alloc.clone());
    }
}

#[test]
fn test_typed_proj_index_map_truncate_drop_to_above_length() {
    let max_length = 128;
    let build_hasher = hash::RandomState::new();
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_typed_proj_index_map_truncate_drop_to_above_length(length, build_hasher.clone(), alloc.clone());
    }
}
