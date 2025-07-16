use opaque_vec::TypeProjectedVec;

use core::any;
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

fn create_drop_counter_vec_in<A>(len: usize, alloc: A) -> (DropCounter, TypeProjectedVec<(usize, DropCounter), A>)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let drop_counter = DropCounter::new(Rc::new(RefCell::new(0)));
    let mut vec = TypeProjectedVec::with_capacity_in(len, alloc);
    for i in 0..len {
        vec.push((i, drop_counter.clone()));
    }

    (drop_counter, vec)
}

fn run_test_type_projected_vec_retain_true<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut vec) = create_drop_counter_vec_in(length, alloc);
    let expected = 0;

    vec.retain(|_| true);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_projected_vec_retain_false<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut vec) = create_drop_counter_vec_in(length, alloc);
    let expected = vec.len();

    vec.retain(|_| false);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_projected_vec_retain_even<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn count_even_indices(slice: &[(usize, DropCounter)]) -> usize {
        slice.iter().fold(0, |acc, (i, _)| if i % 2 == 0 { acc + 1 } else { acc })
    }

    let (drop_counter, mut vec) = create_drop_counter_vec_in(length, alloc);
    let expected = count_even_indices(vec.as_slice());

    vec.retain(|(i, _)| i % 2 != 0);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_projected_vec_retain_odd<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn count_odd_indices(slice: &[(usize, DropCounter)]) -> usize {
        slice.iter().fold(0, |acc, (i, _)| if i % 2 != 0 { acc + 1 } else { acc })
    }

    let (drop_counter, mut vec) = create_drop_counter_vec_in(length, alloc);
    let expected = count_odd_indices(vec.as_slice());

    vec.retain(|(i, _)| i % 2 == 0);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_vec_retain_true_range() {
    let max_length = 128;
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_vec_retain_true(length, alloc.clone());
    }
}

#[test]
fn test_type_projected_vec_retain_false_range() {
    let max_length = 128;
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_vec_retain_false(length, alloc.clone());
    }
}

#[test]
fn test_type_projected_vec_retain_even_range() {
    let max_length = 128;
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_vec_retain_even(length, alloc.clone());
    }
}

#[test]
fn test_type_projected_vec_retain_odd_range() {
    let max_length = 128;
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_projected_vec_retain_odd(length, alloc.clone());
    }
}
