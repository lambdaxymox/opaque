use opaque_vec::TypeErasedVec;

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

fn create_drop_counter_vec_in<A>(len: usize, alloc: A) -> (DropCounter, TypeErasedVec)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let drop_counter = DropCounter::new(Rc::new(RefCell::new(0)));
    let mut vec = TypeErasedVec::with_capacity_in::<DropCounter, A>(len, alloc);
    for _ in 0..len {
        vec.push::<DropCounter, A>(drop_counter.clone());
    }

    (drop_counter, vec)
}

fn run_test_type_erased_vec_into_iter_drop<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, vec) = create_drop_counter_vec_in(length, alloc);
    let _ = vec.into_iter::<DropCounter, A>();

    let expected = length;
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_type_erased_vec_into_iter_take_then_drop<A>(length: usize, take_count: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let (drop_counter, vec) = create_drop_counter_vec_in(length, alloc.clone());
    let mut taken_vec = TypeErasedVec::with_capacity_in::<Option<DropCounter>, A>(take_count, alloc.clone());
    {
        let mut iterator = vec.into_iter::<DropCounter, A>();

        assert_eq!(drop_counter.drop_count(), 0);

        let mut i = 0;
        while i < take_count {
            taken_vec.push::<Option<DropCounter>, A>(iterator.next());
            i += 1;
        }
    }

    let expected = length - take_count;
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

#[test]
fn test_type_erased_vec_into_iter_drop() {
    let max_length = 128;
    let alloc = alloc::Global;
    for length in 0..max_length {
        run_test_type_erased_vec_into_iter_drop(length, alloc.clone());
    }
}

#[test]
fn test_type_erased_vec_into_iter_take_then_drop() {
    let max_length = 128;
    let alloc = alloc::Global;
    for length in 0..max_length {
        for take_count in 0..length {
            run_test_type_erased_vec_into_iter_take_then_drop(length, take_count, alloc.clone());
        }
    }
}
