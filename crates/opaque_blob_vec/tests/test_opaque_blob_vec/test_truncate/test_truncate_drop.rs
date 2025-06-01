use crate::common;

use core::any;
use core::ptr::NonNull;
use std::alloc;
use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::rc::Rc;

use opaque_blob_vec::OpaqueBlobVec;

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

fn create_drop_counter_blob_vec_in<A>(len: usize, alloc: A) -> (DropCounter, OpaqueBlobVec)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default,
{
    let drop_counter = DropCounter::new(Rc::new(RefCell::new(0)));
    let mut vec = common::opaque_blob_vec::with_capacity_in::<DropCounter, A>(len, alloc);
    for i in 0..len {
        let ptr = unsafe {
            // SAFETY: The drop counter clone is stored inside the `vec`, so we can wrap it in ManuallyDrop to prevent the
            // drop call from happening, which would increment the drop counter and give us inaccurate drop counts.
            let cloned_drop_counter = drop_counter.clone();
            let me = ManuallyDrop::new(cloned_drop_counter);
            NonNull::new_unchecked(&*me as *const DropCounter as *mut DropCounter).cast::<u8>()
        };

        vec.push::<A>(ptr);
    }

    assert_eq!(drop_counter.drop_count(), 0);
    assert_eq!(vec.len(), len);

    (drop_counter, vec)
}

fn run_test_opaque_blob_vec_truncate_drop_to_zero_direct<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default,
{
    let (drop_counter, mut vec) = create_drop_counter_blob_vec_in(length, alloc);
    let expected = vec.len();
    vec.truncate::<A>(0);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_opaque_blob_vec_truncate_drop_to_zero_steps<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default,
{
    let (drop_counter, mut vec) = create_drop_counter_blob_vec_in(length, alloc);
    for i in 0..(length + 1) {
        vec.truncate::<A>(length - i);
        let expected = i;
        let result = drop_counter.drop_count();

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_blob_vec_truncate_drop_to_length<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default,
{
    let (drop_counter, mut vec) = create_drop_counter_blob_vec_in(length, alloc);
    let expected = 0;
    vec.truncate::<A>(length);
    let result = drop_counter.drop_count();

    assert_eq!(result, expected);
}

fn run_test_opaque_blob_vec_truncate_drop_to_above_length<A>(length: usize, alloc: A)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default,
{
    let (drop_counter, mut vec) = create_drop_counter_blob_vec_in(length, alloc);
    let expected = 0;
    for i in length..(3 * length) {
        vec.truncate::<A>(i);
        let result = drop_counter.drop_count();

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_blob_vec_truncate_drop_to_zero_direct_range<A>(max_length: usize)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default,
{
    let alloc = A::default();
    for length in 0..max_length {
        run_test_opaque_blob_vec_truncate_drop_to_zero_direct(length, alloc.clone());
    }
}

fn run_test_opaque_blob_vec_truncate_drop_to_zero_steps_range<A>(max_length: usize)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default,
{
    let alloc = A::default();
    for length in 0..max_length {
        run_test_opaque_blob_vec_truncate_drop_to_zero_steps(length, alloc.clone());
    }
}

fn run_test_opaque_blob_vec_truncate_drop_to_length_range<A>(max_length: usize)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default,
{
    let alloc = A::default();
    for length in 0..max_length {
        run_test_opaque_blob_vec_truncate_drop_to_length(length, alloc.clone());
    }
}

fn run_test_opaque_blob_vec_truncate_drop_to_above_length_range<A>(max_length: usize)
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default,
{
    let alloc = A::default();
    for length in 0..max_length {
        run_test_opaque_blob_vec_truncate_drop_to_above_length(length, alloc.clone());
    }
}

#[test]
fn test_opaque_blob_vec_truncate_drop_to_zero_direct_range() {
    let max_length = 128;
    run_test_opaque_blob_vec_truncate_drop_to_zero_direct_range::<alloc::Global>(max_length);
}

#[test]
fn test_opaque_blob_vec_truncate_drop_to_zero_steps_range() {
    let max_length = 128;
    run_test_opaque_blob_vec_truncate_drop_to_zero_steps_range::<alloc::Global>(max_length);
}

#[test]
fn test_opaque_blob_vec_truncate_drop_to_length() {
    let max_length = 128;
    run_test_opaque_blob_vec_truncate_drop_to_length_range::<alloc::Global>(max_length);
}

#[test]
fn test_opaque_blob_vec_truncate_drop_to_above_length() {
    let max_length = 128;
    run_test_opaque_blob_vec_truncate_drop_to_above_length_range::<alloc::Global>(max_length);
}
