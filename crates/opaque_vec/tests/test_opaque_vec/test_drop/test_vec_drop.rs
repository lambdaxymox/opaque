use opaque_vec::OpaqueVec;

use std::cell::RefCell;
use std::rc::Rc;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[derive(Clone)]
struct DropCounter {
    count: Rc<RefCell<u32>>,
}

impl DropCounter {
    #[inline]
    const fn new(count: Rc<RefCell<u32>>) -> Self {
        Self { count }
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        *self.count.borrow_mut() += 1;
    }
}

#[test]
fn test_opaque_vec_double_drop() {
    struct TwoOpaqueVec {
        x: OpaqueVec,
        y: OpaqueVec,
    }

    let (ref_count_x, ref_count_y) = (Rc::new(RefCell::new(0)), Rc::new(RefCell::new(0)));
    {
        let mut tv = TwoOpaqueVec {
            x: OpaqueVec::new::<DropCounter>(),
            y: OpaqueVec::new::<DropCounter>(),
        };

        tv.x.push::<DropCounter, alloc::Global>(DropCounter::new(ref_count_x.clone()));
        tv.y.push::<DropCounter, alloc::Global>(DropCounter::new(ref_count_y.clone()));

        drop(tv.x);
    }

    let count_x = *ref_count_x.borrow();
    let count_y = *ref_count_y.borrow();

    assert_eq!(count_x, 1);
    assert_eq!(count_y, 1);
}

#[test]
fn test_opaque_vec_drop_all_items1() {
    let count = 1024;
    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());
    {
        let mut vec = OpaqueVec::new::<DropCounter>();
        for i in 0..count {
            vec.push::<DropCounter, alloc::Global>(counter.clone());
        }
    }

    let expected = count;
    let result = *ref_count.borrow();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_push_should_not_drop_value() {
    let mut vec = OpaqueVec::new::<DropCounter>();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.push::<DropCounter, alloc::Global>(counter);

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_opaque_vec_replace_insert_should_not_drop_value() {
    let mut vec = OpaqueVec::new::<DropCounter>();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.replace_insert::<DropCounter, alloc::Global>(0, counter);

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_opaque_vec_swap_remove_should_not_drop_return_value() {
    let mut vec = OpaqueVec::new::<DropCounter>();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.push::<DropCounter, alloc::Global>(counter);

    let _counter = vec.swap_remove::<DropCounter, alloc::Global>(0);

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_opaque_vec_shift_remove_should_not_drop_return_value() {
    let mut vec = OpaqueVec::new::<DropCounter>();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.push::<DropCounter, alloc::Global>(counter);

    let _counter = vec.shift_remove::<DropCounter, alloc::Global>(0);

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_opaque_vec_pop_should_not_drop_return_value() {
    let mut vec = OpaqueVec::new::<DropCounter>();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.push::<DropCounter, alloc::Global>(counter);

    let _counter = vec.pop::<DropCounter, alloc::Global>();

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_opaque_vec_clear_should_drop() {
    let mut vec = OpaqueVec::new::<DropCounter>();

    let ref_count = Rc::new(RefCell::new(0));
    let counter = DropCounter::new(ref_count.clone());

    vec.push::<DropCounter, alloc::Global>(counter);

    vec.clear::<DropCounter, alloc::Global>();

    let count = *ref_count.borrow();

    assert_eq!(count, 1);
}

#[test]
fn test_opaque_vec_drop_should_drop_elements() {
    let ref_count = Rc::new(RefCell::new(0));
    {
        let mut vec = OpaqueVec::new::<DropCounter>();

        let counter = DropCounter::new(ref_count.clone());

        vec.push::<DropCounter, alloc::Global>(counter);
        // `vec` drops here.
    }

    let count = *ref_count.borrow();

    assert_eq!(count, 1);
}

#[test]
fn test_opaque_vec_drop_should_not_drop_swap_removed_elements() {
    let ref_count = Rc::new(RefCell::new(0));
    let _counter = {
        let mut vec = OpaqueVec::new::<DropCounter>();

        vec.push::<DropCounter, alloc::Global>(DropCounter::new(ref_count.clone()));

        let counter = vec.swap_remove::<DropCounter, alloc::Global>(0);

        counter
        // `vec` drops here.
    };

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_opaque_vec_drop_should_not_drop_shift_removed_elements() {
    let ref_count = Rc::new(RefCell::new(0));
    let _counter = {
        let mut vec = OpaqueVec::new::<DropCounter>();

        vec.push::<DropCounter, alloc::Global>(DropCounter::new(ref_count.clone()));

        let counter = vec.shift_remove::<DropCounter, alloc::Global>(0);

        counter
        // `vec` drops here.
    };

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}

#[test]
fn test_opaque_vec_drop_should_not_drop_popped_elements() {
    let ref_count = Rc::new(RefCell::new(0));
    let _counter = {
        let mut vec = OpaqueVec::new::<DropCounter>();

        vec.push::<DropCounter, alloc::Global>(DropCounter::new(ref_count.clone()));

        let counter = vec.pop::<DropCounter, alloc::Global>();

        counter
        // `vec` drops here.
    };

    let count = *ref_count.borrow();

    assert_eq!(count, 0);
}
