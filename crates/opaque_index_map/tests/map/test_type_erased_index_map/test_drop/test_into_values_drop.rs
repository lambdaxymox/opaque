use opaque_index_map::map::TypeErasedIndexMap;

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

fn create_drop_counter_index_map_in<S, A>(len: usize, build_hasher: S, alloc: A) -> (DropCounter, TypeErasedIndexMap)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let drop_counter = DropCounter::new(Rc::new(RefCell::new(0)));
    let mut map = TypeErasedIndexMap::with_capacity_and_hasher_in::<usize, DropCounter, S, A>(
        len,
        build_hasher,
        alloc,
    );
    for i in 0..len {
        map.insert::<usize, DropCounter, S, A>(i, drop_counter.clone());
    }

    (drop_counter, map)
}

fn run_test_type_projected_index_map_into_values_drop<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, map) = create_drop_counter_index_map_in(length, build_hasher, alloc);
    let _ = map.into_values::<usize, DropCounter, S, A>();

    let expected = length;
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_erased_index_map_into_values_take_then_drop<S, A>(length: usize, take_count: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, map) = create_drop_counter_index_map_in(
        length,
        build_hasher.clone(),
        alloc.clone(),
    );
    let mut taken_map = TypeErasedIndexMap::with_capacity_and_hasher_in::<usize, DropCounter, S, A>(
        take_count,
        build_hasher.clone(),
        alloc.clone(),
    );
    {
        let mut iterator = map.into_values::<usize, DropCounter, S, A>();

        assert_eq!(drop_counter.drop_count(), 0);

        let mut i = 0;
        while i < take_count {
            let value = iterator.next().unwrap();
            taken_map.insert::<usize, DropCounter, S, A>(i, value);
            i += 1;
        }
    }

    let expected = length - take_count;
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

#[test]
fn test_type_erased_index_map_into_values_drop() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        for take_count in 0..length {
            run_test_type_projected_index_map_into_values_drop(length, build_hasher.clone(), alloc.clone());
        }
    }
}

#[test]
fn test_type_erased_index_map_into_values_take_then_drop() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        for take_count in 0..length {
            run_test_type_erased_index_map_into_values_take_then_drop(length, take_count, build_hasher.clone(), alloc.clone());
        }
    }
}
