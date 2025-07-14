use opaque_index_map::set::TypeErasedIndexSet;

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

fn create_drop_counter_index_set_in<S, A>(len: usize, build_hasher: S, alloc: A) -> (DropCounter, TypeErasedIndexSet)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let drop_counter = DropCounter::new(Rc::new(RefCell::new(0)));
    let mut set = TypeErasedIndexSet::with_capacity_and_hasher_in::<UnhashedValueWrapper<DropCounter>, S, A>(
        len,
        build_hasher,
        alloc,
    );
    for i in 0..len {
        set.insert::<UnhashedValueWrapper<DropCounter>, S, A>(UnhashedValueWrapper::new(i, drop_counter.clone()));
    }

    (drop_counter, set)
}

fn run_test_type_projected_index_map_into_iter_drop<S, A>(length: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, set) = create_drop_counter_index_set_in(length, build_hasher, alloc);
    let _ = set.into_iter::<UnhashedValueWrapper<DropCounter>, S, A>();

    let expected = length;
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_projected_vec_into_iter_take_then_drop<S, A>(length: usize, take_count: usize, build_hasher: S, alloc: A)
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, set) = create_drop_counter_index_set_in(
        length,
        build_hasher.clone(),
        alloc.clone(),
    );
    let mut taken_set = TypeErasedIndexSet::with_capacity_and_hasher_in::<UnhashedValueWrapper<DropCounter>, S, A>(
        take_count,
        build_hasher.clone(),
        alloc.clone(),
    );
    {
        let mut iterator = set.into_iter::<UnhashedValueWrapper<DropCounter>, S, A>();

        assert_eq!(drop_counter.drop_count(), 0);

        let mut i = 0;
        while i < take_count {
            let value = iterator.next().unwrap();
            taken_set.insert::<UnhashedValueWrapper<DropCounter>, S, A>(value);
            i += 1;
        }
    }

    let expected = length - take_count;
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_vec_into_iter_drop() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        for take_count in 0..length {
            run_test_type_projected_index_map_into_iter_drop(length, build_hasher.clone(), alloc.clone());
        }
    }
}

#[test]
fn test_type_projected_vec_into_iter_take_then_drop() {
    let max_length = 128;
    let build_hasher = hash::RandomState::default();
    let alloc = alloc::Global;
    for length in 0..max_length {
        for take_count in 0..length {
            run_test_type_projected_vec_into_iter_take_then_drop(length, take_count, build_hasher.clone(), alloc.clone());
        }
    }
}
