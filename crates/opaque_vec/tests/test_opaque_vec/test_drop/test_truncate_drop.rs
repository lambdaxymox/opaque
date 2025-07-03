use opaque_vec::OpaqueVec;

use core::any;
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

fn create_drop_counter_vec_in<A>(len: usize, alloc: A) -> (DropCounter, OpaqueVec)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let drop_counter = DropCounter::new(Rc::new(RefCell::new(0)));
    let mut vec = OpaqueVec::with_capacity_in::<DropCounter, A>(len, alloc);
    for i in 0..len {
        vec.push::<DropCounter, A>(drop_counter.clone());
    }

    (drop_counter, vec)
}

fn run_test_opaque_vec_truncate_drop_to_zero_direct<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut vec) = create_drop_counter_vec_in(length, alloc);
    let expected = vec.len();
    vec.truncate::<DropCounter, A>(0);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_truncate_drop_to_zero_steps<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut vec) = create_drop_counter_vec_in(length, alloc);
    for i in 0..(length + 1) {
        vec.truncate::<DropCounter, A>(length - i);
        let expected = i;
        let result = drop_counter.drop_count();

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_vec_truncate_drop_to_length<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut vec) = create_drop_counter_vec_in(length, alloc);
    let expected = 0;
    vec.truncate::<DropCounter, A>(length);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_truncate_drop_to_above_length<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, mut vec) = create_drop_counter_vec_in(length, alloc);
    let expected = 0;
    for i in length..(3 * length) {
        vec.truncate::<DropCounter, A>(i);
        let result = drop_counter.drop_count();

        assert_eq!(result, expected);
    }
}

#[test]
fn test_opaque_vec_truncate_drop_to_zero_direct_range() {
    let max_length = 128;
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_opaque_vec_truncate_drop_to_zero_direct(length, alloc.clone());
    }
}

#[test]
fn test_opaque_vec_truncate_drop_to_zero_steps_range() {
    let max_length = 128;
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_opaque_vec_truncate_drop_to_zero_steps(length, alloc.clone());
    }
}

#[test]
fn test_opaque_vec_truncate_drop_to_length() {
    let max_length = 128;
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_opaque_vec_truncate_drop_to_length(length, alloc.clone());
    }
}

#[test]
fn test_opaque_vec_truncate_drop_to_above_length() {
    let max_length = 128;
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_opaque_vec_truncate_drop_to_above_length(length, alloc.clone());
    }
}
